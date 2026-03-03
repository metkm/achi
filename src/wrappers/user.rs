use std::ffi::{c_char, c_void};
use std::os::raw::{c_int, c_uint};

use crate::wrappers::Wrapper;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ISteamUser012Functions {
    pub get_hsteam_user: unsafe extern "C" fn(this: *mut c_int) -> c_int,

    pub logged_on: unsafe extern "C" fn(this: *mut c_int) -> bool,

    pub get_steam_id: unsafe extern "C" fn(this: *mut c_int, steam_id: &mut u64) -> c_void,

    pub initiate_game_connection: unsafe extern "C" fn(
        this: *mut c_int,
        auth_blob: *mut c_void,
        max_len: c_int,
        steam_id: u64,
        app_id: c_uint,
        ip: c_uint,
        port: u16,
        secure: bool,
    ) -> c_int,

    pub terminate_game_connection: unsafe extern "C" fn(this: *mut c_int, ip: c_uint, port: u16),

    pub track_app_usage_event: unsafe extern "C" fn(
        this: *mut c_int,
        game_id: u64,
        event_code: c_int,
        extra_info: *const c_char,
    ),

    pub get_user_data_folder:
        unsafe extern "C" fn(this: *mut c_int, buffer: *mut c_char, buffer_size: c_int) -> bool,

    pub start_voice_recording: unsafe extern "C" fn(this: *mut c_int),

    pub stop_voice_recording: unsafe extern "C" fn(this: *mut c_int),

    pub get_compressed_voice: unsafe extern "C" fn(
        this: *mut c_int,
        want_compressed: bool,
        dest_buffer: *mut c_void,
        dest_size: c_uint,
        bytes_written: *mut c_uint,
        want_uncompressed: bool,
        uncompressed_buffer: *mut c_void,
        uncompressed_size: c_uint,
        uncompressed_written: *mut c_uint,
    ) -> c_int,

    pub decompress_voice: unsafe extern "C" fn(
        this: *mut c_int,
        compressed: *const c_void,
        compressed_size: c_uint,
        dest_buffer: *mut c_void,
        dest_size: c_uint,
        bytes_written: *mut c_uint,
        sample_rate: c_uint,
    ) -> c_int,

    pub get_auth_session_ticket: unsafe extern "C" fn(
        this: *mut c_int,
        ticket: *mut c_void,
        max_ticket: c_int,
        ticket_size: *mut c_uint,
    ) -> c_uint,

    pub begin_auth_session: unsafe extern "C" fn(
        this: *mut c_int,
        ticket: *const c_void,
        ticket_size: c_int,
        steam_id: u64,
    ) -> c_int,

    pub end_auth_session: unsafe extern "C" fn(this: *mut c_int, steam_id: u64),

    pub cancel_auth_ticket: unsafe extern "C" fn(this: *mut c_int, ticket: c_uint),

    pub user_has_license_for_app:
        unsafe extern "C" fn(this: *mut c_int, steam_id: u64, app_id: c_uint) -> c_int,
}

pub struct ISteamUser012 {
    pub vtable: *const ISteamUser012Functions,
}

#[derive(Debug)]
pub struct SteamUser {
    vtable: ISteamUser012Functions,
    object_address: *mut c_int,
}

impl Wrapper for SteamUser {
    fn new(address: *mut c_int) -> Self {
        let ptr = address as *mut ISteamUser012;
        let face = unsafe { ptr.as_mut().unwrap() };

        Self {
            object_address: address,
            vtable: unsafe { (*face.vtable).clone() },
        }
    }
}

impl SteamUser {
    pub fn get_steam_id(&self) -> u64 {
        let mut steam_id: u64 = 0;
        unsafe { (self.vtable.get_steam_id)(self.object_address, &mut steam_id) };
        steam_id
    }
}
