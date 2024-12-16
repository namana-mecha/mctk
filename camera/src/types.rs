use crate::error::CameraGstError;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Resolution {
    pub width_x: u32,
    pub height_y: u32,
}

impl Resolution {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            width_x: x,
            height_y: y,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum FrameFormat {
    MJPEG,
    YUYV,
    NV12,
    GRAY,
    RAWRGB,
}

impl Display for FrameFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FrameFormat::MJPEG => {
                write!(f, "MJPG")
            }
            FrameFormat::YUYV => {
                write!(f, "YUYV")
            }
            FrameFormat::GRAY => {
                write!(f, "GRAY")
            }
            FrameFormat::RAWRGB => {
                write!(f, "RAWRGB")
            }
            FrameFormat::NV12 => {
                write!(f, "NV12")
            }
        }
    }
}

impl FromStr for FrameFormat {
    type Err = CameraGstError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MJPEG" => Ok(FrameFormat::MJPEG),
            "YUYV" => Ok(FrameFormat::YUYV),
            "GRAY" => Ok(FrameFormat::GRAY),
            "RAWRGB" => Ok(FrameFormat::RAWRGB),
            "NV12" => Ok(FrameFormat::NV12),
            _ => Err(CameraGstError::StructureError {
                structure: "FrameFormat".to_string(),
                error: format!("No match for {s}"),
            }),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct CameraFormat {
    resolution: Resolution,
    format: FrameFormat,
    frame_rate: u32,
}

impl Default for CameraFormat {
    fn default() -> Self {
        Self {
            resolution: Resolution::new(640, 480),
            format: FrameFormat::MJPEG,
            frame_rate: 30,
        }
    }
}

impl CameraFormat {
    /// create a new CameraFormat
    pub fn new(resolution: Resolution, format: FrameFormat, frame_rate: u32) -> Self {
        Self {
            resolution,
            format,
            frame_rate,
        }
    }

    /// create a new CameraFormat from a resolution and a format
    pub fn new_from(res_x: u32, res_y: u32, format: FrameFormat, fps: u32) -> Self {
        CameraFormat {
            resolution: Resolution {
                width_x: res_x,
                height_y: res_y,
            },
            format,
            frame_rate: fps,
        }
    }

    /// get camera resolution width
    pub fn width(&self) -> u32 {
        self.resolution.width_x
    }

    /// get camera resolution height
    pub fn height(&self) -> u32 {
        self.resolution.height_y
    }

    /// get camera resolution
    pub fn resolution(&self) -> Resolution {
        self.resolution
    }

    /// get camera frame rate
    pub fn frame_rate(&self) -> u32 {
        self.frame_rate
    }

    /// get camera frame format
    pub fn format(&self) -> FrameFormat {
        self.format
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub struct CameraInfo {
    human_name: String,
    description: String,
    misc: String,
    index: usize,
}

impl CameraInfo {
    pub fn new(human_name: &str, description: &str, misc: &str, index: usize) -> Self {
        CameraInfo {
            human_name: human_name.to_string(),
            description: description.to_string(),
            misc: misc.to_string(),
            index,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum CameraIndex {
    Index(u32),
    String(String),
}

pub fn mjpeg_to_rgb24(in_buf: &[u8]) -> Result<Vec<u8>, CameraGstError> {
    let mut decoder = jpeg_decoder::Decoder::new(in_buf);

    let d = match decoder.decode() {
        Ok(d) => d,
        Err(err) => {
            return Err(CameraGstError::ProcessFrameError {
                src: FrameFormat::MJPEG,
                destination: "RGB888".to_string(),
                error: format!("Could not decode MJPEG: {}", err),
            });
        }
    };

    Ok(d)
}
pub fn yuyv422_to_rgb(data: &[u8], rgba: bool) -> Result<Vec<u8>, CameraGstError> {
    if data.len() % 4 != 0 {
        return Err(CameraGstError::ProcessFrameError {
            src: FrameFormat::YUYV,
            destination: "RGB888".to_string(),
            error: "Assertion failure, the YUV stream isn't 4:2:2! (wrong number of bytes)"
                .to_string(),
        });
    }

    let pixel_size = if rgba { 4 } else { 3 };
    // yuyv yields 2 3-byte pixels per yuyv chunk
    let rgb_buf_size = (data.len() / 4) * (2 * pixel_size);

    let mut dest = vec![0; rgb_buf_size];
    buf_yuyv422_to_rgb(data, &mut dest, rgba)?;

    Ok(dest)
}

/// Same as [`yuyv422_to_rgb`] but with a destination buffer instead of a return `Vec<u8>`
/// # Errors
/// If the stream is invalid YUYV, or the destination buffer is not large enough, this will error.
pub fn buf_yuyv422_to_rgb(data: &[u8], dest: &mut [u8], rgba: bool) -> Result<(), CameraGstError> {
    if data.len() % 4 != 0 {
        return Err(CameraGstError::ProcessFrameError {
            src: FrameFormat::YUYV,
            destination: "RGB888".to_string(),
            error: "Assertion failure, the YUV stream isn't 4:2:2! (wrong number of bytes)"
                .to_string(),
        });
    }

    let pixel_size = if rgba { 4 } else { 3 };
    // yuyv yields 2 3-byte pixels per yuyv chunk
    let rgb_buf_size = (data.len() / 4) * (2 * pixel_size);

    if dest.len() != rgb_buf_size {
        return Err(CameraGstError::ProcessFrameError {
            src: FrameFormat::YUYV,
            destination: "RGB888".to_string(),
            error: format!("Assertion failure, the destination RGB buffer is of the wrong size! [expected: {rgb_buf_size}, actual: {}]", dest.len()),
        });
    }

    let mut buf: Vec<u8> = Vec::new();
    // let iter = data.chunks_exact(4);
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

    Ok(())
}

// equation from https://en.wikipedia.org/wiki/YUV#Converting_between_Y%E2%80%B2UV_and_RGB
/// Convert `YCbCr` 4:4:4 to a RGB888. [For further reading](https://en.wikipedia.org/wiki/YUV#Converting_between_Y%E2%80%B2UV_and_RGB)
#[allow(clippy::many_single_char_names)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[must_use]
#[inline]
pub fn yuyv444_to_rgb(y: i32, u: i32, v: i32) -> [u8; 3] {
    let c298 = (y - 16) * 298;
    let d = u - 128;
    let e = v - 128;
    let r = ((c298 + 409 * e + 128) >> 8) as u8;
    let g = ((c298 - 100 * d - 208 * e + 128) >> 8) as u8;
    let b = ((c298 + 516 * d + 128) >> 8) as u8;
    [r, g, b]
}

#[allow(clippy::many_single_char_names)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[must_use]
#[inline]
pub fn yuv422_to_rgb(y: i32, u: i32, v: i32) -> [u8; 3] {
    let c298 = (y - 16) * 298;
    let d = u - 128;
    let e = v - 128;
    let r = ((c298 + 409 * e + 128) >> 8).clamp(0, 255) as u8;
    let g = ((c298 - 100 * d - 208 * e + 128) >> 8).clamp(0, 255) as u8;
    let b = ((c298 + 516 * d + 128) >> 8).clamp(0, 255) as u8;
    [r, g, b]
}

// equation from https://en.wikipedia.org/wiki/YUV#Converting_between_Y%E2%80%B2UV_and_RGB
/// Convert `YCbCr` 4:4:4 to a RGBA8888. [For further reading](https://en.wikipedia.org/wiki/YUV#Converting_between_Y%E2%80%B2UV_and_RGB)
///
/// Equivalent to [`yuyv444_to_rgb`] but with an alpha channel attached.
#[allow(clippy::many_single_char_names)]
#[must_use]
#[inline]
pub fn yuyv444_to_rgba(y: i32, u: i32, v: i32) -> [u8; 4] {
    let [r, g, b] = yuyv444_to_rgb(y, u, v);
    [r, g, b, 255]
}
