use mctk_core::layout::Alignment;
use mctk_core::style::{FontWeight, Styled};
use mctk_core::widgets::{IconButton, IconType, RadioButtons, Text};
use mctk_core::{component::Component, node, widgets::Div, Color};
use mctk_core::{lay, msg, rect, size, size_pct, txt};

use crate::gui::Message;

#[derive(Debug)]
pub struct Radios {}

impl Component for Radios {
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
            Text::new(txt!("Radio")).with_class("text-l font-bold font-space-grotesk text-black"),
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
                    direction: mctk_core::layout::Direction::Column
                ]
            )
            .push(node!(
                Text::new(txt!("Dark")).with_class("text-md font-semibold leading-5"),
                lay![
                    margin:[0., 0., 8., 0.]
                ]
            ))
            .push(node!(Div::new().bg(Color::BLACK), lay![]).push(node!(
                RadioButtons::new(
                    vec![
                        txt!("Rust".to_string()),
                        txt!("Javascript".to_string()),
                        txt!("C++".to_string()),
                    ],
                    0,
                )
                .direction(mctk_core::layout::Direction::Column)
                .style("font_size", 18.0)
                .style("padding", 10.)
                .max_columns(1)
                .on_change(Box::new(|s| msg!(Message::Radio { selection: s }))),
                lay![margin: [0, 10], size: [400, Auto]]
            ))), // .push(node!(
                 //     RadioButtons::new(
                 //          vec![txt!("Bell".to_string()), txt!("Book".to_string()), txt!("Bolt".to_string())],
                 //          vec![0],
                 //      )
                 //          .style("font_size", 18.0)
                 //          .style("font", "open iconic")
                 //          .style("padding", 10.)
                 //      // .tool_tips(vec![
                 //      //     "Bell".to_string(),
                 //      //     "Book".to_string(),
                 //      //     "Bolt".to_string(),
                 //      // ])
                 //      .nullable(false)
                 //      //.multi_select(true)
                 //      .max_columns(3)
                 //      .direction(mctk_core::layout::Direction::Column)
                 //      ,
                 //      // .on_change(Box::new(|s| msg!(HelloEvent::RadioSelect { selection: s }))),
                 //      [margin: [10]]
                 //  ))
        );
        Some(start)
    }
}
