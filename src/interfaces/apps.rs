use crate::interfaces::native::steam_apps001::{ISteamApps001, ISteamApps001Functions};
use crate::interfaces::native::steam_apps008::{ISteamApps008, ISteamApps008Functions};
use crate::interfaces::steam_interface;
use std::ffi::{CString, c_int};

steam_interface!(Apps001, ISteamApps001, ISteamApps001Functions);

impl Apps001 {
    pub fn get_appdata(&self, app_id: c_int, key: &str) -> Option<String> {
        let c_key = CString::new(key).unwrap();

        let mut buffer: Vec<u8> = Vec::with_capacity(256);

        let out_len = unsafe {
            (self.vtable.get_app_data)(
                self.object_address,
                app_id,
                c_key.as_ptr(),
                buffer.as_mut_ptr() as *mut i8,
                1024,
            )
        };

        if out_len == 0 {
            return None;
        }

        unsafe {
            buffer.set_len(out_len as usize - 1);
        };

        let Ok(st) = String::from_utf8(buffer) else {
            return None;
        };

        Some(st)
    }
}

steam_interface!(Apps008, ISteamApps008, ISteamApps008Functions);

impl Apps008 {
    pub fn is_subscribed_app(&self, app_id: c_int) -> bool {
        (self.vtable.is_subscribed_app)(self.object_address, app_id)
    }
}
