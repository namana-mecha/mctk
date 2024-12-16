use std::{collections::HashMap, sync::Mutex};

use crate::{Pos, Scale};

use super::types;
use super::types::Canvas;
use derive_builder::Builder;
use femtovg::{CompositeOperation, ImageFlags, ImageId, Paint, Path};
use image::ImageBuffer;
use imgref::{Img, ImgExt};
use rgb::{FromSlice, Rgb, RGBA8};

type Point = types::Point<f32>;
type Size = types::Size<f32>;

#[derive(Clone, Debug, PartialEq, Builder)]
pub struct Instance {
    pub pos: Pos,
    pub scale: Scale,
    #[builder(default = "CompositeOperation::SourceOver")]
    pub composite_operation: CompositeOperation,
    #[builder(default = "0.0")]
    pub radius: f32,
    pub height: usize,
    pub width: usize,
    pub img_buffer: Option<Box<[Rgb<u8>]>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RawImage {
    pub instance_data: Instance,
}

impl RawImage {
    pub fn new<S: Into<String>>(pos: Pos, scale: Scale) -> Self {
        Self {
            instance_data: Instance {
                pos,
                scale,
                composite_operation: CompositeOperation::SourceOver,
                radius: Default::default(),
                height: 0,
                width: 0,
                img_buffer: None,
            },
        }
    }

    pub fn composite_operation(mut self, co: CompositeOperation) -> Self {
        self.instance_data.composite_operation = co;
        self
    }

    pub fn render(&self, canvas: &mut Canvas) -> Option<ImageId> {
        let Instance {
            pos,
            scale,
            composite_operation,
            radius,
            height: image_height,
            width: image_width,
            img_buffer,
            ..
        } = self.instance_data.clone();
        let Scale { width, height } = scale;

        // let frame_rgb8 = img_buffer.as_rgb();

        canvas.global_composite_operation(composite_operation);

        let image = img_buffer.as_ref()?.as_ref();
        if image.len() == 0 {
            return None;
        }

        // create image in canvas
        let canvas_image_id = canvas
            .create_image(
                Img::new(image, image_width, image_height).as_ref(),
                ImageFlags::empty(),
            )
            .unwrap();

        let Pos { x, y, z: _ } = pos;
        let Scale { width, height } = scale;

        let paint = Paint::image(canvas_image_id, x, y, width, height, 0.0, 1.0);
        let mut path = Path::new();
        path.rounded_rect(x, y, width, height, radius);
        canvas.fill_path(&path, &paint);
        canvas.global_composite_operation(CompositeOperation::SourceOver);
        Some(canvas_image_id)
    }

    pub fn from_instance_data(instance_data: Instance) -> Self {
        Self { instance_data }
    }
}
