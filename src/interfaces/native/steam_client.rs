use super::CallableDefaultNativeFunction;
use std::ffi::{c_char, c_int};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ISteamClient018Functions {
    pub create_steam_pipe: unsafe extern "C" fn(this: *mut c_int) -> c_int,

    pub release_steam_pipe: CallableDefaultNativeFunction,

    pub connect_to_global_user: unsafe extern "C" fn(this: *mut c_int, pipe: c_int) -> c_int,

    pub create_local_user: CallableDefaultNativeFunction,
    pub release_user: CallableDefaultNativeFunction,

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
