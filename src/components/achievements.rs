use crate::{components::games::SelectedGameState, keyvalue::KeyValue, models, steam::Steam};

use gpui::{Context, Entity, ParentElement, Render, div};
use log::error;

pub struct Achievements {
    state: Entity<SelectedGameState>,
    achievements: Vec<models::achievement::Achievement>
    // kvt: Result<KeyValue, AppError>,
}

impl Achievements {
    pub fn new(cx: &mut Context<Achievements>, state: &Entity<SelectedGameState>) -> Self {
        cx.observe(state, |this, _, cx| {
            let Some(game_id) = this.state.read(cx).game_id else {
                return;
            };

            this.load_achievements(cx, game_id);
        }).detach();

        Self {
            state: state.clone(),
            achievements: vec![]
        }
    }

    fn load_achievements(&self, cx: &mut Context<Achievements>, game_id: i32) {
        cx.spawn(async move |this, cx| {

            let result = cx.background_executor().spawn(async move {
                Steam::get_install_path()
                    .and_then(|path| KeyValue::from_install_path(&path, game_id))
                    .and_then(|kvt| {
                        let achievements = kvt
                            .get_kv_by_name(&game_id.to_string())
                            .and_then(|kv| kv.get_kv_by_name("stats"))
                            .and_then(|stats| {
                                let bits = stats
                                    .children
                                    .clone()
                                    .into_iter()
                                    .map(|stat| {
                                        stat.children
                                            .into_iter()
                                            .filter_map(|b| {
                                                if b.name == "bits" {
                                                    return Some(b.children)
                                                }

                                                None
                                            })
                                            .flatten()
                                            // .filter(|b| b.name == "bits")
                                    })
                                    .flatten()
                                    .collect::<Vec<KeyValue>>();

                                let achievements = bits
                                    .into_iter()
                                    .filter_map(|bit| {
                                        models::achievement::Achievement::from_bit_kv(&bit)
                                    })
                                    .collect::<Vec<models::achievement::Achievement>>();

                                Some(achievements)
                            });

                        Ok(achievements)
                    })
            }).await;

            match result {
                Ok(op) => {
                    this.update(cx, |this, cx| {
                        this.achievements = op.unwrap_or(vec![]);
                        cx.notify();
                    }).ok();
                }
                Err(error) => {
                    error!("error loading achievements {error}");
                }
            };
        }).detach();
    }
}

impl Render for Achievements {
    fn render(
        &mut self,
        _: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .children(self.achievements.iter().map(|achi| achi.name.to_string()))
    }
}
