use std::ffi::c_char;
use std::os::raw::c_int;

use crate::interfaces::interface::Interface;
use crate::interfaces::native::steam_userstats::{ISteamUserStats013, ISteamUserStats013Functions};

impl Interface<ISteamUserStats013> {
    pub fn set_achievement(&self, info_id: *const c_char) -> bool {
        unsafe { (self.vtable.set_achievement)(self.address, info_id) }
    }

    pub fn clear_achievement(&self, info_id: *const c_char) -> bool {
        unsafe { (self.vtable.clear_achievement)(self.address, info_id) }
    }
}
