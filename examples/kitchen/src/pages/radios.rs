use mctk_core::style::{FontWeight, Styled};
use mctk_core::widgets::{RadioButtons, Text};
use mctk_core::{component::Component, node, widgets::Div, Color};
use mctk_core::{lay, rect, size, size_pct, txt};

#[derive(Debug)]
pub struct Radios {}

impl Component for Radios {
    fn view(&self) -> Option<mctk_core::Node> {
        let mut radios = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100],
                direction: mctk_core::layout::Direction::Column,
                padding: [20],
                axis_alignment: mctk_core::layout::Alignment::Stretch,
                cross_alignment: mctk_core::layout::Alignment::Stretch
            ]
        );
        let title = node!(
            Text::new(txt!("Radios"))
                .style("color", Color::BLACK)
                .style("size", 20.)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Bold),
            lay![
                margin:[0., 0., 8., 0.]
            ]
        );
        radios = radios.push(title);
        radios = radios.push(
            node!(
                Div::new().border(Color::rgba(5., 5., 5., 0.06), 1., (8., 8., 8., 8.)),
                lay![
                    padding: [14.],
                    direction: mctk_core::layout::Direction::Column
                ]
            )
            .push(node!(
                RadioButtons::new(
                    vec![
                        txt!("10s".to_string()),
                        txt!("30s".to_string()),
                        txt!("60s".to_string()),
                        txt!("5m".to_string()),
                        txt!("Never".to_string()),
                    ],
                    0,
                )
                .direction(mctk_core::layout::Direction::Column)
                .style("font_size", 18.0)
                .style("padding", 10.)
                //.multi_select(true)
                .max_columns(1),
                // .on_change(Box::new(|s| msg!(HelloEvent::RadioSelect { selection: s }))),
                lay![margin: [10], size: [400, Auto]]
            )), // .push(node!(
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
        Some(radios)
    }
}
