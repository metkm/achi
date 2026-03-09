use crate::error::{Error, Result};

use std::{
    ffi::{CString, c_void},
    os::windows::ffi::OsStrExt,
    path::Path,
    sync::Arc,
};

use super::{
    CreateInterfaceFn, Interface,
    native::{
        steam_apps001::ISteamApps001, steam_apps008::ISteamApps008, steam_client::ISteamClient018,
        steam_userstats::ISteamUserStats013,
    },
};

use windows::{
    Win32::{
        Foundation::HMODULE,
        System::LibraryLoader::{GetProcAddress, LOAD_WITH_ALTERED_SEARCH_PATH, LoadLibraryExW},
    },
    core::{PCSTR, PCWSTR},
};

use winreg::{RegKey, enums::HKEY_LOCAL_MACHINE};

#[derive(Clone, Debug)]
pub struct SteamClient {
    pub client: Arc<Interface<ISteamClient018>>,
    pub apps001: Arc<Interface<ISteamApps001>>,
    pub apps008: Arc<Interface<ISteamApps008>>,
    pub user_stats: Arc<Interface<ISteamUserStats013>>,
}

impl SteamClient {
    pub fn new(id: Option<i32>) -> Result<Self> {
        if let Some(id) = id {
            unsafe {
                std::env::set_var("SteamAppId", id.to_string());
            }
        }

        let steam = Steam::new()?;
        let client = steam.get_steam_client()?;

        let pipe = client.create_steam_pipe()?;
        let user = client.connect_to_global_user(pipe);

        println!("pipe: {}, user: {}", pipe, user);

        let apps001 = client.get_steam_apps001(user, pipe);
        let apps008 = client.get_steam_apps008(user, pipe);
        let user_stats = client.get_steam_user_stats(user, pipe);

        Ok(Self {
            client: Arc::new(client),
            apps001: Arc::new(apps001),
            apps008: Arc::new(apps008),
            user_stats: Arc::new(user_stats),
        })
    }

    // pub fn get_apps001(&self) -> Arc<Interface<ISteamApps001>> {
    //     Arc::new(self.client.get_steam_apps001(self.user, self.pipe))
    // }

    // pub fn get_apps008(&self) -> Arc<Interface<ISteamApps008>> {
    //     Arc::new(self.client.get_steam_apps008(self.user, self.pipe))
    // }

    // pub fn get_user_stats(&self) -> Arc<Interface<ISteamUserStats013>> {
    //     Arc::new(self.client.get_steam_user_stats(self.user, self.pipe))
    // }

    // pub fn unload(&mut self) {
    //     self.client.release_steam_pipe(self.pipe);
    //     self.client.release_user(self.pipe, self.user);

    //     self.pipe = 0;
    //     self.user = 0;

    //     self.steam.release_module();
    // }

    // pub fn reload(&mut self) -> Result<()> {
    //     self.unload();

    //     let steam = Steam::new()?;

    //     self.steam = steam;
    //     self.client = Arc::new(self.steam.get_steam_client()?);

    //     Ok(())
    // }
}

#[derive(Clone, Debug)]
pub struct Steam {
    pub module: Arc<HMODULE>,
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
            module: Arc::new(module),
        })
    }

    // fn release_module(&self) {
    //     unsafe { FreeLibrary(*self.module).expect("Error freeing steamclient.dll") };
    // }

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
        let fnc = unsafe { GetProcAddress(*self.module, pc)? };

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
}
