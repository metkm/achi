use std::os::raw::c_int;

use crate::interfaces::{interface::VTable, native::CallableDefaultNativeFunction};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ISteamUser012Functions {
    pub get_hsteam_user: CallableDefaultNativeFunction,

    pub logged_on: unsafe extern "C" fn(this: *mut c_int) -> bool,
    pub get_steam_id: unsafe extern "C" fn(this: *mut c_int, steam_id: *mut u64),

    pub initiate_game_connection: CallableDefaultNativeFunction,
    pub terminate_game_connection: CallableDefaultNativeFunction,
    pub track_app_usage_event: CallableDefaultNativeFunction,
    pub get_user_data_folder: CallableDefaultNativeFunction,
    pub start_voice_recording: CallableDefaultNativeFunction,
    pub stop_voice_recording: CallableDefaultNativeFunction,
    pub get_compressed_voice: CallableDefaultNativeFunction,
    pub decompress_voice: CallableDefaultNativeFunction,
    pub get_auth_session_ticket: CallableDefaultNativeFunction,
    pub begin_auth_session: CallableDefaultNativeFunction,
    pub end_auth_session: CallableDefaultNativeFunction,
    pub cancel_auth_ticket: CallableDefaultNativeFunction,
    pub user_has_license_for_app: CallableDefaultNativeFunction,
}

#[repr(C)]
pub struct ISteamUser012 {
    pub vtable: *const ISteamUser012Functions,
}

impl VTable for ISteamUser012 {
    type Functions = ISteamUser012Functions;

    fn vtable(&self) -> *const Self::Functions {
        self.vtable
    }
}
