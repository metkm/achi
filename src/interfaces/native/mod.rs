pub mod steam_apps001;
pub mod steam_apps008;
pub mod steam_client;
pub mod steam_user;
pub mod steam_userstats;

use std::ffi::c_void;

type CallableDefaultNativeFunction = unsafe extern "C" fn() -> c_void;
