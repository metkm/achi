use crate::{models, models::game::Game};

use crate::api::games::{RequestStatus, get_library};
use crate::states::steam::SteamState;

use std::sync::Arc;

use gpui::{
    App, AppContext, Context, Entity, EventEmitter, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, Styled, StyledImage, Window, div, img,
};

use gpui_component::{
    ActiveTheme, IconName, PixelsExt, Sizable, StyledExt,
    input::{Input, InputEvent, InputState},
    label::Label,
    spinner::Spinner,
};

use log::{error, info};

#[derive(Clone, Debug)]
pub enum LibraryEvent {
    Select(Option<i32>),
}

#[derive(Debug)]
pub struct LibraryState {
    pub selected: Option<i32>,
    status: RequestStatus,
    input: Entity<InputState>,
    games: Arc<Vec<models::game::Game>>,
    games_filtered: Arc<Vec<models::game::Game>>,
}

impl LibraryState {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        steam_entity: Entity<SteamState>,
    ) -> Self {
        let input = cx.new(|cx| InputState::new(window, cx).placeholder("Search.."));

        cx.subscribe_in(&input, window, Self::on_input).detach();
        cx.subscribe_in(&steam_entity, window, |_, entity, _, _, cx| {
            LibraryState::fetch_games(entity, cx);
        })
        .detach();

        Self::fetch_games(&steam_entity, cx);

        Self {
            selected: None,
            status: RequestStatus::Idle,
            input,
            games: Arc::new(vec![]),
            games_filtered: Arc::new(vec![]),
        }
    }

    pub fn fetch_games(steam_entity: &Entity<SteamState>, cx: &mut Context<Self>) {
        let Ok(ref client) = steam_entity.read(cx).client else {
            return;
        };

        let apps001 = client.apps001.clone();
        let apps008 = client.apps008.clone();

        cx.spawn(async move |this, cx| {
            this.update(cx, |this, cx| {
                this.status = RequestStatus::Pending;
                cx.notify();
            })
            .ok();

            let result = cx
                .background_executor()
                .spawn(async move { get_library(apps001, apps008) })
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

    pub fn select_game(entity: &Entity<Self>, cx: &mut App, game_id: Option<i32>) {
        entity.update(cx, |this, cx| {
            this.selected = game_id;
            info!("Selected game with id {:?}", game_id);

            cx.emit(LibraryEvent::Select(game_id));
            cx.notify();
        })
    }
}

impl EventEmitter<LibraryEvent> for LibraryState {}

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
        let entity = self.state.clone();

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
                                LibraryState::select_game(&entity, cx, Some(game_id));

                                // entity.update(cx, |this, cx| {
                                //     this.selected = Some(game_id);

                                // if let Ok(clients) = &this.clients {
                                //     let user_stats = clients.user_stats.clone();
                                //     this.get_achievements(cx, game_id, user_stats);
                                // };

                                // });
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
