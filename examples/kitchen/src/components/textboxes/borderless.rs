use mctk_core::style::Styled;
use mctk_core::{component::Component, node, widgets::TextBox, Color};
use mctk_core::{lay, msg, size};

#[derive(Debug)]
pub struct Borderless {}

impl Component for Borderless {
    fn view(&self) -> Option<mctk_core::Node> {
        Some(node!(
            TextBox::new(Some("".to_string()))
                .with_class("text-xl")
                .style("text_color", Color::BLACK)
                .style("border_width", 9.)
                .style("border_color", Color::TRANSPARENT)
                .style("cursor_color", Color::BLACK)
                .style("placeholder_color", Color::rgb(168., 168., 168.))
                .placeholder("Borderless"), // .on_change(Box::new(|s| msg!(gui::Message::SearchTextChanged(s.to_string()))))
            lay![
                size: [410, 56],
            ]
        ))
    }
}
