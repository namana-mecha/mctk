use std::fmt;
use std::hash::Hash;

use crate::component::{Component, ComponentHasher, Message};
use crate::layout::{Alignment, PositionType};
use crate::{event, lay, node, rect, size, txt, Color};

use crate::style::{FontWeight, Styled};
use mctk_macros::{component, state_component_impl};

use super::{Div, Text};

#[derive(Debug, Default)]
struct ToggleState {
    pressed: bool,
}

#[derive(Debug)]
pub enum ToggleType {
    Type1,
    Type2,
    Type3,
}

#[component(State = "ToggleState", Styled, Internal)]
pub struct Toggle {
    active: bool,
    toggle_type: ToggleType,
    on_change: Option<Box<dyn Fn(bool) -> Message + Send + Sync>>,
}

impl fmt::Debug for Toggle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Toggle")
            .field("active", &self.active)
            .finish()
    }
}

impl Toggle {
    pub fn new(active: bool) -> Self {
        Self {
            active,
            toggle_type: ToggleType::Type3,
            on_change: None,
            state: Some(ToggleState { pressed: active }),
            dirty: false,
            class: Default::default(),
            style_overrides: Default::default(),
        }
    }

    pub fn on_change(mut self, change_fn: Box<dyn Fn(bool) -> Message + Send + Sync>) -> Self {
        self.on_change = Some(change_fn);
        self
    }

    pub fn toggle_type(mut self, t: ToggleType) -> Self {
        self.toggle_type = t;
        self
    }
    fn toogle_type_1(&self) -> Option<crate::Node> {
        let background_color: Color = self.style_val("background_color").into();
        let active_color: Color = self.style_val("active_color").into();
        let border_color: Color = self.style_val("border_color").into();
        let highlight_color: Color = self.style_val("highlight_color").into();
        let border_width: f32 = self.style_val("border_width").unwrap().f32();
        let active = self.state_ref().pressed;

        let (width, height): (f64, f64) = (90., 42.);

        let mut base = node!(
            Div::new().bg(Color::TRANSPARENT),
            lay![
                size: [58., 38.],
                cross_alignment: Alignment::Center,
                padding: [0., 0.75, 0., 0.75]
            ]
        );

        let mut t_div = node!(
            Div::new()
                .bg(if active {
                    Color::rgb(45., 138., 255.)
                } else {
                    Color::rgb(255., 255., 255.)
                })
                .border(Color::TRANSPARENT, 1., (16., 16., 16., 16.)),
            lay![
                size: [58., 30.],
                cross_alignment: Alignment::Center,
                axis_alignment: if active {
                    Alignment::Start
                } else {
                    Alignment::End
                },
            ]
        );

        let m_div = node!(
            Div::new()
                .bg(if active {
                    Color::rgb(255., 255., 255.)
                } else {
                    Color::rgb(97., 97., 97.)
                })
                .border(Color::TRANSPARENT, 1., (50., 50., 50., 50.)),
            lay![
                position_type: PositionType::Absolute,
                position:  if active {rect!(0., Auto, 0., 0.)} else {rect!(0., 0., 0., 0.)},
                margin: if active  {rect![5., 0., 0., 5.]} else {rect![5., 2., 0., 0.]},
                size: [28., 28.]
            ]
        );

        base = base.push(m_div);
        base = base.push(t_div);

        Some(base)
    }

    fn toggle_type_2(&self) -> Option<crate::Node> {
        let background_color: Color = self.style_val("background_color").into();
        let active_color: Color = self.style_val("active_color").into();
        let border_color: Color = self.style_val("border_color").into();
        let highlight_color: Color = self.style_val("highlight_color").into();
        let border_width: f32 = self.style_val("border_width").unwrap().f32();
        let active = self.state_ref().pressed;

        let (width, height): (f64, f64) = (90., 42.);

        let mut base = node!(
            Div::new().bg(Color::TRANSPARENT),
            lay![
                size: [80., 40.],
                cross_alignment: Alignment::Center,
                padding: [0., 0.75, 0., 0.75]
            ]
        );

        let mut t_div = node!(
            Div::new()
                .bg(if active {
                    Color::rgb(2., 19., 55.)
                } else {
                    Color::rgb(25., 25., 25.)
                })
                .border(
                    if active {
                        Color::rgb(45., 138., 255.)
                    } else {
                        Color::rgb(132., 132., 132.)
                    },
                    2.5,
                    (0., 0., 0., 0.)
                ),
            lay![
                size: [76., 28.],
                cross_alignment: Alignment::Center,
                axis_alignment: if active {
                    Alignment::Start
                } else {
                    Alignment::End
                },
            ]
        );

        let m_div = node!(
            Div::new().bg(if active {
                Color::rgb(45., 138., 255.)
            } else {
                Color::rgb(132., 132., 132.)
            }),
            lay![
                position_type: PositionType::Absolute,
                position:  if active {rect!(0., Auto, 0., 0.)} else {rect!(0., 0., 0., 0.)},
                size: [38., 38.]
            ]
        );

        t_div = t_div.push(
            node!(
                Div::new(),
                lay![
                    margin:[0., 8., 0., 10.]
                ]
            )
            .push(node!(Text::new(txt!(if active { "ON" } else { "OFF" }))
                .with_class("text-sm text-white font-semibold leading-3")
                .style("font", "Space Grotesk"),)),
        );
        base = base.push(m_div);
        base = base.push(t_div);

        Some(base)
    }

    fn toggle_type_3(&self) -> Option<crate::Node> {
        let background_color: Color = self.style_val("background_color").into();
        let active_color: Color = self.style_val("active_color").into();
        let border_color: Color = self.style_val("border_color").into();
        let highlight_color: Color = self.style_val("highlight_color").into();
        let border_width: f32 = self.style_val("border_width").unwrap().f32();
        let active = self.state_ref().pressed;

        let (width, height): (f64, f64) = (90., 42.);

        let mut base = node!(
            Div::new().bg(Color::TRANSPARENT),
            lay![
                size: [58., 26.],
                cross_alignment: Alignment::Center,
                padding: [0., 0.75, 0., 0.8]
            ]
        );

        let mut t_div = node!(
            Div::new()
                .bg(if active {
                    Color::rgb(2., 19., 55.)
                } else {
                    Color::rgb(25., 25., 25.)
                })
                .border(
                    if active {
                        Color::rgba(45., 138., 255., 1.)
                    } else {
                        Color::rgba(219., 219., 219., 1.)
                    },
                    2.5,
                    (0., 0., 0., 0.)
                ),
            lay![
                size: [60., 26.],
                cross_alignment: Alignment::Center,
                axis_alignment: if active {
                    Alignment::Start
                } else {
                    Alignment::End
                },
            ]
        );

        let m_div = node!(
            Div::new()
                .bg(if active {
                    Color::rgba(45., 138., 255., 1.)
                } else {
                    Color::rgba(219., 219., 219., 1.)
                })
                .border(Color::TRANSPARENT, 1., (2., 2., 2., 2.)),
            lay![
                position_type: PositionType::Absolute,
                position:  if active {rect!(0., Auto, 0., 0.)} else {rect!(0., 0., 0., 0.)},
                size: [18., 18.],
                cross_alignment: Alignment::Center,
                margin: if active {rect!(3.5, 0., 0., 8.)} else {rect!(3.5, 4., 0., 4.)},
            ]
        );

        t_div = t_div.push(
            node!(
                Div::new(),
                lay![
                    margin: if active {rect!(0., 8., 0., 0.)} else {rect!(0., 0., 0., 10.)},
                ]
            )
            .push(node!(Text::new(txt!(if active { "ON" } else { "OFF" }))
                .with_class(" text-white font-bold leading-3")
                .style("font", "Space Grotesk"),)),
        );
        base = base.push(m_div);
        base = base.push(t_div);

        Some(base)
    }
}

#[state_component_impl(ToggleState)]
impl Component for Toggle {
    // fn on_mouse_leave(&mut self, _event: &mut event::Event<event::MouseLeave>) {
    //     self.state_mut().pressed = false;
    // }

    fn on_mouse_down(&mut self, _event: &mut event::Event<event::MouseDown>) {
        self.state_mut().pressed = !self.state_ref().pressed;
    }

    fn on_touch_down(&mut self, _event: &mut event::Event<event::TouchDown>) {
        self.state_mut().pressed = !self.state_ref().pressed;
    }

    // fn on_mouse_up(&mut self, _event: &mut event::Event<event::MouseUp>) {
    //     self.state_mut().pressed = false;
    // }

    fn on_click(&mut self, event: &mut event::Event<event::Click>) {
        if let Some(f) = &self.on_change {
            event.emit(f(self.state_ref().pressed));
        }
    }

    // Same as on_click
    fn on_double_click(&mut self, event: &mut event::Event<event::DoubleClick>) {
        if let Some(f) = &self.on_change {
            event.emit(f(self.state_ref().pressed));
        }
    }

    fn render_hash(&self, hasher: &mut ComponentHasher) {
        self.active.hash(hasher);
        self.state_ref().pressed.hash(hasher);
    }

    fn props_hash(&self, hasher: &mut ComponentHasher) {
        self.active.hash(hasher);
    }

    fn new_props(&mut self) {
        self.state_mut().pressed = self.active;
    }

    fn view(&self) -> Option<crate::Node> {
        let base = match self.toggle_type {
            ToggleType::Type1 => self.toogle_type_1(),
            ToggleType::Type2 => self.toggle_type_2(),
            ToggleType::Type3 => self.toggle_type_3(),
        };
        base
    }
}
