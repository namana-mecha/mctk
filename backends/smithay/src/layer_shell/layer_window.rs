use layer_surface::{LayerOptions, LayerShellSctkWindow};
use mctk_core::component::{self, Component, RootComponent};
use mctk_core::input::{Button, Input, Motion, MouseButton, TouchAction};
use mctk_core::raw_handle::RawWaylandHandle;
use mctk_core::reexports::cosmic_text;
use mctk_core::types::AssetParams;
use mctk_core::types::PixelSize;
use mctk_core::ui::UI;
use pointer::{MouseEvent, ScrollDelta};
use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
};
use smithay_client_toolkit::reexports::calloop::channel::{Channel, Event, Sender};
use smithay_client_toolkit::reexports::calloop::{self, EventLoop};
use std::any::Any;
use std::collections::HashMap;

use crate::input::keyboard::{keysym_to_key, keysym_to_text, KeyboardEvent};
use crate::input::touch::TouchEvent;
use crate::WindowInfo;
use crate::{
    input::pointer, layer_shell::layer_surface, WindowEvent, WindowMessage, WindowOptions,
};

pub struct LayerWindow {
    width: u32,
    height: u32,
    scale_factor: f32,
    handle: Option<RawWaylandHandle>,
    window_tx: Sender<WindowMessage>,
    fonts: cosmic_text::fontdb::Database,
    assets: HashMap<String, AssetParams>,
    svgs: HashMap<String, String>,
    layer_tx: Option<Sender<LayerWindowMessage>>,
}
unsafe impl Send for LayerWindow {}
unsafe impl Sync for LayerWindow {}

#[derive(Default)]
pub struct LayerWindowParams {
    pub window_info: WindowInfo,
    pub window_opts: WindowOptions,
    pub fonts: cosmic_text::fontdb::Database,
    pub assets: HashMap<String, AssetParams>,
    pub svgs: HashMap<String, String>,
    pub layer_shell_opts: LayerOptions,
    pub layer_tx: Option<Sender<LayerWindowMessage>>,
    pub layer_rx: Option<Channel<LayerWindowMessage>>,
}

#[derive(Debug)]
pub enum LayerWindowMessage {
    ReconfigureLayerOpts { opts: LayerOptions },
}

impl LayerWindow {
    pub fn open_blocking<A, B>(
        params: LayerWindowParams,
        app_params: B,
    ) -> (
        LayerShellSctkWindow,
        EventLoop<'static, LayerShellSctkWindow>,
        Sender<WindowMessage>,
    )
    where
        A: 'static + RootComponent<B> + Component + Default + Send + Sync,
        B: 'static + Any + Clone,
    {
        let LayerWindowParams {
            window_info,
            window_opts,
            fonts,
            assets,
            svgs,
            layer_shell_opts,
            layer_tx,
            layer_rx,
        } = params;

        let (window_tx, window_rx) = calloop::channel::channel();

        let (app_window, event_loop) = LayerShellSctkWindow::new(
            window_tx.clone(),
            window_opts,
            window_info,
            layer_shell_opts,
            layer_rx,
        )
        .expect("failed to create application");

        // let (app_window, event_loop) =
        //     SessionLockWindow::new(window_tx.clone(), window_opts)
        //         .expect("failed to create application");

        let mut ui: UI<LayerWindow, A, B> = UI::new(
            LayerWindow {
                width: app_window.width,
                height: app_window.height,
                handle: None,
                scale_factor: app_window.scale_factor,
                window_tx: window_tx.clone(),
                fonts,
                assets,
                svgs,
                layer_tx,
            },
            app_params,
        );

        // insert handle
        let handle = event_loop.handle();
        let _ = handle.insert_source(
            window_rx,
            move |ev: Event<WindowMessage>, &mut _, app_window| {
                let _ = match ev {
                    calloop::channel::Event::Msg(event) => {
                        match event {
                            WindowMessage::Configure {
                                width,
                                height,
                                wayland_handle,
                            } => {
                                ui.configure(width, height, wayland_handle);
                                ui.draw();
                            }
                            WindowMessage::Send { message } => {
                                ui.update(message);
                                ui.draw(); // TODO: make this conditional
                            }
                            WindowMessage::Resize { width, height } => {
                                app_window.resize(width, height);
                                ui.resize(width, height);
                                ui.draw();
                            }
                            WindowMessage::MainEventsCleared => {
                                ui.draw();
                            }
                            WindowMessage::RedrawRequested => {
                                // ui.handle_input(&Input::Timer);
                                ui.render();
                            }
                            WindowMessage::RequestNextFrame => {
                                app_window.next_frame();
                            }
                            WindowMessage::CompositorFrame => {
                                ui.handle_input(&Input::Timer);
                            }
                            WindowMessage::WindowEvent { event: w_ev } => {
                                // println!("window_event::{:?}", w_ev);
                                match w_ev {
                                    WindowEvent::CloseRequested => {
                                        ui.handle_input(&Input::Exit);
                                        app_window.close();
                                    }
                                    WindowEvent::Focused => {
                                        ui.handle_input(&Input::Focus(true));
                                    }
                                    WindowEvent::Unfocused => {
                                        ui.handle_input(&Input::Focus(false));
                                    }
                                    WindowEvent::Mouse(m_event) => match m_event {
                                        MouseEvent::CursorEntered => {
                                            ui.handle_input(&Input::MouseEnterWindow);
                                        }
                                        MouseEvent::CursorLeft => {
                                            ui.handle_input(&Input::MouseLeaveWindow);
                                        }
                                        MouseEvent::CursorMoved {
                                            position,
                                            scale_factor,
                                        } => {
                                            ui.handle_input(&Input::Motion(Motion::Mouse {
                                                x: position.x as f32 / scale_factor as f32,
                                                y: position.y as f32 / scale_factor as f32,
                                            }));
                                        }
                                        MouseEvent::ButtonPressed { button } => match button {
                                            pointer::Button::Left => ui.handle_input(
                                                &Input::Press(Button::Mouse(MouseButton::Left)),
                                            ),
                                            pointer::Button::Right => ui.handle_input(
                                                &Input::Press(Button::Mouse(MouseButton::Right)),
                                            ),
                                            pointer::Button::Middle => ui.handle_input(
                                                &Input::Press(Button::Mouse(MouseButton::Middle)),
                                            ),
                                        },
                                        MouseEvent::ButtonReleased { button } => match button {
                                            pointer::Button::Left => ui.handle_input(
                                                &Input::Release(Button::Mouse(MouseButton::Left)),
                                            ),
                                            pointer::Button::Right => ui.handle_input(
                                                &Input::Release(Button::Mouse(MouseButton::Right)),
                                            ),
                                            pointer::Button::Middle => ui.handle_input(
                                                &Input::Release(Button::Mouse(MouseButton::Middle)),
                                            ),
                                        },
                                        MouseEvent::WheelScrolled { delta } => {
                                            let scroll = match delta {
                                                ScrollDelta::Lines { x, y } => Motion::Scroll {
                                                    x: x * -30.0,
                                                    y: y * -30.0,
                                                },
                                                ScrollDelta::Pixels { x, y } => Motion::Scroll {
                                                    x: -x as f32,
                                                    y: -y as f32,
                                                },
                                            };
                                            ui.handle_input(&Input::Motion(scroll));
                                        }
                                    },
                                    WindowEvent::Keyboard(k_ev) => match k_ev {
                                        KeyboardEvent::KeyPressed { key } => {
                                            ui.handle_input(&Input::Press(Button::Keyboard(
                                                keysym_to_key(key),
                                            )));
                                            ui.handle_input(&Input::Text(
                                                keysym_to_text(key).to_string(),
                                            ));
                                        }
                                        KeyboardEvent::KeyReleased { key } => {
                                            ui.handle_input(&Input::Release(Button::Keyboard(
                                                keysym_to_key(key),
                                            )));
                                        }
                                    },
                                    WindowEvent::Touch(t_ev) => match t_ev {
                                        TouchEvent::Up {
                                            position,
                                            scale_factor,
                                            ..
                                        } => ui.handle_input(&Input::Touch(TouchAction::Up {
                                            x: position.x / scale_factor,
                                            y: position.y / scale_factor,
                                        })),
                                        TouchEvent::Down {
                                            position,
                                            scale_factor,
                                            ..
                                        } => ui.handle_input(&Input::Touch(TouchAction::Down {
                                            x: position.x / scale_factor,
                                            y: position.y / scale_factor,
                                        })),
                                        TouchEvent::Motion {
                                            position,
                                            scale_factor,
                                            ..
                                        } => ui.handle_input(&Input::Touch(TouchAction::Moved {
                                            x: position.x / scale_factor,
                                            y: position.y / scale_factor,
                                        })),
                                        TouchEvent::Cancel {
                                            position,
                                            scale_factor,
                                            ..
                                        } => ui.handle_input(&Input::Touch(TouchAction::Cancel {
                                            x: position.x / scale_factor,
                                            y: position.y / scale_factor,
                                        })),
                                    },
                                }
                            }
                        }
                    }
                    calloop::channel::Event::Closed => {}
                };
            },
        );

        (app_window, event_loop, window_tx.clone())
    }

    pub fn sender(&self) -> Option<Sender<LayerWindowMessage>> {
        self.layer_tx.clone()
    }
}

impl mctk_core::window::Window for LayerWindow {
    fn logical_size(&self) -> PixelSize {
        PixelSize {
            width: self.width,
            height: self.height,
        }
    }

    fn physical_size(&self) -> PixelSize {
        // let size = self.inner_window.inner_size();
        self.logical_size() // This should transform to device size
    }

    fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    fn redraw(&self) {
        let _ = self.window_tx.send(WindowMessage::RedrawRequested);
    }

    fn next_frame(&self) {
        let _ = self.window_tx.send(WindowMessage::RequestNextFrame);
    }

    fn fonts(&self) -> cosmic_text::fontdb::Database {
        self.fonts.clone()
    }

    fn assets(&self) -> HashMap<String, AssetParams> {
        self.assets.clone()
    }

    fn svgs(&self) -> HashMap<String, String> {
        self.svgs.clone()
    }

    fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn exit(&mut self) {
        let _ = self.window_tx.send(WindowMessage::WindowEvent {
            event: WindowEvent::CloseRequested,
        });
    }

    fn set_wayland_handle(&mut self, wayland_handle: RawWaylandHandle) {
        self.handle = Some(wayland_handle);
    }

    fn has_handle(&self) -> bool {
        self.handle.is_some()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

unsafe impl HasRawWindowHandle for LayerWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.handle.unwrap().raw_window_handle()
    }
}

unsafe impl HasRawDisplayHandle for LayerWindow {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        self.handle.unwrap().raw_display_handle()
    }
}
