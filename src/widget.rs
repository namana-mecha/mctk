use std::any::Any;

use taffy::{AvailableSpace, NodeId, Size};

use crate::taffy::TaffyLayoutEngine;

pub trait Render: 'static + Sized {
    fn render(&mut self) -> impl IntoWidget;
}

pub trait RenderOnce: 'static {
    fn render(self) -> impl IntoWidget;
}

pub trait ParentWidget {
    /// Extend this element's children with the given child elements.
    fn extend(&mut self, widgets: impl IntoIterator<Item = AnyWidget>);

    /// Add a single child element to this element.
    fn child(mut self, child: impl IntoWidget) -> Self
    where
        Self: Sized,
    {
        self.extend(std::iter::once(child.into_widget().into_any()));
        self
    }

    /// Add multiple child elements to this element.
    fn children(mut self, children: impl IntoIterator<Item = impl IntoWidget>) -> Self
    where
        Self: Sized,
    {
        self.extend(children.into_iter().map(|child| child.into_any_widget()));
        self
    }
}

pub trait Widget: 'static + IntoWidget {
    type RequestLayoutState: 'static;
    type PrepaintState: 'static;

    fn request_layout(
        &mut self,
        taffy_layout_engine: &mut TaffyLayoutEngine,
    ) -> (NodeId, Self::RequestLayoutState);
    fn prepaint(&mut self) -> Self::PrepaintState;
    fn paint(&mut self);

    fn into_any(self) -> AnyWidget {
        AnyWidget::new(self)
    }
}

pub struct Drawable<W: Widget> {
    pub widget: W,
    phase: WidgetDrawPhase<W::RequestLayoutState, W::PrepaintState>,
}
#[derive(Default)]
enum WidgetDrawPhase<RequestLayoutState, PrepaintState> {
    #[default]
    Start,
    RequestLayout {
        node_id: NodeId,
        request_layout: RequestLayoutState,
    },
    LayoutComputed {
        node_id: NodeId,
        available_space: Size<AvailableSpace>,
        request_layout: RequestLayoutState,
    },
    Prepaint {
        node_id: NodeId,
        request_layout: RequestLayoutState,
        prepaint: PrepaintState,
    },
    Painted,
}

impl<W: Widget> Drawable<W> {
    pub fn new(widget: W) -> Self {
        Drawable {
            widget,
            phase: Default::default(),
        }
    }

    pub fn request_layout(&mut self, taffy_layout_engine: &mut TaffyLayoutEngine) -> NodeId {
        match &mut self.phase {
            WidgetDrawPhase::Start => {
                let (node_id, request_layout) = self.widget.request_layout(taffy_layout_engine);
                self.phase = WidgetDrawPhase::RequestLayout {
                    node_id,
                    request_layout,
                };
                node_id
            }
            _ => panic!("Must call request_layout only once!"),
        }
    }
}

trait WidgetObject {
    fn inner_widget(&mut self) -> &mut dyn Any;
    fn request_layout(&mut self, taffy_layout_engine: &mut TaffyLayoutEngine) -> NodeId;
    fn prepaint(&mut self);
    fn paint(&mut self);
}

impl<W: Widget> WidgetObject for Drawable<W> {
    fn inner_widget(&mut self) -> &mut dyn Any {
        &mut self.widget as &mut dyn Any
    }

    fn request_layout(&mut self, taffy_layout_engine: &mut TaffyLayoutEngine) -> NodeId {
        Drawable::request_layout(self, taffy_layout_engine)
    }

    fn prepaint(&mut self) {
        todo!()
    }

    fn paint(&mut self) {
        todo!()
    }
}

#[allow(unused)]
pub struct AnyWidget(Box<dyn WidgetObject>);
impl AnyWidget {
    pub fn new<W: 'static + Widget>(widget: W) -> Self {
        let drawable = Drawable::new(widget);
        AnyWidget(Box::new(drawable))
    }

    pub fn request_layout(&mut self, taffy_layout_engine: &mut TaffyLayoutEngine) -> NodeId {
        self.0.request_layout(taffy_layout_engine)
    }
}

impl Widget for AnyWidget {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn request_layout(
        &mut self,
        taffy_layout_engine: &mut TaffyLayoutEngine,
    ) -> (NodeId, Self::RequestLayoutState) {
        let node_id = self.0.request_layout(taffy_layout_engine);
        (node_id, ())
    }

    fn prepaint(&mut self) -> Self::PrepaintState {
        self.0.prepaint()
    }

    fn paint(&mut self) {
        self.0.paint()
    }
}

impl IntoWidget for AnyWidget {
    type Widget = Self;

    fn into_widget(self) -> Self::Widget {
        self
    }

    fn into_any_widget(self) -> AnyWidget {
        self
    }
}

pub trait IntoWidget: Sized {
    type Widget: Widget;
    fn into_widget(self) -> Self::Widget;
    fn into_any_widget(self) -> AnyWidget;
}
