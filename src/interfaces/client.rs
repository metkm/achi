use crate::error::AppError;
use crate::interfaces::Wrapper;
use crate::interfaces::apps::{Apps001, Apps008};
use crate::interfaces::native::steam_client::{ISteamClient018, ISteamClient018Functions};
use crate::interfaces::user::SteamUser;

use std::ffi::{CString, c_int};

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
            vtable: unsafe { *face.vtable },
        }
    }
}

impl SteamClient {
    pub fn create_stream_pipe(&self) -> Result<c_int, AppError> {
        let result = unsafe { (self.vtable.create_steam_pipe)(self.object_address) };

        if result == 0 {
            Err(AppError::ErrorCreatingStreamPipe)
        } else {
            Ok(result)
        }
    }

    pub fn connect_to_global_user(&self, pipe: c_int) -> c_int {
        unsafe { (self.vtable.connect_to_global_user)(self.object_address, pipe) }
    }

    pub fn get_steam_user(&self, user: c_int, pipe: c_int) -> SteamUser {
        let result = unsafe {
            (self.vtable.get_isteam_user)(
                self.object_address,
                user,
                pipe,
                CString::new("SteamUser012").unwrap().as_ptr(),
            )
        };

        SteamUser::new(result)
    }

    pub fn get_steam_apps001(&self, user: c_int, pipe: c_int) -> Apps001 {
        let result = unsafe {
            (self.vtable.get_isteam_apps)(
                self.object_address,
                user,
                pipe,
                CString::new("STEAMAPPS_INTERFACE_VERSION001")
                    .unwrap()
                    .as_ptr(),
            )
        };

        Apps001::new(result)
    }

    pub fn get_steam_apps008(&self, user: c_int, pipe: c_int) -> Apps008 {
        let result = unsafe {
            (self.vtable.get_isteam_apps)(
                self.object_address,
                user,
                pipe,
                CString::new("STEAMAPPS_INTERFACE_VERSION008")
                    .unwrap()
                    .as_ptr(),
            )
        };

        Apps008::new(result)
    }
}
