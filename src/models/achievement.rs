use std::sync::Arc;

use crate::api::{
    interfaces::{interface::Interface, native::steam_userstats::ISteamUserStats013},
    keyvalue::KeyValue,
};

#[derive(Debug)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub icon_normal: String,
    pub icon_locked: String,
    pub is_achieved: bool,
}

impl Achievement {
    pub fn from_bit_kv(
        kv: &KeyValue,
        user_stats: Arc<Interface<ISteamUserStats013>>,
    ) -> Option<Achievement> {
        let name_node = kv.get_kv_by_name("name")?;
        let display_node = kv.get_kv_by_name("display")?;

        let name = display_node
            .get_kv_by_name("name")
            .and_then(|n| n.get_kv_by_name("english"))?;

        let desc = display_node
            .get_kv_by_name("desc")
            .and_then(|n| n.get_kv_by_name("english"))?;

        let icon_normal = display_node.get_kv_by_name("icon")?;
        let icon_locked = display_node.get_kv_by_name("icon")?;

        let mut is_achieved = false;
        let mut unlock_time = 0;

        user_stats.get_achievement_and_unlock_time(name_node.value.as_ptr() as *const i8, &mut is_achieved, &mut unlock_time);

        Some(Achievement {
            id: name_node.value.clone(),
            name: name.value.clone(),
            desc: desc.value.clone(),
            icon_normal: icon_normal.value.clone(),
            icon_locked: icon_locked.value.clone(),
            is_achieved: is_achieved,
        })
    }
}
