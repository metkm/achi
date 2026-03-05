use std::ffi::c_int;

use crate::interfaces::native::steam_user::{ISteamUser012, ISteamUser012Functions};

pub trait VTable {
    type Functions: Copy;

    fn vtable(&self) -> *const Self::Functions;
}

pub struct Interface<I: VTable> {
    pub vtable: I::Functions,
    pub address: *mut c_int,
}

impl<I: VTable> Interface<I> {
    pub fn new(address: *mut c_int) -> Self {
        let ptr = address as *mut I;
        let f = unsafe { ptr.as_mut().unwrap() };

        Self {
            vtable: unsafe { *f.vtable() },
            address,
        }
    }
}
