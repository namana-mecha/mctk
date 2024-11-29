use mctk_core::layout::{Alignment, Direction};
use mctk_core::style::Styled;
use mctk_core::widgets::{IconButton, IconType, Text, Toggle};
use mctk_core::{component::Component, node, widgets::Div, Color};
use mctk_core::{lay, msg, rect, size, size_pct, txt};

use crate::gui::Message;

#[derive(Debug)]
pub struct Toggles {}

impl Component for Toggles {
    fn view(&self) -> Option<mctk_core::Node> {
        let mut start = node!(
            Div::new().bg(Color::LIGHT_GREY),
            lay![
                size_pct: [100],
                direction: Direction::Column,
                padding: [20],
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Stretch
            ]
        );
        let title = node!(
            Text::new(txt!("Toggle")).with_class("text-l font-bold font-space-grotesk text-black"),
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
        start = start.push(
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
        start = start.push(
            node!(
                Div::new().border(Color::rgba(5., 5., 5., 0.06), 1., (8., 8., 8., 8.)),
                lay![
                    padding: [14.],
                    direction: Direction::Column,
                    cross_alignment: Alignment::Stretch
                ]
            )
            .push(node!(
                Text::new(txt!("Dark")).with_class(
                    "text-l text-md text-black leading-5 font-semibold font-space-grotesk"
                ),
                lay![
                    margin:[0., 0., 8., 0.]
                ]
            ))
            .push(
                node!(
                    Div::new().bg(Color::BLACK),
                    lay![
                        cross_alignment: Alignment::Center,
                        padding: [6]
                    ]
                )
                .push(node!(
                    Toggle::new(true).on_change(Box::new(|v| msg!(Message::Toggle { value: v }))),
                    lay![
                        margin:[0., 0., 0., 28.],
                    ]
                )),
            ),
        );

        Some(start)
    }
}
