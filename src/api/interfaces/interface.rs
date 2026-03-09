use std::{ffi::c_int, sync::atomic::AtomicPtr};

pub trait VTable {
    type Functions: Copy;

    fn vtable(&self) -> *const Self::Functions;
}

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
