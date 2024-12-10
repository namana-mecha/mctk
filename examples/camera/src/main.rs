// mod contexts;
use anyhow::Error;
use camera::Camera;
use image::{ImageBuffer, Rgb};
use mctk_camera::camera::GstCamera;
use mctk_camera::types::{CameraFormat, FrameFormat};
use mctk_core::context::Model;
use mctk_core::prelude::*;
use mctk_core::reexports::smithay_client_toolkit::{
    reexports::calloop::{self, channel::Event},
    shell::wlr_layer,
};
use mctk_core::renderables::Renderable;
use mctk_core::widgets::Text;
use mctk_smithay::layer_shell::layer_surface::LayerOptions;
use mctk_smithay::layer_shell::layer_window::{LayerWindow, LayerWindowParams};
use mctk_smithay::xdg_shell::xdg_window::{self, XdgWindowMessage, XdgWindowParams};
use mctk_smithay::{WindowInfo, WindowMessage, WindowOptions};
use smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::any::Any;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time;

mod camera;
// use crate::contexts::camera::Camera;

// App level channel
#[derive(Debug)]
pub enum AppMessage {
    Exit,
}

#[derive(Debug, Clone)]
pub struct AppParams {
    app_channel: Option<calloop::channel::Sender<AppMessage>>,
}

#[derive(Debug, Default)]
pub struct AppState {
    value: f32,
    btn_pressed: bool,
    app_channel: Option<Sender<AppMessage>>,
    camera: Option<GstCamera>,
    camera_fb: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

#[derive(Debug, Clone)]
enum HelloEvent {
    ButtonPressed {
        name: String,
    },
    TextBox {
        name: String,
        value: String,
        update_type: String,
    },
    Exit,
}

#[component(State = "AppState")]
#[derive(Debug, Default)]
pub struct App {}

impl App {
    fn get_camera_frame(&mut self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let camera = self.state_mut().camera.as_mut().unwrap();

        let frame: ImageBuffer<Rgb<u8>, Vec<u8>> = match camera.frame() {
            Ok(f) => {
                println!("got frame!");
                f
            }
            Err(e) => {
                println!("error frome frame {:?}", e);
                let imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(1, 1);
                imgbuf
            }
        };

        println!("frame fetched from camera {:?}", frame.len());

        frame
    }
}

#[state_component_impl(AppState)]
impl Component for App {
    fn init(&mut self) {
        // init app state
        let mut app_state = AppState {
            value: 30.,
            btn_pressed: false,
            app_channel: None,
            camera: None,
            camera_fb: ImageBuffer::default(),
        };
        Camera::start_fetching();
        //
        // // init camera
        // let camera = match GstCamera::new(
        //     0,
        //     Some(CameraFormat::new_from(640, 480, FrameFormat::MJPEG, 30)),
        // ) {
        //     Ok(mut c) => {
        //         match c.open_stream() {
        //             Ok(()) => {
        //                 println!("camera open success");
        //             }
        //             Err(err) => {
        //                 println!("failed to open camera stream: {}", err);
        //             }
        //         };
        //         Some(c)
        //     }
        //     Err(e) => {
        //         println!("failed to create camera, err - {:?}", e);
        //         None
        //     }
        // };
        //
        // std::thread::sleep(Duration::from_millis(2000));
        // app_state.camera = camera;
        //
        self.state = Some(app_state);
    }

    fn render(&mut self, _: RenderContext) -> Option<Vec<Renderable>> {
        None
    }

    fn on_tick(&mut self, _: &mut mctk_core::event::Event<mctk_core::event::Tick>) {
        // let frame = self.get_camera_frame();
        // self.state_mut().value = self.state_ref().value + 1.;
        // // Note: there is some issue, below does not re-render without using
        // // some other state
        // self.state_mut().camera_fb = frame;
    }

    fn view(&self) -> Option<Node> {
        let value = self.state_ref().value;

        println!("value is {:?}", value);

        Some(
            node!(
                Div::new().bg(Color::rgb(255., 0., 0.)),
                lay![
                    size: size_pct!(100.0),
                    direction: Direction::Column
                ]
            )
            .push(node!(
                // Text::new(txt!(camera::Camera::get().test_value.get().to_string()))
                //     .style("color", Color::WHITE)
                //     .style("size", 40.0)
                //     .style("h_alignment", HorizontalPosition::Center),
                Image::from_buffer(
                    camera::Camera::get_buffer(),
                    camera::Camera::get_width() as usize,
                    camera::Camera::get_height() as usize
                ),
                lay![size: size!(360.0, 300.0)]
            ))
            .push(node!(
                Button::new(txt!("Click"))
                    .on_click(Box::new(|| msg!(HelloEvent::Exit)))
                    .on_double_click(Box::new(|| msg!(HelloEvent::ButtonPressed {
                        name: "Double clicked".to_string()
                    })))
                    .style("color", Color::rgb(255., 0., 0.))
                    .style("background_color", Color::rgb(value % 255., 255., 255.))
                    .style("active_color", Color::rgb(200., 200., 200.))
                    .style("font_size", 24.0),
                lay![size: size!(40.0, 40.0), margin: [0., 0., 0., 0.]]
            )),
        )
    }

    fn update(&mut self, message: Message) -> Vec<Message> {
        println!("App has sent: {:?}", message.downcast_ref::<HelloEvent>());
        match message.downcast_ref::<HelloEvent>() {
            Some(HelloEvent::ButtonPressed { name }) => {
                println!("{}", name);
                self.state_mut().btn_pressed = true;
            }
            Some(HelloEvent::Exit) => {
                println!("button clicked");
            }
            _ => (),
        }
        vec![]
    }
}

// Layer Surface App
#[tokio::main]
async fn main() {
    let id = 1;
    let ui_t = std::thread::spawn(move || {
        let _ = launch_ui(id);
    });
    ui_t.join().unwrap();
}

impl RootComponent<AppParams> for App {
    fn root(&mut self, w: &dyn std::any::Any, app_params: &dyn Any) {
        println!("root initialized");
        let app_params = app_params.downcast_ref::<AppParams>().unwrap();
        self.state_mut().app_channel = app_params.app_channel.clone();
    }
}

fn launch_ui(id: i32) -> anyhow::Result<()> {
    // let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("debug"));
    // tracing_subscriber::fmt()
    //     .compact()
    //     .with_env_filter(env_filter)
    //     .init();

    // let mut fonts: Vec<String> = Vec::new();
    let assets: HashMap<String, AssetParams> = HashMap::new();
    let svgs: HashMap<String, String> = HashMap::new();

    let mut fonts = cosmic_text::fontdb::Database::new();
    fonts.load_system_fonts();

    let window_opts = WindowOptions {
        height: 300 as u32,
        width: 350 as u32,
        scale_factor: 1.0,
    };

    println!("id: {id:?}");
    let window_info = WindowInfo {
        id: format!("{:?}{:?}", "mctk.examples.camera".to_string(), id),
        title: format!("{:?}{:?}", "mctk.examples.camera".to_string(), id),
        namespace: format!("{:?}{:?}", "mctk.examples.camera".to_string(), id),
    };
    let layer_shell_opts = LayerOptions {
        anchor: wlr_layer::Anchor::LEFT | wlr_layer::Anchor::RIGHT | wlr_layer::Anchor::TOP,
        layer: wlr_layer::Layer::Top,
        keyboard_interactivity: wlr_layer::KeyboardInteractivity::Exclusive,
        namespace: Some(window_info.namespace.clone()),
        zone: 0 as i32,
    };

    let (app_channel_tx, app_channel_rx) = calloop::channel::channel();
    let (mut app, mut event_loop, window_tx) = LayerWindow::open_blocking::<App, AppParams>(
        LayerWindowParams {
            window_info,
            window_opts,
            fonts,
            assets,
            layer_shell_opts,
            svgs,
            ..Default::default()
        },
        AppParams {
            app_channel: Some(app_channel_tx),
        },
    );
    let handle = event_loop.handle();
    let window_tx_2 = window_tx.clone();
    let window_tx_channel = window_tx.clone();
    let context_handler = context::get_static_context_handler();
    context_handler.register_on_change(Box::new(move || {
        println!("Context Changed");
        window_tx_channel
            .send(WindowMessage::Send { message: msg!(0) })
            .unwrap();
    }));
    Camera::get().register_context_handler(context_handler);

    let _ = handle.insert_source(app_channel_rx, move |event: Event<AppMessage>, _, app| {
        let _ = match event {
            // calloop::channel::Event::Msg(msg) => app.app.push_message(msg),
            calloop::channel::Event::Msg(msg) => match msg {
                AppMessage::Exit => {
                    println!("app channel message {:?}", AppMessage::Exit);
                    let _ = window_tx_2.send(WindowMessage::WindowEvent {
                        event: mctk_smithay::WindowEvent::CloseRequested,
                    });
                }
            },
            calloop::channel::Event::Closed => {
                println!("calloop::event::closed");
            }
        };
    });

    loop {
        let _ = event_loop.dispatch(None, &mut app);

        if app.is_exited {
            break;
        }
    }

    Ok(())
}
