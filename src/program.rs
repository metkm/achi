use std::sync::Arc;

use crate::components::owned_games::OwnedGames;

use crate::error::AppError;
use crate::games::get_game_list;
use crate::interfaces::interface::Interface;
use crate::interfaces::native::steam_apps001::ISteamApps001;
use crate::interfaces::native::steam_apps008::{self, ISteamApps008};
use crate::interfaces::native::steam_client::ISteamClient018;
use crate::steam::Steam;

use gpui::prelude::FluentBuilder;
use gpui::{
    AbsoluteLength, AppContext, AsyncApp, Context, Entity, InteractiveElement, ParentElement,
    Pixels, Render, StatefulInteractiveElement, Styled, WeakEntity, div, img, px, rgb, rgba,
};

use gpui_component::button::{Button, ButtonVariants};
use gpui_component::scroll::{ScrollableElement, ScrollbarAxis, ScrollbarShow};
use gpui_component::{ActiveTheme, StyledExt, TitleBar, v_flex};

use log::error;

pub struct Program {
    steam_client: Option<Arc<Interface<ISteamClient018>>>,
    steam_apps001: Option<Arc<Interface<ISteamApps001>>>,
    steam_apps008: Option<Arc<Interface<ISteamApps008>>>,
    owned_games: Option<Entity<OwnedGames>>,
}

impl Program {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let prog = Self {
            steam_client: None,
            steam_apps001: None,
            steam_apps008: None,
            owned_games: None,
        };

        prog.try_initialize(cx);

        prog
    }

    fn try_initialize(&self, cx: &mut Context<Self>) {
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

                        this.owned_games = Some(
                            cx.new(|cx| OwnedGames::new(apps001.clone(), apps008.clone(), cx)),
                        );

                        cx.notify();
                    });
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
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let content = match self.steam_client {
            None => div()
                .v_flex()
                .flex_grow()
                .justify_center()
                .items_center()
                .child("failed to initialize steam. Is it open?")
                .child(Button::new("Retry").label("Retry").on_click(cx.listener(
                    |this, _, _, cx| {
                        this.try_initialize(cx);
                    },
                ))),
            Some(_) => {
                if let Some(owned_games) = &self.owned_games {
                    div().flex().flex_grow().child(owned_games.clone())
                } else {
                    div().child("owned games component in none for some reason")
                }
            }
        };

        div()
            .v_flex()
            .size_full()
            .max_h_full()
            .max_w_full()
            .overflow_hidden()
            .font_family("Inter 18pt 18pt")
            .child(TitleBar::new().bg(rgba(0x00000000)).border_0())
            .child(
                div()
                    .id("scrollable-content")
                    .flex_grow()
                    .overflow_scroll()
                    .p_4()
                    .child(content), // .child(
                                     //     img("https://pub.lbkrs.com/files/202503/vEnnmgUM6bo362ya/sdk.svg").h_24(),
                                     // ), // .child(content),
            )
    }
}
