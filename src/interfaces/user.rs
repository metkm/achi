use std::os::raw::c_int;

use crate::interfaces::Wrapper;
use crate::interfaces::native::steam_user::{ISteamUser012, ISteamUser012Functions};

#[derive(Debug)]
pub struct SteamUser {
    vtable: ISteamUser012Functions,
    object_address: *mut c_int,
}

impl Wrapper for SteamUser {
    fn new(address: *mut c_int) -> Self {
        let ptr = address as *mut ISteamUser012;
        let face = unsafe { ptr.as_mut().unwrap() };

        Self {
            object_address: address,
            vtable: unsafe { (*face.vtable).clone() },
        }
    }
}

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
