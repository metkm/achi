use std::sync::{Arc, Mutex};

use interfaces::worker::{self, Cmd, SteamWorker};

use crate::api::keyvalue::KeyValue;

pub struct Achievement {
    // pub id: String,
    pub name: String,
    pub desc: String,
    pub icon_normal: String,
    pub icon_locked: String,
    pub is_achieved: bool,
}

impl Achievement {
    pub fn from_bit_kv(kv: &KeyValue, worker: Arc<Mutex<SteamWorker>>) -> Option<Achievement> {
        let name_node = kv.get_kv_by_name("name")?;
        let display_node = kv.get_kv_by_name("display")?;

        let name = display_node
            .get_kv_by_name("name")
            .and_then(|n| n.get_kv_by_name("english"))?;

        let desc = display_node
            .get_kv_by_name("desc")
            .and_then(|n| n.get_kv_by_name("english"))?;

        let icon_normal = display_node.get_kv_by_name("icon")?;
        let icon_locked = display_node.get_kv_by_name("icon_gray")?;

        let Ok(mut lock) = worker.lock() else {
            return None;
        };

        let res = Cmd::GetAchievement(worker::GetAchievement {
            id: name_node.value.to_string(),
        });

        let Ok(result) = lock.send::<worker::GetAchievementResponse>(res) else {
            return None;
        };

        Some(Achievement {
            // id: name_node.value.clone(),
            name: name.value.clone(),
            desc: desc.value.clone(),
            icon_normal: icon_normal.value.clone(),
            icon_locked: icon_locked.value.clone(),
            is_achieved: result.is_achieved,
        })
    }
}
