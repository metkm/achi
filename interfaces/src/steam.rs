use crate::{
    callbacks::{
        CallbackMessage, FreeLastCallbackFn, GetCallbackFn, user_stats_received::ICallback,
    },
    error::{Error, Result},
};

use std::{
    ffi::{CString, c_int, c_void},
    os::windows::ffi::OsStrExt,
    path::Path,
    sync::Arc,
};

use super::{
    Interface,
    native::{
        CreateInterfaceFn, steam_apps001::ISteamApps001, steam_apps008::ISteamApps008,
        steam_client::ISteamClient018, steam_userstats::ISteamUserStats013,
    },
};

use log::{debug, info};
use windows::{
    Win32::{
        Foundation::{FreeLibrary, HMODULE},
        System::LibraryLoader::{GetProcAddress, LOAD_WITH_ALTERED_SEARCH_PATH, LoadLibraryExW},
    },
    core::{PCSTR, PCWSTR},
};

use winreg::{RegKey, enums::HKEY_LOCAL_MACHINE};

pub struct SteamClient {
    _steam: Steam,
    pub client: Arc<Interface<ISteamClient018>>,
    pub apps001: Arc<Interface<ISteamApps001>>,
    pub apps008: Arc<Interface<ISteamApps008>>,
    pub user_stats: Arc<Interface<ISteamUserStats013>>,
    _pipe: i32,
    _user: i32,
}

impl SteamClient {
    pub fn new() -> Result<Self> {
        let _steam = Steam::new()?;
        let client = _steam.get_steam_client()?;

        let _pipe = client.create_steam_pipe()?;
        let _user = client.connect_to_global_user(_pipe);

        let apps001 = client.get_steam_apps001(_user, _pipe);
        let apps008 = client.get_steam_apps008(_user, _pipe);
        let user_stats = client.get_steam_user_stats(_user, _pipe);

        Ok(Self {
            _steam,
            client: Arc::new(client),
            apps001: Arc::new(apps001),
            apps008: Arc::new(apps008),
            user_stats: Arc::new(user_stats),
            _pipe,
            _user,
        })
    }
}

pub struct Steam {
    pub module: HMODULE,
    pub callbacks: Vec<Box<dyn ICallback>>,
}

impl Steam {
    pub fn new() -> Result<Self> {
        let install_path = Steam::get_install_path()?;

        let target_path: Vec<u16> = Path::new(&install_path)
            .join("steamclient64.dll")
            .as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let module = unsafe {
            LoadLibraryExW(
                PCWSTR(target_path.as_ptr()),
                None,
                LOAD_WITH_ALTERED_SEARCH_PATH,
            )
            .map_err(|_| {
                Error::UnableToLoadSteam(
                    String::from_utf16(&target_path).unwrap_or_else(|_| String::from("")),
                )
            })?
        };

        Ok(Self {
            module,
            callbacks: vec![],
        })
    }

    pub fn get_install_path() -> Result<String> {
        let target = "SOFTWARE\\Valve\\Steam";

        let st = RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey(target)
            .map_err(|_| Error::UnableToReadRegistry(target.to_string()))?;

        let path: String = st.get_value("InstallPath")?;

        Ok(path)
    }

    pub fn get_export_function<T>(&self, name: &str) -> Option<T> {
        let pc = PCSTR(name.as_ptr());
        let fnc = unsafe { GetProcAddress(self.module, pc)? };

        unsafe { std::mem::transmute_copy(&fnc) }
    }

    pub fn get_steam_client(&self) -> Result<Interface<ISteamClient018>> {
        let c_interface = self
            .get_export_function::<CreateInterfaceFn>("CreateInterface\0")
            .ok_or(Error::UnableToCreateInterface)?;

        let address = unsafe {
            c_interface(
                CString::new("SteamClient018").unwrap().as_ptr(),
                std::ptr::null_mut::<c_void>(),
            )
        };

        Ok(Interface::<ISteamClient018>::new(address))
    }

    pub unsafe fn get_callback(&self, pipe: c_int, message: *mut CallbackMessage) -> Result<bool> {
        let _get_callback = self
            .get_export_function::<GetCallbackFn>("Steam_BGetCallback\0")
            .ok_or(Error::UnableToCreateCallback)?;

        let result = unsafe { _get_callback(pipe, message) };

        Ok(result)
    }

    pub fn free_last_callback(&self, pipe: c_int) -> Result<bool> {
        let _free_last_callback = self
            .get_export_function::<FreeLastCallbackFn>("Steam_FreeLastCallback\0")
            .ok_or(Error::UnableToCreateCallback)?;

        let result = unsafe { _free_last_callback(pipe) };

        Ok(result)
    }

    pub fn register_callback<C>(&mut self, cb: C)
    where
        C: ICallback + 'static,
    {
        self.callbacks.push(Box::new(cb));
    }

    pub fn run_callbacks(&mut self, pipe: i32) {
        let mut cb_msg = CallbackMessage::default();
        info!("Running callbacks");

        unsafe {
            while self.get_callback(pipe, &mut cb_msg).unwrap_or(false) {
                for cb in &mut self.callbacks {
                    debug!("this id: {}, msg cb id: {}", cb.id(), cb_msg.call_id);

                    if cb.id() != cb_msg.call_id {
                        continue;
                    }

                    cb.run(cb_msg.param_pointer);
                }

                self.free_last_callback(pipe).ok();
            }
        }
    }
}

impl Drop for Steam {
    fn drop(&mut self) {
        unsafe {
            FreeLibrary(self.module).ok();
        }
    }
}
