use std::ffi::c_char;
use std::os::raw::c_int;

use crate::interfaces::Wrapper;
use crate::interfaces::native::steam_userstats::{ISteamUserStats013, ISteamUserStats013Functions};

#[derive(Debug)]
pub struct SteamUserStats {
    vtable: ISteamUserStats013Functions,
    object_address: *mut c_int,
}

impl Wrapper for SteamUserStats {
    fn new(address: *mut c_int) -> Self {
        let ptr = address as *mut ISteamUserStats013;
        let face = unsafe { ptr.as_mut().unwrap() };

        Self {
            object_address: address,
            vtable: unsafe { *face.vtable },
        }
    }
}

impl SteamUserStats {
    pub fn set_achievement(&self, info_id: *const c_char) -> bool {
        unsafe { (self.vtable.set_achievement)(self.object_address, info_id) }
    }

    pub fn clear_achievement(&self, info_id: *const c_char) -> bool {
        unsafe { (self.vtable.clear_achievement)(self.object_address, info_id) }
    }

    // pub fn clear_achievement(&self, active: bool) {
    //     unsafe { (self.vtable.set_achievement)(self.object_address) };
    // }
}
