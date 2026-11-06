use crate::hal::{self, Hal};
use crate::{logd, loge, logi, logw};
use std::fs::{self, File, OpenOptions};
use std::io::Read;
use std::os::fd::{AsRawFd, RawFd};
use std::os::unix::fs::{FileExt, OpenOptionsExt};
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const DISP_FEATURE: &str = "/dev/mi_display/disp_feature";
const FOD_STATUS: &str = "/sys/class/touch/touch_dev/fod_press_status";
const RESOLUTION_FACTOR: &str = "/sys/class/touch/touch_dev/resolution_factor";
const IOCTL_SET_LOCAL_HBM: libc::c_int = 0x400c440e;
const IOCTL_REGISTER_EVENT: libc::c_int = 0x400c4407;

const HBM_OFF: u32 = 0;
const HBM_OFF_AUTH_STOP: u32 = 1;
const HBM_ON_STATE_1: u32 = 2;
const HBM_ON_LOW_LIGHT: u32 = 3;
const HBM_ON_STATE_5: u32 = 4;

const EVENT_DISPLAY: u32 = 2;
const EVENT_BRIGHTNESS: u32 = 5;

const MISTOUCH_VENDOR_CODE: i32 = 28;
const UI_READY_TIMEOUT_VENDOR_CODE: i32 = 40;

#[derive(Default)]
struct State {
    hal_state: u32,
    hbm_on: bool,
    touch_down: bool,
    capture: bool,
    x: u32,
    y: u32,
    brightness: u32,
    gxzw_anim: bool,
    colorfade: bool,
    power_mode: u32,
    fp_mode: u32,
    lockout_mode: u32,
    touch_enable: u32,
    hbm_target: bool,
}

pub struct FodEngine {
    enabled: bool,
    hal: Option<Arc<Hal>>,
    hbm_file: Option<Arc<File>>,
    state: Mutex<State>,
    anti_mistouch_enable: AtomicBool,
    anti_mistouch_threshold: AtomicI32,
    anti_mistouch_count: AtomicI32,
    mistouch_reported: AtomicBool,
}

impl FodEngine {
    fn new(enabled: bool, hal: Option<Arc<Hal>>, hbm_file: Option<Arc<File>>) -> Self {
        let fod = hal::get_prop_bool("ro.hardware.fp.fod");
        let fod_c = hal::get_prop_bool("ro.hardware.fp.fod.c");
        let side_unlock = hal::get_prop("persist.vendor.sys.fp.side_unlock_type")
            .and_then(|value| value.into_bytes().first().copied())
            .is_some_and(|first| first != b'0');
        let enabled_prop = hal::get_prop_bool("persist.vendor.sys.fp.fp_anti_mistouch");
        let anti_mistouch_enable = enabled_prop && (fod || fod_c || !side_unlock);
        let anti_mistouch_threshold =
            hal::get_prop("persist.vendor.sys.fp.valid_times_per_screenlock")
                .and_then(|value| value.into_bytes().first().map(|b| *b as i32 - b'0' as i32))
                .unwrap_or(3);
        Self {
            enabled,
            hal,
            hbm_file,
            state: Mutex::new(State::default()),
            anti_mistouch_enable: AtomicBool::new(anti_mistouch_enable),
            anti_mistouch_threshold: AtomicI32::new(anti_mistouch_threshold),
            anti_mistouch_count: AtomicI32::new(0),
            mistouch_reported: AtomicBool::new(false),
        }
    }

    pub fn start(hal: Option<Arc<Hal>>) -> Arc<Self> {
        let enabled =
            hal::get_prop("ro.hardware.fp.fod.touch.ctl.version").as_deref() == Some("2.0");
        if !enabled {
            logi!("FodEngine disabled: ro.hardware.fp.fod.touch.ctl.version != 2.0");
            return Arc::new(Self::new(enabled, hal, None));
        }

        let hbm_file = match open_disp_feature() {
            Ok(file) => Some(Arc::new(file)),
            Err(e) => {
                loge!("FodEngine open {DISP_FEATURE} failed: {e}");
                None
            }
        };
        let engine = Arc::new(Self::new(enabled, hal, hbm_file));
        if engine.hbm_file.is_some() {
            engine.set_local_hbm(HBM_OFF);
        }
        engine.start_display_monitor();
        engine.start_touch_monitor();
        engine.start_evdev_monitor();
        engine
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn cloud_update_ocp_control(&self, param: u32) {
        if param != 0 && param != 0x10 {
            return;
        }
        let enabled = param == 0x10;
        hal::set_prop(
            "persist.vendor.sys.fp.fp_ocp_control",
            if enabled { "true" } else { "false" },
        );
        logi!("handlerCloudUpdateOcpControl: ocp control function is {enabled}");
        self.hal_ext_cmd(11, enabled as u32);
    }

    pub fn cloud_update_anti_mistouch(&self, param: u32) {
        if param == 0 || param == 0x10 {
            let enabled = param == 0x10;
            hal::set_prop(
                "persist.vendor.sys.fp.fp_anti_mistouch",
                if enabled { "true" } else { "false" },
            );
            let fod = hal::get_prop_bool("ro.hardware.fp.fod");
            let fod_c = hal::get_prop_bool("ro.hardware.fp.fod.c");
            let side_unlock = hal::get_prop("persist.vendor.sys.fp.side_unlock_type")
                .and_then(|value| value.into_bytes().first().copied())
                .is_some_and(|first| first != b'0');
            let enable = enabled && (fod || fod_c || !side_unlock);
            self.anti_mistouch_enable.store(enable, Ordering::SeqCst);
            logi!("handlerCloudUpdateAntiMistouch: enabled={enable}");
            self.hal_ext_cmd(14, enable as u32);
        } else if param & 1 != 0 {
            let threshold = ((param >> 4) & 0xf) as i32;
            self.anti_mistouch_threshold
                .store(threshold, Ordering::SeqCst);
            hal::set_prop(
                "persist.vendor.sys.fp.valid_times_per_screenlock",
                &threshold.to_string(),
            );
            logi!("handlerCloudUpdateAntiMistouch: threshold={threshold}");
        }
    }

    pub fn anti_mistouch_should_report(&self, vendor_code: i32) -> bool {
        vendor_code == MISTOUCH_VENDOR_CODE
            && self.anti_mistouch_enable.load(Ordering::SeqCst)
            && self.anti_mistouch_count.load(Ordering::SeqCst)
                < self.anti_mistouch_threshold.load(Ordering::SeqCst)
    }

    pub fn note_mistouch(&self) {
        self.set_local_hbm(HBM_ON_STATE_1);
        self.mistouch_reported.store(true, Ordering::SeqCst);
        let count = self.anti_mistouch_count.fetch_add(1, Ordering::SeqCst) + 1;
        logi!("anti mistouch enabled, report mistouch (vendor {MISTOUCH_VENDOR_CODE})");
        logi!("mistouch count is add up to = {count}");
    }

    pub fn take_mistouch_reported(&self) -> bool {
        self.mistouch_reported.swap(false, Ordering::SeqCst)
    }

    pub fn reset_anti_mistouch_count(&self) {
        self.anti_mistouch_count.store(0, Ordering::SeqCst);
    }

    pub fn handle_ui_ready_timeout(&self) {
        self.set_local_hbm(HBM_OFF);
        logi!("UI Ready timeout: onAcquired(7) ({UI_READY_TIMEOUT_VENDOR_CODE})");
    }

    pub fn on_ext_cmd(&self, cmd: u32, param: u32) {
        if !self.enabled {
            return;
        }
        match cmd {
            1 => {
                logd!("FodEngine condition cmd=1 gxzwAnim={param}");
                self.state
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .gxzw_anim = param != 0;
                self.check_fod_touch_status();
            }
            2 => {
                logd!("FodEngine condition cmd=2 colorfade={param}");
                self.state
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .colorfade = param != 0;
                self.check_fod_touch_status();
            }
            5 => {
                logd!("FodEngine condition cmd=5 lockout={param}");
                self.state
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .lockout_mode = param;
                self.check_fod_touch_status();
            }
            3 => {
                logd!("FodEngine power mode {param}");
                self.check_fod_touch_status_pre_power(param);
                self.state
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .power_mode = param;
                self.check_fod_touch_status();
                self.hal_ext_cmd(14, param);
            }
            4 => {
                let previous = {
                    let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
                    let previous = state.hal_state;
                    state.hal_state = param;
                    previous
                };
                logi!("FodEngine hal state {previous} -> {param}");
                match param {
                    1 | 3 => {
                        if self.hbm_on() {
                            self.set_local_hbm(HBM_OFF);
                        }
                    }
                    0 | 2 | 4 => {
                        self.set_local_hbm(HBM_OFF_AUTH_STOP);
                    }
                    _ => {}
                }
                self.state.lock().unwrap_or_else(|e| e.into_inner()).fp_mode = param;
                self.check_fod_touch_status();
            }
            6 | 7 | 8 => {
                logd!("FodEngine condition cmd={cmd} param={param}");
            }
            _ => {
                logw!("unknown FodEngine condition cmd={cmd} param={param}");
            }
        }
    }

    pub fn reset(&self, reason: &str) {
        if !self.enabled {
            return;
        }
        let (was_down, was_capturing, was_hbm_on) = {
            let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
            let was_down = state.touch_down;
            let was_capturing = state.capture;
            let was_hbm_on = state.hbm_on;
            state.touch_down = false;
            state.capture = false;
            state.hbm_target = false;
            state.x = 0;
            state.y = 0;
            (was_down, was_capturing, was_hbm_on)
        };
        if was_capturing {
            self.hal_ext_cmd(10, 0);
        }
        if was_down {
            self.hal_ext_cmd(1, 0);
        }
        if was_hbm_on {
            self.set_local_hbm(HBM_OFF);
        }
        logi!("FodEngine reset: {reason}");
    }

    fn check_fod_touch_status(&self) {
        let (gxzw_anim, colorfade, power_mode, fp_mode, lockout_mode, cached) = {
            let state = self.state.lock().unwrap_or_else(|e| e.into_inner());
            (
                state.gxzw_anim,
                state.colorfade,
                state.power_mode,
                state.fp_mode,
                state.lockout_mode,
                state.touch_enable,
            )
        };
        let mut touch_enable = 0;
        let mut logged_power = power_mode;
        if gxzw_anim {
            if power_mode == 2 {
                logged_power = 2;
                if !colorfade {
                    if lockout_mode == 0
                        && (fp_mode > 5 || (1u32.wrapping_shl(fp_mode & 0x1f) & 0x2a) == 0)
                    {
                        touch_enable = 0;
                    } else {
                        touch_enable = 1;
                    }
                } else {
                    touch_enable = cached;
                }
            } else {
                touch_enable = 1;
            }
        }
        logi!(
            "checkFodTouchStatus FP:{fp_mode} gxzwAnim:{} colorfade:{} lockoutmode:{lockout_mode} powermode:{logged_power} TouchEnable:{touch_enable}",
            gxzw_anim as u32,
            colorfade as u32
        );
        if cached == touch_enable {
            return;
        }
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .touch_enable = touch_enable;
        logi!("HandleFodTouchCtl:{touch_enable}");
        crate::touchfeature::set_touch_mode(touch_enable);
    }

    fn check_fod_touch_status_pre_power(&self, new_power_mode: u32) {
        if new_power_mode != 0 {
            return;
        }
        let (fp_mode, gxzw_anim, colorfade, cached) = {
            let state = self.state.lock().unwrap_or_else(|e| e.into_inner());
            (
                state.fp_mode,
                state.gxzw_anim,
                state.colorfade,
                state.touch_enable,
            )
        };
        if (fp_mode == 3 || gxzw_anim) && cached == 0 {
            self.state
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .touch_enable = 1;
            logi!(
                "checkFodTouchStatusPrePowerMode FP:{fp_mode} gxzwAnim:{} colorfade:{} powermode:0 TouchEnable:1",
                gxzw_anim as u32,
                colorfade as u32
            );
            logi!("HandleFodTouchCtl:3");
            crate::touchfeature::set_touch_mode(3);
        }
    }

    fn hbm_on(&self) -> bool {
        self.state.lock().unwrap_or_else(|e| e.into_inner()).hbm_on
    }

    fn set_local_hbm(&self, value: u32) {
        let Some(file) = self.hbm_file.as_ref() else {
            return;
        };
        let mut buf = [0u8; 12];
        buf[..8].copy_from_slice(&1u64.to_ne_bytes());
        buf[8..].copy_from_slice(&value.to_ne_bytes());
        let ret = unsafe { libc::ioctl(file.as_raw_fd(), IOCTL_SET_LOCAL_HBM, buf.as_ptr()) };
        if ret < 0 {
            let e = std::io::Error::last_os_error();
            loge!("setLocalHBM {value} failed: {e}");
        } else {
            logi!("setLocalHBM set local hbm:{value}");
            let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
            state.hbm_on = value >= HBM_ON_STATE_1;
            if !state.hbm_on {
                state.hbm_target = false;
            }
        }
    }

    fn hal_ext_cmd(&self, cmd: u32, param: u32) {
        let Some(hal) = self.hal.as_ref() else {
            return;
        };
        logd!("SendExtCmd cmd {cmd},param {param}");
        let ret = hal.ext_cmd(cmd, param);
        if ret != 0 {
            logw!("SendExtCmd cmd {cmd},param {param} returned {ret}");
        }
    }

    fn start_display_monitor(self: &Arc<Self>) {
        let engine = Arc::clone(self);
        if let Err(e) = thread::Builder::new()
            .name("fod-display".into())
            .spawn(move || engine.display_monitor())
        {
            loge!("spawn fod-display failed: {e}");
        }
    }

    fn display_monitor(&self) {
        loop {
            let mut file = match open_disp_feature() {
                Ok(file) => file,
                Err(e) => {
                    loge!("DisplayMonitor open {DISP_FEATURE} failed: {e}");
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }
            };
            logi!("DisplayMonitor open {DISP_FEATURE} success");
            register_display_events(file.as_raw_fd());
            let mut pfd = libc::pollfd {
                fd: file.as_raw_fd(),
                events: 0x49,
                revents: 0,
            };
            let mut buf = [0u8; 1024];
            loop {
                let ret = unsafe { libc::poll(&mut pfd, 1, 1000) };
                if ret < 0 {
                    let e = std::io::Error::last_os_error();
                    if e.kind() == std::io::ErrorKind::Interrupted {
                        continue;
                    }
                    loge!("DisplayMonitor poll failed: {e}");
                    thread::sleep(Duration::from_secs(1));
                    break;
                }
                if ret == 0 {
                    continue;
                }
                logd!("DisplayMonitor poll revents={:#x}", pfd.revents);
                match file.read(&mut buf) {
                    Ok(0) => {
                        logd!("DisplayMonitor read 0");
                        thread::sleep(Duration::from_millis(10));
                    }
                    Ok(n) => {
                        logd!("DisplayMonitor read {n} bytes {:02x?}", &buf[..n.min(32)]);
                        self.handle_display_buffer(&buf[..n]);
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                    Err(e) => {
                        loge!("DisplayMonitor read failed: {e}");
                        thread::sleep(Duration::from_secs(1));
                        break;
                    }
                }
            }
        }
    }

    fn handle_display_buffer(&self, buf: &[u8]) {
        let mut offset = 0;
        while offset + 16 <= buf.len() {
            let typ = u32::from_ne_bytes(buf[offset + 4..offset + 8].try_into().unwrap());
            let size =
                u32::from_ne_bytes(buf[offset + 8..offset + 12].try_into().unwrap()) as usize;
            if size < 16 || offset + size > buf.len() {
                logw!(
                    "DisplayMonitor invalid size={size} offset={offset} len={}",
                    buf.len()
                );
                break;
            }
            let value = u32::from_ne_bytes(buf[offset + 12..offset + 16].try_into().unwrap());
            logd!("DisplayMonitor event size={size} type={typ} value={value}");
            match typ {
                EVENT_DISPLAY => {
                    logi!("MI_DISP_EVENT_Display:{value}");
                    self.handle_display_event(value);
                }
                EVENT_BRIGHTNESS => {
                    let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
                    state.brightness = value;
                    logd!("MI_DISP_EVENT_Brightness:{value}");
                }
                other => {
                    logd!("MI_DISP_EVENT type={other} value={value}");
                }
            }
            offset += size;
        }
    }

    fn handle_display_event(&self, value: u32) {
        let start = value != 0 && ((value & 3) == 3 || (value & 8) != 0);
        let low_brightness = (value & 4) != 0;
        if start {
            let (x, y, already) = {
                let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
                let already = state.capture;
                state.capture = true;
                (state.x, state.y, already)
            };
            if already {
                return;
            }
            logi!("fod capture start low_brightness={low_brightness}");
            if x != 0 && y != 0 {
                self.hal_ext_cmd(2, x);
                self.hal_ext_cmd(3, y);
            }
            self.hal_ext_cmd(10, if low_brightness { 6 } else { 1 });
        } else {
            let was_capturing = {
                let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
                let was_capturing = state.capture;
                state.capture = false;
                was_capturing
            };
            if was_capturing {
                logi!("fod capture stop");
                self.hal_ext_cmd(10, 0);
            }
        }
    }

    fn start_touch_monitor(self: &Arc<Self>) {
        let engine = Arc::clone(self);
        if let Err(e) = thread::Builder::new()
            .name("fod-touch".into())
            .spawn(move || engine.touch_monitor())
        {
            loge!("spawn fod-touch failed: {e}");
        }
    }

    fn touch_monitor(&self) {
        let file = match File::open(FOD_STATUS) {
            Ok(file) => file,
            Err(e) => {
                loge!("open {FOD_STATUS} failed: {e}");
                return;
            }
        };
        logi!("open {FOD_STATUS} success");
        let mut last = None;
        let mut buf = [0u8; 1];
        let mut last_error = false;
        loop {
            match file.read_at(&mut buf, 0) {
                Ok(n) if n > 0 => {
                    last_error = false;
                    let down = buf[0] != b'0';
                    if last != Some(down) {
                        last = Some(down);
                        if down {
                            logi!("fod_press_status down");
                            self.on_finger_down();
                        } else {
                            logi!("fod_press_status up");
                            self.on_finger_up();
                        }
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    if !last_error {
                        loge!("read {FOD_STATUS} failed: {e}");
                        last_error = true;
                    }
                }
            }
            thread::sleep(Duration::from_millis(20));
        }
    }

    fn on_finger_down(&self) {
        let hal_state = self
            .state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .hal_state;
        if !matches!(hal_state, 1 | 3 | 5) {
            logi!("skip FOD touch down: hal state {hal_state}");
            return;
        }
        let (x, y) = {
            let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
            if state.touch_down {
                return;
            }
            state.touch_down = true;
            (state.x, state.y)
        };
        logi!("SYS_NODE_EVENT_FINGER_DOWN state={hal_state}");
        self.hal_ext_cmd(2, 0);
        self.hal_ext_cmd(3, 0);
        self.handle_local_hbm_set(true);
        self.hal_ext_cmd(1, 1);
        if x != 0 && y != 0 {
            self.hal_ext_cmd(2, x);
            self.hal_ext_cmd(3, y);
        }
        crate::touchfeature::set_auth_state(0);
    }

    fn on_finger_up(&self) {
        let (was_down, x, y) = {
            let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
            let was_down = state.touch_down;
            state.touch_down = false;
            let x = state.x;
            let y = state.y;
            state.x = 0;
            state.y = 0;
            (was_down, x, y)
        };
        if x != 0 && y != 0 {
            self.hal_ext_cmd(2, x);
            self.hal_ext_cmd(3, y);
        }
        self.handle_local_hbm_set(false);
        if was_down {
            logi!("SYS_NODE_EVENT_FINGER_UP");
            self.hal_ext_cmd(1, 0);
        }
    }

    fn handle_local_hbm_set(&self, down: bool) {
        if !down {
            let had_target = {
                let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
                let had_target = state.hbm_target;
                state.hbm_target = false;
                had_target
            };
            if had_target {
                self.set_local_hbm(HBM_OFF);
            }
            return;
        }
        let fp_mode = self.state.lock().unwrap_or_else(|e| e.into_inner()).fp_mode;
        let eligible = fp_mode < 6 && (1u32.wrapping_shl(fp_mode & 0x1f) & 0x2a) != 0;
        if !eligible {
            logi!("skip enable lhbm as fingerprint is idle");
            return;
        }
        let want_low = fp_mode == 3
            && self
                .state
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .brightness
                <= 411;
        let value = match fp_mode {
            3 if want_low => HBM_ON_LOW_LIGHT,
            5 => HBM_ON_STATE_5,
            _ => HBM_ON_STATE_1,
        };
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .hbm_target = true;
        if !self.hbm_on() {
            self.set_local_hbm(value);
        }
    }

    fn start_evdev_monitor(self: &Arc<Self>) {
        let engine = Arc::clone(self);
        if let Err(e) = thread::Builder::new()
            .name("fod-evdev".into())
            .spawn(move || engine.evdev_monitor())
        {
            loge!("spawn fod-evdev failed: {e}");
        }
    }

    fn evdev_monitor(&self) {
        let Some(path) = find_touch_event_path() else {
            loge!("touch event device not found");
            return;
        };
        let mut file = match OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NONBLOCK)
            .open(&path)
        {
            Ok(file) => file,
            Err(e) => {
                loge!("open {path} failed: {e}");
                return;
            }
        };
        let factor = fs::read_to_string(RESOLUTION_FACTOR)
            .ok()
            .and_then(|s| s.trim().parse::<u32>().ok())
            .filter(|v| *v > 0)
            .unwrap_or(16);
        let (fod_x, fod_y) = hal::parse_pair(
            hal::get_prop("persist.vendor.sys.fp.fod.location.X_Y"),
            (505, 2333),
        );
        let (fod_w, fod_h) = hal::parse_pair(
            hal::get_prop("persist.vendor.sys.fp.fod.size.width_height"),
            (210, 210),
        );
        let margin = 120;
        let left = fod_x - margin;
        let right = fod_x + fod_w + margin;
        let top = fod_y - margin;
        let bottom = fod_y + fod_h + margin;
        logi!("open {path} success, resolution factor {factor}");
        let mut buf = [0u8; 24 * 16];
        let mut raw_down = false;
        loop {
            match file.read(&mut buf) {
                Ok(0) => thread::sleep(Duration::from_millis(10)),
                Ok(n) => {
                    let mut coordinate_updated = false;
                    for event in buf[..n].chunks_exact(24) {
                        let typ = u16::from_ne_bytes([event[16], event[17]]);
                        let code = u16::from_ne_bytes([event[18], event[19]]);
                        let value =
                            i32::from_ne_bytes([event[20], event[21], event[22], event[23]]);
                        if typ == 3 && (code == 0x35 || code == 0x36) && value > 0 {
                            let value = value as u32 / factor;
                            if value == 0 {
                                continue;
                            }
                            let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
                            if code == 0x35 {
                                state.x = value;
                            } else {
                                state.y = value;
                            }
                            coordinate_updated = true;
                        } else if typ == 1 && (code == 0x14a || code == 0x145) {
                            if value == 1 {
                                if !raw_down {
                                    raw_down = true;
                                    logd!("BTN_TOUCH down");
                                }
                            } else if value == 0 && raw_down {
                                raw_down = false;
                                logd!("BTN_TOUCH up");
                            }
                        }
                    }
                    let (x, y, touch_down, hal_state) = {
                        let state = self.state.lock().unwrap_or_else(|e| e.into_inner());
                        (state.x, state.y, state.touch_down, state.hal_state)
                    };
                    let in_fod = (left..=right).contains(&(x as i32))
                        && (top..=bottom).contains(&(y as i32));
                    let ready = matches!(hal_state, 1 | 3 | 5);
                    if coordinate_updated && in_fod && (ready || touch_down) {
                        logd!("mTouchCenterX:{x}");
                        logd!("mTouchCenterY:{y}");
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(5));
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => {
                    loge!("read {path} failed: {e}");
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }
    }
}

fn open_disp_feature() -> std::io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(libc::O_CLOEXEC)
        .open(DISP_FEATURE)
}

fn register_display_events(fd: RawFd) {
    for cookie in [0u64, 1u64 << 32] {
        for typ in [EVENT_DISPLAY, EVENT_BRIGHTNESS] {
            let mut buf = [0u8; 12];
            buf[..8].copy_from_slice(&cookie.to_ne_bytes());
            buf[8..].copy_from_slice(&typ.to_ne_bytes());
            let ret = unsafe { libc::ioctl(fd, IOCTL_REGISTER_EVENT, buf.as_ptr()) };
            if ret < 0 {
                let e = std::io::Error::last_os_error();
                logw!("register display event cookie={cookie} type={typ} failed: {e}");
            } else {
                logi!("register display event cookie={cookie} type={typ} success");
            }
        }
    }
}

fn find_touch_event_path() -> Option<String> {
    let text = fs::read_to_string("/proc/bus/input/devices").ok()?;
    let mut name = String::new();
    let mut handlers = String::new();
    for line in text.lines().chain(std::iter::once("")) {
        if line.is_empty() {
            let lower = name.to_ascii_lowercase();
            if lower.contains("goodix") || lower.contains("fod") || lower.contains("fingerprint") {
                if let Some(event) = handlers.split_whitespace().find(|h| h.starts_with("event")) {
                    return Some(format!("/dev/input/{event}"));
                }
            }
            name.clear();
            handlers.clear();
        } else if let Some(value) = line.strip_prefix("N: Name=") {
            name = value.trim_matches('"').to_string();
        } else if let Some(value) = line.strip_prefix("H: Handlers=") {
            handlers = value.to_string();
        }
    }
    let fallback = "/dev/input/event7";
    Path::new(fallback).exists().then(|| fallback.to_string())
}
