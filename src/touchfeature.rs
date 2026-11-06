use binder_ndk_sys as sys;
use std::ffi::{CString, c_void};
use std::ptr;
use std::sync::OnceLock;

use crate::{logd, logw};

const SERVICE: &str = "vendor.xiaomi.hw.touchfeature.ITouchFeature/default";
const DESCRIPTOR: &str = "vendor.xiaomi.hw.touchfeature.ITouchFeature";
const TX_SET_MODE_VALUE: u32 = 9;

static CLASS: OnceLock<usize> = OnceLock::new();

unsafe extern "C" fn on_create(_args: *mut c_void) -> *mut c_void {
    ptr::null_mut()
}

unsafe extern "C" fn on_destroy(_user: *mut c_void) {}

unsafe extern "C" fn on_transact(
    _binder: *mut sys::AIBinder,
    _code: u32,
    _input: *const sys::AParcel,
    _output: *mut sys::AParcel,
) -> sys::binder_status_t {
    0
}

fn class() -> *const sys::AIBinder_Class {
    let ptr = *CLASS.get_or_init(|| {
        let desc = CString::new(DESCRIPTOR).expect("static descriptor");
        unsafe {
            sys::AIBinder_Class_define(
                desc.as_ptr(),
                Some(on_create),
                Some(on_destroy),
                Some(on_transact),
            ) as usize
        }
    });
    ptr as *const sys::AIBinder_Class
}

fn set_mode_value(touch_id: i32, mode: i32, value: i32) -> bool {
    let name = CString::new(SERVICE).expect("static service name");
    let binder = unsafe { sys::AServiceManager_getService(name.as_ptr()) };
    if binder.is_null() {
        logw!("TouchFeature service {SERVICE} unavailable");
        return false;
    }
    let clazz = class();
    if clazz.is_null() {
        logw!("TouchFeature class definition failed");
        unsafe { sys::AIBinder_decStrong(binder) };
        return false;
    }
    unsafe { sys::AIBinder_associateClass(binder, clazz) };

    let mut input = ptr::null_mut();
    let status = unsafe { sys::AIBinder_prepareTransaction(binder, &mut input) };
    if status != 0 || input.is_null() {
        logw!("TouchFeature prepareTransaction failed: {status}");
        unsafe { sys::AIBinder_decStrong(binder) };
        return false;
    }
    for word in [touch_id, mode, value] {
        let status = unsafe { sys::AParcel_writeInt32(input, word) };
        if status != 0 {
            logw!("TouchFeature writeInt32({word}) failed: {status}");
            unsafe {
                sys::AParcel_delete(input);
                sys::AIBinder_decStrong(binder);
            }
            return false;
        }
    }

    let mut output = ptr::null_mut();
    let status =
        unsafe { sys::AIBinder_transact(binder, TX_SET_MODE_VALUE, &mut input, &mut output, 0) };
    let ok = status == 0;
    if !ok {
        logw!("TouchFeature setModeValue({touch_id}, {mode}, {value}) transact failed: {status}");
    } else {
        logd!("TouchFeature setModeValue({touch_id}, {mode}, {value})");
    }
    unsafe {
        if !output.is_null() {
            sys::AParcel_delete(output);
        }
        sys::AIBinder_decStrong(binder);
    }
    ok
}

pub fn set_touch_mode(value: u32) {
    if value == 5 || value == 6 {
        set_mode_value(0, 35, value as i32);
    } else {
        set_mode_value(0, 10, value as i32);
    }
}

pub fn set_auth_state(value: u32) {
    set_mode_value(0, 41, value as i32);
}
