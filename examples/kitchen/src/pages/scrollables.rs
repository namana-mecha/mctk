use mctk_core::layout::Alignment;
use mctk_core::style::Styled;
use mctk_core::widgets::{Button, IconButton, IconType, Scrollable, Text};
use mctk_core::{component::Component, node, widgets::Div, Color};
use mctk_core::{lay, msg, rect, size, size_pct, txt};

use crate::gui::Message;

#[derive(Debug)]
pub struct Scrollables {}

impl Component for Scrollables {
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
            Text::new(txt!("Scrollable"))
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
                    direction: mctk_core::layout::Direction::Column,
                ]
            )
            .push(
                node!(
                    Scrollable::new(),
                    lay![
                        size: [300, 280],
                    ]
                )
                .push(node!(
                    Button::new(txt!("Click me!"))
                    .on_click(Box::new(|| msg!(Message::Button { name: "Click Me!".to_string() })))
                    .with_class("text-md leading-5 rounded-sm p-2 text-white font-normal font-space-grotesk")
                    .style("background_color", Color::rgb(22., 119., 255.))
                    .style("active_color", Color::rgb(9., 88., 217.)),
                    lay![size: [100, 50] ],
                ))
                .push(node!(Div::new().bg(Color::GREEN), lay![size: [100, 100] ]))
                .push(node!(Div::new().bg(Color::BLUE), lay![size: [100, 100] ]))
                .push(node!(
                    Div::new().bg(Color::DARK_GREY),
                    lay![size: [100, 100] ]
                ))
                .push(node!(Div::new().bg(Color::RED), lay![size: [100, 150] ]))
                .push(node!(
                    Div::new().bg(Color::MAGENTA),
                    lay![size: [100, 150] ]
                )),
            ),
        );
        Some(start)
    }
}
