use mctk_core::prelude::*;
use mctk_core::renderables::types;
use mctk_smithay::layer_shell::layer_surface::LayerOptions;
use mctk_smithay::layer_shell::layer_window;
use mctk_smithay::layer_shell::layer_window::LayerWindowParams;
use mctk_smithay::{WindowInfo, WindowOptions};
use smithay_client_toolkit::shell::wlr_layer;
use std::collections::HashMap;
use std::time::Duration;
use tracing_subscriber::EnvFilter;

// App level channel
pub enum AppMessage {}

#[derive(Debug, Clone)]
enum HelloEvent {
    Button { name: String },
}

#[derive(Debug, Default)]
pub struct App {}

#[derive(Debug, Clone)]
pub struct AppParams {}

impl Component for App {
    fn view(&self) -> Option<Node> {
        // println!("app view called");

        Some(
            node!(Carousel::new().scroll_x(), [
                size: [480, 60],
                direction: Row,
            ])
            .push(node!(
                Div::new().bg([255.0, 0.0, 0.0]),
                [ size: [40, 40]],
            ))
            .push(node!(
                Div::new().bg([255.0, 133.5, 0.0]),
                [ size: [40, 40]],
            ))
            .push(node!(
                Div::new().bg([255.0, 255.0, 0.0]),
                [ size: [40, 40]],
            ))
            .push(node!(
                Div::new().bg([0.0, 255.0, 0.0]),
                [ size: [40, 40]],
            ))
            .push(node!(
                Div::new().bg([255.0, 0.0, 0.0]),
                [ size: [40, 40]],
            ))
            .push(node!(
                Div::new().bg([255.0, 133.5, 0.0]),
                [ size: [40, 40]],
            ))
            .push(node!(
                Div::new().bg([255.0, 255.0, 0.0]),
                [ size: [40, 40]],
            ))
            .push(node!(
                Div::new().bg([255.0, 0.0, 0.0]),
                [ size: [40, 40]],
            ))
            .push(node!(
                Div::new().bg([255.0, 133.5, 0.0]),
                [ size: [40, 40]],
            ))
            .push(node!(
                Div::new().bg([255.0, 255.0, 0.0]),
                [ size: [40, 40]],
            ))
            .push(node!(
                Div::new().bg([0.0, 255.0, 0.0]),
                [ size: [40, 40]],
            ))
            .push(node!(
                Div::new().bg([255.0, 0.0, 0.0]),
                [ size: [40, 40]],
            ))
            .push(node!(
                Div::new().bg([255.0, 133.5, 0.0]),
                [ size: [40, 40]],
            ))
            .push(node!(
                Div::new().bg([255.0, 255.0, 0.0]),
                [ size: [40, 40]],
            )),
        )
    }
}

// Layer Surface App
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("inside main");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("debug"));
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(env_filter)
        .init();

    let mut fonts = cosmic_text::fontdb::Database::new();
    fonts.load_system_fonts();

    fonts.load_font_data(include_bytes!("assets/fonts/SpaceGrotesk-Regular.ttf").into());

    let mut assets: HashMap<String, AssetParams> = HashMap::new();

    assets.insert(
        "bg".to_string(),
        AssetParams::new("src/assets/icons/bg.png".to_string()),
    );

    let mut svgs = HashMap::new();
    svgs.insert(
        "battery".to_string(),
        "src/assets/icons/battery.svg".to_string(),
    );

    let namespace = "mctk.examples.carousel".to_string();

    let layer_shell_opts = LayerOptions {
        anchor: wlr_layer::Anchor::TOP | wlr_layer::Anchor::LEFT | wlr_layer::Anchor::RIGHT,
        layer: wlr_layer::Layer::Overlay,
        keyboard_interactivity: wlr_layer::KeyboardInteractivity::Exclusive,
        namespace: Some(namespace.clone()),
        zone: 0,
    };

    let window_info = WindowInfo {
        id: "mctk.examples.carousel".to_string(),
        title: "Carousel".to_string(),
        namespace,
    };

    let window_opts = WindowOptions {
        height: 480 as u32,
        width: 480 as u32,
        scale_factor: 1.0,
    };

    let (mut app, mut event_loop, ..) = layer_window::LayerWindow::open_blocking::<App, AppParams>(
        LayerWindowParams {
            window_info,
            window_opts,
            fonts,
            assets,
            svgs,
            layer_shell_opts,
            ..Default::default()
        },
        AppParams {},
    );

    loop {
        event_loop
            .dispatch(Duration::from_millis(16), &mut app)
            .unwrap();
    }
}

impl RootComponent<AppParams> for App {}
