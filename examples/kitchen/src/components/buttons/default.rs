use mctk_core::style::{FontWeight, Styled};
use mctk_core::{component::Component, node, txt, widgets::Button};
use mctk_core::{lay, Color};

#[derive(Debug)]
pub struct DefaultButton {}

impl Component for DefaultButton {
    fn view(&self) -> Option<mctk_core::Node> {
        Some(node!(
            Button::new(txt!("Default Button"))
                .style("background_color", Color::TRANSPARENT)
                .style("active_color", Color::TRANSPARENT)
                .style("text_color", Color::BLACK)
                .style("font", "Space Grotesk")
                .style("font_size", 18.)
                .style("line_height", 22.)
                .style("font_weight", FontWeight::Normal)
                .style("radius", 2.)
                .style("padding", 8.)
                .style("border_color", Color::rgb(217., 217., 217.)),
            // .on_click(Box::new(|| msg!(Message::SlideUp))),
            lay![]
        ))
    }
}
