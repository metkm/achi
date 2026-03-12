pub mod apps;
pub mod callbacks;
pub mod error;
pub mod native;
pub mod steam;
pub mod userstats;
pub mod worker;

use native::VTable;
use std::{ffi::c_int, sync::atomic::AtomicPtr};

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
