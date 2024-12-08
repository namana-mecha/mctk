pub mod circle;
pub mod curve;
pub mod image;
pub mod line;
pub mod radial_gradient;
pub mod raw_image;
pub mod rect;
pub mod svg;
pub mod text;
pub mod types;

pub use circle::Circle;
pub use curve::Curve;
pub use image::Image;
pub use line::Line;
pub use radial_gradient::RadialGradient;
pub use raw_image::RawImage;
pub use rect::Rect;
pub use svg::Svg;
pub use text::Text;

#[derive(Debug, Clone)]
pub enum Renderable {
    Rect(Rect),
    Line(Line),
    Circle(Circle),
    Image(Image),
    RawImage(RawImage),
    Text(Text),
    Svg(Svg),
    RadialGradient(RadialGradient),
    Curve(Curve),
}
