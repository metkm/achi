use std::os::raw::c_int;

pub mod apps;
pub mod client;
pub mod native;
pub mod user;

pub trait Wrapper {
    fn new(address: *mut c_int) -> Self;
}
