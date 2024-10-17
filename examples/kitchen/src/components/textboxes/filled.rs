use mctk_core::style::Styled;
use mctk_core::{component::Component, node, widgets::TextBox, Color};
use mctk_core::{lay, msg, size};

#[derive(Debug)]
pub struct Filled {}

impl Component for Filled {
    fn view(&self) -> Option<mctk_core::Node> {
        Some(node!(
            TextBox::new(Some("".to_string()))
                .style("background_color", Color::rgba(0., 0., 0., 0.04))
                .style("font_size", 18.)
                .style("text_color", Color::BLACK)
                .style("border_width", 1.)
                .style("border_color", Color::rgb(180., 180., 180.))
                .style("cursor_color", Color::BLACK)
                .style("placeholder_color", Color::rgb(168., 168., 168.))
                .placeholder("Filled"), // .on_change(Box::new(|s| msg!(gui::Message::SearchTextChanged(s.to_string()))))
            lay![
                size: [410, 40],
            ]
        ))
    }
}
