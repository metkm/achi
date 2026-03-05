use crate::error::AppError;
use crate::games::get_game_list;
use crate::interfaces::interface::Interface;
use crate::interfaces::native::steam_client::ISteamClient018;
use crate::steam::Steam;

use gpui::prelude::FluentBuilder;
use gpui::{AppContext, AsyncApp, Context, ParentElement, Render, Styled, WeakEntity, div};

use gpui_component::button::{Button, ButtonVariants};
use gpui_component::scroll::{ScrollableElement, ScrollbarAxis};
use gpui_component::{StyledExt, TitleBar, v_flex};

use log::error;

#[derive(Default)]
pub struct Program {
    steam_client: Option<Interface<ISteamClient018>>,
    owned_games: Vec<i32>,
}

impl Program {
    pub fn new(cx: &mut Context<Program>) -> Self {
        let prog = Self {
            ..Default::default()
        };

        prog.try_initialize(cx);

        prog
    }

    fn try_initialize(&self, cx: &mut Context<Program>) {
        cx.spawn(async move |this, cx| {
            let result = cx.background_executor().spawn(async {
                let steam = Steam::new()?;
                let client = steam.get_steam_client()?;

                let pipe = client.create_stream_pipe()?;
                let user = client.connect_to_global_user(pipe);

                Ok::<(), AppError>(())
            }).await;

            // we can show these errors in ui later.
            if let Err(error) = result {
                error!("{}", error);
            }
        }).detach();
    }

    pub fn load_owned_games(&mut self, cx: &mut Context<Program>) {
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
                        |_| {
                            div().child(
                                v_flex()
                                    .children(self.owned_games.iter().map(|id| id.to_string()))
                                    .overflow_y_scrollbar(),
                            )
                        },
                        |_| div().child("Steam is not found/open or games are still loading"),
                    ),
            )
    }
}
