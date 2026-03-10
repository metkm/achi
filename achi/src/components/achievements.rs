use std::sync::{Arc, Mutex};

use gpui::{Context, Entity, IntoElement, ParentElement, RenderOnce, Styled, div, img};
use gpui_component::{StyledExt, switch::Switch};

use interfaces::{steam::Steam, worker::SteamWorker};
use log::error;

use crate::{
    api::keyvalue::KeyValue,
    error::{AppError, Result},
    models::achievement::Achievement,
};

pub struct AchievementsState {
    worker: Option<Arc<Mutex<SteamWorker>>>,
    achievements: Vec<Achievement>,
    game_id: Option<i32>,
}

impl AchievementsState {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            worker: None,
            achievements: vec![],
            game_id: None,
        }
    }

    pub fn init(&mut self, game_id: i32, cx: &mut Context<Self>) -> Result<()> {
        let worker = SteamWorker::new(game_id)?;

        self.worker = Some(Arc::new(Mutex::new(worker)));
        self.game_id = Some(game_id);

        self.load_achievements(game_id, cx);
        cx.notify();
        Ok(())
    }

    pub fn load_achievements(&mut self, game_id: i32, cx: &mut Context<Self>) {
        let Some(worker) = &self.worker else {
            return;
        };

        let worker = worker.clone();

        cx.spawn(async move |this, cx| {
            let results = cx
                .background_executor()
                .spawn(async move {
                    let kvt = KeyValue::from_install_path(&Steam::get_install_path()?, game_id)?;

                    let game_id = game_id.to_string();
                    let worker = worker.clone();

                    let achievements = kvt
                        .get_kv_by_name(&game_id)
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
                                    bits.into_iter().filter_map({
                                        let worker = worker.clone();

                                        move |bit| Achievement::from_bit_kv(&bit, worker.clone())
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
                this.achievements = achievements;
                cx.notify();
            })
            .ok();
        })
        .detach();
    }
}

#[derive(IntoElement)]
pub struct Achievements {
    state: Entity<AchievementsState>,
}

impl Achievements {
    pub fn new(state: &Entity<AchievementsState>) -> Self {
        Self {
            state: state.clone(),
        }
    }
}

impl RenderOnce for Achievements {
    fn render(self, _: &mut gpui::Window, cx: &mut gpui::App) -> impl gpui::IntoElement {
        let state = self.state.read(cx);

        div()
            .v_flex()
            .flex_grow()
            .gap_2()
            .children(state.achievements.iter().map(|achi| {
                div()
                    .flex()
                    .items_center()
                    .w_full()
                    .gap_2()
                    .child(
                        img(format!(
                            "https://cdn.steamstatic.com/steamcommunity/public/images/apps/{}/{}",
                            state.game_id.unwrap_or(3450310),
                            if achi.is_achieved { &achi.icon_normal } else { &achi.icon_locked }
                        ))
                        .rounded_md()
                        .size_24(),
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .flex_grow()
                            .items_center()
                            .p_2()
                            .child(
                                div()
                                    .child(achi.name.clone())
                                    .child(div().child(achi.desc.clone()).text_sm()),
                            )
                            .child(Switch::new("is_achieved").checked(achi.is_achieved)),
                    )
            }))
    }
}
