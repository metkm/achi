use crate::components::achievements::Achievements;
use crate::components::library::{Library, LibraryState};
use crate::error::Result;

use crate::api::{
    interfaces::{
        interface::Interface,
        native::{
            steam_apps001::ISteamApps001, steam_apps008::ISteamApps008,
            steam_client::ISteamClient018,
        },
    },
    steam::{Clients, Steam},
};

use gpui::prelude::FluentBuilder;
use gpui::{
    AppContext, Context, Entity, InteractiveElement, ParentElement, Render,
    StatefulInteractiveElement, Styled, Window, div, rgba,
};

use gpui_component::button::Button;
use gpui_component::label::Label;
use gpui_component::{StyledExt, TitleBar};

use log::error;
use std::sync::Arc;

pub struct Program {
    clients: Result<Clients>,
    library_state: Entity<LibraryState>,
}

impl Program {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let clients = Self::init_clients();
        let library_state = cx.new(|cx| LibraryState::new(window, cx));

        // we do this in 2 different places (inside try init). It's kinda stupid
        if let Ok((_, apps001, apps008)) = &clients {
            library_state
                .downgrade()
                .update(cx, |this, cx| {
                    this.fetch_games(cx, apps001.clone(), apps008.clone());
                })
                .ok();
        }

        Self {
            clients,
            library_state,
        }
    }

    fn init_clients() -> Result<(
        Arc<Interface<ISteamClient018>>,
        Arc<Interface<ISteamApps001>>,
        Arc<Interface<ISteamApps008>>,
    )> {
        let steam = Steam::new()?;
        let client = steam.get_steam_client()?;

        let pipe = client.create_stream_pipe()?;
        let user = client.connect_to_global_user(pipe);

        let steam_apps001 = client.get_steam_apps001(user, pipe);
        let steam_apps008 = client.get_steam_apps008(user, pipe);

        Ok((
            Arc::new(client),
            Arc::new(steam_apps001),
            Arc::new(steam_apps008),
        ))
    }

    fn try_init(cx: &mut Context<Self>) {
        cx.spawn(async move |this, cx| {
            let clients = cx
                .background_executor()
                .spawn(async { Self::init_clients() })
                .await;

            let clients = match clients {
                Ok(res) => res,
                Err(error) => {
                    error!("{error}");
                    return;
                }
            };

            this.update(cx, |this, cx| {
                let apps001 = clients.1.clone();
                let apps008 = clients.2.clone();

                this.library_state
                    .downgrade()
                    .update(cx, |this, cx| {
                        this.fetch_games(cx, apps001, apps008);
                    })
                    .ok();

                this.clients = Ok(clients);
                cx.notify();
            })
            .ok();
        })
        .detach();
    }
}

impl Render for Program {
    fn render(
        &mut self,
        _: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let content = 'block: {
            if let Err(error) = &self.clients {
                break 'block div()
                    .v_flex()
                    .size_full()
                    .justify_center()
                    .items_center()
                    .gap_2()
                    .child(format!("Failed to initialize steam: {error}"))
                    .child(Button::new("retry").label("Retry").on_click(cx.listener(
                        |_, _, _, cx| {
                            Self::try_init(cx);
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
