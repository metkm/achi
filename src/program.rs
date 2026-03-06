use crate::error::AppError;
use crate::games::get_game_list;
use crate::interfaces::interface::Interface;
use crate::interfaces::native::steam_apps001::ISteamApps001;
use crate::interfaces::native::steam_apps008::{self, ISteamApps008};
use crate::interfaces::native::steam_client::ISteamClient018;
use crate::steam::Steam;

use gpui::prelude::FluentBuilder;
use gpui::{AppContext, AsyncApp, Context, InteractiveElement, ParentElement, Render, Styled, WeakEntity, div};

use gpui_component::button::{Button, ButtonVariants};
use gpui_component::scroll::{ScrollableElement, ScrollbarAxis};
use gpui_component::{StyledExt, TitleBar, v_flex};

use log::error;

#[derive(Default)]
pub struct Program {
    steam_client: Option<Interface<ISteamClient018>>,
    steam_apps001: Option<Interface<ISteamApps001>>,
    steam_apps008: Option<Interface<ISteamApps008>>,
    owned_games: Vec<i32>,
}

impl Program {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let prog = Self {
            ..Default::default()
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
                        this.steam_client = Some(client);
                        this.steam_apps001 = Some(apps001);
                        this.steam_apps008 = Some(apps008);

                        cx.notify();
                        this.load_owned_games(cx);
                    });
                }
                // we can show these errors in ui later.
                Err(error) => {
                    error!("{}", error);
                }
            }
        })
        .detach();
    }

    pub fn load_owned_games(&mut self, cx: &mut Context<Self>) {
        if self.steam_client.is_none() {
            return;
        }

        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let all_games = get_game_list();

                    all_games
                })
                .await;

            match result {
                Ok(mut games) => {
                    this.update(cx, |this, cx| {
                        this.owned_games.clear();
                        this.owned_games.extend_from_slice(&games[0..20]);
                        // this.games.append((&mut games)[0..20]);
                        cx.notify();
                    });
                }
                Err(err) => {
                    error!("error getting game list {:?}", err);
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
        div()
            .v_flex()
            .size_full()
            .font_family("Inter 18pt 18pt")
            .child(TitleBar::new())
            .child(
                div()
                    .v_flex()
                    .flex_grow()
                    .items_center()
                    .justify_center()
                    .when_else(
                        (self.steam_client.is_some() && !self.owned_games.is_empty()),
                        |this| {
                            div().child(
                                v_flex()
                                    .children(self.owned_games.iter().map(|id| id.to_string()))
                                    .overflow_y_scrollbar(),
                            )
                        },
                        |this| {
                            div()
                                .child("Steam is not found/open or games are still loading")
                                .child(
                                    Button::new("retry")
                                        .label("Retry")
                                        .on_mouse_up(gpui::MouseButton::Left, cx.listener(|this, _, _, cx| {
                                            this.try_initialize(cx);
                                        }))
                                )
                        },
                    ),
            )
    }
}
