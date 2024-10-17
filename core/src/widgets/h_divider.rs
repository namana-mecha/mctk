use crate::layout::{Alignment, Direction};
use crate::{component, Color};
use crate::{component::Component, lay, node, size, size_pct, widgets::Div, Node};
use std::hash::Hash;

#[derive(Debug)]
pub struct HDivider {
    pub size: f32,
    pub color: Color,
}

impl Component for HDivider {
    fn props_hash(&self, hasher: &mut component::ComponentHasher) {
        (self.size as u32).hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new().border(self.color, self.size, (0., 0., 0., 0.)),
                lay![
                    direction: Direction::Row,
                    size_pct: [100, Auto],
                    cross_alignment: Alignment::Stretch
                ]
            )
            .push(node!(
                Div::new(),
                lay![
                    size: [ Auto, 1 ]
                ]
            )),
        )
    }
}
