use crate::gui::Message;
use mctk_core::layout::{Alignment, Direction};
use mctk_core::style::Styled;
use mctk_core::widgets::{IconButton, IconType, Text, TextBox};
use mctk_core::{component::Component, node, widgets::Div, Color};
use mctk_core::{lay, msg, rect, size, size_pct, txt};

#[derive(Debug)]
pub struct TextBoxes {}

impl Component for TextBoxes {
    fn view(&self) -> Option<mctk_core::Node> {
        let mut start = node!(
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
            Text::new(txt!("Textbox")).with_class("text-l font-bold font-space-grotesk text-black"),
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
                    direction: Direction::Column
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
                        direction: Direction::Column,
                        padding: [6]
                    ]
                )
                .push(node!(
                    TextBox::new(Some("".to_string()))
                        .with_class("text-md")
                        .placeholder("Basic usage")
                        .on_change(Box::new(|s| msg!(Message::Textbox {
                            textbox_type: "Basic".to_string(),
                            text: s.to_string()
                        }))),
                    lay![
                        size: [410, 40],
                        margin: [0., 0., 18., 0.]
                    ]
                ))
                .push(node!(
                    TextBox::new(Some("".to_string()))
                        .with_class("text-md border-0")
                        .placeholder("Borderless")
                        .on_change(Box::new(|s| msg!(Message::Textbox {
                            textbox_type: "Borderless".to_string(),
                            text: s.to_string()
                        }))),
                    lay![
                        size: [410, 40],
                        margin: [0., 0., 8., 0.]
                    ]
                ))
                .push(node!(
                    TextBox::new(Some("".to_string()))
                        .with_class("text-md border-1 bg-transparent")
                        .placeholder("Transparent")
                        .on_change(Box::new(|s| msg!(Message::Textbox {
                            textbox_type: "Transparent".to_string(),
                            text: s.to_string()
                        }))),
                    lay![
                        size: [410, 40],
                        margin: [0., 0., 8., 0.]
                    ]
                )),
            ),
        );

        Some(start)
    }
}
