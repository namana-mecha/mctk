mod contexts;
use mctk_core::context::Model;
use mctk_core::layout::Alignment;
use mctk_core::prelude::*;
use mctk_core::reexports::smithay_client_toolkit::{
    reexports::calloop::{self, channel::Event},
    shell::wlr_layer,
};
use mctk_core::widgets::Text;
use mctk_smithay::layer_shell::layer_surface::LayerOptions;
use mctk_smithay::layer_shell::layer_window::{LayerWindow, LayerWindowParams};
use mctk_smithay::xdg_shell::xdg_window::{self, XdgWindowMessage, XdgWindowParams};
use mctk_smithay::{WindowInfo, WindowMessage, WindowOptions};
use smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::contexts::weather_api::WeatherAPI;

#[derive(Debug)]
pub enum AppMessage {
    Exit,
}

#[derive(Clone)]
pub struct AppParams {
    app_channel: Option<calloop::channel::Sender<AppMessage>>,
}

pub struct AppState {
    window_sender: Option<Sender<XdgWindowMessage>>,
    app_channel: Option<Sender<AppMessage>>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            window_sender: None,
            app_channel: None,
        }
    }
}

impl Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState").finish()
    }
}

#[component(State = "AppState")]
#[derive(Debug, Default)]
pub struct App {}

#[state_component_impl(AppState)]
impl Component for App {
    fn init(&mut self) {
        self.state = Some(AppState {
            window_sender: None,
            app_channel: None,
        });
    }

    fn view(&self) -> Option<Node> {
        let temperature_message = if !*WeatherAPI::get().is_loading.get() {
            format!("Temperature: {:.2}*C", *WeatherAPI::get().temperature.get())
        } else {
            format!("Loading...")
        };
        Some(
            node!(
                Div::new().bg(Color::BLACK),
                lay![
                    size: size_pct!(100.0),
                    direction: Direction::Column,
                    axis_alignment: Alignment::Center,
                    cross_alignment: Alignment::Center
                ]
            )
            .push(node!(
                Text::new(txt!(temperature_message))
                    .style("color", Color::WHITE)
                    .style("size", 40.0)
                    .style("h_alignment", HorizontalPosition::Center),
                lay![
                    size: size!(100.0, 100.0),
                    direction: Direction::Column
                ]
            ))
            .push(node!(
                Button::new(txt!("Fetch Weather"))
                    .on_click(Box::new(|| {
                        WeatherAPI::fetch();
                        msg!(0)
                    }))
                    .style("color", Color::WHITE)
                    .style("size", 48.0),
                lay![
                    size: size!(100.0, 50.0),
                    direction: Direction::Column
                ]
            )),
        )
    }

    fn update(&mut self, message: Message) -> Vec<Message> {
        vec![message]
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
        let app_params = app_params.downcast_ref::<AppParams>().unwrap();
        self.state_mut().app_channel = app_params.app_channel.clone();
    }
}

fn launch_ui(id: i32) -> anyhow::Result<()> {
    let assets: HashMap<String, AssetParams> = HashMap::new();
    let mut svgs: HashMap<String, String> = HashMap::new();

    svgs.insert(
        "eye_icon".to_string(),
        "./src/assets/icons/eye.svg".to_string(),
    );

    let mut fonts = cosmic_text::fontdb::Database::new();
    fonts.load_system_fonts();

    fonts.load_font_data(include_bytes!("assets/fonts/SpaceGrotesk-Regular.ttf").into());

    let window_opts = WindowOptions {
        height: 480_u32,
        width: 480_u32,
        scale_factor: 1.0,
    };

    println!("id: {id:?}");
    let window_info = WindowInfo {
        id: format!("{:?}{:?}", "mctk.examples.hello-world".to_string(), id),
        title: format!("{:?}{:?}", "mctk.examples.hello-world".to_string(), id),
        namespace: format!("{:?}{:?}", "mctk.examples.hello-world".to_string(), id),
    };
    let layer_shell_opts = LayerOptions {
        anchor: wlr_layer::Anchor::LEFT | wlr_layer::Anchor::RIGHT | wlr_layer::Anchor::TOP,
        layer: wlr_layer::Layer::Top,
        keyboard_interactivity: wlr_layer::KeyboardInteractivity::Exclusive,
        namespace: Some(window_info.namespace.clone()),
        zone: 0_i32,
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
    WeatherAPI::get().register_context_handler(context_handler);

    let _ = handle.insert_source(app_channel_rx, move |event: Event<AppMessage>, _, app| {
        match event {
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
