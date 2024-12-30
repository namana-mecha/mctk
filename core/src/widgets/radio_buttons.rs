use std::fmt;
use std::hash::Hash;

// use super::ToolTip;
use crate::component::{Component, ComponentHasher, Message};
use crate::font_cache::TextSegment;
use crate::renderables::circle::InstanceBuilder as CircleInstanceBuilder;
use crate::renderables::{Circle, Renderable};
use crate::style::{FontWeight, HorizontalPosition, Styled};
use crate::{event, lay, msg, rect, size, size_pct, txt, Point, Pos, AABB};
use crate::{layout::*, Color};
use crate::{node, Node};
use mctk_macros::{component, state_component_impl};

use super::{Div, HDivider, Text};

#[derive(Debug, Default, Clone, Copy)]
pub enum RadioButtonsType {
    Basic,
    #[default]
    Group,
    Block,
}

#[derive(Debug, Default)]
struct RadioButtonsState {
    selected: usize,
}

#[component(State = "RadioButtonsState", Styled = "RadioButton", Internal)]
pub struct RadioButtons {
    buttons: Vec<Vec<TextSegment>>,
    selected: usize,
    direction: Direction,
    max_rows: Option<usize>,
    max_columns: Option<usize>,
    on_change: Option<Box<dyn Fn(usize) -> Message + Send + Sync>>,
    radio_buttons_type: RadioButtonsType,
}

impl fmt::Debug for RadioButtons {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RadioButtons")
            .field("buttons", &self.buttons)
            .field("selected", &self.selected)
            .finish()
    }
}

enum RadioButtonMsg {
    Clicked(usize),
}

impl RadioButtons {
    pub fn new(buttons: Vec<Vec<TextSegment>>, selected: usize) -> Self {
        let mut state = RadioButtonsState::default();
        state.selected = selected;
        Self {
            buttons,
            selected,
            direction: Direction::Row,
            max_rows: None,
            max_columns: None,
            on_change: None,
            class: Default::default(),
            style_overrides: Default::default(),
            radio_buttons_type: Default::default(),
            dirty: false,
            state: Some(state),
        }
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn max_rows(mut self, max_rows: usize) -> Self {
        self.max_rows = Some(max_rows);
        self
    }

    pub fn max_columns(mut self, max_columns: usize) -> Self {
        self.max_columns = Some(max_columns);
        self
    }

    pub fn on_change(mut self, change_fn: Box<dyn Fn(usize) -> Message + Send + Sync>) -> Self {
        self.on_change = Some(change_fn);
        self
    }

    pub fn radio_buttons_type(mut self, t: RadioButtonsType) -> Self {
        self.radio_buttons_type = t;
        self
    }
}

#[state_component_impl(RadioButtonsState)]
impl Component for RadioButtons {
    fn new_props(&mut self) {
        self.state_mut().selected = self.selected;
    }

    fn props_hash(&self, hasher: &mut ComponentHasher) {
        self.selected.hash(hasher);
    }

    fn render_hash(&self, hasher: &mut ComponentHasher) {
        self.state_ref().selected.hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        // println!("RadioButtons::view() {:?}", self.state_ref().selected);
        let mut base = node!(
            super::Div::new(),
            lay![direction: match self.direction {
                Direction::Row => Direction::Column,
                Direction::Column => Direction::Row,
            },  size_pct: [100, Auto]]
        );

        let limit = match self.direction {
            Direction::Row => self.max_columns.unwrap_or(10000),
            Direction::Column => self.max_rows.unwrap_or(10000),
        };
        let len = self.buttons.len();
        let n_rows = match self.direction {
            Direction::Column => {
                if len > limit {
                    limit
                } else {
                    len
                }
            }
            Direction::Row => (len + limit - 1) / limit,
        };
        let n_columns = match self.direction {
            Direction::Column => (len + limit - 1) / limit,
            Direction::Row => {
                if len > limit {
                    limit
                } else {
                    len
                }
            }
        };

        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut container = node!(
            super::Div::new(),
            lay![
                direction: self.direction,
                size_pct: [100, Auto]
            ]
        )
        .key(i as u64);
        for (position, b) in self.buttons.iter().enumerate() {
            if j >= limit {
                j = 0;
                i += 1;
                let old_container = container;
                container = node!(
                    super::Div::new(),
                    lay!(direction: self.direction,
                         cross_alignment: Alignment::Stretch,
                         // axis_alignment: Alignment::Stretch, // TODO: This is broken
                    )
                )
                .key(i as u64);
                base = base.push(old_container);
            }
            let row = match self.direction {
                Direction::Row => i,
                Direction::Column => j,
            };
            let col = match self.direction {
                Direction::Row => j,
                Direction::Column => i,
            };

            let selected = self.state_ref().selected == position;
            let radius: f32 = self.style_val("radius").unwrap().f32();

            let radio_button_radius = match self.radio_buttons_type {
                RadioButtonsType::Basic => (14., 14., 14., 14.),
                RadioButtonsType::Group => (10., 10., 10., 10.),
                RadioButtonsType::Block => (
                    if row == 0 && col == 0 { radius } else { 0.0 },
                    if row == 0 && (col + 1 == n_columns || position + 1 == len) {
                        radius
                    } else {
                        0.0
                    },
                    if position + 1 == len { radius } else { 0.0 },
                    if col == 0 && (row + 1 == n_rows || position + 1 == len) {
                        radius
                    } else {
                        0.0
                    },
                ),
            };

            container = container.push(
                node!(
                    RadioButton {
                        label: b.clone(),
                        radio_button_type: self.radio_buttons_type,
                        position,
                        selected,
                        radius: radio_button_radius,
                        class: self.class,
                        style_overrides: self.style_overrides.clone(),
                    },
                    lay![size_pct: [100, Auto]]
                )
                .key((j + j + 1) as u64),
            );

            container = container.push(
                node!(
                    HDivider {
                        size: 1.5,
                        color: Color::rgb(83., 83., 83.)
                    },
                    lay![size_pct: [100, Auto]]
                )
                .key((j + j + 2) as u64),
            );

            j += 1;
        }

        Some(base.push(container))
    }

    fn update(&mut self, message: Message) -> Vec<Message> {
        let mut m: Vec<Message> = vec![];

        match message.downcast_ref::<RadioButtonMsg>() {
            Some(RadioButtonMsg::Clicked(n)) => {
                self.state_mut().selected = *n;
                if let Some(change_fn) = &self.on_change {
                    m.push(change_fn(*n));
                }
            }
            None => panic!(),
        }
        m
    }
}

#[component(Styled, Internal)]
#[derive(Debug)]
struct RadioButton {
    label: Vec<TextSegment>,
    position: usize,
    selected: bool,
    radius: (f32, f32, f32, f32),
    radio_button_type: RadioButtonsType,
}

impl Component for RadioButton {
    fn props_hash(&self, hasher: &mut ComponentHasher) {
        self.selected.hash(hasher);
    }

    fn render(
        &mut self,
        context: crate::component::RenderContext,
    ) -> Option<Vec<crate::renderables::Renderable>> {
        let active_color: Color = self.style_val("active_color").into();
        let border_color: Color = self.style_val("border_color").into();

        let width = context.aabb.width();
        let height = context.aabb.height();
        let AABB { pos, .. } = context.aabb;
        let mut rs = vec![];

        let radius = 9.;
        let circle_instance_data = CircleInstanceBuilder::default()
            .origin(Pos {
                x: pos.x + width - 10. - radius / 2.,
                y: pos.y + height / 2.,
                z: 0.,
            })
            .radius(radius)
            .border_width(2.)
            .border_color(Some(if self.selected {
                active_color
            } else {
                border_color
            }))
            .build()
            .unwrap();
        rs.push(Renderable::Circle(Circle::from_instance_data(
            circle_instance_data,
        )));

        let circle_instance_data_2 = CircleInstanceBuilder::default()
            .origin(Pos {
                x: pos.x + width - 10. - radius / 2.,
                y: pos.y + height / 2.,
                z: 0.,
            })
            .radius(radius - 2.)
            .color(Some(active_color))
            .build()
            .unwrap();

        if self.selected {
            rs.push(Renderable::Circle(Circle::from_instance_data(
                circle_instance_data_2,
            )));
        }

        Some(rs)
    }

    fn view(&self) -> Option<Node> {
        let padding: f64 = self.style_val("padding").unwrap().into();
        let active_color: Color = self.style_val("active_color").into();
        let highlight_color: Color = self.style_val("highlight_color").into();
        let background_color: Color = self.style_val("background_color").into();
        let border_color: Color = self.style_val("border_color").into();
        let border_width: f32 = self.style_val("border_width").unwrap().f32();

        match self.radio_button_type {
            RadioButtonsType::Basic => {
                let mut base = node!(
                    Div::new(),
                    lay![
                        size_pct: [100],
                        direction: Direction::Row,
                        cross_alignment: crate::layout::Alignment::Center,
                        axis_alignment: crate::layout::Alignment::Stretch
                    ]
                );

                let circle = node!(
                    super::RoundedRect {
                        background_color: if self.selected {
                            active_color
                        } else {
                            background_color
                        },
                        border_color,
                        border_width: (border_width, border_width, border_width, border_width),
                        radius: self.radius,
                        scissor: None,
                        swipe: 0
                    },
                    lay!(
                        size: size_pct!(100.0),
                        padding: rect!(padding),
                        cross_alignment: crate::layout::Alignment::Center,
                        axis_alignment: crate::layout::Alignment::Center
                    )
                );

                let label = node!(super::Text::new(self.label.clone())
                    .style("size", self.style_val("font_size").unwrap())
                    .style("color", self.style_val("text_color").unwrap())
                    .style("h_alignment", HorizontalPosition::Center)
                    .maybe_style("font", self.style_val("font")));

                base = base.push(circle);
                base = base.push(label);

                Some(base)
            }
            RadioButtonsType::Group => {
                let mut base = node!(
                    Div::new(),
                    lay![
                        direction: Direction::Row,
                        cross_alignment: Alignment::Center,
                        padding: [22., 10.],
                        size_pct: [100, Auto],
                        axis_alignment: Alignment::Stretch
                    ]
                );

                let text = node!(Div::new().bg(Color::TRANSPARENT), lay![]).push(node!(Text::new(
                    self.label.clone()
                )
                .with_class("light text-l")
                .maybe_style("font", self.style_val("font"))
                .style("font_weight", FontWeight::Bold),));

                base = base.push(text);
                Some(base)
            }
            RadioButtonsType::Block => {
                let base = node!(
                    super::RoundedRect {
                        background_color: if self.selected {
                            active_color
                        } else {
                            background_color
                        },
                        border_color,
                        border_width: (border_width, border_width, border_width, border_width),
                        radius: self.radius,
                        scissor: None,
                        swipe: 0
                    },
                    lay!(
                        size: size_pct!(100.0),
                        padding: rect!(padding),
                        cross_alignment: crate::layout::Alignment::Center,
                        axis_alignment: crate::layout::Alignment::Center
                    )
                )
                .push(node!(super::Text::new(self.label.clone())
                    .style("size", self.style_val("font_size").unwrap())
                    .style("color", self.style_val("text_color").unwrap())
                    .style("h_alignment", HorizontalPosition::Center)
                    .maybe_style("font", self.style_val("font"))));

                Some(base)
            }
        }
    }

    fn on_click(&mut self, event: &mut event::Event<event::Click>) {
        event.stop_bubbling();
        event.emit(msg!(RadioButtonMsg::Clicked(self.position)));
    }

    // Same as on_click
    fn on_double_click(&mut self, event: &mut event::Event<event::DoubleClick>) {
        event.stop_bubbling();
        event.emit(msg!(RadioButtonMsg::Clicked(self.position)));
    }
}
