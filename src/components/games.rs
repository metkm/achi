use crate::interfaces::{
    interface::Interface,
    native::{steam_apps001::ISteamApps001, steam_apps008::ISteamApps008},
};

use crate::games::get_game_list;
use crate::models;
use crate::{error::AppError, models::game::Game};

use std::sync::Arc;

use gpui::{
    AppContext, Context, Entity, InteractiveElement, ParentElement, Render, Styled, StyledImage,
    Window, div, img,
};

use gpui_component::{
    ActiveTheme, IconName, PixelsExt, Sizable, StyledExt,
    input::{Input, InputEvent, InputState},
    label::Label,
    spinner::Spinner,
};

pub struct SelectedGameState {
    pub game_id: Option<i32>,
}

impl SelectedGameState {
    pub fn new() -> Self {
        Self { game_id: None }
    }
}

pub struct OwnedGames {
    steam_apps008: Arc<Interface<ISteamApps008>>,
    steam_apps001: Arc<Interface<ISteamApps001>>,

    owned_games: Arc<Vec<models::game::Game>>,
    filtered_games: Arc<Vec<models::game::Game>>,

    selected_game_state: Entity<SelectedGameState>,
    loading: bool,
    error: Option<AppError>,
    fetched: bool,
    input: Entity<InputState>,
}

impl OwnedGames {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<OwnedGames>,
        steam_apps001: Arc<Interface<ISteamApps001>>,
        steam_apps008: Arc<Interface<ISteamApps008>>,
        selected_game_state: &Entity<SelectedGameState>,
    ) -> Self {
        let input = cx.new(|cx| InputState::new(window, cx).placeholder("Search.."));

        cx.subscribe_in(&input, window, Self::on_search_input)
            .detach();

        Self {
            steam_apps001,
            steam_apps008,
            owned_games: Arc::new(vec![]),
            filtered_games: Arc::new(vec![]),

            selected_game_state: selected_game_state.clone(),
            loading: false,
            error: None,
            fetched: false,
            input,
        }
    }

    fn on_search_input(
        this: &mut OwnedGames,
        state: &Entity<InputState>,
        event: &InputEvent,
        _: &mut Window,
        cx: &mut Context<OwnedGames>,
    ) {
        match event {
            InputEvent::Change => {
                let query = state.read(cx).value();
                let owned_games = this.owned_games.clone();

                cx.spawn(async move |this, cx| {
                    let filtered_games = cx
                        .background_executor()
                        .spawn(async move {
                            owned_games
                                .iter()
                                .cloned()
                                .filter(|g| g.name.to_lowercase().contains(&query.to_lowercase()))
                                .collect::<Vec<Game>>()
                        })
                        .await;

                    this.update(cx, |this, cx| {
                        this.filtered_games = Arc::new(filtered_games);

                        cx.notify();
                    })
                    .ok();
                })
                .detach();
            }
            _ => {}
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
                            if id == 480 { // spacewar
                                return None;
                            }

                            apps008.is_subscribed_app(id).then(|| models::game::Game {
                                id,
                                name: apps001
                                    .get_appdata(id, "name")
                                    .unwrap_or("unknown".to_string()),
                                image_url: format!("https://shared.cloudflare.steamstatic.com/store_item_assets/steam/apps/{id}/{}", apps001.get_appdata(id, "small_capsule/english").unwrap_or("".to_string()))
                            })
                        })
                        .collect::<Vec<models::game::Game>>();

                    Ok::<Vec<models::game::Game>, AppError>(owned_games)
                })
                .await;

            this.update(cx, |this, cx| {
                this.fetched = true;
                this.loading = false;

                match result {
                    Ok(games) => {
                        this.owned_games = Arc::new(games);
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
        let window_width = window.viewport_size().width.as_f32();

        let col_count = match window_width {
            ..1280.0 => 4,
            1280.0..1740.0 => 5,
            1740.0.. => 7,
            _ => 4,
        };

        match (self.loading, &self.error, self.fetched) {
            (false, None, true) => match self.owned_games.is_empty() {
                true => div().child("No games found!"),
                false => div()
                    .v_flex()
                    .gap_2()
                    .child(Input::new(&self.input).w_96())
                    .child(div().grid().grid_cols(col_count).gap_2().children({
                        let items = if self.input.read(cx).value().is_empty() {
                            &self.owned_games
                        } else {
                            &self.filtered_games
                        };

                        items.iter().map(|game| {
                            let mut img = img(game.image_url.clone());

                            img.style().aspect_ratio = Some(231.0 / 87.0);
                            let game_id = game.id.clone();

                            div()
                                .w_full()
                                .rounded_md()
                                .hover(|this| this.bg(cx.theme().muted))
                                .on_mouse_down(gpui::MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                    this.selected_game_state.update(cx, move |this, cx| {
                                        this.game_id = Some(game_id);
                                        cx.notify();
                                    });
                                }))
                                .child(
                                    img.w_full()
                                        .h_auto()
                                        .object_fit(gpui::ObjectFit::Fill)
                                        .rounded_md(),
                                )
                                .child(
                                    Label::new(format!("{} - {}", game.id, game.name))
                                        .pl_1()
                                        .text_sm(),
                                )
                        })
                    })),
            },
            (false, Some(error), true) => div().child(error.to_string()),
            (_, None, false) => {
                if !self.loading {
                    self.fetch_games(cx);
                }

                div()
                    .m_auto()
                    .flex()
                    .gap_2()
                    .child(Spinner::new().icon(IconName::LoaderCircle).large())
                    .child("Loading")
            }
            _ => div(),
        }
    }
}
