pub mod client;

pub trait Wrapper {
    fn new(address: *mut u32) -> Self;
}
