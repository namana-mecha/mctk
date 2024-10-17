use mctk_core::style::{FontWeight, Styled};
use mctk_core::{component::Component, node, txt, widgets::Button};
use mctk_core::{lay, Color};

#[derive(Debug)]
pub struct Text {}

impl Component for Text {
    fn view(&self) -> Option<mctk_core::Node> {
        Some(node!(
            Button::new(txt!("Text Button"))
                .style("background_color", Color::TRANSPARENT)
                .style("active_color", Color::TRANSPARENT)
                .style("text_color", Color::BLACK)
                .style("font", "Space Grotesk")
                .style("font_size", 18.)
                .style("line_height", 22.)
                .style("font_weight", FontWeight::Normal)
                .style("radius", 2.)
                .style("padding", 8.)
                .style("border_color", Color::TRANSPARENT),
            // .on_click(Box::new(|| msg!(Message::SlideUp))),
            lay![]
        ))
    }
}
