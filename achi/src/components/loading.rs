use gpui::{App, IntoElement, ParentElement, RenderOnce, Styled, div};
use gpui_component::{IconName, Sizable, spinner::Spinner};

#[derive(IntoElement)]
pub struct Loading;

impl RenderOnce for Loading {
    fn render(self, _: &mut gpui::Window, _: &mut App) -> impl gpui::IntoElement {
        div()
            .m_auto()
            .flex()
            .gap_2()
            .child(Spinner::new().icon(IconName::LoaderCircle).large())
            .child("Loading")
    }
}
