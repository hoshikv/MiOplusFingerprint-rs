use binder_ndk_sys as sys;
use std::ffi::{CString, c_void};
use std::ptr;
use std::sync::OnceLock;

use crate::{logd, logw};

const SERVICE: &str = "vendor.xiaomi.hardware.displayfeature_aidl.IDisplayFeature/default";
const DESCRIPTOR: &str = "vendor.xiaomi.hardware.displayfeature_aidl.IDisplayFeature";
const TX_SET_FEATURE: u32 = 7;
const FEATURE_DOZE_BRIGHTNESS: i32 = 25;

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

fn set_feature(display_id: i32, mode: i32, value: i32, cookie: i32) -> bool {
    let name = CString::new(SERVICE).expect("static service name");
    let binder = unsafe { sys::AServiceManager_getService(name.as_ptr()) };
    if binder.is_null() {
        logw!("displayfeature=unavailable service={SERVICE}");
        return false;
    }
    let clazz = class();
    if clazz.is_null() {
        logw!("displayfeature=error stage=class");
        unsafe { sys::AIBinder_decStrong(binder) };
        return false;
    }
    unsafe { sys::AIBinder_associateClass(binder, clazz) };

    let mut input = ptr::null_mut();
    let status = unsafe { sys::AIBinder_prepareTransaction(binder, &mut input) };
    if status != 0 || input.is_null() {
        logw!("displayfeature=error stage=prepare rc={status}");
        unsafe { sys::AIBinder_decStrong(binder) };
        return false;
    }
    for word in [display_id, mode, value, cookie] {
        let status = unsafe { sys::AParcel_writeInt32(input, word) };
        if status != 0 {
            logw!("displayfeature=error stage=word word={word} rc={status}");
            unsafe {
                sys::AParcel_delete(input);
                sys::AIBinder_decStrong(binder);
            }
            return false;
        }
    }

    let mut output = ptr::null_mut();
    let status =
        unsafe { sys::AIBinder_transact(binder, TX_SET_FEATURE, &mut input, &mut output, 0) };
    let ok = status == 0;
    if !ok {
        logw!("displayfeature=transact-error mode={mode} val={value} rc={status}");
    }
    unsafe {
        if !output.is_null() {
            sys::AParcel_delete(output);
        }
        sys::AIBinder_decStrong(binder);
    }
    ok
}

fn set_doze_brightness(value: i32) {
    set_feature(0, FEATURE_DOZE_BRIGHTNESS, value, 0);
}

pub fn aod_doze() {
    set_doze_brightness(0);
    set_doze_brightness(1);
    logd!("displayfeature=aod doze");
}
