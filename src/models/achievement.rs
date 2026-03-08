use crate::keyvalue::KeyValue;

#[derive(Debug)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub icon_normal: String,
    pub icon_locked: String,
}

impl Achievement {
    pub fn from_bit_kv(kv: &KeyValue) -> Option<Achievement> {
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

        Some(Achievement {
            id: name_node.value.clone(),
            name: name.value.clone(),
            desc: desc.value.clone(),
            icon_normal: icon_normal.value.clone(),
            icon_locked: icon_locked.value.clone(),
        })
    }
}
