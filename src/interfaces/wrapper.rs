use std::ffi::c_int;

struct Wrapper<IFunctions> {
    pub vtable: IFunctions,
    pub object_address: *mut c_int
}

impl<I> Wrapper<I> {
    pub fn new() -> Self {
        todo!()
    }
}
