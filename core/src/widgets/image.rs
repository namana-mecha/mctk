use std::hash::Hash;

use image::ImageBuffer;
use mctk_macros::component;
use rand::random;
use rgb::{FromSlice, Rgb};

use crate::component::{Component, ComponentHasher, RenderContext};

use crate::renderables::image::InstanceBuilder as ImageInstanceBuilder;
use crate::renderables::raw_image::InstanceBuilder as RawImageInstanceBuilder;
use crate::renderables::types::{Point, Size};
use crate::renderables::{self, Rect, Renderable};
use crate::style::{self, Styled};
use crate::types::*;

#[derive(Debug, PartialEq)]
pub enum ImageSource {
    Asset,
    Path,
    Buffer,
}

#[component(Styled)]
#[derive(Debug)]
pub struct Image {
    pub name: String,
    pub src: ImageSource,
    pub path: String,
    pub height: usize,
    pub width: usize,
    pub buffer: Box<[Rgb<u8>]>,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            path: "".to_string(),
            buffer: Box::from([]),
            height: 0,
            width: 0,
            src: ImageSource::Asset,
            class: Default::default(),
            style_overrides: Default::default(),
        }
    }
}

impl Image {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            path: Default::default(),
            buffer: Box::from([]),
            height: 0,
            width: 0,
            src: ImageSource::Asset,
            class: Default::default(),
            style_overrides: Default::default(),
        }
    }

    pub fn from_buffer(buffer: Box<[Rgb<u8>]>, width: usize, height: usize) -> Self {
        Self {
            name: "".to_string(),
            path: "".to_string(),
            src: ImageSource::Buffer,
            class: Default::default(),
            style_overrides: Default::default(),
            width,
            height,
            buffer,
        }
    }
}

impl Component for Image {
    fn render_hash(&self, hasher: &mut ComponentHasher) {
        match self.src {
            ImageSource::Asset => {
                return self.name.hash(hasher);
            }
            ImageSource::Path => {
                return self.path.hash(hasher);
            }
            ImageSource::Buffer => {
                // return random always, force to re-render
                // if someone wants to use constant buffer
                // then convert to path and use
                return random::<u8>().hash(hasher);
            }
        }
    }

    fn render(&mut self, context: RenderContext) -> Option<Vec<Renderable>> {
        let src = &self.src;
        let width = context.aabb.width();
        let height = context.aabb.height();
        let AABB { pos, .. } = context.aabb;
        let radius = self.style_val("radius").unwrap().f32();

        let renderables = match src {
            ImageSource::Asset => {
                let instance = ImageInstanceBuilder::default()
                    .pos(pos)
                    .scale(Scale { width, height })
                    .name(self.name.clone())
                    .radius(radius)
                    .build()
                    .unwrap();

                vec![Renderable::Image(renderables::Image::from_instance_data(
                    instance,
                ))]
            }
            ImageSource::Path => {
                // to be implemented
                vec![]
            }
            ImageSource::Buffer => {
                let instance = RawImageInstanceBuilder::default()
                    .pos(pos)
                    .scale(Scale { width, height })
                    .radius(radius)
                    // todo! figure out without cloning
                    .height(self.height)
                    .width(self.width)
                    .img_buffer(Some(self.buffer.clone()))
                    .build()
                    .unwrap();

                vec![Renderable::RawImage(
                    renderables::RawImage::from_instance_data(instance),
                )]
            }
        };

        Some(renderables)
    }
}
