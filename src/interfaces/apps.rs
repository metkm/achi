use crate::interfaces::Wrapper;
use crate::interfaces::native::steam_apps001::{ISteamApps001, ISteamApps001Functions};
use std::ffi::{CString, c_int};

pub struct Apps {
    vtable: ISteamApps001Functions,
    object_address: *mut c_int,
}

impl Wrapper for Apps {
    fn new(address: *mut c_int) -> Self {
        let ptr = address as *mut ISteamApps001;
        let face = unsafe { ptr.as_mut().unwrap() };

        Self {
            object_address: address,
            vtable: unsafe { (*face.vtable).clone() },
        }
    }
}

impl Apps {
    pub fn get_appdata(&self, app_id: c_int, key: &str) -> Option<String> {
        let c_key = CString::new(key).unwrap();

        let mut buffer: [i8; 1024] = [0; 1024];

        let out_len = unsafe {
            (self.vtable.get_app_data)(
                self.object_address,
                app_id,
                c_key.as_ptr(),
                buffer.as_mut_ptr(),
                1024,
            )
        };

        if out_len == 0 {
            return None
        }

        Some(unsafe { CString::from_raw(buffer.as_mut_ptr()).into_string().unwrap() })
    }
}
