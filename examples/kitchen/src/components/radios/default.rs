use mctk_core::component::Component;

#[derive(Debug)]
pub struct DefaultRadios {}

impl Component for DefaultRadios {
    fn view(&self) -> Option<mctk_core::Node> {
        None
    }
}
