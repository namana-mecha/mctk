use mctk_core::style::{FontWeight, IconButtonSize, Styled};
use mctk_core::widgets::{IconButton, IconType, Text, Toggle};
use mctk_core::{component::Component, node, widgets::Div, Color};
use mctk_core::{lay, rect, size, size_pct, txt};

#[derive(Debug)]
pub struct Toggles {}

impl Component for Toggles {
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
            Text::new(txt!("Toggle"))
                .with_class("light text-l")
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Bold),
            lay![
                margin:[0., 0., 8., 0.]
            ]
        );
        buttons = buttons.push(title);
        buttons = buttons.push(
            node!(
                Div::new().border(Color::rgba(5., 5., 5., 0.06), 1., (8., 8., 8., 8.)),
                lay![
                    padding: [14.],
                    direction: mctk_core::layout::Direction::Column,
                    cross_alignment: mctk_core::layout::Alignment::Stretch
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
                        cross_alignment: mctk_core::layout::Alignment::Center,
                        padding: [6]
                    ]
                )
                .push(node!(
                    Toggle::new(true),
                    lay![
                        margin:[0., 0., 0., 28.],
                    ]
                )),
            )
            .push(node!(
                Text::new(txt!("Light"))
                    .style("color", Color::BLACK)
                    .style("size", 18.)
                    .style("line_height", 18.)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Semibold),
                lay![
                    margin:[10., 0., 8., 0.]
                ]
            ))
            .push(
                node!(
                    Div::new().bg(Color::WHITE),
                    lay![
                        cross_alignment: mctk_core::layout::Alignment::Center,
                        padding: [6]
                    ]
                )
                .push(node!(
                    IconButton::new("wifi_icon")
                        .icon_type(IconType::Png)
                        .style("size", IconButtonSize::XXl)
                        .with_class("light"),
                    lay![
                        margin:[0., 0., 0., 28.],
                    ]
                )),
            ),
        );

        Some(buttons)
    }
}
