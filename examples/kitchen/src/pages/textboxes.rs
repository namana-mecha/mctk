use mctk_core::layout::Direction;
use mctk_core::style::{FontWeight, Styled};
use mctk_core::widgets::{Text, TextBox};
use mctk_core::{component::Component, node, widgets::Div, Color};
use mctk_core::{lay, rect, size_pct, txt, size};
use crate::components::textboxes::basic::Basic;
use crate::components::textboxes::borderless::Borderless;
use crate::components::textboxes::filled::Filled;

#[derive(Debug)]
pub struct TextBoxes {}

impl Component for TextBoxes {
    fn view(&self) -> Option<mctk_core::Node> {
        let mut textboxes = node!(
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
            Text::new(txt!("Text boxes"))
                .style("color", Color::BLACK)
                .style("size", 20.)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Bold),
            lay![
                margin:[0., 0., 8., 0.]
            ]
        );
        textboxes = textboxes.push(title);
        textboxes = textboxes.push(
            node!(
                Div::new().border(Color::rgba(5., 5., 5., 0.06), 1., (8., 8., 8., 8.)),
                lay![
                    padding: [14.],
                    direction: Direction::Column
                ]
            )
            .push(node!(
                Text::new(txt!("Dark"))
                    .style("color", Color::BLACK)
                    .style("size", 18.)
                    .style("line_height", 18.)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Semibold),
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
                    .placeholder("Basic usage"), // .on_change(Box::new(|s| msg!(gui::Message::SearchTextChanged(s.to_string()))))
                lay![
                    size: [410, 40],
                    margin: [0., 0., 8., 0.]
                ]
            ))
            // .push(node!(Filled {}, lay![margin: [0., 0., 8., 0.]]))
            // .push(node!(Borderless {}, lay![margin: [0., 0., 8., 0.]]))
        
        )
            ,
        );

        Some(textboxes)
    }
}
