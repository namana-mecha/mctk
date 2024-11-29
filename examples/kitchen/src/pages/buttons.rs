use mctk_core::layout::Alignment;
use mctk_core::style::Styled;
use mctk_core::widgets::{Button, IconButton, IconType, Text};
use mctk_core::{component::Component, node, widgets::Div, Color};
use mctk_core::{lay, msg, rect, size, size_pct, txt};

use crate::gui::Message;

#[derive(Debug)]
pub struct Buttons {}

impl Component for Buttons {
    fn view(&self) -> Option<mctk_core::Node> {
        let mut buttons = node!(
            Div::new().bg(Color::LIGHT_GREY),
            lay![
                size_pct: [100],
                direction: mctk_core::layout::Direction::Column,
                padding: [20],
                axis_alignment: mctk_core::layout::Alignment::Stretch,
                cross_alignment: mctk_core::layout::Alignment::Stretch
            ]
        );

        let title = node!(
            Text::new(txt!("Button")).with_class("text-l font-bold font-space-grotesk text-black"),
            lay![
                margin:[0., 0., 8., 0.]
            ]
        );
        let back = node!(
            IconButton::new("back_icon")
                .on_click(Box::new(|| msg!(Message::ChangePage {
                    page: crate::gui::Pages::Home
                })))
                .with_class("btn-md border-0 bg-transparent")
                .icon_type(IconType::Svg),
            lay![
                margin:[0., 0., 0., 6.],
            ]
        );
        buttons = buttons.push(
            node!(
                Div::new(),
                lay![
                    size: [Auto, 40],
                    cross_alignment: Alignment::Center
                ]
            )
            .push(back)
            .push(title),
        );
        buttons = buttons.push(
            node!(
                Div::new().border(Color::rgba(5., 5., 5., 0.06), 1., (8., 8., 8., 8.)),
                lay![
                    padding: [14.],
                    wrap: true
                ]
            )
            .push(node!(
                Button::new(txt!("Primary Button"))
                    .on_click(Box::new(|| msg!(Message::Button { name: "Primary".to_string() })))
                    .with_class("text-md leading-5 rounded-sm p-2 text-white font-normal font-space-grotesk")
                    .style("background_color", Color::rgb(22., 119., 255.))
                    .style("active_color", Color::rgb(9., 88., 217.))
                    , 
                lay![margin:[0., 0., 0., 28.]]
            ))
            .push(
                node!(Button::new(txt!("Default Button"))
                    .on_click(Box::new(|| msg!(Message::Button { name: "Default".to_string() })))
                    .with_class("text-md leading-5 rounded-sm border p-2 font-normal text-black font-space-grotesk bg-transparent")
                    .style("active_color", Color::DARK_GREY)
                    .style("border_color", Color::rgb(217., 217., 217.)),
                    lay![]
            ))
            .push(node!(
                Button::new(txt!("Text Button"))
                .on_click(Box::new(|| msg!(Message::Button { name: "Text".to_string() })))
                .with_class("text-black font-space-grotesk text-md leading-5 font-normal border-none p-2 bg-transparent")
                .style("active_color", Color::TRANSPARENT),
                lay![margin:[20., 0., 0., 48.]]
            ))
            .push(node!(
                Button::new(txt!("Link Button"))
                .on_click(Box::new(|| msg!(Message::Button { name: "Link".to_string() })))
                .with_class("text-black font-space-grotesk text-md leading-5 font-normal border-none p-2 bg-transparent")
                .style("active_color", Color::TRANSPARENT)
                .style("text_color", Color::rgb(22., 119., 255.)),
                lay![margin:[20., 0., 0., 0.]]
            )),
        );

        Some(buttons)
    }
}
