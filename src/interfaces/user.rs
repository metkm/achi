use std::os::raw::c_int;

use crate::interfaces::native::steam_user::{ISteamUser012, ISteamUser012Functions};
use crate::interfaces::steam_interface;

steam_interface!(SteamUser, ISteamUser012, ISteamUser012Functions);

impl SteamUser {
    pub fn get_steam_id(&self) -> u64 {
        let mut steam_id: u64 = 0;
        unsafe { (self.vtable.get_steam_id)(self.object_address, &mut steam_id) };
        steam_id
    }

    pub fn get_is_logged_on(&self) -> bool {
        unsafe { (self.vtable.logged_on)(self.object_address) }
    }
}
