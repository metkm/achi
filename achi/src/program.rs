use crate::components::achievements::Achievements;
use crate::components::library::{Library, LibraryState};

use gpui::prelude::FluentBuilder;
use gpui::{
    AppContext, Context, Entity, InteractiveElement, ParentElement, Render,
    StatefulInteractiveElement, Styled, Window, div, rgba,
};

use gpui_component::button::Button;
use gpui_component::label::Label;
use gpui_component::{StyledExt, TitleBar};

pub struct Program {
    library_state: Entity<LibraryState>,
}

impl Program {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let library_state = cx.new(|cx| LibraryState::new(window, cx));

        Self { library_state }
    }
}

impl Render for Program {
    fn render(
        &mut self,
        _: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let library_state = self.library_state.clone();
        let state = self.library_state.read(cx);

        let content = 'block: {
            if let Some(Err(error)) = &state.clients {
                break 'block div()
                    .v_flex()
                    .size_full()
                    .justify_center()
                    .items_center()
                    .gap_2()
                    .child(format!("Failed to initialize steam: {error}"))
                    .child(Button::new("retry").label("Retry").on_click(cx.listener(
                        move |_, _, _, cx| {
                            library_state.update(cx, |_, cx| {
                                LibraryState::try_init_clients(cx);
                            });
                        },
                    )));
            }

            div().v_flex().flex_grow().when_else(
                self.library_state.read(cx).selected.is_some(),
                |_| {
                    div()
                        .v_flex()
                        .flex_grow()
                        .child(Achievements::new(&self.library_state))
                },
                |_| {
                    div()
                        .v_flex()
                        .flex_grow()
                        .child(Library::new(&self.library_state))
                },
            )
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
