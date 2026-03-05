use std::os::raw::c_int;

pub mod apps;
pub mod client;
pub mod native;
pub mod user;
pub mod userstats;

macro_rules! steam_interface {
    ($name:ident, $raw:ident, $vtable:ident) => {
        pub struct $name {
            pub vtable: $vtable,
            pub object_address: *mut c_int,
        }

        impl $name {
            pub fn new(address: *mut c_int) -> Self {
                let ptr = address as *mut $raw;
                let face = unsafe { ptr.as_mut().unwrap() };

                Self {
                    object_address: address,
                    vtable: unsafe { *face.vtable },
                }
            }
        }
    };
}

pub(crate) use steam_interface;
