use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::sync::OnceLock;
use std::sync::mpsc::Sender;

const RTLD_NOW: c_int = 2;
const RTLD_LOCAL: c_int = 0;

const FINGERPRINT_DEVICE_VERSION: u32 = 0x201;

const OFF_SET_NOTIFY: usize = 0x80;
const OFF_GENERATE_CHALLENGE: usize = 0x88;
const OFF_REVOKE_CHALLENGE: usize = 0x90;
const OFF_ENROLL: usize = 0x98;
const OFF_GET_AUTHENTICATOR_ID: usize = 0xa0;
const OFF_INVALIDATE_AUTHENTICATOR_ID: usize = 0xa8;
const OFF_CANCEL: usize = 0xb0;
const OFF_ENUMERATE: usize = 0xb8;
const OFF_REMOVE: usize = 0xc0;
const OFF_SET_ACTIVE_GROUP: usize = 0xc8;
const OFF_AUTHENTICATE: usize = 0xd0;
const OFF_RESET_LOCKOUT: usize = 0xd8;
const OFF_ON_POINTER_DOWN: usize = 0xe0;
const OFF_ON_POINTER_UP: usize = 0xe8;
const OFF_EXT_CMD: usize = 0xf0;
const OFF_DETECT_INTERACTION: usize = 0x110;

const TOKEN_LEN: usize = 69;

unsafe extern "C" {
    fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn __system_property_get(name: *const c_char, value: *mut c_char) -> c_int;
    fn __system_property_set(name: *const c_char, value: *const c_char) -> c_int;
}

#[repr(C)]
struct HwModule {
    tag: u32,
    version: u32,
    id: *const c_char,
    name: *const c_char,
    author: *const c_char,
    methods: *const HwModuleMethods,
    dso: *mut c_void,
    reserved: [u32; 32],
}

#[repr(C)]
struct HwModuleMethods {
    open: Option<
        unsafe extern "C" fn(
            module: *const HwModule,
            id: *const c_char,
            device: *mut *mut HwDevice,
        ) -> c_int,
    >,
}

#[repr(C)]
struct HwDevice {
    tag: u32,
    version: u32,
    module: *mut HwModule,
    reserved: [u32; 12],
    close: Option<unsafe extern "C" fn(*mut HwDevice) -> c_int>,
}

type GetModuleByClass = unsafe extern "C" fn(
    id: *const c_char,
    vendor: *const c_char,
    module: *mut *mut HwModule,
) -> c_int;

type SetNotify = unsafe extern "C" fn(*mut c_void, NotifyCallback) -> c_int;
type NotifyCallback = unsafe extern "C" fn(*const FingerprintMsg);
type GenerateChallenge = unsafe extern "C" fn(*mut c_void) -> c_int;
type RevokeChallenge = unsafe extern "C" fn(*mut c_void, u64) -> c_int;
type Enroll = unsafe extern "C" fn(*mut c_void, *const u8) -> c_int;
type GetAuthenticatorId = unsafe extern "C" fn(*mut c_void) -> u64;
type InvalidateAuthenticatorId = unsafe extern "C" fn(*mut c_void, u64) -> c_int;
type Cancel = unsafe extern "C" fn(*mut c_void) -> c_int;
type Enumerate = unsafe extern "C" fn(*mut c_void) -> c_int;
type Remove = unsafe extern "C" fn(*mut c_void, *const u32, u32) -> c_int;
type SetActiveGroup = unsafe extern "C" fn(*mut c_void, u32, *const c_char) -> c_int;
type Authenticate = unsafe extern "C" fn(*mut c_void, u64) -> c_int;
type ResetLockout = unsafe extern "C" fn(*mut c_void, *const u8) -> c_int;
type OnPointerDown = unsafe extern "C" fn(*mut c_void, u32, u32, u32, f32, f32) -> c_int;
type OnPointerUp = unsafe extern "C" fn(*mut c_void, u32) -> c_int;
type ExtCmd = unsafe extern "C" fn(*mut c_void, u32, u32) -> c_int;
type DetectInteraction = unsafe extern "C" fn(*mut c_void) -> c_int;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FingerprintMsg {
    pub typ: i32,
    padding: i32,
    pub data: [u8; 80],
}

static NOTIFY_TX: OnceLock<Sender<FingerprintMsg>> = OnceLock::new();

pub fn set_notify_sender(tx: Sender<FingerprintMsg>) {
    let _ = NOTIFY_TX.set(tx);
}

unsafe extern "C" fn hal_notify(msg: *const FingerprintMsg) {
    if msg.is_null() {
        return;
    }
    if let Some(tx) = NOTIFY_TX.get() {
        let msg = unsafe { ptr::read(msg) };
        let _ = tx.send(msg);
    }
}

pub struct Hal {
    _lib: *mut c_void,
    _module_lib: *mut c_void,
    _get_module: GetModuleByClass,
    dev: *mut c_void,
}

unsafe impl Send for Hal {}
unsafe impl Sync for Hal {}

impl Hal {
    pub fn open() -> Option<Hal> {
        unsafe {
            let lib = dlopen(c"libhardware.so".as_ptr(), RTLD_NOW | RTLD_LOCAL);
            if lib.is_null() {
                crate::loge!("hal=error stage=libhardware-open");
                return None;
            }

            let sym = dlsym(lib, c"hw_get_module_by_class".as_ptr());
            if sym.is_null() {
                crate::loge!("hal=error stage=libhardware-symbol");
                return None;
            }
            let get_module: GetModuleByClass = std::mem::transmute(sym);

            let configured = get_prop("persist.vendor.sys.fp.vendor").unwrap_or_default();
            if !configured.is_empty() {
                if let Some((dev, module_lib)) = open_vendor(get_module, &configured) {
                    crate::logi!("hal=open vendor={configured}");
                    return Some(Hal {
                        _lib: lib,
                        _module_lib: module_lib,
                        _get_module: get_module,
                        dev,
                    });
                }
            }

            set_prop("persist.vendor.sys.fp.vendor", "none");
            for vendor in [
                "goodix_us",
                "qcom_us",
                "goodix_fod",
                "fpc_fod",
                "goodix_fod6",
                "goodix_fod7s",
                "goodix",
                "fpc",
                "jiiov",
            ] {
                if vendor == configured {
                    continue;
                }
                match open_vendor(get_module, vendor) {
                    Some((dev, module_lib)) => {
                        crate::logi!("hal=open vendor={vendor}");
                        set_prop("persist.vendor.sys.fp.vendor", vendor);
                        return Some(Hal {
                            _lib: lib,
                            _module_lib: module_lib,
                            _get_module: get_module,
                            dev,
                        });
                    }
                    None => {
                        crate::logd!("hal=skip vendor={vendor} reason=unavailable");
                    }
                }
            }

            crate::loge!("hal=error stage=no-vendor");
            None
        }
    }

    pub fn generate_challenge(&self) -> i32 {
        let Some(f) = (unsafe { slot::<GenerateChallenge>(self.dev, OFF_GENERATE_CHALLENGE) })
        else {
            return -1;
        };
        unsafe { f(self.dev) }
    }

    pub fn revoke_challenge(&self, challenge: u64) -> i32 {
        let Some(f) = (unsafe { slot::<RevokeChallenge>(self.dev, OFF_REVOKE_CHALLENGE) }) else {
            return -1;
        };
        unsafe { f(self.dev, challenge) }
    }

    pub fn enroll(&self, token: &[u8; TOKEN_LEN]) -> i32 {
        let Some(f) = (unsafe { slot::<Enroll>(self.dev, OFF_ENROLL) }) else {
            return -1;
        };
        unsafe { f(self.dev, token.as_ptr()) }
    }

    pub fn get_authenticator_id(&self) -> u64 {
        let Some(f) =
            (unsafe { slot::<GetAuthenticatorId>(self.dev, OFF_GET_AUTHENTICATOR_ID) })
        else {
            return 0;
        };
        unsafe { f(self.dev) }
    }

    pub fn invalidate_authenticator_id(&self, challenge: u64) -> i32 {
        let Some(f) = (unsafe {
            slot::<InvalidateAuthenticatorId>(self.dev, OFF_INVALIDATE_AUTHENTICATOR_ID)
        }) else {
            return -1;
        };
        unsafe { f(self.dev, challenge) }
    }

    pub fn cancel(&self) -> i32 {
        let Some(f) = (unsafe { slot::<Cancel>(self.dev, OFF_CANCEL) }) else {
            return -1;
        };
        unsafe { f(self.dev) }
    }

    pub fn enumerate(&self) -> i32 {
        let Some(f) = (unsafe { slot::<Enumerate>(self.dev, OFF_ENUMERATE) }) else {
            return -1;
        };
        unsafe { f(self.dev) }
    }

    pub fn remove(&self, ids: &[u32]) -> i32 {
        let Some(f) = (unsafe { slot::<Remove>(self.dev, OFF_REMOVE) }) else {
            return -1;
        };
        unsafe { f(self.dev, ids.as_ptr(), ids.len() as u32) }
    }

    pub fn set_active_group(&self, gid: u32, path: &CStr) -> i32 {
        let Some(f) = (unsafe { slot::<SetActiveGroup>(self.dev, OFF_SET_ACTIVE_GROUP) }) else {
            return -1;
        };
        unsafe { f(self.dev, gid, path.as_ptr()) }
    }

    pub fn authenticate(&self, operation_id: u64) -> i32 {
        let Some(f) = (unsafe { slot::<Authenticate>(self.dev, OFF_AUTHENTICATE) }) else {
            return -1;
        };
        unsafe { f(self.dev, operation_id) }
    }

    pub fn reset_lockout(&self, token: &[u8; TOKEN_LEN]) -> i32 {
        let Some(f) = (unsafe { slot::<ResetLockout>(self.dev, OFF_RESET_LOCKOUT) }) else {
            return -1;
        };
        unsafe { f(self.dev, token.as_ptr()) }
    }

    pub fn on_pointer_down(&self, pointer_id: i32, x: i32, y: i32, minor: f32, major: f32) -> i32 {
        let Some(f) = (unsafe { slot::<OnPointerDown>(self.dev, OFF_ON_POINTER_DOWN) }) else {
            return -1;
        };
        unsafe { f(self.dev, pointer_id as u32, x as u32, y as u32, minor, major) }
    }

    pub fn on_pointer_up(&self, pointer_id: i32) -> i32 {
        let Some(f) = (unsafe { slot::<OnPointerUp>(self.dev, OFF_ON_POINTER_UP) }) else {
            return -1;
        };
        unsafe { f(self.dev, pointer_id as u32) }
    }

    pub fn ext_cmd(&self, cmd: u32, param: u32) -> i32 {
        let Some(f) = (unsafe { slot::<ExtCmd>(self.dev, OFF_EXT_CMD) }) else {
            return 0;
        };
        unsafe { f(self.dev, cmd, param) }
    }

    pub fn detect_interaction(&self) -> i32 {
        let Some(f) = (unsafe { slot::<DetectInteraction>(self.dev, OFF_DETECT_INTERACTION) })
        else {
            return -1;
        };
        unsafe { f(self.dev) }
    }

    #[allow(dead_code)]
    pub fn close(&self) {
        if let Some(f) = unsafe { slot::<unsafe extern "C" fn(*mut HwDevice) -> c_int>(self.dev, 0x40) } {
            unsafe {
                f(self.dev as *mut HwDevice);
            }
        }
    }
}

unsafe fn slot<F>(dev: *mut c_void, offset: usize) -> Option<F> {
    let entry = unsafe { (dev as *const u8).add(offset) as *const *const c_void };
    let fun = unsafe { *entry };
    if fun.is_null() {
        None
    } else {
        Some(unsafe { std::mem::transmute_copy(&fun) })
    }
}

unsafe fn open_module(module: *mut HwModule) -> Option<*mut c_void> {
    let methods = unsafe { (*module).methods };
    if methods.is_null() {
        return None;
    }
    let Some(open) = (unsafe { (*methods).open }) else {
        return None;
    };
    let mut device: *mut HwDevice = ptr::null_mut();
    let ret = unsafe { open(module, ptr::null(), &mut device) };
    if ret != 0 || device.is_null() {
        return None;
    }
    if unsafe { (*device).version } != FINGERPRINT_DEVICE_VERSION {
        crate::loge!(
            "hal=error stage=device-version got={:#x} want={:#x}",
            unsafe { (*device).version },
            FINGERPRINT_DEVICE_VERSION
        );
        return None;
    }
    let Some(set_notify) = (unsafe { slot::<SetNotify>(device as *mut c_void, OFF_SET_NOTIFY) })
    else {
        crate::loge!("hal=error stage=no-set-notify");
        return None;
    };
    if unsafe { set_notify(device as *mut c_void, hal_notify) } != 0 {
        crate::loge!("hal=error stage=set-notify");
        return None;
    }
    Some(device as *mut c_void)
}

unsafe fn open_vendor(
    get_module: GetModuleByClass,
    vendor: &str,
) -> Option<(*mut c_void, *mut c_void)> {
    let id = c"fingerprint";
    let Ok(vendor) = CString::new(vendor) else {
        return None;
    };
    let mut module: *mut HwModule = ptr::null_mut();
    let ret = unsafe { get_module(id.as_ptr(), vendor.as_ptr(), &mut module) };
    if ret == 0 && !module.is_null() {
        if let Some(dev) = unsafe { open_module(module) } {
            return Some((dev, ptr::null_mut()));
        }
    }

    for dir in ["/odm/lib64/hw", "/vendor/lib64/hw"] {
        let path = format!("{dir}/fingerprint.{}.so", vendor.to_string_lossy());
        if !std::path::Path::new(&path).exists() {
            continue;
        }
        let Ok(cpath) = CString::new(path.as_str()) else {
            continue;
        };
        let handle = unsafe { dlopen(cpath.as_ptr(), RTLD_NOW | RTLD_LOCAL) };
        if handle.is_null() {
            crate::logd!("hal=direct-open-error path={path}");
            continue;
        }
        let sym = unsafe { dlsym(handle, c"HMI".as_ptr()) };
        if sym.is_null() {
            crate::logd!("hal=direct-symbol-error path={path}");
            continue;
        }
        if let Some(dev) = unsafe { open_module(sym as *mut HwModule) } {
            crate::logi!("hal=direct-open path={path}");
            return Some((dev, handle));
        }
    }
    None
}

pub fn get_prop(name: &str) -> Option<String> {
    let name = CString::new(name).ok()?;
    let mut buf = [0u8; 256];
    let len = unsafe { __system_property_get(name.as_ptr(), buf.as_mut_ptr() as *mut c_char) };
    if len <= 0 {
        return None;
    }
    let value = unsafe { CStr::from_ptr(buf.as_ptr() as *const c_char) };
    Some(value.to_string_lossy().into_owned())
}

pub fn set_prop(name: &str, value: &str) {
    let (Ok(name), Ok(value)) = (CString::new(name), CString::new(value)) else {
        return;
    };
    unsafe {
        __system_property_set(name.as_ptr(), value.as_ptr());
    }
}

pub fn get_prop_bool(name: &str) -> bool {
    matches!(get_prop(name).as_deref(), Some("true") | Some("1"))
}

pub fn sensor_type_from_props() -> i32 {
    let fod = get_prop_bool("ro.hardware.fp.fod");
    let side = get_prop_bool("ro.hardware.fp.sideCap");
    let fod_c = get_prop_bool("ro.hardware.fp.fod.c");
    if fod && !fod_c {
        3
    } else if side {
        4
    } else if fod_c {
        2
    } else {
        0
    }
}

pub fn parse_pair(value: Option<String>, default: (i32, i32)) -> (i32, i32) {
    let Some(value) = value else {
        return default;
    };
    let mut parts = value.split(',');
    let (Some(x), Some(y), None) = (parts.next(), parts.next(), parts.next()) else {
        return default;
    };
    match (x.trim().parse(), y.trim().parse()) {
        (Ok(x), Ok(y)) => (x, y),
        _ => default,
    }
}
