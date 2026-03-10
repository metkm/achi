use crate::components::achievements::{Achievements, AchievementsState};
use crate::components::library::{Library, LibraryEvent, LibraryState};
use crate::states::steam::SteamState;

use gpui::{
    AppContext, Context, Entity, InteractiveElement, ParentElement, Render,
    StatefulInteractiveElement, Styled, Window, div, rgba,
};

use log::error;

use gpui_component::button::Button;
use gpui_component::label::Label;
use gpui_component::{StyledExt, TitleBar};

pub struct Program {
    library_state: Entity<LibraryState>,
    steam_state: Entity<SteamState>,
    achievements_state: Entity<AchievementsState>,
}

impl Program {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let steam_state = cx.new(|_| SteamState::new());
        let library_state = cx.new(|cx| LibraryState::new(window, cx, steam_state.clone()));

        let achievements_state = cx.new(AchievementsState::new);
        let achievements_state_c = achievements_state.clone();

        cx.subscribe_in(&library_state, window, move |_, _, event, _, cx| {
            let LibraryEvent::Select(id) = event;

            achievements_state_c.update(cx, |this, cx| {
                if let Err(error) = this.init(*id, cx) {
                    error!("error initializing worker {error}");
                };
            });
        })
        .detach();

        Self {
            library_state,
            steam_state,
            achievements_state,
        }
    }
}

impl Render for Program {
    fn render(
        &mut self,
        _: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let steam_entity = self.steam_state.clone();
        let steam_state = steam_entity.read(cx);

        let content = 'block: {
            if let Err(error) = &steam_state.client {
                break 'block div()
                    .v_flex()
                    .size_full()
                    .justify_center()
                    .items_center()
                    .gap_2()
                    .child(format!("Failed to initialize steam: {error}"))
                    .child(Button::new("retry").label("Retry").on_click(cx.listener(
                        move |_, _, _, cx| {
                            steam_entity.update(cx, |_, cx| {
                                SteamState::reload(cx);
                            })
                        },
                    )));
            }

            let library_entity = self.library_state.clone();
            let library_state = library_entity.read(cx);

            div().v_flex().flex_grow().child({
                let Some(_) = library_state.selected else {
                    break 'block div()
                        .v_flex()
                        .flex_grow()
                        .child(Library::new(&self.library_state));
                };

                div()
                    .v_flex()
                    .flex_grow()
                    .gap_2()
                    .child(Button::new("back").label("Go Back").on_click(cx.listener(
                        move |_, _, _, cx| {
                            library_entity.update(cx, |this, cx| {
                                this.selected = None;
                                cx.notify();
                            })
                        },
                    )))
                    .child(Achievements::new(&self.achievements_state))
            })
        };

        div()
            .v_flex()
            .size_full()
            .max_h_full()
            .max_w_full()
            .overflow_hidden()
            .font_family("Inter 18pt 18pt")
            .child(
                TitleBar::new().bg(rgba(0x00000000)).child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(Label::new("Achi").text_sm()),
                ),
            )
            .child(
                div()
                    .id("scrollable-content")
                    .v_flex()
                    .flex_grow()
                    .overflow_scroll()
                    .p_2()
                    .child(content),
            )
    }
}
