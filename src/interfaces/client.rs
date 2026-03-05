use crate::error::AppError;
use crate::interfaces::apps::{Apps001, Apps008};
use crate::interfaces::native::steam_client::{ISteamClient018, ISteamClient018Functions};
use crate::interfaces::steam_interface;
use crate::interfaces::user::SteamUser;
use crate::interfaces::userstats::SteamUserStats;

use std::ffi::{CString, c_int};

steam_interface!(SteamClient, ISteamClient018, ISteamClient018Functions);

impl SteamClient {
    pub fn create_stream_pipe(&self) -> Result<c_int, AppError> {
        let result = unsafe { (self.vtable.create_steam_pipe)(self.object_address) };

        if result == 0 {
            Err(AppError::SteamPipeCreation)
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

    pub fn get_steam_user_stats(&self, user: c_int, pipe: c_int) -> SteamUserStats {
        let result = unsafe {
            (self.vtable.get_isteam_user_stats)(
                self.object_address,
                user,
                pipe,
                CString::new("STEAMUSERSTATS_INTERFACE_VERSION013")
                    .unwrap()
                    .as_ptr(),
            )
        };

        SteamUserStats::new(result)
    }
}
