use crate::interfaces::{interface::VTable, native::CallableDefaultNativeFunction};

use std::ffi::c_int;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ISteamApps008Functions {
    pub is_subscribed: CallableDefaultNativeFunction,
    pub is_low_violence: CallableDefaultNativeFunction,
    pub is_cybercafe: CallableDefaultNativeFunction,
    pub is_vac_banned: CallableDefaultNativeFunction,
    pub get_current_game_language: CallableDefaultNativeFunction,
    pub get_available_game_languages: CallableDefaultNativeFunction,

    pub is_subscribed_app: extern "C" fn(this: *mut c_int, app_id: c_int) -> bool,

    pub is_dlc_installed: CallableDefaultNativeFunction,
    pub get_earliest_purchase_unix_time: CallableDefaultNativeFunction,
    pub is_subscribed_from_free_weekend: CallableDefaultNativeFunction,
    pub get_dlc_count: CallableDefaultNativeFunction,
    pub get_dlc_data_by_index: CallableDefaultNativeFunction,
    pub install_dlc: CallableDefaultNativeFunction,
    pub uninstall_dlc: CallableDefaultNativeFunction,
    pub request_app_proof_of_purchase_key: CallableDefaultNativeFunction,
    pub get_current_beta_name: CallableDefaultNativeFunction,
    pub mark_content_corrupt: CallableDefaultNativeFunction,
    pub get_installed_depots: CallableDefaultNativeFunction,
    pub get_app_install_dir: CallableDefaultNativeFunction,
    pub is_app_installed: CallableDefaultNativeFunction,
    pub get_app_owner: CallableDefaultNativeFunction,
    pub get_launch_query_param: CallableDefaultNativeFunction,
    pub get_dlc_download_progress: CallableDefaultNativeFunction,
    pub get_app_build_id: CallableDefaultNativeFunction,
    pub request_all_proof_of_purchase_keys: CallableDefaultNativeFunction,
    pub get_file_details: CallableDefaultNativeFunction,
    pub get_launch_command_line: CallableDefaultNativeFunction,
    pub is_subscribed_from_family_sharing: CallableDefaultNativeFunction,
}

#[repr(C)]
pub struct ISteamApps008 {
    pub vtable: *const ISteamApps008Functions,
}

impl VTable for ISteamApps008 {
    type Functions = ISteamApps008Functions;

    fn vtable(&self) -> *const Self::Functions {
        self.vtable
    }
}
