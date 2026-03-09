use gpui::{Entity, IntoElement, ParentElement, RenderOnce, Styled, div, img};
use gpui_component::{StyledExt, button::Button, checkbox::Checkbox};

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
        let entity = self.state.clone();

        if state.achievements.is_empty() {
            return div().m_auto().child("Achievements are empty!").child(
                Button::new("clear_achievements")
                    .label("Go Back")
                    .on_click(move |_, _, cx| {
                        LibraryState::select_game(&entity, cx, None);

                        // entity.update(cx, |this, cx| {
                        //     this.selected = None;
                        //     cx.notify();
                        // });
                    }),
            );
        }

        div()
            .v_flex()
            .gap_2()
            .child(
                Button::new("clear_achievements_a")
                    .label("Go Back")
                    .on_click(move |_, _, cx| {
                        LibraryState::select_game(&entity, cx, None);

                        // entity.update(cx, |this, cx| {
                        //     this.selected = None;
                        //     cx.notify();
                        // });
                    }),
            )
            .children(state.achievements.iter().map(|achi| {
                // println!("{:?}", achi);

                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        img(format!(
                            "https://cdn.steamstatic.com/steamcommunity/public/images/apps/{}/{}",
                            state.selected.unwrap_or(3450310),
                            achi.icon_normal
                        ))
                        .rounded_md()
                        .size_24(),
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(
                                div()
                                    .child(achi.name.clone())
                                    .child(div().child(achi.desc.clone()).text_sm()),
                            )
                            .child(Checkbox::new("is_achieved").checked(achi.is_achieved)),
                    )
            }))
    }
}
