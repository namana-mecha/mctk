use std::hash::Hash;

use mctk_core::component::{self, Component, RootComponent};
use mctk_core::widgets::{Div, SlideBar, SlideBarType};
use mctk_core::{lay, size_pct};
use mctk_core::{node, node::Node};
use mctk_macros::{component, state_component_impl};

use crate::pages::buttons::Buttons;
use crate::pages::home::Home;
use crate::pages::icon_buttons::IconButtons;
use crate::pages::radios::Radios;
use crate::pages::scrollables::Scrollables;
use crate::pages::slider_bars::SlideBars;
use crate::pages::textboxes::TextBoxes;
use crate::pages::toggles::Toggles;

#[derive(Debug, Copy, Clone, Hash)]
pub enum Pages {
    Home,
    Buttons,
    IconButtons,
    Radios,
    Toggles,
    Textboxes,
    SlideBars,
    Scrollables,
}

#[derive(Debug, Clone)]
pub enum Message {
    Button { name: String },
    Textbox { textbox_type: String, text: String },
    IconButton { name: String },
    ChangePage { page: Pages },
    Toggle { value: bool },
    Radio { selection: usize },
    SlideBar { value: u8 },
}

#[derive(Debug)]
pub struct KitchenState {
    current_page: Pages,
}

#[component(State = "KitchenState")]
#[derive(Debug, Default)]
pub struct Kitchen {}

#[derive(Debug, Clone)]
pub struct KitchenParams {}

#[state_component_impl(KitchenState)]
impl Component for Kitchen {
    fn render_hash(&self, hasher: &mut component::ComponentHasher) {
        self.state_ref().current_page.hash(hasher);
    }

    fn init(&mut self) {
        self.state = Some(KitchenState {
            current_page: Pages::Home,
        });
    }

    fn update(&mut self, msg: component::Message) -> Vec<component::Message> {
        if let Some(message) = msg.downcast_ref::<Message>() {
            match message {
                Message::ChangePage { page } => {
                    self.state_mut().current_page = page.clone();
                }
                Message::Button { name } => {
                    println!("Button click: {:?}", name);
                }
                Message::Textbox { textbox_type, text } => {
                    println!("Textbox {:?} change: {:?}", textbox_type, text);
                }
                Message::IconButton { name } => {
                    println!("Icon button click: {:?}", name);
                }
                Message::Toggle { value } => {
                    println!("Toggle change: {:?}", value);
                }
                Message::Radio { selection } => {
                    println!("Radio change: {:?}", selection);
                }
                Message::SlideBar { value } => {
                    println!("Slide bar change: {:?}", value);
                }
            }
        }
        vec![]
    }

    fn view(&self) -> Option<Node> {
        let current_page = self.state_ref().current_page;

        let screen = match current_page {
            Pages::Home => node!(Home {}, lay![size_pct:[100]]),
            Pages::Buttons => node!(Buttons {}, lay![size_pct:[100]]),
            Pages::IconButtons => node!(IconButtons {}, lay![size_pct:[100]]),
            Pages::Radios => node!(Radios {}, lay![size_pct:[100]]),
            Pages::Toggles => node!(Toggles {}, lay![size_pct:[100]]),
            Pages::Textboxes => node!(TextBoxes {}, lay![size_pct:[100]]),
            Pages::SlideBars => node!(SlideBars {}, lay![size_pct:[100]]),
            Pages::Scrollables => node!(Scrollables {}, lay![size_pct:[100]]),
        };
        Some(screen)
    }
}

impl RootComponent<KitchenParams> for Kitchen {}
