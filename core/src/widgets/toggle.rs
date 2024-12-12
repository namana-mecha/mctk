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

#[component(State = "ToggleState", Styled, Internal)]
pub struct Toggle {
    active: bool,
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
                size: [72., 40.],
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
                size: [72., 32.],
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
                margin: if active  {rect![5., 0., 0., 2.]} else {rect![5., 2., 0., 0.]},
                size: [30., 30.]
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
                .with_class(if active {
                    "text-sm text-white font-semibold leading-3"
                } else {
                    "text-sm text-gray font-semibold leading-3"
                })
                .style("font", "Space Grotesk"),)),
        );
        base = base.push(m_div);
        base = base.push(t_div);

        Some(base)
    }
}
