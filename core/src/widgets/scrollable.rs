use std::ops::Neg;

use super::{Div, RoundedRect};
use crate::component::Component;
use crate::layout::{Direction, PositionType, ScrollPosition, Size};
use crate::prelude::Message;
use crate::types::*;
use crate::{event, lay, rect, size};
use crate::{node, node::Node};
use mctk_macros::{component, state_component_impl};

#[derive(Debug, Default)]
pub struct ScrollableState {
    //Current scroll position
    scroll_position: Point,

    //Position of scrollable when drag was started
    drag_start_position: Point,

    aabb: Option<AABB>,
}

#[component(State = "ScrollableState", Styled, Internal)]
// #[derive(Debug, Default)]
pub struct Scrollable {
    pub size: Size,
    pub on_scroll_end: Option<Box<dyn Fn() -> Message + Send + Sync>>,
}

impl std::fmt::Debug for Scrollable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Scrollable")
            .field("size", &self.size)
            .finish()
    }
}

impl Scrollable {
    pub fn new(s: Size) -> Self {
        Self {
            state: Some(ScrollableState::default()),
            dirty: false,
            size: s,
            class: Default::default(),
            style_overrides: Default::default(),
            on_scroll_end: None,
        }
    }

    pub fn on_scroll_end(mut self, f: Box<dyn Fn() -> Message + Send + Sync>) -> Self {
        self.on_scroll_end = Some(f);
        self
    }
}

#[state_component_impl(ScrollableState)]
impl Component for Scrollable {
    fn render_hash(&self, hasher: &mut crate::component::ComponentHasher) {
        // if self.state.is_some() {
        //     self.state_ref().scroll_position.hash(hasher);
        // }
        // println!("Scrollable::render_hash() {:?}", hasher.finish());
    }

    fn on_drag_end(&mut self, event: &mut crate::event::Event<crate::event::DragEnd>) {
        if let Some(f) = &self.on_scroll_end {
            event.emit(f());
        }
    }

    fn on_drag_start(&mut self, event: &mut crate::event::Event<crate::event::DragStart>) {
        event.stop_bubbling();
        //Current scroll position will become drag start position when drag is started
        let drag_start = self.state_ref().scroll_position;
        self.state_mut().drag_start_position = drag_start;
    }

    fn on_touch_drag_start(
        &mut self,
        event: &mut crate::event::Event<crate::event::TouchDragStart>,
    ) {
        event.stop_bubbling();
        //Current scroll position will become drag start position when drag is started
        let drag_start = self.state_ref().scroll_position;
        self.state_mut().drag_start_position = drag_start;
    }

    fn on_drag(&mut self, event: &mut crate::event::Event<crate::event::Drag>) {
        //on drag we will update scroll position
        let start_position = self.state_ref().drag_start_position;
        let size = event.current_physical_aabb().size();
        let inner_scale = event.current_inner_scale().unwrap();
        let mut scroll_position = self.state_ref().scroll_position;
        let drag = event.physical_delta().y.neg();
        let delta_position = drag * (inner_scale.height / size.height);
        let max_position = inner_scale.height - size.height;
        scroll_position.y = (start_position.y + delta_position)
            .round()
            .min(max_position)
            .max(0.0);
        self.state_mut().scroll_position = scroll_position;
        // println!("scroll_position {:?}", scroll_position);
    }

    fn on_touch_drag(&mut self, event: &mut crate::event::Event<crate::event::TouchDrag>) {
        //on drag we will update scroll position
        let start_position = self.state_ref().drag_start_position;
        let size = event.current_physical_aabb().size();
        let inner_scale = event.current_inner_scale().unwrap();
        let mut scroll_position = self.state_ref().scroll_position;
        let drag = event.physical_delta().y.neg();
        let delta_position = drag * (inner_scale.height / size.height);
        let max_position = inner_scale.height - size.height;
        scroll_position.y = (start_position.y + delta_position)
            .round()
            .min(max_position)
            .max(0.0);
        self.state_mut().scroll_position = scroll_position;
        // println!("scroll_position {:?}", scroll_position);
    }

    fn container(&self) -> Option<Vec<usize>> {
        Some(vec![0, 1])
    }

    fn scroll_position(&self) -> Option<ScrollPosition> {
        let p = self.state_ref().scroll_position;
        Some(ScrollPosition {
            x: None,
            y: Some(p.y),
        })
    }

    fn full_control(&self) -> bool {
        true
    }

    fn set_aabb(
        &mut self,
        aabb: &mut AABB,
        _parent_aabb: AABB,
        _children: Vec<(&mut AABB, Option<Scale>, Option<Point>)>,
        _frame: AABB,
        _scale_factor: f32,
    ) {
        aabb.set_scale(self.size.width.into(), self.size.height.into());
    }

    fn view(&self) -> Option<Node> {
        let size = self.size;
        let scroll_y = self.state_ref().scroll_position.y;

        Some(
            node!(
                Div::new(),
                lay![
                    size: [Auto]
                ]
            )
            .key(scroll_y as u64)
            .push(node!(
                RoundedRect {
                    scissor: Some(false),
                    background_color: Color::TRANSPARENT,
                    border_color: Color::TRANSPARENT,
                    border_width: (0., 0., 0., 0.),
                    radius: (0., 0., 0., 0.),
                    swipe: 0
                },
                lay![
                    size: [size.width, size.height],
                    position_type: PositionType::Absolute,
                    position: [0., 0., 0., 0.]
                ]
            ))
            .push(node!(
                Div::new(),
                lay![
                    direction: Direction::Column,
                ]
            ))
            .push(node!(
                RoundedRect {
                    scissor: Some(true),
                    background_color: Color::TRANSPARENT,
                    border_color: Color::TRANSPARENT,
                    border_width: (0., 0., 0., 0.),
                    radius: (0., 0., 0., 0.),
                    swipe: 0
                },
                lay![
                    size: [size.width, size.height],
                    position_type: PositionType::Absolute,
                    position: [0., 0., 0., 0.]
                ]
            )),
        )
    }
}
