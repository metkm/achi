use super::{Interface, native::steam_userstats::ISteamUserStats013};

use std::ffi::{c_char, c_int, c_uint};
use std::sync::atomic::Ordering::SeqCst;

impl Interface<ISteamUserStats013> {
    pub unsafe fn set_achievement(&self, id: *const c_char) -> bool {
        unsafe { (self.vtable.set_achievement)(self.address.load(SeqCst), id) }
    }

    pub unsafe fn clear_achievement(&self, id: *const c_char) -> bool {
        unsafe { (self.vtable.clear_achievement)(self.address.load(SeqCst), id) }
    }

    pub unsafe fn get_achievement_and_unlock_time(
        &self,
        id: *const c_char,
        is_achieved: &mut bool,
        unlock_time: &mut c_uint,
    ) -> bool {
        unsafe {
            (self.vtable.get_achievement_and_unlock_time)(
                self.address.load(SeqCst),
                id,
                is_achieved,
                unlock_time,
            )
        }
    }

    pub unsafe fn request_userstats(&self) -> c_int {
        unsafe { (self.vtable.request_user_stats)(self.address.load(SeqCst), 76561198261085683) }
    }
}
