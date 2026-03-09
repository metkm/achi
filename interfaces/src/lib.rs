pub mod apps;
pub mod error;
pub mod native;
pub mod steam;
pub mod user;

use native::VTable;
use std::{
    ffi::{c_char, c_int, c_void},
    sync::atomic::AtomicPtr,
};

#[derive(Debug)]
pub struct Interface<I: VTable> {
    pub vtable: I::Functions,
    pub address: AtomicPtr<c_int>,
}

impl<I: VTable> Interface<I> {
    pub fn new(address: *mut c_int) -> Self {
        let ptr = address as *mut I;
        let f = unsafe { ptr.as_mut().unwrap() };

        Self {
            vtable: unsafe { *f.vtable() },
            address: AtomicPtr::new(address),
        }
    }
}

pub type CreateInterfaceFn =
    unsafe extern "C" fn(version: *const c_char, return_code: *mut c_void) -> *mut c_int;
