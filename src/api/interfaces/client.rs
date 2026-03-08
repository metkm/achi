use crate::error::AppError;

use crate::api::interfaces::{
    native::{
        steam_apps001::ISteamApps001, steam_apps008::ISteamApps008, steam_client::ISteamClient018,
        steam_user::ISteamUser012, steam_userstats::ISteamUserStats013,
    },
    interface::Interface
};

use std::ffi::{CString, c_int};
use std::sync::atomic::Ordering::SeqCst;

impl Interface<ISteamClient018> {
    pub fn create_stream_pipe(&self) -> Result<c_int, AppError> {
        let result = unsafe { (self.vtable.create_steam_pipe)(self.address.load(SeqCst)) };

        if result == 0 {
            Err(AppError::SteamPipeCreation)
        } else {
            Ok(result)
        }
    }

    pub fn connect_to_global_user(&self, pipe: c_int) -> c_int {
        unsafe { (self.vtable.connect_to_global_user)(self.address.load(SeqCst), pipe) }
    }

    pub fn get_steam_user(&self, user: c_int, pipe: c_int) -> Interface<ISteamUser012> {
        let result = unsafe {
            (self.vtable.get_isteam_user)(
                self.address.load(SeqCst),
                user,
                pipe,
                CString::new("SteamUser012").unwrap().as_ptr(),
            )
        };

        Interface::<ISteamUser012>::new(result)
    }

    pub fn get_steam_apps001(&self, user: c_int, pipe: c_int) -> Interface<ISteamApps001> {
        let result = unsafe {
            (self.vtable.get_isteam_apps)(
                self.address.load(SeqCst),
                user,
                pipe,
                CString::new("STEAMAPPS_INTERFACE_VERSION001")
                    .unwrap()
                    .as_ptr(),
            )
        };

        Interface::<ISteamApps001>::new(result)
    }

    pub fn get_steam_apps008(&self, user: c_int, pipe: c_int) -> Interface<ISteamApps008> {
        let result = unsafe {
            (self.vtable.get_isteam_apps)(
                self.address.load(SeqCst),
                user,
                pipe,
                CString::new("STEAMAPPS_INTERFACE_VERSION008")
                    .unwrap()
                    .as_ptr(),
            )
        };

        Interface::<ISteamApps008>::new(result)
    }

    pub fn get_steam_user_stats(&self, user: c_int, pipe: c_int) -> Interface<ISteamUserStats013> {
        let result = unsafe {
            (self.vtable.get_isteam_user_stats)(
                self.address.load(SeqCst),
                user,
                pipe,
                CString::new("STEAMUSERSTATS_INTERFACE_VERSION013")
                    .unwrap()
                    .as_ptr(),
            )
        };

        Interface::<ISteamUserStats013>::new(result)
    }
}
