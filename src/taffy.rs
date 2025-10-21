use taffy::{AvailableSpace, NodeId, Size, Style, TaffyTree};

pub struct TaffyLayoutEngine {
    taffy: TaffyTree<()>,
}

impl TaffyLayoutEngine {
    pub fn new() -> Self {
        Self {
            taffy: TaffyTree::new(),
        }
    }

    pub fn clear(&mut self) {
        self.taffy.clear();
    }

    pub fn request_layout(&mut self, style: Style, children: &[NodeId]) -> NodeId {
        if children.is_empty() {
            self.taffy.new_leaf(style).expect("Taffy error!").into()
        } else {
            self.taffy
                .new_with_children(style, children)
                .expect("Taffy error!")
                .into()
        }
    }

    pub fn compute_layout(&mut self, root: NodeId, available_space: Size<AvailableSpace>) {
        self.taffy
            .compute_layout(root, available_space)
            .expect("Taffy error!")
    }

    pub fn print_tree(&mut self, root: NodeId) {
        self.taffy.print_tree(root);
    }
}
