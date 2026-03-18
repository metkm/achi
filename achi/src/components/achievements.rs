use crate::{
    api::keyvalue::KeyValue,
    components,
    error::{AppError, Result},
    models::achievement::Achievement,
};

use std::sync::{Arc, Mutex};

use gpui::{Context, ElementId, Entity, IntoElement, ParentElement, RenderOnce, Styled, div, img, uniform_list};
use gpui_component::{ActiveTheme, switch::Switch};

use interfaces::{steam::Steam, worker::Cmd, worker::SteamWorker};

use log::{error, info};

pub struct AchievementsState {
    worker: Option<Arc<Mutex<SteamWorker>>>,
    achievements: Vec<Achievement>,
    game_id: Option<i32>,
}

impl AchievementsState {
    pub fn new() -> Self {
        Self {
            worker: None,
            achievements: vec![],
            game_id: None,
        }
    }

    pub fn init(&mut self, game_id: i32, cx: &mut Context<Self>) -> Result<()> {
        info!("Starting worker process for game id {:?}", game_id);

        cx.spawn(async move |this, cx| {
            let worker = cx
                .background_executor()
                .spawn(async move { SteamWorker::new(game_id) })
                .await;

            let worker = match worker {
                Ok(w) => w,
                Err(error) => {
                    error!("Error creating worker! {}", error);
                    return;
                }
            };

            this.update(cx, |this, cx| {
                this.worker = Some(Arc::new(Mutex::new(worker)));
                this.game_id = Some(game_id);

                cx.notify();
                this.load_achievements(game_id, cx);
            })
            .ok();
        })
        .detach();
        Ok(())
    }

    pub fn stop(&mut self, cx: &mut Context<Self>) {
        if let Some(worker) = &self.worker
            && let Ok(mut lock) = worker.lock()
        {
            lock.child.kill().expect("Unable to kill worker");
        }

        info!("Killed worker process for game id {:?}", self.game_id);

        self.worker = None;
        self.achievements = vec![];
        self.game_id = None;

        cx.notify();
    }

    pub fn load_achievements(&mut self, game_id: i32, cx: &mut Context<Self>) {
        let Some(worker) = &self.worker else {
            return;
        };

        let worker = worker.clone();

        cx.spawn(async move |this, cx| {
            info!("Loading achievements for {game_id}");

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
                    error!("Failed to load achievements: {error}");
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
        let entity = self.state;
        let state = entity.read(cx);

        if state.worker.is_none() {
            return components::loading::Loading.into_any_element();
        }

        let count = state.achievements.len();
        let entity = entity.clone();

        return uniform_list("achievements-list", count, move |range, _window, cx| {
            let state = entity.read(cx);

            let result: Vec<gpui::Div> = range.map(|i| {
                let entity = entity.clone();

                let Some(achi) = entity.read(cx).achievements.get(i) else {
                    return div().child("hm");
                };

                div()
                    .flex()
                    .items_center()
                    .w_full()
                    .gap_2()
                    .child(
                        img(format!(
                            "https://cdn.steamstatic.com/steamcommunity/public/images/apps/{}/{}",
                            state.game_id.unwrap_or(3450310),
                            if achi.is_achieved {
                                &achi.icon_normal
                            } else {
                                &achi.icon_locked
                            }
                        ))
                        .rounded_md()
                        .size_16(),
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .flex_grow()
                            .items_center()
                            .p_1()
                            .child(
                                div()
                                    .child(achi.name.clone())
                                    .child(div().child(achi.desc.clone()).text_sm().text_color(cx.theme().muted_foreground)),
                            )
                            .child(
                                Switch::new(ElementId::Name(format!("{}_enabled", achi.id).into()))
                                    .checked(achi.is_achieved)
                                    .on_click({
                                        let achi_id = achi.id.clone();

                                        move |checked, _, cx| {
                                            entity.update(cx, |state, _cx| {
                                                if let Some(achi) = state
                                                    .achievements
                                                    .iter_mut()
                                                    .find(|a| a.id == achi_id)
                                                {
                                                    achi.is_achieved = *checked;
                                                }

                                                if let Some(worker) = &state.worker
                                                    && let Ok(mut lock) = worker.lock()
                                                {
                                                    if *checked {
                                                        match lock.send::<bool>(Cmd::SetAchievement(achi_id.clone())) {
                                                            Ok(_) => {
                                                                info!("Enabled achievement {achi_id}")
                                                            },
                                                            Err(error) => {
                                                                error!("Failed to enable achievement: {error}");
                                                            }
                                                        }
                                                    } else {
                                                        match lock.send::<bool>(Cmd::ClearAchievement(achi_id.clone())) {
                                                            Ok(_) => {
                                                                info!("Disabled achievement {achi_id}")
                                                            },
                                                            Err(error) => {
                                                                error!("Failed to disable achievement: {error}");
                                                            }
                                                        }
                                                    }
                                                }
                                            });
                                        }
                                    }),
                            ),
                    )
            })
                .collect();

            result
        })
            .h_full()
            .into_any_element();
    }
}
