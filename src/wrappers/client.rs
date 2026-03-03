use crate::wrappers::Wrapper;
use std::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ISteamClient018Functions {
    pub create_steam_pipe: unsafe extern "C" fn(this: *mut c_int) -> c_int,

    pub release_steam_pipe: unsafe extern "C" fn(this: *mut c_int, pipe: c_int) -> bool,

    pub connect_to_global_user: unsafe extern "C" fn(this: *mut c_int, pipe: c_int) -> c_int,

    pub create_local_user:
        unsafe extern "C" fn(this: *mut c_int, user: *mut c_int, account_type: c_int) -> c_int,

    pub release_user: unsafe extern "C" fn(this: *mut c_int, pipe: c_int, user: c_int),

    pub get_isteam_user: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_int,

    pub get_isteam_gameserver: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub set_local_ip_binding:
        unsafe extern "C" fn(this: *mut c_int, ip: *const c_void, port: c_uint),

    pub get_isteam_friends: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub get_isteam_utils:
        unsafe extern "C" fn(this: *mut c_int, pipe: c_int, version: *const c_char) -> *mut c_void,

    pub get_isteam_matchmaking: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub get_isteam_matchmaking_servers: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub get_isteam_generic_interface: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub get_isteam_user_stats: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub get_isteam_gameserver_stats: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub get_isteam_apps: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub get_isteam_networking: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub get_isteam_remote_storage: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub get_isteam_screenshots: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub get_isteam_game_search: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub run_frame: unsafe extern "C" fn(this: *mut c_int),

    pub get_ipc_call_count: unsafe extern "C" fn(this: *mut c_int) -> c_uint,

    pub set_warning_message_hook: unsafe extern "C" fn(this: *mut c_int, hook: *mut c_void),

    pub shutdown_if_all_pipes_closed: unsafe extern "C" fn(this: *mut c_int),

    pub get_isteam_http: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub deprecated_get_isteam_unified_messages: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub get_isteam_controller: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub get_isteam_ugc: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub get_isteam_app_list: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub get_isteam_music: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub get_isteam_music_remote: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub get_isteam_html_surface: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub deprecated_set_steamapi_cpostapiresult_in_process:
        unsafe extern "C" fn(this: *mut c_int, callback: *mut c_void),

    pub deprecated_remove_steamapi_cpostapiresult_in_process:
        unsafe extern "C" fn(this: *mut c_int, callback: *mut c_void),

    pub set_steamapi_ccheckcallbackregistered_in_process:
        unsafe extern "C" fn(this: *mut c_int, func: *mut c_void),

    pub get_isteam_inventory: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub get_isteam_video: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub get_isteam_parental_settings: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub get_isteam_input: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,

    pub get_isteam_parties: unsafe extern "C" fn(
        this: *mut c_int,
        user: c_int,
        pipe: c_int,
        version: *const c_char,
    ) -> *mut c_void,
}

// #[repr(C)]
// #[derive(Clone, Copy)]
// pub struct ISteamClient018Functions {
//     pub create_steam_pipe: unsafe extern "C" fn(this: *mut c_int) -> c_int,
//     pub releate_steam_pipe: unsafe extern "C" fn(this: *mut c_int, pipe: c_int) -> c_int,
//     pub connect_to_global_user: unsafe extern "C" fn(this: *mut c_int, pipe: c_int) -> c_int,
//     pub create_local_user: unsafe extern "C" fn(this: *mut c_int, pipe: *mut c_int, account_type: c_int) -> c_int,
//     pub release_user: unsafe extern "C" fn(this: *mut c_int, pipe: c_int, user: c_int),
//     pub get_isteam_user: unsafe extern "C" fn(this: *mut c_int, user: c_int, pipe: c_int, version: *mut c_char),
//     pub  get_isteam_gameserver: unsafe extern "C" fn(this: *mut c_int),
//     pub  set_local_ip_binding: unsafe extern "C" fn(this: *mut c_int),
//     pub  get_isteam_friends: unsafe extern "C" fn(this: *mut c_int),
//     pub  get_isteam_utils: unsafe extern "C" fn(this: *mut c_int),
//     pub  get_isteam_matchmaking: unsafe extern "C" fn(this: *mut c_int),
//     pub  get_isteam_matchmaking_servers: unsafe extern "C" fn(this: *mut c_int),
//     pub  get_isteam_generic_interface: unsafe extern "C" fn(this: *mut c_int),
//     pub  get_isteam_user_stats: unsafe extern "C" fn(this: *mut c_int, user: c_int, pipe: c_int, version: *mut c_char),

//     pub get_isteam_gameserver_stats: unsafe extern "C" fn(
//         this: *mut c_int,
//         user: c_int,
//         pipe: c_int,
//         version: *const c_char,
//     ) -> *mut c_void,

//     pub get_isteam_apps:
//         unsafe extern "C" fn(this: *mut c_int, user: c_int, pipe: c_int, version: *const c_char) -> *mut c_void,

//     pub get_isteam_networking:
//         unsafe extern "C" fn(this: *mut c_int, user: c_int, pipe: c_int, version: *const c_char) -> *mut c_void,

//     pub get_isteam_remote_storage:
//         unsafe extern "C" fn(this: *mut c_int, user: c_int, pipe: c_int, version: *const c_char) -> *mut c_void,

//     pub get_isteam_screenshots:
//         unsafe extern "C" fn(this: *mut c_int, user: c_int, pipe: c_int, version: *const c_char) -> *mut c_void,

//     pub get_isteam_game_search:
//         unsafe extern "C" fn(this: *mut c_int, user: c_int, pipe: c_int, version: *const c_char) -> *mut c_void,

//     pub run_frame:
//         unsafe extern "C" fn(this: *mut c_int),

//     pub get_ipc_call_count:
//         unsafe extern "C" fn(this: *mut c_int) -> *mut c_void,

//     pub set_warning_message_hook:
//         unsafe extern "C" fn(this: *mut c_int, hook: *mut c_void),

//     pub shutdown_if_all_pipes_closed:
//         unsafe extern "C" fn(this: *mut c_int),

//     pub get_isteam_http:
//         unsafe extern "C" fn(this: *mut c_int, user: c_int, pipe: c_int, version: *const c_char) -> *mut c_void,

//     pub deprecated_get_isteam_unified_messages:
//         unsafe extern "C" fn(this: *mut c_int, user: c_int, pipe: c_int, version: *const c_char) -> *mut c_void,

//     pub get_isteam_controller:
//         unsafe extern "C" fn(this: *mut c_int, user: c_int, pipe: c_int, version: *const c_char) -> *mut c_void,

//     pub get_isteam_ugc:
//         unsafe extern "C" fn(this: *mut c_int, user: c_int, pipe: c_int, version: *const c_char) -> *mut c_void,

//     pub get_isteam_app_list:
//         unsafe extern "C" fn(this: *mut c_int, user: c_int, pipe: c_int, version: *const c_char) -> *mut c_void,

//     pub get_isteam_music:
//         unsafe extern "C" fn(this: *mut c_int, user: c_int, pipe: c_int, version: *const c_char) -> *mut c_void,

//     pub get_isteam_music_remote:
//         unsafe extern "C" fn(this: *mut c_int, user: c_int, pipe: c_int, version: *const c_char) -> *mut c_void,

//     pub get_isteam_html_surface:
//         unsafe extern "C" fn(this: *mut c_int, user: c_int, pipe: c_int, version: *const c_char) -> *mut c_void,

//     pub deprecated_set_steamapi_cpostapiresult_in_process:
//         unsafe extern "C" fn(this: *mut c_int, callback: *mut c_void),

//     pub deprecated_remove_steamapi_cpostapiresult_in_process:
//         unsafe extern "C" fn(this: *mut c_int, callback: *mut c_void),

//     pub set_steamapi_ccheckcallbackregistered_in_process:
//         unsafe extern "C" fn(this: *mut c_int, func: *mut c_void),

//     pub get_isteam_inventory:
//         unsafe extern "C" fn(this: *mut c_int, user: c_int, pipe: c_int, version: *const c_char) -> *mut c_void,

//     pub get_isteam_video:
//         unsafe extern "C" fn(this: *mut c_int, user: c_int, pipe: c_int, version: *const c_char) -> *mut c_void,

//     pub get_isteam_parental_settings:
//         unsafe extern "C" fn(this: *mut c_int, user: c_int, pipe: c_int, version: *const c_char) -> *mut c_void,

//     pub get_isteam_input:
//         unsafe extern "C" fn(this: *mut c_int, user: c_int, pipe: c_int, version: *const c_char) -> *mut c_void,

//     pub get_isteam_parties:
//         unsafe extern "C" fn(this: *mut c_int, user: c_int, pipe: c_int, version: *const c_char) -> *mut c_void,
// }

#[derive(Debug)]
pub struct ISteamClient018 {
    pub vtable: *const ISteamClient018Functions,
}

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
    ) -> crate::wrappers::user::SteamUser {
        let mut native_version = String::from("SteamUser012\0");

        let result = unsafe {
            (self.vtable.get_isteam_user)(
                self.object_address,
                user,
                pipe,
                native_version.as_mut_ptr() as *mut i8,
            )
        };

        println!("{:?} - {:?} - {:?}", result, user, pipe);

        crate::wrappers::user::SteamUser::new(result)
    }
}
