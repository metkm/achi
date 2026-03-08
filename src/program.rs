use crate::components::achievements::Achievements;
use crate::components::games::{OwnedGames, SelectedGameState};
use crate::error::AppError;

use crate::api::{
    interfaces::{
        interface::Interface,
        native::{
            steam_apps001::ISteamApps001, steam_apps008::ISteamApps008,
            steam_client::ISteamClient018,
        },
    },
    steam::Steam,
};

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
    steam_client: Option<Arc<Interface<ISteamClient018>>>,
    steam_apps001: Option<Arc<Interface<ISteamApps001>>>,
    steam_apps008: Option<Arc<Interface<ISteamApps008>>>,
    owned_games_entity: Option<Entity<OwnedGames>>,
    achievements_entity: Entity<Achievements>,
    selected_game_state: Entity<SelectedGameState>,
}

impl Program {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let game_state = cx.new(|_| SelectedGameState::new());

        let prog = Self {
            steam_client: None,
            steam_apps001: None,
            steam_apps008: None,
            owned_games_entity: None,
            achievements_entity: cx.new(|cx| Achievements::new(cx, &game_state)),
            selected_game_state: game_state,
        };

        prog.try_initialize(window, cx);

        prog
    }

    fn try_initialize(&self, window: &mut Window, cx: &mut Context<Self>) {
        let window_handle = window.window_handle();

        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async {
                    let steam = Steam::new()?;
                    let client = steam.get_steam_client()?;

                    let pipe = client.create_stream_pipe()?;
                    let user = client.connect_to_global_user(pipe);

                    let steam_apps001 = client.get_steam_apps001(user, pipe);
                    let steam_apps008 = client.get_steam_apps008(user, pipe);

                    Ok::<
                        (
                            Interface<ISteamClient018>,
                            Interface<ISteamApps001>,
                            Interface<ISteamApps008>,
                        ),
                        AppError,
                    >((client, steam_apps001, steam_apps008))
                })
                .await;

            match result {
                Ok((client, apps001, apps008)) => {
                    this.update(cx, |this, cx| {
                        let client = Arc::new(client);
                        let apps001 = Arc::new(apps001);
                        let apps008 = Arc::new(apps008);

                        this.steam_client = Some(client);
                        this.steam_apps001 = Some(apps001.clone());
                        this.steam_apps008 = Some(apps008.clone());

                        window_handle
                            .update(cx, |_, window, cx| {
                                this.owned_games_entity = Some(cx.new(|cx| {
                                    OwnedGames::new(
                                        window,
                                        cx,
                                        apps001.clone(),
                                        apps008.clone(),
                                        &this.selected_game_state,
                                    )
                                }));
                            })
                            .ok();

                        cx.notify();
                    })
                    .ok();
                }
                // we can show these errors in ui later.
                Err(error) => {
                    error!("{error}");
                }
            }
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
        let content = match self.steam_client {
            None => div()
                .v_flex()
                .size_full()
                .justify_center()
                .items_center()
                .child("Failed to initialize steam. Is it open?")
                .child(Button::new("Retry").label("Retry").on_click(cx.listener(
                    |this, _, window, cx| {
                        this.try_initialize(window, cx);
                    },
                ))),
            Some(_) => match self.selected_game_state.read(cx).game_id {
                Some(_) => div()
                    .v_flex()
                    .flex_grow()
                    .child(self.achievements_entity.clone()),
                None => div()
                    .v_flex()
                    .flex_grow()
                    .child((self.owned_games_entity.as_ref().unwrap()).clone()),
            },
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
