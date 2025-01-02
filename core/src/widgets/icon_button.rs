use std::hash::Hash;
use std::time::Instant;

// use super::ToolTip;
use crate::component::{Component, Message};
use crate::font_cache::TextSegment;
use crate::layout::Size;
use crate::style::{HorizontalPosition, Styled};
use crate::{event, lay, rect};
use crate::{node, node::Node};
use crate::{size, size_pct, types::*};
use mctk_macros::{component, state_component_impl};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IconType {
    Svg,
    Png,
}

#[derive(Debug, Default)]
struct IconButtonState {
    hover: bool,
    pressed: bool,
    tool_tip_open: Option<Point>,
    hover_start: Option<Instant>,
}

#[component(State = "IconButtonState", Styled, Internal)]
pub struct IconButton {
    pub icon: String,
    pub icon_type: IconType,
    pub on_click: Option<Box<dyn Fn() -> Message + Send + Sync>>,
    pub tool_tip: Option<String>,
    pub on_press: Option<Box<dyn Fn() -> Message + Send + Sync>>,
    pub on_release: Option<Box<dyn Fn() -> Message + Send + Sync>>,
    pub disabled: bool,
    pub dynamic_load_icon: Option<String>,
}

impl std::fmt::Debug for IconButton {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IconButton")
            .field("icon", &self.icon)
            .finish()
    }
}

impl IconButton {
    pub fn new<S: Into<String>>(icon: S) -> Self {
        Self {
            icon: icon.into(),
            icon_type: IconType::Svg,
            on_click: None,
            tool_tip: None,
            on_press: None,
            on_release: None,
            disabled: false,
            dynamic_load_icon: None,
            state: Some(IconButtonState::default()),
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

    pub fn tool_tip(mut self, t: String) -> Self {
        self.tool_tip = Some(t);
        self
    }

    pub fn icon_type(mut self, it: IconType) -> Self {
        self.icon_type = it;
        self
    }

    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }

    pub fn dynamic_load_icon(mut self, p: String) -> Self {
        self.dynamic_load_icon = Some(p);
        self
    }
}

#[state_component_impl(IconButtonState)]
impl Component for IconButton {
    fn view(&self) -> Option<Node> {
        let radius: f32 = self.style_val("radius").unwrap().f32();
        let padding: f64 = self.style_val("padding").unwrap().into();
        let active_color: Color = self.style_val("active_color").into();
        let highlight_color: Color = self.style_val("highlight_color").into();
        let background_color: Color = self.style_val("background_color").into();
        let border_color: Color = self.style_val("border_color").into();
        let border_width: f32 = self.style_val("border_width").unwrap().f32();
        let size: Size = self.style_val("size").unwrap().into();
        let (width, height) = size.fixed();

        let icon = match self.icon_type {
            IconType::Svg => node!(
                super::Svg::new(self.icon.clone())
                    .dynamic_load_from(self.dynamic_load_icon.clone()),
                lay![
                    size: [width as f64 - padding, height as f64 - padding],
                ],
            ),
            IconType::Png => node!(
                super::Image::new(self.icon.clone())
                    .dynamic_load_from(self.dynamic_load_icon.clone()),
                lay![
                    size: [width as f64 - padding, height as f64 - padding],
                ],
            ),
        };

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
                size: [width as f64, height as f64],
                padding: rect!(padding),
                margin: rect!(border_width / 2.0),
                cross_alignment: crate::layout::Alignment::Center,
                axis_alignment: crate::layout::Alignment::Center,
            )
        )
        .push(icon);

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
        // *self.state_mut() = IconButtonState::default();
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
        if self.disabled {
            return;
        }

        event.stop_bubbling();
        self.state_mut().pressed = false;
    }

    fn on_drag_start(&mut self, event: &mut event::Event<event::DragStart>) {
        if self.disabled {
            return;
        }
        event.stop_bubbling();
        self.state_mut().pressed = false;
    }

    fn on_drag_end(&mut self, _event: &mut event::Event<event::DragEnd>) {
        if self.disabled {
            return;
        }
        self.state_mut().pressed = false;
    }

    fn on_touch_drag_end(&mut self, _event: &mut event::Event<event::TouchDragEnd>) {
        if self.disabled {
            return;
        }
        self.state_mut().pressed = false;
    }

    fn on_mouse_down(&mut self, event: &mut event::Event<event::MouseDown>) {
        if self.disabled {
            return;
        }
        self.state_mut().pressed = true;
        if let Some(f) = &self.on_press {
            event.emit(f());
        }
    }

    fn on_mouse_up(&mut self, event: &mut event::Event<event::MouseUp>) {
        if self.disabled {
            return;
        }
        self.state_mut().pressed = false;
        if let Some(f) = &self.on_release {
            event.emit(f());
        }
    }

    fn on_touch_down(&mut self, event: &mut event::Event<event::TouchDown>) {
        if self.disabled {
            return;
        }
        self.state_mut().pressed = true;
        if let Some(f) = &self.on_press {
            event.emit(f());
        }
    }

    fn on_touch_up(&mut self, event: &mut event::Event<event::TouchUp>) {
        if self.disabled {
            return;
        }
        self.state_mut().pressed = false;
        if let Some(f) = &self.on_release {
            event.emit(f());
        }
    }

    fn on_click(&mut self, event: &mut event::Event<event::Click>) {
        if self.disabled {
            return;
        }
        if let Some(f) = &self.on_click {
            event.emit(f());
        }
    }
}
