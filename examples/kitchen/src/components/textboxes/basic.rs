use mctk_core::style::Styled;
use mctk_core::{component::Component, node, widgets::TextBox, Color};
use mctk_core::{lay, msg, size};

#[derive(Debug)]
pub struct Basic {}

impl Component for Basic {
    fn view(&self) -> Option<mctk_core::Node> {
        Some(node!(
            TextBox::new(Some("".to_string()))
                .with_class("text-md")
                .placeholder("Basic usage"), // .on_change(Box::new(|s| msg!(gui::Message::SearchTextChanged(s.to_string()))))
            lay![
                size: [410, 40],
            ]
        ))
    }
}
