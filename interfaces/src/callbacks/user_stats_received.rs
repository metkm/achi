use std::ffi::c_void;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct UserStatsReceivedT {
    pub game_id: u64,
    pub result: i32,
    pub steam_id: u64,
}

pub trait ICallback {
    fn id(&self) -> i32;
    fn is_server(&self) -> bool;
    fn run(&mut self, param: *mut c_void);
}

pub struct Callback<T> {
    pub id: i32,
    pub is_server: bool,
    pub on_run: Box<dyn FnMut(T) + 'static>,
}

impl ICallback for Callback<UserStatsReceivedT> {
    fn id(&self) -> i32 {
        self.id
    }

    fn is_server(&self) -> bool {
        self.is_server
    }

    fn run(&mut self, param: *mut c_void) {
        let p = param as *mut UserStatsReceivedT;
        unsafe { (self.on_run)((*p).clone()) }
    }
}

// pub struct UserStatsReceivedCallback {
//     id: i32,
//     is_server: bool,
// }

// impl UserStatsReceivedCallback {
//     pub fn new() -> Self {
//         Self {
//             id: 1101,
//             is_server: false,
//         }
//     }
// }

// impl Callback for UserStatsReceivedCallback {
//     fn id(&self) -> i32 {
//         self.id
//     }

//     fn is_server(&self) -> bool {
//         self.is_server
//     }

//     fn run(&self, param: *mut c_void) -> Self::Result {
//         let data = param as *const UserStatsReceivedT;
//         unsafe { (*data).clone() }
//     }
// }
