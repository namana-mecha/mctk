use taffy::{NodeId, Style};

use crate::{
    taffy::TaffyLayoutEngine,
    widget::{AnyWidget, IntoWidget, ParentWidget, Widget},
};

pub struct Div {
    children: Vec<AnyWidget>,
    pub style: Style,
}

impl Div {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: Style::default(),
        }
    }
}

impl Widget for Div {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn request_layout(
        &mut self,
        taffy_layout_engine: &mut TaffyLayoutEngine,
    ) -> (taffy::NodeId, Self::RequestLayoutState) {
        let child_node_ids: Vec<NodeId> = self
            .children
            .iter_mut()
            .map(|child| child.request_layout(taffy_layout_engine))
            .collect();

        (
            taffy_layout_engine.request_layout(self.style.clone(), &child_node_ids),
            (),
        )
    }

    fn prepaint(&mut self) -> Self::PrepaintState {
        todo!()
    }

    fn paint(&mut self) {
        todo!()
    }
}
impl IntoWidget for Div {
    type Widget = Div;

    fn into_widget(self) -> Self::Widget {
        self
    }

    fn into_any_widget(self) -> crate::prelude::AnyWidget {
        AnyWidget::new(self)
    }
}

impl ParentWidget for Div {
    fn extend(&mut self, widgets: impl IntoIterator<Item = AnyWidget>) {
        self.children.extend(widgets);
    }
}
