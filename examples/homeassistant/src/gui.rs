use std::hash::Hash;

use mctk_core::component::{self, Component, RootComponent};
use mctk_core::style::Styled;
use mctk_core::{layout::*, widgets::*, *};
use mctk_core::{node, node::Node};
use mctk_macros::{component, state_component_impl};

use crate::wled::Wled;

#[derive(Debug, Copy, Clone, Hash)]
pub enum Pages {
    Home,
    Buttons,
    IconButtons,
    Radios,
    Toggles,
    Textboxes,
    SlideBars,
    Scrollables,
}

#[derive(Debug, Clone)]
pub enum Message {
    Button { name: String },
    Textbox { textbox_type: String, text: String },
    IconButton { name: String },
    ChangePage { page: Pages },
    Toggle { value: bool },
    Radio { selection: usize },
    SlideBar { value: u8 },
}

#[derive(Debug)]
pub struct KitchenState {
    current_page: Pages,
}

#[component(State = "KitchenState")]
#[derive(Debug, Default)]
pub struct Kitchen {}

#[derive(Debug, Clone)]
pub struct KitchenParams {}

#[state_component_impl(KitchenState)]
impl Component for Kitchen {
    fn render_hash(&self, hasher: &mut component::ComponentHasher) {
        self.state_ref().current_page.hash(hasher);
    }

    fn init(&mut self) {
        self.state = Some(KitchenState {
            current_page: Pages::Home,
        });
    }

    fn view(&self) -> Option<Node> {
        let mut base = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size: [480, 480],
                direction: mctk_core::layout::Direction::Column,
                padding: [20],
                axis_alignment: mctk_core::layout::Alignment::Stretch,
                cross_alignment: mctk_core::layout::Alignment::Stretch
            ]
        );

        let toggle = node!(
            Div::new(),
            lay![
                size: [400, 40],
                padding: [0.],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Stretch
            ]
        )
        .push(node!(
            Text::new(txt!("WLED"))
                .style("color", Color::WHITE)
                .style("size", 30.0)
                .with_class("text-l text-md text-black leading-5 font-semibold font-space-grotesk"),
            lay![
                size: [320, 40],
                margin:[0., 0., 0., 0.]
            ]
        ))
        .push(
            node!(
                Div::new().bg(Color::BLACK),
                lay![
                    cross_alignment: Alignment::End,
                    padding: [0]
                ]
            )
            .push(node!(
                Toggle::new(true).on_change(Box::new(|v| {
                    Wled::set_state(v);
                    msg!(Message::Toggle { value: v })
                })),
                lay![
                    margin:[0., 0., 0., 0.],
                ]
            )),
        );
        let brightness = node!(
            Div::new().border(Color::rgba(5., 5., 5., 0.06), 1., (8., 8., 8., 8.)),
            lay![
                padding: [14.],
                direction: mctk_core::layout::Direction::Column,
            ]
        )
        .push(node!(
            Text::new(txt!("Brightness"))
                .style("color", Color::WHITE)
                .with_class("text-md font-semibold leading-5"),
            lay![
                margin:[0., 0., 0., 0.]
            ]
        ))
        .push(node!(Div::new().bg(Color::BLACK), lay![]).push(node!(
                SlideBar::new()
                    .value(50)
                    .slider_type(SlideBarType::Line)
                    .active_color(Color::WHITE)
                    .on_slide_end(Box::new(|value| {
                        Wled::set_brightness(value as f32);
                        msg!(Message::SlideBar { value })
                    }))
                    .col_spacing(7.75)
                    .row_spacing(7.75)
                    .col_width(4.),
                lay![size: [410, 46], margin:[0., 0., 0., 0.]]
            )));

        let color = node!(
            Div::new(),
            lay![
                padding: [14.],
                direction: mctk_core::layout::Direction::Column,
            ]
        )
        .push(node!(
            Text::new(txt!("Color"))
                .style("color", Color::WHITE)
                .with_class("text-md font-semibold leading-5"),
            lay![
                margin:[0., 0., 0., 0.]
            ]
        ))
        .push(node!(Div::new().bg(Color::BLACK), lay![]).push(node!(
                SlideBar::new()
                    .value(50)
                    .slider_type(SlideBarType::Line)
                    .active_color(Color::RED)
                    .on_slide_end(Box::new(|value| {
                        Wled::set_r(value as f32);
                        msg!(Message::SlideBar { value })
                    }))
                    .col_spacing(7.75)
                    .row_spacing(7.75)
                    .col_width(4.),
                lay![size: [410, 46], margin:[0., 0., 0., 0.]]
            )))
        .push(node!(Div::new().bg(Color::BLACK), lay![]).push(node!(
                SlideBar::new()
                    .value(50)
                    .slider_type(SlideBarType::Line)
                    .active_color(Color::GREEN)
                    .on_slide_end(Box::new(|value| {
                        Wled::set_g(value as f32);
                        msg!(Message::SlideBar { value })
                    }))
                    .col_spacing(7.75)
                    .row_spacing(7.75)
                    .col_width(4.),
                lay![size: [410, 46], margin:[10., 0., 0., 0.]]
            )))
        .push(node!(Div::new().bg(Color::BLACK), lay![]).push(node!(
                SlideBar::new()
                    .value(50)
                    .slider_type(SlideBarType::Line)
                    .active_color(Color::BLUE)
                    .on_slide_end(Box::new(|value| {
                        Wled::set_b(value as f32);
                        msg!(Message::SlideBar { value })
                    }))
                    .col_spacing(7.75)
                    .row_spacing(7.75)
                    .col_width(4.),
                lay![size: [410, 46], margin:[10., 0., 0., 0.]]
            )));
        base = base.push(toggle);
        base = base.push(brightness);
        base = base.push(color);
        Some(base)
    }
}

impl RootComponent<KitchenParams> for Kitchen {}
