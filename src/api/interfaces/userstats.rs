use crate::api::interfaces::{
    interface::Interface,
    native::steam_userstats::ISteamUserStats013
};

use std::ffi::c_char;
use std::sync::atomic::Ordering::SeqCst;

impl Interface<ISteamUserStats013> {
    pub fn set_achievement(&self, info_id: *const c_char) -> bool {
        unsafe { (self.vtable.set_achievement)(self.address.load(SeqCst), info_id) }
    }

    pub fn clear_achievement(&self, info_id: *const c_char) -> bool {
        unsafe { (self.vtable.clear_achievement)(self.address.load(SeqCst), info_id) }
    }
}
