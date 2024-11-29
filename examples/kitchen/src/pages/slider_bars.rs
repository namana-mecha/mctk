use mctk_core::layout::Alignment;
use mctk_core::style::Styled;
use mctk_core::widgets::{IconButton, IconType, SlideBar, SlideBarType, Text};
use mctk_core::{component::Component, node, widgets::Div, Color};
use mctk_core::{lay, msg, rect, size, size_pct, txt};

use crate::gui::Message;

#[derive(Debug)]
pub struct SlideBars {}

impl Component for SlideBars {
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
            Text::new(txt!("Slider bar"))
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
            .push(node!(
                Text::new(txt!("Dark")).with_class("text-md font-semibold leading-5"),
                lay![
                    margin:[0., 0., 8., 0.]
                ]
            ))
            .push(node!(Div::new().bg(Color::BLACK), lay![]).push(node!(
                SlideBar::new()
                    .value(50)
                    .slider_type(SlideBarType::Box)
                    .active_color(Color::rgb(15., 168., 255.))
                    .on_slide(Box::new(|value| msg!(Message::SlideBar { value })))
                    .col_spacing(7.75)
                    .row_spacing(7.75)
                    .col_width(4.),
                lay![size: [410, 46], margin:[10., 0., 0., 0.]]
            ))),
        );
        Some(start)
    }
}
