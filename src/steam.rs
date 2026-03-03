use std::ffi::CString;

use windows::{
    Win32::{
        Foundation::HMODULE,
        System::LibraryLoader::{GetProcAddress, LOAD_WITH_ALTERED_SEARCH_PATH, LoadLibraryExA},
    },
    core::PCSTR,
};

use crate::error::AppError;

type CreateInterfaceFn = unsafe extern "C" fn(
    version: *const std::ffi::c_char,
    return_code: *mut std::ffi::c_void,
) -> *mut std::ffi::c_int;

pub struct Steam {
    pub module: HMODULE,
}

impl Steam {
    pub fn new() -> Result<Self, AppError> {
        let path = b"C:\\Program Files (x86)\\Steam\\steamclient64.dll\0";

        let module =
            unsafe { LoadLibraryExA(PCSTR(path.as_ptr()), None, LOAD_WITH_ALTERED_SEARCH_PATH)? };

        Ok(Self { module })
    }

    pub fn get_export_function<T>(&self, name: &str) -> Option<T> {
        let pc = PCSTR(name.as_ptr());
        let fnc = unsafe { GetProcAddress(self.module, pc)? };

        unsafe { std::mem::transmute_copy(&fnc) }
    }

    pub fn create_interface<T: crate::interfaces::Wrapper>(&self) -> Option<T> {
        let c_interface = self.get_export_function::<CreateInterfaceFn>("CreateInterface\0")?;

        let address = unsafe {
            c_interface(
                CString::new("SteamClient018").unwrap().as_ptr(),
                std::ptr::null_mut::<std::ffi::c_void>(),
            )
        };

        Some(T::new(address))
    }
}
