use std::ffi::c_char;
use std::os::raw::c_int;

use crate::interfaces::native::steam_userstats::{ISteamUserStats013, ISteamUserStats013Functions};
use crate::interfaces::steam_interface;

steam_interface!(
    SteamUserStats,
    ISteamUserStats013,
    ISteamUserStats013Functions
);

impl SteamUserStats {
    pub fn set_achievement(&self, info_id: *const c_char) -> bool {
        unsafe { (self.vtable.set_achievement)(self.object_address, info_id) }
    }

    pub fn clear_achievement(&self, info_id: *const c_char) -> bool {
        unsafe { (self.vtable.clear_achievement)(self.object_address, info_id) }
    }
}
