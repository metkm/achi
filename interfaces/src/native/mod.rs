pub mod steam_apps001;
pub mod steam_apps008;
pub mod steam_client;
pub mod steam_user;
pub mod steam_userstats;

use std::ffi::{c_char, c_int, c_void};

type CallableDefaultNativeFunction = unsafe extern "C" fn() -> c_void;

pub trait VTable {
    type Functions: Copy;

    fn vtable(&self) -> *const Self::Functions;
}

pub type CreateInterfaceFn =
    unsafe extern "C" fn(version: *const c_char, return_code: *mut c_void) -> *mut c_int;
