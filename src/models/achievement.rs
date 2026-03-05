use crate::keyvalue::KeyValue;

#[derive(Debug)]
pub struct Achievement<'a> {
    id: &'a str,
    name: &'a str,
    desc: &'a str,
    icon_normal: &'a str,
    icon_locked: &'a str,
}

impl<'a> Achievement<'a> {
    pub fn from_bit_kv(kv: &'a KeyValue) -> Option<Achievement<'a>> {
        let name_node = kv.get_kv_by_name("name")?;
        let display_node = kv.get_kv_by_name("display")?;
        
        let name = display_node
            .get_kv_by_name("name")
            .and_then(|n| n.get_kv_by_name("english"))?;

        let desc = display_node
            .get_kv_by_name("desc")
            .and_then(|n| n.get_kv_by_name("english"))?;

        let icon_normal = display_node
            .get_kv_by_name("icon")?;

        let icon_locked = display_node
            .get_kv_by_name("icon")?;

        Some(
            Achievement {
                id: &name_node.value,
                name: &name.value,
                desc: &desc.value,
                icon_normal: &icon_normal.value,
                icon_locked: &icon_locked.value,
            }
        )
    }
}
