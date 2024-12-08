use std::collections::HashMap;

use crate::{Pos, Scale};

use super::types;
use super::types::Canvas;
use derive_builder::Builder;
use femtovg::{CompositeOperation, ImageFlags, ImageId, Paint, Path};
use image::{ImageBuffer, Rgb};
use imgref::{Img, ImgExt};
use rgb::{FromSlice, RGBA8};

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
    pub img_buffer: ImageBuffer<Rgb<u8>, Vec<u8>>,
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
                img_buffer: ImageBuffer::default(),
            },
        }
    }

    pub fn composite_operation(mut self, co: CompositeOperation) -> Self {
        self.instance_data.composite_operation = co;
        self
    }

    pub fn render(&self, canvas: &mut Canvas) {
        let Instance {
            pos,
            scale,
            composite_operation,
            radius,
            img_buffer,
            ..
        } = self.instance_data.clone();
        let Scale { width, height } = scale;

        println!("drawing raw image");

        let frame_rgb8 = img_buffer.as_rgb();

        // dont draw if nothing to draw
        if frame_rgb8.len() == 0 {
            return;
        }

        canvas.global_composite_operation(composite_operation);

        // create image in canvas
        let canvas_image_id = canvas
            .create_image(
                Img::new(
                    frame_rgb8,
                    img_buffer.width() as usize,
                    img_buffer.height() as usize,
                )
                .as_ref(),
                ImageFlags::empty(),
            )
            .unwrap();

        let Pos { x, y, z: _ } = pos;
        let Scale { width, height } = scale;

        let paint = Paint::image(canvas_image_id, x, y, width, height, 0.0, 1.0);
        let mut path = Path::new();
        path.rounded_rect(x, y, width, height, radius);
        canvas.fill_path(&path, &paint);

        println!("drawing raw image: done");

        canvas.global_composite_operation(CompositeOperation::SourceOver);
    }

    pub fn from_instance_data(instance_data: Instance) -> Self {
        Self { instance_data }
    }
}
