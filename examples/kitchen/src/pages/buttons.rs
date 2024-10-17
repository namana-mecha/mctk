use mctk_core::style::{FontWeight, Styled};
use mctk_core::widgets::Text;
use mctk_core::{component::Component, node, widgets::Div, Color};
use mctk_core::{lay, rect, size_pct, txt};

use crate::components;
use crate::components::buttons::primary::Primary;

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
            Text::new(txt!("Buttons"))
                .style("color", Color::BLACK)
                .style("size", 20.)
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
                    wrap: true
                ]
            )
            .push(node!(Primary {}, lay![margin:[0., 0., 0., 28.]]))
            .push(node!(components::buttons::default::DefaultButton {},))
            .push(node!(
                components::buttons::text::Text {},
                lay![margin:[20., 0., 0., 48.]]
            ))
            .push(node!(
                components::buttons::link::Link {},
                lay![margin:[20., 0., 0., 0.]]
            )),
        );

        Some(buttons)
    }
}
