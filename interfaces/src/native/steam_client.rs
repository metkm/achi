use super::CallableDefaultNativeFunction;
use super::{
    steam_apps001::ISteamApps001, steam_apps008::ISteamApps008, steam_user::ISteamUser012,
    steam_userstats::ISteamUserStats013,
};

use crate::error::{Error, Result};
use crate::{Interface, VTable};

use std::ffi::{CString, c_char, c_int};
use std::sync::atomic::Ordering::SeqCst;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ISteamClient018Functions {
    pub create_steam_pipe: unsafe extern "C" fn(this: *mut c_int) -> c_int,
    pub release_steam_pipe: unsafe extern "C" fn(this: *mut c_int, pipe: c_int) -> c_int,

    pub connect_to_global_user: unsafe extern "C" fn(this: *mut c_int, pipe: c_int) -> c_int,
    pub create_local_user: CallableDefaultNativeFunction,

    pub release_user: unsafe extern "C" fn(this: *mut c_int, pipe: c_int, user: c_int) -> c_int,
    pub get_isteam_user: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_int,

    pub get_isteam_gameserver: CallableDefaultNativeFunction,
    pub set_local_ip_binding: CallableDefaultNativeFunction,
    pub get_isteam_friends: CallableDefaultNativeFunction,
    pub get_isteam_utils: CallableDefaultNativeFunction,
    pub get_isteam_matchmaking: CallableDefaultNativeFunction,
    pub get_isteam_matchmaking_servers: CallableDefaultNativeFunction,
    pub get_isteam_generic_interface: CallableDefaultNativeFunction,

    pub get_isteam_user_stats: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_int,

    pub get_isteam_gameserver_stats: CallableDefaultNativeFunction,

    pub get_isteam_apps: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_int,

    pub get_isteam_networking: CallableDefaultNativeFunction,
    pub get_isteam_remote_storage: CallableDefaultNativeFunction,
    pub get_isteam_screenshots: CallableDefaultNativeFunction,
    pub get_isteam_game_search: CallableDefaultNativeFunction,
    pub run_frame: CallableDefaultNativeFunction,
    pub get_ipc_call_count: CallableDefaultNativeFunction,
    pub set_warning_message_hook: CallableDefaultNativeFunction,
    pub shutdown_if_all_pipes_closed: CallableDefaultNativeFunction,
    pub get_isteam_http: CallableDefaultNativeFunction,
    pub deprecated_get_isteam_unified_messages: CallableDefaultNativeFunction,
    pub get_isteam_controller: CallableDefaultNativeFunction,
    pub get_isteam_ugc: CallableDefaultNativeFunction,
    pub get_isteam_app_list: CallableDefaultNativeFunction,
    pub get_isteam_music: CallableDefaultNativeFunction,
    pub get_isteam_music_remote: CallableDefaultNativeFunction,
    pub get_isteam_html_surface: CallableDefaultNativeFunction,
    pub deprecated_set_steamapi_cpostapiresult_in_process: CallableDefaultNativeFunction,
    pub deprecated_remove_steamapi_cpostapiresult_in_process: CallableDefaultNativeFunction,
    pub set_steamapi_ccheckcallbackregistered_in_process: CallableDefaultNativeFunction,
    pub get_isteam_inventory: CallableDefaultNativeFunction,
    pub get_isteam_video: CallableDefaultNativeFunction,
    pub get_isteam_parental_settings: CallableDefaultNativeFunction,
    pub get_isteam_input: CallableDefaultNativeFunction,
    pub get_isteam_parties: CallableDefaultNativeFunction,
}

#[derive(Debug)]
pub struct ISteamClient018 {
    pub vtable: *const ISteamClient018Functions,
}

impl VTable for ISteamClient018 {
    type Functions = ISteamClient018Functions;

    fn vtable(&self) -> *const Self::Functions {
        self.vtable
    }
}

// justreturn option for now.
impl Interface<ISteamClient018> {
    pub fn create_steam_pipe(&self) -> Result<c_int> {
        let result = unsafe { (self.vtable.create_steam_pipe)(self.address.load(SeqCst)) };
        if result == 0 {
            Err(Error::UnableToCreateSteamPipe)
        } else {
            Ok(result)
        }
    }

    pub fn release_steam_pipe(&self, pipe: c_int) {
        unsafe { (self.vtable.release_steam_pipe)(self.address.load(SeqCst), pipe) };
    }

    pub fn connect_to_global_user(&self, pipe: c_int) -> c_int {
        unsafe { (self.vtable.connect_to_global_user)(self.address.load(SeqCst), pipe) }
    }

    pub fn release_user(&self, pipe: c_int, user: c_int) {
        unsafe { (self.vtable.release_user)(self.address.load(SeqCst), pipe, user) };
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
