use std::sync::Mutex;

use image::{GenericImageView, ImageBuffer};
use lazy_static::lazy_static;
use mctk_camera::{
    camera::GstCamera,
    types::{CameraFormat, FrameFormat},
};
use mctk_core::{context::Context, reexports::femtovg::rgb::FromSlice};
use mctk_macros::Model;
use rgb::Rgb;

lazy_static! {
    pub static ref RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    pub static ref CAMERA: Camera = Camera {
        frame_buffer: Context::new(ImageBuffer::default()),
        is_initialized: Context::new(false),
        test_value: Context::new(0)
    };
    pub static ref GST_CAMERA: Mutex<Option<GstCamera>> = Mutex::new(None);
}

#[derive(Model)]
pub struct Camera {
    is_initialized: Context<bool>,
    pub frame_buffer: Context<ImageBuffer<image::Rgb<u8>, Vec<u8>>>,
    pub test_value: Context<u64>,
}

impl Camera {
    pub fn get() -> &'static Self {
        if !*CAMERA.is_initialized.get() {
            println!("initializing camera");
            let camera = match GstCamera::new(
                0,
                Some(CameraFormat::new_from(640, 480, FrameFormat::MJPEG, 30)),
            ) {
                Ok(mut c) => {
                    match c.open_stream() {
                        Ok(()) => {
                            println!("camera open success");
                        }
                        Err(err) => {
                            println!("failed to open camera stream: {}", err);
                        }
                    };
                    Some(c)
                }
                Err(e) => {
                    println!("failed to create camera, err - {:?}", e);
                    None
                }
            };
            *GST_CAMERA.lock().unwrap() = camera;
            CAMERA.is_initialized.set(true);
        }
        &CAMERA
    }

    pub fn get_buffer() -> Box<[Rgb<u8>]> {
        Box::from(Self::get().frame_buffer.get().as_rgb())
    }

    pub fn get_height() -> u32 {
        Self::get().frame_buffer.get().height()
    }

    pub fn get_width() -> u32 {
        Self::get().frame_buffer.get().width()
    }

    pub fn start_fetching() {
        RUNTIME.spawn(async move {
            let mut i = 0;
            std::thread::sleep(std::time::Duration::from_millis(2000));
            loop {
                i += 1;
                Camera::get();
                match GST_CAMERA.lock().unwrap().as_mut().unwrap().frame() {
                    Ok(f) => {
                        println!("got frame!");
                        Self::get().frame_buffer.set(f);
                    }
                    Err(e) => {
                        println!("error from frame {:?}", e);
                    }
                };
                std::thread::sleep(std::time::Duration::from_millis(1000 / 30));
            }
        });
    }
}
