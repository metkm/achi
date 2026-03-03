use crate::interfaces::Wrapper;
use crate::interfaces::native::steam_client::{ISteamClient018, ISteamClient018Functions};

use std::ffi::c_int;

pub struct SteamClient {
    pub vtable: ISteamClient018Functions,
    pub object_address: *mut c_int,
}

impl Wrapper for SteamClient {
    fn new(address: *mut c_int) -> Self {
        let ptr = address as *mut ISteamClient018;
        let face = unsafe { ptr.as_mut().unwrap() };

        Self {
            object_address: address,
            vtable: unsafe { (*face.vtable).clone() },
        }
    }
}

impl SteamClient {
    pub fn create_stream_pipe(&self) -> std::ffi::c_int {
        unsafe { (self.vtable.create_steam_pipe)(self.object_address) }
    }

    pub fn connect_to_global_user(&self, pipe: std::ffi::c_int) -> std::ffi::c_int {
        unsafe { (self.vtable.connect_to_global_user)(self.object_address, pipe) }
    }

    pub fn get_steam_user(
        &self,
        user: std::ffi::c_int,
        pipe: std::ffi::c_int,
    ) -> crate::interfaces::user::SteamUser {
        let result = unsafe {
            (self.vtable.get_isteam_user)(
                self.object_address,
                user,
                pipe,
                String::from("SteamUser012\0").as_mut_ptr() as *mut i8,
            )
        };

        crate::interfaces::user::SteamUser::new(result)
    }

    pub fn get_steam_apps(
        &self,
        user: std::ffi::c_int,
        pipe: std::ffi::c_int,
    ) -> crate::interfaces::apps::Apps {
        let result = unsafe {
            (self.vtable.get_isteam_apps)(
                self.object_address,
                user,
                pipe,
                String::from("STEAMAPPS_INTERFACE_VERSION001\0").as_mut_ptr() as *mut i8,
            )
        };

        println!("{:?}-", result);

        crate::interfaces::apps::Apps::new(result)
    }
}
