use gpui::IntoElement;
use gpui::ParentElement;
use gpui::Render;
use gpui::RenderOnce;
use gpui::div;

pub struct OwnedGames;

impl OwnedGames {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for OwnedGames {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div().child("owned games")
    }
}
