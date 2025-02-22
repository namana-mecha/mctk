use std::time::Instant;

// use super::ToolTip;
use crate::component::{Component, Message};
use crate::font_cache::TextSegment;
use crate::style::Styled;
use crate::{event, lay, rect};
use crate::{node, node::Node};
use crate::{size_pct, types::*};
use mctk_macros::{component, state_component_impl};

#[derive(Debug, Default)]
struct ButtonState {
    hover: bool,
    pressed: bool,
    tool_tip_open: Option<Point>,
    hover_start: Option<Instant>,
}

#[component(State = "ButtonState", Styled, Internal)]
pub struct Button {
    pub label: Vec<TextSegment>,
    pub on_click: Option<Box<dyn Fn() -> Message + Send + Sync>>,
    pub on_press: Option<Box<dyn Fn() -> Message + Send + Sync>>,
    pub on_release: Option<Box<dyn Fn() -> Message + Send + Sync>>,
    pub on_double_click: Option<Box<dyn Fn() -> Message + Send + Sync>>,
    pub tool_tip: Option<String>,
}

impl std::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Button")
            .field("label", &self.label)
            .finish()
    }
}

impl Button {
    pub fn new(label: Vec<TextSegment>) -> Self {
        Self {
            label,
            on_click: None,
            on_double_click: None,
            tool_tip: None,
            on_press: None,
            on_release: None,
            state: Some(ButtonState::default()),
            dirty: false,
            class: Default::default(),
            style_overrides: Default::default(),
        }
    }

    pub fn on_click(mut self, f: Box<dyn Fn() -> Message + Send + Sync>) -> Self {
        self.on_click = Some(f);
        self
    }

    pub fn on_press(mut self, f: Box<dyn Fn() -> Message + Send + Sync>) -> Self {
        self.on_press = Some(f);
        self
    }
    pub fn on_release(mut self, f: Box<dyn Fn() -> Message + Send + Sync>) -> Self {
        self.on_release = Some(f);
        self
    }

    pub fn on_double_click(mut self, f: Box<dyn Fn() -> Message + Send + Sync>) -> Self {
        self.on_double_click = Some(f);
        self
    }

    pub fn tool_tip(mut self, t: String) -> Self {
        self.tool_tip = Some(t);
        self
    }
}

#[state_component_impl(ButtonState)]
impl Component for Button {
    fn view(&self) -> Option<Node> {
        let radius: f32 = self.style_val("radius").unwrap().f32();
        let padding: f64 = self.style_val("padding").unwrap().into();
        let active_color: Color = self.style_val("active_color").into();
        let highlight_color: Color = self.style_val("highlight_color").into();
        let background_color: Color = self.style_val("background_color").into();
        let border_color: Color = self.style_val("border_color").into();
        let border_width: f32 = self.style_val("border_width").unwrap().f32();

        let mut base = node!(
            super::RoundedRect {
                background_color: if self.state_ref().pressed {
                    active_color
                } else if self.state_ref().hover {
                    highlight_color
                } else {
                    background_color
                },
                border_color,
                border_width: (border_width, border_width, border_width, border_width),
                radius: (radius, radius, radius, radius),
                ..Default::default()
            },
            lay!(
                size: size_pct!(100.0),
                padding: rect!(padding),
                margin: rect!(border_width / 2.0),
                cross_alignment: crate::layout::Alignment::Center,
                axis_alignment: crate::layout::Alignment::Center,
            )
        )
        .push(node!(super::Text::new(self.label.clone())
            .style("size", self.style_val("font_size").unwrap())
            .style("color", self.style_val("text_color").unwrap())
            .style("h_alignment", self.style_val("h_alignment").unwrap())
            .maybe_style("font", self.style_val("font"))
            .maybe_style("font_weight", self.style_val("font_weight"))
            .maybe_style("line_height", self.style_val("line_height")),));

        // if let (Some(p), Some(tt)) = (self.state_ref().tool_tip_open, self.tool_tip.as_ref()) {
        //     base = base.push(node!(
        //         ToolTip::new(tt.clone()),
        //         lay!(position_type: PositionType::Absolute,
        //              z_index_increment: 1000.0,
        //              position: (p + ToolTip::MOUSE_OFFSET).into(),
        //         ),
        //     ));
        // }

        Some(base)
    }

    fn on_mouse_motion(&mut self, event: &mut event::Event<event::MouseMotion>) {
        let dirty = self.dirty;
        self.state_mut().hover_start = Some(Instant::now());
        // This state mutation should not trigger a redraw. We use whatever value was previously set.
        self.dirty = dirty;
        // event.stop_bubbling();
    }

    fn on_mouse_enter(&mut self, _event: &mut event::Event<event::MouseEnter>) {
        // self.state_mut().hover = true;
        // if let Some(w) = current_window() {
        //     w.set_cursor("PointingHand");
        // }
    }

    fn on_mouse_leave(&mut self, _event: &mut event::Event<event::MouseLeave>) {
        // *self.state_mut() = ButtonState::default();
        // if let Some(w) = current_window() {
        //     w.unset_cursor();
        // }
    }

    fn on_tick(&mut self, event: &mut event::Event<event::Tick>) {
        // if self.state_ref().hover_start.is_some()
        //     && self
        //         .state_ref()
        //         .hover_start
        //         .map(|s| s.elapsed().as_millis() > ToolTip::DELAY)
        //         .unwrap_or(false)
        //     && self.state_ref().tool_tip_open.is_none()
        // {
        //     self.state_mut().tool_tip_open = Some(event.relative_logical_position());
        // }
    }

    fn on_touch_drag_start(&mut self, event: &mut event::Event<event::TouchDragStart>) {
        event.stop_bubbling();
        self.state_mut().pressed = false;
    }

    fn on_drag_start(&mut self, event: &mut event::Event<event::DragStart>) {
        event.stop_bubbling();
        self.state_mut().pressed = false;
    }

    fn on_drag_end(&mut self, _event: &mut event::Event<event::DragEnd>) {
        self.state_mut().pressed = false;
    }

    fn on_touch_drag_end(&mut self, _event: &mut event::Event<event::TouchDragEnd>) {
        self.state_mut().pressed = false;
    }

    fn on_mouse_down(&mut self, event: &mut event::Event<event::MouseDown>) {
        self.state_mut().pressed = true;
        if let Some(f) = &self.on_press {
            event.emit(f());
        }
    }

    fn on_mouse_up(&mut self, event: &mut event::Event<event::MouseUp>) {
        self.state_mut().pressed = false;
        if let Some(f) = &self.on_release {
            event.emit(f());
        }
    }

    fn on_click(&mut self, event: &mut event::Event<event::Click>) {
        if let Some(f) = &self.on_click {
            event.emit(f());
        }
    }

    fn on_touch_down(&mut self, event: &mut event::Event<event::TouchDown>) {
        self.state_mut().pressed = true;
        if let Some(f) = &self.on_press {
            event.emit(f());
        }
    }

    fn on_touch_up(&mut self, event: &mut event::Event<event::TouchUp>) {
        self.state_mut().pressed = false;
        if let Some(f) = &self.on_release {
            event.emit(f());
        }
    }

    fn on_double_click(&mut self, event: &mut event::Event<event::DoubleClick>) {
        if let Some(f) = &self.on_double_click {
            event.emit(f());
        }
    }
}
