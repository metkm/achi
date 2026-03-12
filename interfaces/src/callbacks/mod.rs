pub mod user_stats_received;

use std::ffi::{c_int, c_void};

// pub trait ICallback {
//     fn id(&self) -> i32;
//     fn is_server(&self) -> bool;
//     fn run(&self, param_ptr: *mut c_void);
// }

// #[repr(C)]
// pub struct Callback<T: Copy> {
//     pub id: i32,
//     pub is_server: bool,
//     pub on_run: Option<Box<dyn Fn(T)>>,
// }

// impl<T: Copy> ICallback for Callback<T> {
//     fn id(&self) -> i32 {
//         self.id
//     }

//     fn is_server(&self) -> bool {
//         self.is_server
//     }

//     fn run(&self, param_ptr: *mut c_void) {
//         unsafe {
//             let data = *(param_ptr as *const T);
//             if let Some(cb) = &self.on_run {
//                 cb(data);
//             }
//         }
//     }
// }

// pub unsafe trait Callback {
//     const ID: i32;
//     unsafe fn from_raw(raw: *mut c_void) -> Self;
// }

#[repr(C, packed)]
#[derive(Debug, Default)]
pub struct CallbackMessage {
    pub user: c_int,
    pub id: c_int,
    pub param_pointer: *mut c_void,
    pub param_size: c_int,
}

pub type GetCallbackFn =
    unsafe extern "C" fn(pipe: c_int, message: *mut CallbackMessage, call: &mut c_int) -> bool;
