use gpui::{ParentElement, Render, div};

use crate::{error::AppError, keyvalue::KeyValue, steam::Steam};

pub struct Achievements {
    game_id: i32,
    kvt: Result<KeyValue, AppError>,
}

impl Achievements {
    pub fn new(game_id: i32) -> Self {
        let kvt =
            Steam::get_install_path().and_then(|path| KeyValue::from_install_path(&path, game_id));

        Self { game_id, kvt }
    }
}

impl Render for Achievements {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div().child("achievements")
    }
}
