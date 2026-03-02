use crate::wrappers::Wrapper;

#[derive(Debug, Clone)]
#[repr(C, packed(1))]
pub struct ISteamClient018Functions {
    pub create_steam_pipe: unsafe extern "C" fn() -> u32,
    pub release_steam_pipe: *mut u32,
    pub connect_to_global_user: *mut u32,
    pub create_local_user: *mut u32,
    pub release_user: *mut u32,
    pub get_isteam_user: *mut u32,
    pub get_isteam_gameserver: *mut u32,
    pub set_local_ip_binding: *mut u32,
    pub get_isteam_friends: *mut u32,
    pub get_isteam_utils: *mut u32,
    pub get_isteam_matchmaking: *mut u32,
    pub get_isteam_matchmaking_servers: *mut u32,
    pub get_isteam_generic_interface: *mut u32,
    pub get_isteam_user_stats: unsafe extern "C" fn(),
    pub get_isteam_gameserver_stats: *mut u32,
    pub get_isteam_apps: *mut u32,
    pub get_isteam_networking: *mut u32,
    pub get_isteam_remote_storage: *mut u32,
    pub get_isteam_screenshots: *mut u32,
    pub get_isteam_game_search: *mut u32,
    pub run_frame: *mut u32,
    pub get_ipc_call_count: *mut u32,
    pub set_warning_message_hook: *mut u32,
    pub shutdown_if_all_pipes_closed: *mut u32,
    pub get_isteam_http: *mut u32,
    pub deprecated_get_isteam_unified_messages: *mut u32,
    pub get_isteam_controller: *mut u32,
    pub get_isteam_ugc: *mut u32,
    pub get_isteam_app_list: *mut u32,
    pub get_isteam_music: *mut u32,
    pub get_isteam_music_remote: *mut u32,
    pub get_isteam_html_surface: *mut u32,
    pub deprecated_set_steamapi_cpostapiresult_in_process: *mut u32,
    pub deprecated_remove_steamapi_cpostapiresult_in_process: *mut u32,
    pub set_steamapi_ccheckcallbackregistered_in_process: *mut u32,
    pub get_isteam_inventory: *mut u32,
    pub get_isteam_video: *mut u32,
    pub get_isteam_parental_settings: *mut u32,
    pub get_isteam_input: *mut u32,
    pub get_isteam_parties: *mut u32,
}

#[derive(Debug)]
pub struct ISteamClient018 {
    pub vtable: *const ISteamClient018Functions,
}

pub struct SteamClient {
    pub vtable: ISteamClient018Functions,
}

impl Wrapper for SteamClient {
    fn new(object_address: *mut u32) -> Self {
        let ptr = object_address as *mut ISteamClient018;
        let face = unsafe { ptr.as_mut().unwrap() };

        Self {
            vtable: unsafe { (*face.vtable).clone() },
        }
    }
}

impl SteamClient {
    pub fn create_stream_pipe(&self) -> u32 {
        let pipe = unsafe { (self.vtable.create_steam_pipe)() };
        pipe
    }
}
