use mctk::prelude::*;
use taffy::{
    AvailableSpace, Dimension, Display, FlexDirection, Size,
    prelude::{TaffyMaxContent, auto, length},
};

struct HelloWorld;
impl Render for HelloWorld {
    fn render(&mut self) -> impl IntoWidget {
        let mut header_node = Div::new();
        header_node.style.size = Size {
            width: length(800.0),
            height: length(100.0),
        };

        let mut body_node = Div::new();
        body_node.style.size = Size {
            width: length(800.0),
            height: auto(),
        };
        body_node.style.flex_grow = 1.0;

        let mut root_node = Div::new();
        root_node.style.size = Size {
            width: length(800.0),
            height: length(600.0),
        };
        root_node.style.flex_direction = FlexDirection::Column;

        root_node.child(header_node).child(body_node)
    }
}

fn main() {
    let mut hello_world = HelloWorld;
    let mut root = hello_world.render().into_any_widget();

    let mut taffy_layout_engine = TaffyLayoutEngine::new();
    let node_id = root.request_layout(&mut taffy_layout_engine);
    let available_space = Size::max_content();
    taffy_layout_engine.compute_layout(node_id, available_space);
    taffy_layout_engine.print_tree(node_id);
}
