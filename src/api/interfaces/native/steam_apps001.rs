use crate::api::interfaces::interface::VTable;

use std::ffi::{c_char, c_int};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ISteamApps001Functions {
    pub get_app_data: unsafe extern "C" fn(
        this: *mut c_int,
        app_id: c_int,
        key: *const c_char,
        out_ptr: *mut c_char,
        out_len: c_int,
    ) -> c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct ISteamApps001 {
    pub vtable: *const ISteamApps001Functions,
}

impl VTable for ISteamApps001 {
    type Functions = ISteamApps001Functions;

    fn vtable(&self) -> *const Self::Functions {
        self.vtable
    }
}
