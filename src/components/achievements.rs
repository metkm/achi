use gpui::{Entity, IntoElement, ParentElement, RenderOnce, Styled, div, img};

use crate::components::library::LibraryState;

#[derive(IntoElement)]
pub struct Achievements {
    state: Entity<LibraryState>,
}

impl Achievements {
    pub fn new(state: &Entity<LibraryState>) -> Self {
        Self {
            state: state.clone(),
        }
    }
}

impl RenderOnce for Achievements {
    fn render(self, _: &mut gpui::Window, cx: &mut gpui::App) -> impl gpui::IntoElement {
        let state = self.state.read(cx);

        if state.achievements.is_empty() {
            return div().m_auto().child("Achievements are empty!");
        }

        div().children(state.achievements.iter().map(|achi| {
            div()
                .flex()
                .child(img(format!(
                    "https://cdn.steamstatic.com/steamcommunity/public/images/apps/{}/{}",
                    state.selected.unwrap_or(3450310),
                    achi.icon_normal
                )))
                .child(achi.name.clone())
                .child(achi.desc.clone())
        }))
    }
}
