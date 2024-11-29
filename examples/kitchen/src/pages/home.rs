use mctk_core::{
    component::Component,
    lay,
    layout::Direction,
    msg, node, rect, size_pct,
    style::{FontWeight, Styled},
    txt,
    widgets::{Button, Div},
    Color, Node,
};

use crate::gui::{Message, Pages};

#[derive(Debug)]
pub struct Home {}
impl Component for Home {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new().bg(Color::LIGHT_GREY),
                lay![
                    size_pct: [100],
                    direction: Direction::Column,
                    padding: [20]
                ]
            )
            .push(node!(Button::new(txt!("1. Button"))
                .on_click(Box::new(|| msg!(Message::ChangePage {
                    page: Pages::Buttons
                })))
                .with_class("text-l font-bold font-space-grotesk leading-7 p-2 bg-transparent")
                .style("active_color", Color::TRANSPARENT)))
            .push(node!(Button::new(txt!("2. Icon button"))
                .on_click(Box::new(|| msg!(Message::ChangePage {
                    page: Pages::IconButtons
                })))
                .with_class("text-l font-bold font-space-grotesk leading-7 p-2 bg-transparent")
                .style("active_color", Color::TRANSPARENT)))
            .push(node!(Button::new(txt!("3. Radio"))
                .on_click(Box::new(|| msg!(Message::ChangePage {
                    page: Pages::Radios
                })))
                .with_class("text-l font-bold font-space-grotesk leading-7 p-2 bg-transparent")
                .style("active_color", Color::TRANSPARENT)))
            .push(node!(Button::new(txt!("4. Toggle"))
                .on_click(Box::new(|| msg!(Message::ChangePage {
                    page: Pages::Toggles
                })))
                .with_class("text-l font-bold font-space-grotesk leading-7 p-2 bg-transparent")
                .style("active_color", Color::TRANSPARENT)))
            .push(node!(Button::new(txt!("5. Textbox"))
                .on_click(Box::new(|| msg!(Message::ChangePage {
                    page: Pages::Textboxes
                })))
                .with_class("text-l font-bold font-space-grotesk leading-7 p-2 bg-transparent")
                .style("active_color", Color::TRANSPARENT)))
            .push(node!(Button::new(txt!("6. Slide bar"))
                .on_click(Box::new(|| msg!(Message::ChangePage {
                    page: Pages::SlideBars
                })))
                .with_class("text-l font-bold font-space-grotesk leading-7 p-2 bg-transparent")
                .style("active_color", Color::TRANSPARENT)))
            .push(node!(Button::new(txt!("7. Scrollable"))
                .on_click(Box::new(|| msg!(Message::ChangePage {
                    page: Pages::Scrollables
                })))
                .with_class("text-l font-bold font-space-grotesk leading-7 p-2 bg-transparent")
                .style("active_color", Color::TRANSPARENT))),
        )
    }
}
