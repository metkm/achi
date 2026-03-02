use windows::{
    Win32::{
        Foundation::HMODULE,
        System::LibraryLoader::{GetProcAddress, LOAD_WITH_ALTERED_SEARCH_PATH, LoadLibraryExA},
    },
    core::PCSTR,
};

type CreateInterfaceFn = unsafe extern "C" fn(
    version: *const std::ffi::c_char,
    return_code: *mut std::ffi::c_void,
) -> *mut std::ffi::c_uint;

pub struct Steam {
    pub module: HMODULE,
}

impl Steam {
    pub fn new() -> Self {
        let path = b"C:\\Program Files (x86)\\Steam\\steamclient64.dll\0";
        let module = unsafe {
            LoadLibraryExA(PCSTR(path.as_ptr()), None, LOAD_WITH_ALTERED_SEARCH_PATH).unwrap()
        };

        Self { module }
    }

    pub fn get_export_function<T>(&self, name: &str) -> T
    where
        T: Copy,
    {
        let pc = PCSTR(name.as_ptr());
        let fnc = unsafe { GetProcAddress(self.module, pc).unwrap() };

        println!("{:?}", fnc);

        unsafe { std::mem::transmute_copy(&fnc) }
    }

    pub fn create_interface<T: crate::wrappers::Wrapper>(&self) -> T {
        let c_interface = self.get_export_function::<CreateInterfaceFn>("CreateInterface\0");

        let address = unsafe {
            c_interface(
                "SteamClient018\0".as_ptr() as *const i8,
                std::ptr::null_mut::<std::ffi::c_void>(),
            )
        };

        let res = T::new(address);
        res
    }
}
