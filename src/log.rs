use std::ffi::CString;
use std::os::raw::{c_char, c_int};

unsafe extern "C" {
    fn __android_log_print(prio: c_int, tag: *const c_char, fmt: *const c_char, ...) -> c_int;
}

pub const ERROR: i32 = 6;
pub const WARN: i32 = 5;
pub const INFO: i32 = 4;
pub const DEBUG: i32 = 3;

pub fn log(prio: i32, msg: &str) {
    let tag = c"OplusMiFP";
    match CString::new(format!("hoshikv | ~logs~ {msg}")) {
        Ok(msg) => unsafe {
            __android_log_print(prio, tag.as_ptr(), c"%s".as_ptr(), msg.as_ptr());
        },
        Err(_) => unsafe {
            __android_log_print(prio, tag.as_ptr(), c"%s".as_ptr(), c"hoshikv | ~logs~ bad-text".as_ptr());
        },
    }
}

#[macro_export]
macro_rules! logd {
    ($($arg:tt)*) => { $crate::log::log($crate::log::DEBUG, &format!($($arg)*)) };
}

#[macro_export]
macro_rules! logi {
    ($($arg:tt)*) => { $crate::log::log($crate::log::INFO, &format!($($arg)*)) };
}

#[macro_export]
macro_rules! logw {
    ($($arg:tt)*) => { $crate::log::log($crate::log::WARN, &format!($($arg)*)) };
}

#[macro_export]
macro_rules! loge {
    ($($arg:tt)*) => { $crate::log::log($crate::log::ERROR, &format!($($arg)*)) };
}
