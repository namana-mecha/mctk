use std::{collections::HashMap, sync::Mutex};

use crate::{Pos, Scale};

use super::types;
use super::types::Canvas;
use derive_builder::Builder;
use femtovg::{CompositeOperation, ImageFlags, ImageId, Paint, Path};
use image::{flat::Error, ImageBuffer};
use imgref::{Img, ImgExt};
use rgb::{ComponentBytes, FromSlice, Rgba, RGBA8};

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
    pub img_buffer: Option<Box<[Rgba<u8>]>>,
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

        // let image = yuyv422_to_rgb(img_buffer.as_ref()?.as_bytes(), true);
        let image: Vec<u8> = img_buffer.as_ref()?.as_bytes().to_vec();
        if image.len() == 0 {
            return None;
        }

        // create image in canvas
        let canvas_image_id = canvas
            .create_image(
                Img::new(image.as_rgba(), image_width, image_height).as_ref(),
                ImageFlags::empty(),
            )
            .unwrap();

        let Pos { x, y, z: _ } = pos;
        let Scale { width, height } = scale;

        let paint = Paint::image(canvas_image_id, x, y, width, height, 0.0, 1.0);
        let mut path = Path::new();
        path.rounded_rect(x, y, width, height, radius);
        canvas.fill_path_yuyv(&path, &paint);
        canvas.global_composite_operation(CompositeOperation::SourceOver);
        Some(canvas_image_id)
    }

    pub fn from_instance_data(instance_data: Instance) -> Self {
        Self { instance_data }
    }
}

pub fn buf_yuyv422_to_rgb(data: &[u8], dest: &mut [u8], rgba: bool) {
    let pixel_size = if rgba { 4 } else { 3 };
    let rgb_buf_size = (data.len() / 4) * (2 * pixel_size);
    let mut buf: Vec<u8> = Vec::new();
    for chunk in data.chunks_exact(4) {
        let y0 = f32::from(chunk[0]);
        let u = f32::from(chunk[1]);
        let y1 = f32::from(chunk[2]);
        let v = f32::from(chunk[3]);

        let r0 = y0 + 1.370_705 * (v - 128.);
        let g0 = y0 - 0.698_001 * (v - 128.) - 0.337_633 * (u - 128.);
        let b0 = y0 + 1.732_446 * (u - 128.);

        let r1 = y1 + 1.370_705 * (v - 128.);
        let g1 = y1 - 0.698_001 * (v - 128.) - 0.337_633 * (u - 128.);
        let b1 = y1 + 1.732_446 * (u - 128.);

        if rgba {
            buf.extend_from_slice(&[
                r0 as u8, g0 as u8, b0 as u8, 255, r1 as u8, g1 as u8, b1 as u8, 255,
            ]);
        } else {
            buf.extend_from_slice(&[r0 as u8, g0 as u8, b0 as u8, r1 as u8, g1 as u8, b1 as u8]);
        }
    }
    dest.copy_from_slice(&buf);
}

pub fn yuyv422_to_rgb(data: &[u8], rgba: bool) -> Vec<u8> {
    let pixel_size = if rgba { 4 } else { 3 };
    // yuyv yields 2 3-byte pixels per yuyv chunk
    let rgb_buf_size = (data.len() / 4) * (2 * pixel_size);

    let mut dest = vec![0; rgb_buf_size];
    buf_yuyv422_to_rgb(data, &mut dest, rgba);

    dest
}
