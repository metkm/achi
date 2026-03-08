use crate::api::interfaces::{interface::Interface, native::steam_user::ISteamUser012};

use std::sync::atomic::Ordering::SeqCst;

impl Interface<ISteamUser012> {
    pub fn get_steam_id(&self) -> u64 {
        let mut steam_id: u64 = 0;
        unsafe { (self.vtable.get_steam_id)(self.address.load(SeqCst), &mut steam_id) };
        steam_id
    }

    pub fn get_is_logged_on(&self) -> bool {
        unsafe { (self.vtable.logged_on)(self.address.load(SeqCst)) }
    }
}
