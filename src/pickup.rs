use crate::{logi, logw};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::process::{Command, Stdio};
use std::ptr;
use std::thread;
use std::time::{Duration, Instant};

const HELPER_DIR: &str = "/data/misc/mfp";
const HELPER: &str = "/data/misc/mfp/mfp-pickup";
const HELPER_TMP: &str = "/data/misc/mfp/.mfp-pickup.tmp";
const RETRY: Duration = Duration::from_secs(5);

pub fn start() {
    let _ = thread::Builder::new()
        .name("native-pickup".into())
        .spawn(run);
}

fn run() {
    loop {
        if let Err(e) = install_self() {
            logw!("native-pickup helper=install-error err={e} path={HELPER}");
        }
        logi!("native-pickup helper=spawn path={HELPER}");
        let result = Command::new(HELPER)
            .arg("--pickup")
            .arg("--child")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        match result {
            Ok(status) => logw!("native-pickup helper=exit status={status}"),
            Err(e) => logw!("native-pickup helper=spawn-error err={e} path={HELPER}"),
        }
        thread::sleep(RETRY);
    }
}

fn install_self() -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    std::fs::create_dir_all(HELPER_DIR)?;
    let mut dir_perms = std::fs::metadata(HELPER_DIR)?.permissions();
    dir_perms.set_mode(0o755);
    std::fs::set_permissions(HELPER_DIR, dir_perms)?;

    std::fs::copy("/proc/self/exe", HELPER_TMP)?;
    let mut file_perms = std::fs::metadata(HELPER_TMP)?.permissions();
    file_perms.set_mode(0o755);
    std::fs::set_permissions(HELPER_TMP, file_perms)?;
    std::fs::rename(HELPER_TMP, HELPER)?;
    Ok(())
}

const PICKUP_TYPE: c_int = 33171036;
const PACKAGE: &str = "com.hoshikv.mfpdaemon";
const WAKE_KEYCODE: &str = "224";
const EVENT_RATE_US: c_int = 100_000;
const WAKE_DEBOUNCE: Duration = Duration::from_secs(2);
const ALOOPER_PREPARE_ALLOW_NON_CALLBACKS: c_int = 1;
const ALOOPER_POLL_ERROR: c_int = -4;

#[repr(C)]
struct ASensorEvent {
    version: i32,
    sensor: i32,
    type_: i32,
    reserved0: i32,
    timestamp: i64,
    data: [f32; 16],
    flags: u32,
    reserved1: [i32; 3],
}

enum ASensorManager {}
enum ASensor {}
enum ASensorEventQueue {}
enum ALooper {}

type ALooperCallbackFunc = Option<unsafe extern "C" fn(c_int, c_int, *mut c_void) -> c_int>;

struct AndroidApi {
    get_instance: unsafe extern "C" fn(*const c_char) -> *mut ASensorManager,
    get_sensor: unsafe extern "C" fn(*mut ASensorManager, c_int, bool) -> *const ASensor,
    create_queue: unsafe extern "C" fn(
        *mut ASensorManager,
        *mut ALooper,
        c_int,
        ALooperCallbackFunc,
        *mut c_void,
    ) -> *mut ASensorEventQueue,
    enable_sensor: unsafe extern "C" fn(*mut ASensorEventQueue, *const ASensor) -> c_int,
    set_rate: unsafe extern "C" fn(*mut ASensorEventQueue, *const ASensor, c_int) -> c_int,
    get_events:
        unsafe extern "C" fn(*mut ASensorEventQueue, *mut ASensorEvent, usize) -> isize,
    get_handle: unsafe extern "C" fn(*const ASensor) -> c_int,
    looper_prepare: unsafe extern "C" fn(c_int) -> *mut ALooper,
    looper_poll: unsafe extern "C" fn(
        c_int,
        *mut c_int,
        *mut c_int,
        *mut *mut c_void,
    ) -> c_int,
}

impl AndroidApi {
    fn load() -> Option<Self> {
        let handle = unsafe {
            libc::dlopen(
                c"libandroid.so".as_ptr(),
                libc::RTLD_NOW | libc::RTLD_LOCAL,
            )
        };
        if handle.is_null() {
            logw!("native-pickup dlopen=error err={}", dlerror());
            return None;
        }

        unsafe {
            Some(Self {
                get_instance: symbol(handle, c"ASensorManager_getInstanceForPackage")?,
                get_sensor: symbol(handle, c"ASensorManager_getDefaultSensorEx")?,
                create_queue: symbol(handle, c"ASensorManager_createEventQueue")?,
                enable_sensor: symbol(handle, c"ASensorEventQueue_enableSensor")?,
                set_rate: symbol(handle, c"ASensorEventQueue_setEventRate")?,
                get_events: symbol(handle, c"ASensorEventQueue_getEvents")?,
                get_handle: symbol(handle, c"ASensor_getHandle")?,
                looper_prepare: symbol(handle, c"ALooper_prepare")?,
                looper_poll: symbol(handle, c"ALooper_pollOnce")?,
            })
        }
    }
}

pub fn run_helper() {
    if std::env::args().any(|arg| arg == "--child") {
        unsafe {
            libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGTERM);
            if libc::getppid() == 1 {
                return;
            }
        }
    }

    let Some(api) = AndroidApi::load() else {
        return;
    };
    let package = match CString::new(PACKAGE) {
        Ok(value) => value,
        Err(_) => return,
    };

    let manager = unsafe { (api.get_instance)(package.as_ptr()) };
    if manager.is_null() {
        logw!("native-pickup sensor=no-manager");
        return;
    }

    let sensor = unsafe { (api.get_sensor)(manager, PICKUP_TYPE, true) };
    if sensor.is_null() {
        logw!("native-pickup sensor=missing type={PICKUP_TYPE}");
        return;
    }
    let handle = unsafe { (api.get_handle)(sensor) };

    let looper = unsafe { (api.looper_prepare)(ALOOPER_PREPARE_ALLOW_NON_CALLBACKS) };
    if looper.is_null() {
        logw!("native-pickup sensor=no-looper");
        return;
    }

    let queue =
        unsafe { (api.create_queue)(manager, looper, 1, None, ptr::null_mut()) };
    if queue.is_null() {
        logw!("native-pickup sensor=no-queue");
        return;
    }

    let rc = unsafe { (api.enable_sensor)(queue, sensor) };
    if rc < 0 {
        logw!("native-pickup sensor=enable-error rc={rc}");
        return;
    }
    unsafe { (api.set_rate)(queue, sensor, EVENT_RATE_US) };
    logi!("native-pickup sensor=ready handle={handle} type={PICKUP_TYPE}");

    let mut last = Instant::now() - WAKE_DEBOUNCE;
    loop {
        let ident =
            unsafe { (api.looper_poll)(-1, ptr::null_mut(), ptr::null_mut(), ptr::null_mut()) };
        if ident == ALOOPER_POLL_ERROR {
            logw!("native-pickup sensor=poll-error");
            thread::sleep(Duration::from_secs(1));
            continue;
        }

        loop {
            let mut event: ASensorEvent = unsafe { std::mem::zeroed() };
            let count = unsafe { (api.get_events)(queue, &mut event, 1) };
            if count <= 0 {
                break;
            }
            if event.type_ == PICKUP_TYPE {
                if event.data[0] != 1.0 {
                    logi!("native-pickup ignore data0={:.1}", event.data[0]);
                    continue;
                }
                let now = Instant::now();
                if now.duration_since(last) >= WAKE_DEBOUNCE {
                    last = now;
                    on_pickup();
                }
            }
        }
    }
}

fn on_pickup() {
    if !wake_enabled() {
        logi!("native-pickup trigger wake=disabled");
        return;
    }

    logi!("native-pickup trigger wake=key");
    let result = Command::new("/system/bin/input")
        .args(["keyevent", WAKE_KEYCODE])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    match result {
        Ok(status) if status.success() => logi!("native-pickup wake=ok"),
        Ok(status) => logw!("native-pickup wake=fail status={status}"),
        Err(e) => logw!("native-pickup wake=error err={e}"),
    }
}

fn wake_enabled() -> bool {
    read_setting("secure", "oplus_customize_gesture_wake_up_arouse")
        .or_else(|| read_setting("global", "ambient_tilt_to_wake"))
        .unwrap_or(true)
}

fn read_setting(namespace: &str, key: &str) -> Option<bool> {
    let output = Command::new("/system/bin/settings")
        .args(["get", namespace, key])
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let value = String::from_utf8_lossy(&output.stdout);
    match value.trim() {
        "1" | "true" => Some(true),
        "0" | "false" => Some(false),
        _ => None,
    }
}

unsafe fn symbol<T: Copy>(handle: *mut c_void, name: &CStr) -> Option<T> {
    let ptr = unsafe { libc::dlsym(handle, name.as_ptr()) };
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { std::mem::transmute_copy(&ptr) })
    }
}

fn dlerror() -> String {
    let ptr = unsafe { libc::dlerror() };
    if ptr.is_null() {
        return "unknown".to_string();
    }
    unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned()
}
