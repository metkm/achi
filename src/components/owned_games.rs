use gpui_component::StyledExt;
use gpui_component::scroll::ScrollableElement;

use log::error;
use std::sync::Arc;

use gpui::{Context, IntoElement, ParentElement, Render, RenderOnce, Styled, div, img, px, rgb};

use crate::error::AppError;
use crate::games::get_game_list;
use crate::interfaces::interface::Interface;
use crate::interfaces::native::steam_apps001::ISteamApps001;
use crate::interfaces::native::steam_apps008::ISteamApps008;
use crate::models;
use crate::program::Program;

pub struct OwnedGames {
    steam_apps008: Arc<Interface<ISteamApps008>>,
    steam_apps001: Arc<Interface<ISteamApps001>>,
    owned_games: Vec<models::game::Game>,

    loading: bool,
    error: Option<AppError>,
    fetched: bool,
}

impl OwnedGames {
    pub fn new(
        steam_apps001: Arc<Interface<ISteamApps001>>,
        steam_apps008: Arc<Interface<ISteamApps008>>,
        cx: &mut Context<OwnedGames>,
    ) -> Self {
        Self {
            steam_apps001,
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
        let apps001 = self.steam_apps001.clone();

        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let owned_games = get_game_list()?
                        .into_iter()
                        .filter_map(|id| {
                            apps008.is_subscribed_app(id).then(|| models::game::Game {
                                id,
                                name: apps001
                                    .get_appdata(id, "name")
                                    .unwrap_or("unknown".to_string()),
                                image_url: format!("https://shared.cloudflare.steamstatic.com/store_item_assets/steam/apps/{id}/{}", apps001.get_appdata(id, "small_capsule/english").unwrap_or("".to_string()))
                            })
                        })
                        // .filter(|id| apps008.is_subscribed_app(*id))
                        .collect::<Vec<models::game::Game>>();

                    Ok::<Vec<models::game::Game>, AppError>(owned_games)
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
                false => div()
                    .grid()
                    .grid_cols(4)
                    .gap_4()
                    .children(self.owned_games.iter().map(|game| {
                        div()
                            .child(img(game.image_url.clone()).w_full())
                            .child(format!("{} - {}", game.id, game.name))
                    })),
                
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
