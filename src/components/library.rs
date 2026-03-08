use crate::models;
use crate::models::achievement::Achievement;
use crate::{
    api::{
        games::{RequestStatus, get_library},
        interfaces::{
            interface::Interface,
            native::{steam_apps001::ISteamApps001, steam_apps008::ISteamApps008},
        },
        keyvalue::KeyValue,
        steam::Steam,
    },
    error::{AppError},
};

use crate::models::game::Game;

use std::sync::Arc;

use gpui::{
    AppContext, Context, Entity, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    Styled, StyledImage, Window, div, img,
};

use gpui_component::{
    ActiveTheme, IconName, PixelsExt, Sizable, StyledExt,
    input::{Input, InputEvent, InputState},
    label::Label,
    spinner::Spinner,
};

use log::error;

pub struct LibraryState {
    status: RequestStatus,
    input: Entity<InputState>,
    games: Arc<Vec<models::game::Game>>,
    games_filtered: Arc<Vec<models::game::Game>>,
    pub selected: Option<i32>,
    pub achievements: Arc<Vec<models::achievement::Achievement>>,
}

impl LibraryState {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| InputState::new(window, cx).placeholder("Search.."));

        cx.subscribe_in(&input, window, Self::on_input).detach();

        Self {
            status: RequestStatus::Idle,
            input,
            games: Arc::new(vec![]),
            games_filtered: Arc::new(vec![]),
            selected: None,
            achievements: Arc::new(vec![]),
        }
    }

    pub fn fetch_games(
        &mut self,
        cx: &mut Context<Self>,
        apps001: Arc<Interface<ISteamApps001>>,
        apps008: Arc<Interface<ISteamApps008>>,
    ) {
        self.status = RequestStatus::Pending;
        cx.notify();

        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async { get_library(apps001, apps008) })
                .await;

            this.update(cx, |this, cx| {
                match result {
                    Ok(games) => {
                        this.games = Arc::new(games);
                        this.status = RequestStatus::Success;
                    }
                    Err(error) => {
                        error!("Error getting games: {error}");
                        this.status = RequestStatus::Error(error.to_string());
                    }
                };

                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    fn on_input(
        this: &mut Self,
        state: &Entity<InputState>,
        event: &InputEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let InputEvent::Change = event else {
            return;
        };

        let query = state.read(cx).value();
        let games = this.games.clone();

        cx.spawn(async move |this, cx| {
            let filtered = cx
                .background_executor()
                .spawn(async move {
                    games
                        .iter()
                        .filter(|g| g.name.to_lowercase().contains(&query.to_lowercase()))
                        .cloned()
                        .collect::<Vec<Game>>()
                })
                .await;

            this.update(cx, |this, cx| {
                this.games_filtered = Arc::new(filtered);
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    fn get_achievements(&self, cx: &mut Context<Self>, game_id: i32) {
        cx.spawn(async move |this, cx| {
            let results = cx
                .background_executor()
                .spawn(async move {
                    let kvt = KeyValue::from_install_path(&Steam::get_install_path()?, game_id)?;

                    let achievements = kvt
                        .get_kv_by_name(&game_id.to_string())
                        .and_then(|kv| kv.get_kv_by_name("stats"))
                        .map(|stats| {
                            stats
                                .children
                                .clone()
                                .into_iter()
                                .flat_map(|stat| {
                                    stat.children
                                        .into_iter()
                                        .filter(|b| b.name == "bits")
                                        .map(|bits| bits.children)
                                })
                                .flat_map(|bits| {
                                    bits.into_iter().filter_map(|bit| {
                                        models::achievement::Achievement::from_bit_kv(&bit)
                                    })
                                })
                                .collect::<Vec<_>>()
                        });

                    Ok::<Option<Vec<Achievement>>, AppError>(achievements)
                })
                .await;

            let achievements = match results {
                Ok(achi) => achi.unwrap_or(vec![]),
                Err(error) => {
                    error!("{error}");
                    return;
                }
            };

            this.update(cx, |this, cx| {
                this.achievements = Arc::new(achievements);
                cx.notify();
            })
            .ok();
        })
        .detach();
    }
}

#[derive(IntoElement)]
pub struct Library {
    state: Entity<LibraryState>,
}

impl Library {
    pub fn new(state: &Entity<LibraryState>) -> Self {
        Self {
            state: state.clone(),
        }
    }
}

impl RenderOnce for Library {
    fn render(self, window: &mut Window, cx: &mut gpui::App) -> impl IntoElement {
        let state = self.state.read(cx);
        let entity = self.state.downgrade();

        if let RequestStatus::Pending | RequestStatus::Idle = &state.status {
            return div()
                .m_auto()
                .flex()
                .gap_2()
                .child(Spinner::new().icon(IconName::LoaderCircle).large())
                .child("Loading");
        }

        if let RequestStatus::Error(error) = &state.status {
            return div().m_auto().child(error.to_string());
        }

        if state.games.is_empty() {
            return div().child("Game list is empty!");
        }

        let window_width = window.viewport_size().width.as_f32();

        let col_count = match window_width {
            ..1280.0 => 4,
            1280.0..1740.0 => 5,
            1740.0.. => 7,
            _ => 4,
        };

        let items = if state.input.read(cx).value().is_empty() {
            &state.games
        } else {
            &state.games_filtered
        };

        div()
            .v_flex()
            .gap_2()
            .child(Input::new(&state.input).w_96())
            .child(
                div()
                    .grid()
                    .grid_cols(col_count)
                    .gap_2()
                    .children(items.iter().map(|game| {
                        let mut img = img(game.image_url.clone());
                        img.style().aspect_ratio = Some(231.0 / 87.0);

                        let entity = entity.clone();
                        let game_id = game.id;

                        div()
                            .w_full()
                            .rounded_md()
                            .hover(|this| this.bg(cx.theme().muted))
                            .on_mouse_down(gpui::MouseButton::Left, move |_, _, cx| {
                                entity
                                    .clone()
                                    .update(cx, |this, cx| {
                                        this.selected = Some(game_id);
                                        this.get_achievements(cx, game_id);

                                        cx.notify();
                                    })
                                    .ok();
                            })
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
                    })),
            )
    }
}
