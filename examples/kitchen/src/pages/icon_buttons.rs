use mctk_core::layout::{Alignment, Direction};
use mctk_core::style::Styled;
use mctk_core::widgets::{IconButton, IconType, Text};
use mctk_core::{component::Component, node, widgets::Div, Color};
use mctk_core::{lay, msg, rect, size, size_pct, txt};

use crate::gui::Message;

#[derive(Debug)]
pub struct IconButtons {}

impl Component for IconButtons {
    fn view(&self) -> Option<mctk_core::Node> {
        let mut buttons = node!(
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
            Text::new(txt!("Icon button"))
                .with_class("text-l font-bold font-space-grotesk text-black"),
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
                    direction: Direction::Column,
                    cross_alignment: Alignment::Stretch
                ]
            )
            .push(node!(
                Text::new(txt!("Dark")).with_class("text-md font-semibold leading-5"),
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
                    IconButton::new("wifi_icon")
                        .on_click(Box::new(|| msg!(Message::IconButton {
                            name: "wifi xxl".to_string()
                        })))
                        .with_class("btn-xxl border-2")
                        .icon_type(IconType::Png),
                    lay![
                        margin:[0., 0., 0., 28.],
                    ]
                ))
                .push(node!(
                    IconButton::new("wifi_icon")
                        .on_click(Box::new(|| msg!(Message::IconButton {
                            name: "wifi xl".to_string()
                        })))
                        .with_class("btn-xl border-2")
                        .icon_type(IconType::Png),
                    lay![
                        margin:[0., 0., 0., 28.],
                    ]
                ))
                .push(node!(
                    IconButton::new("wifi_icon")
                        .on_click(Box::new(|| msg!(Message::IconButton {
                            name: "wifi md".to_string()
                        })))
                        .with_class("btn-md border-2")
                        .icon_type(IconType::Png),
                    lay![
                        margin:[0., 0., 0., 28.],
                    ]
                ))
                .push(node!(
                    IconButton::new("wifi_icon")
                        .on_click(Box::new(|| msg!(Message::IconButton {
                            name: "wifi sm".to_string()
                        })))
                        .with_class("btn-sm border-2")
                        .icon_type(IconType::Png),
                    lay![
                        margin:[0., 0., 0., 28.],
                    ]
                ))
                .push(node!(
                    IconButton::new("wifi_icon")
                        .on_click(Box::new(|| msg!(Message::IconButton {
                            name: "wifi xs".to_string()
                        })))
                        .with_class("btn-xs border-2")
                        .icon_type(IconType::Png),
                    lay![
                        margin:[0., 0., 0., 28.],
                    ]
                )),
            )
            .push(node!(
                Text::new(txt!("Light")).with_class("text-md font-semibold leading-5"),
                lay![
                    margin:[10., 0., 8., 0.]
                ]
            ))
            .push(
                node!(
                    Div::new().bg(Color::WHITE),
                    lay![
                        cross_alignment: Alignment::Center,
                        padding: [6]
                    ]
                )
                .push(node!(
                    IconButton::new("wifi_icon")
                        .on_click(Box::new(|| msg!(Message::IconButton {
                            name: "wifi xxl".to_string()
                        })))
                        .icon_type(IconType::Png)
                        .with_class("light btn-xxl border-2"),
                    lay![
                        margin:[0., 0., 0., 28.],
                    ]
                ))
                .push(node!(
                    IconButton::new("wifi_icon")
                        .on_click(Box::new(|| msg!(Message::IconButton {
                            name: "wifi xl".to_string()
                        })))
                        .icon_type(IconType::Png)
                        .with_class("light btn-xl border-2"),
                    lay![
                        margin:[0., 0., 0., 28.],
                    ]
                ))
                .push(node!(
                    IconButton::new("wifi_icon")
                        .on_click(Box::new(|| msg!(Message::IconButton {
                            name: "wifi md".to_string()
                        })))
                        .icon_type(IconType::Png)
                        .with_class("light btn-md border-2"),
                    lay![
                        margin:[0., 0., 0., 28.],
                    ]
                ))
                .push(node!(
                    IconButton::new("wifi_icon")
                        .on_click(Box::new(|| msg!(Message::IconButton {
                            name: "wifi sm".to_string()
                        })))
                        .icon_type(IconType::Png)
                        .with_class("light btn-sm border-2"),
                    lay![
                        margin:[0., 0., 0., 28.],
                    ]
                ))
                .push(node!(
                    IconButton::new("wifi_icon")
                        .on_click(Box::new(|| msg!(Message::IconButton {
                            name: "wifi xs".to_string()
                        })))
                        .icon_type(IconType::Png)
                        .with_class("light btn-xs border-2"),
                    lay![
                        margin:[0., 0., 0., 28.],
                    ]
                )),
            ),
        );

        Some(buttons)
    }
}
