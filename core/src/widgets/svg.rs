use std::hash::Hash;

use crate::component::{Component, ComponentHasher, RenderContext};

use crate::renderables::svg::InstanceBuilder;
use crate::renderables::types::{Point, Size};
use crate::renderables::{self, Rect, Renderable};
use crate::types::*;

#[derive(Debug)]
pub struct Svg {
    pub name: String,
    pub dynamic_load_from: Option<String>,
}

impl Default for Svg {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            dynamic_load_from: None,
        }
    }
}

impl Svg {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            dynamic_load_from: None,
        }
    }

    pub fn dynamic_load_from(mut self, v: Option<String>) -> Self {
        self.dynamic_load_from = v;
        self
    }
}

impl Component for Svg {
    fn render_hash(&self, hasher: &mut ComponentHasher) {
        self.name.hash(hasher);
    }

    fn render(&mut self, context: RenderContext) -> Option<Vec<Renderable>> {
        let scale = context.aabb.size();
        let pos = context.aabb.pos;

        let instance = InstanceBuilder::default()
            .pos(pos)
            .scale(scale)
            .name(self.name.clone())
            .dynamic_load_from(self.dynamic_load_from.clone())
            .build()
            .unwrap();

        Some(vec![Renderable::Svg(renderables::Svg::from_instance_data(
            instance,
        ))])
    }
}
