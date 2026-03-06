use gpui::Styled;
use gpui_component::StyledExt;
use gpui_component::scroll::ScrollableElement;
use log::error;
use std::sync::Arc;

use gpui::Context;
use gpui::IntoElement;
use gpui::ParentElement;
use gpui::Render;
use gpui::RenderOnce;
use gpui::div;

use crate::error::AppError;
use crate::games::get_game_list;
use crate::interfaces::interface::Interface;
use crate::interfaces::native::steam_apps008::ISteamApps008;
use crate::program::Program;

pub struct OwnedGames {
    steam_apps008: Arc<Interface<ISteamApps008>>,
    owned_games: Vec<i32>,

    loading: bool,
    error: Option<AppError>,
    fetched: bool,
}

impl OwnedGames {
    pub fn new(steam_apps008: Arc<Interface<ISteamApps008>>, cx: &mut Context<OwnedGames>) -> Self {
        Self {
            steam_apps008,
            owned_games: vec![],
            loading: false,
            error: None,
            fetched: false,
        }
    }

    fn fetch_games(&mut self, cx: &mut Context<OwnedGames>) {
        self.loading = true;
        cx.notify();

        let apps008 = self.steam_apps008.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let owned_games = get_game_list()?
                        .into_iter()
                        .filter(|id| apps008.is_subscribed_app(*id))
                        .collect::<Vec<i32>>();

                    Ok::<Vec<i32>, AppError>(owned_games)
                })
                .await;

            this.update(cx, |this, cx| {
                this.fetched = true;
                this.loading = false;

                match result {
                    Ok(games) => {
                        this.owned_games = games;
                    }
                    Err(error) => {
                        this.error = Some(error);
                    }
                }

                cx.notify();
            })
            .unwrap();
        })
        .detach();
    }
}

impl Render for OwnedGames {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        match (self.loading, &self.error, self.fetched) {
            (false, None, true) => match self.owned_games.is_empty() {
                true => div().child("Owned games are empty"),
                false => div().v_flex().flex_grow().child(
                    div().v_flex().overflow_y_scrollbar().children(
                        self.owned_games
                            .iter()
                            .map(|game| div().child(game.to_string())),
                    ),
                ),
            },
            (true, None, false) => div().child("Loading"),
            (false, Some(error), true) => div().child(error.to_string()),
            (false, None, false) => {
                self.fetch_games(cx);
                div()
            }
            _ => div(),
        }
    }
}
