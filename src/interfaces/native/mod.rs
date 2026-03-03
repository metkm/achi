pub mod steam_apps001;
pub mod steam_client;
pub mod steam_user;

use std::ffi::c_void;

type CallableDefaultNativeFunction = unsafe extern "C" fn() -> c_void;
