use std::os::raw::c_int;

pub mod client;
pub mod user;
pub mod native;

pub trait Wrapper {
    fn new(address: *mut c_int) -> Self;
}
