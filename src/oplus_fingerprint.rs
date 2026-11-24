use crate::service::Daemon;
use crate::{logd, logi, logw};
use binder::binder_impl::{
    Binder, BorrowedParcel, IBinderInternal, InterfaceClass, Remotable, TransactionCode, FLAG_ONEWAY,
};
use binder::{Interface, Status, StatusCode};
use binder_ndk_sys as sys;
use std::ffi::CString;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::Duration;

pub const SEND_FINGERPRINT_CMD_TX: TransactionCode = 1019;

const CB_TX: TransactionCode = 1001;
const TOUCH_DOWN: i32 = 1201;
const TOUCH_UP: i32 = 1202;

const CMD_AUTHENTICATE_TYPE: i32 = 1005;
const CMD_TOUCH_LISTENER: i32 = 1008;
const CMD_GET_ENROLL_TIMES: i32 = 1010;
const CMD_SCREEN_STATE: i32 = 1011;
const CMD_UNLOCK_SWITCH: i32 = 1020;
const CMD_POST_AUTH: i32 = 1024;

const SCREEN_OFF: i32 = 0;
const SCREEN_ON: i32 = 1;
const SCREEN_DOZE: i32 = 2;

const BACKLIGHT_BRIGHTNESS: &str = "/sys/class/backlight/panel0-backlight/brightness";
const FALLBACK_BACKLIGHT: u32 = 48;

const PANEL_SERVICE: &str = "vendor.oplus.hardware.displaypanelfeature.IDisplayPanelFeature/default";
const PANEL_DESCRIPTOR: &str = "vendor.oplus.hardware.displaypanelfeature.IDisplayPanelFeature";
const PANEL_GET_FEATURE: TransactionCode = 1;
const PANEL_SET_FEATURE: TransactionCode = 2;
const PANEL_GET_INFO: TransactionCode = 3;
const AIDL_GET_HASH: TransactionCode = 16777214;
const AIDL_GET_VERSION: TransactionCode = 16777215;

const FEATURE_UDFPS_TYPE: i32 = 211;
const FEATURE_HBM_CONTROL: i32 = 22;
const FEATURE_AOD_SMOOTH: i32 = 217;
const FEATURE_COLOR_CALIBRATION: i32 = 179;
const FEATURE_AOD_COMPAT: i32 = 28;
const FEATURE_DOZE_STATE: i32 = 195;

const UDFPS_TYPE_VALUE: i32 = 0x410;
const AOD_SMOOTH_CAPS: i32 = 0xF;
const INTERFACE_HASH: &str = "86d5512f98a34fe15c70d7bb459d3f7e853bcde3";

const TOUCH_SERVICE: &str = "vendor.oplus.hardware.touch.IOplusTouch/default";
const TOUCH_DESCRIPTOR: &str = "vendor.oplus.hardware.touch.IOplusTouch";
const TOUCH_INTERFACE_VERSION: i32 = 2;
const TOUCH_INTERFACE_HASH: &str = "ef41c5fab372bb5ad6a417f606845c87cf0e9b17";
const TOUCH_INITIALIZE: TransactionCode = 1;
const TOUCH_IS_NODE_SUPPORT: TransactionCode = 2;
const TOUCH_READ_NODE_FILE: TransactionCode = 3;
const TOUCH_WRITE_NODE_FILE: TransactionCode = 4;
const TOUCH_WRITE_BT_INFO: TransactionCode = 5;
const TOUCH_WRITE_NODE_FILE_ONEWAY: TransactionCode = 6;
const TOUCH_NOTIFY_CLIENT: TransactionCode = 7;
const TOUCH_REGISTER_EVENT_CALLBACK: TransactionCode = 8;
const TOUCH_UNREGISTER_EVENT_CALLBACK: TransactionCode = 9;
const TOUCH_NODE_COORDINATE: i32 = 2;
const TOUCH_SINGLE_TAP_COORDS: &str = "16,500:500,500:500,500:500,500:500,500:500,500:500,0";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Enroll,
    Auth,
}

impl Operation {
    fn code(self) -> i32 {
        match self {
            Operation::Enroll => 1,
            Operation::Auth => 2,
        }
    }
}

struct State {
    screen_state: i32,
    listener: bool,
    operation: i32,
    auth_done: bool,
    pending_down: bool,
    physical_down: bool,
    armed: bool,
    unlock_switch: Option<bool>,
    last_backlight: u32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            screen_state: SCREEN_ON,
            listener: false,
            operation: 0,
            auth_done: false,
            pending_down: false,
            physical_down: false,
            armed: false,
            unlock_switch: None,
            last_backlight: 0,
        }
    }
}

pub struct OplusFingerprint {
    daemon: Arc<Daemon>,
    state: Mutex<State>,
    backlight_guard: AtomicBool,
}

static INSTANCE: OnceLock<Arc<OplusFingerprint>> = OnceLock::new();

pub fn start(daemon: Arc<Daemon>) -> Arc<OplusFingerprint> {
    let instance = Arc::new(OplusFingerprint {
        daemon,
        state: Mutex::new(State::default()),
        backlight_guard: AtomicBool::new(false),
    });
    let _ = INSTANCE.set(Arc::clone(&instance));
    register_panel_service();
    register_touch_service();
    instance
}

fn global() -> Option<&'static Arc<OplusFingerprint>> {
    INSTANCE.get()
}

fn state_lock(state: &Mutex<State>) -> std::sync::MutexGuard<'_, State> {
    state.lock().unwrap_or_else(|e| e.into_inner())
}

fn user_unlocked() -> bool {
    crate::hal::get_prop_bool("sys.user.0.ce_available")
}

fn read_backlight() -> Option<u32> {
    std::fs::read_to_string(BACKLIGHT_BRIGHTNESS)
        .ok()?
        .trim()
        .parse::<u32>()
        .ok()
}

fn read_setting_backlight() -> Option<u32> {
    let output = Command::new("/system/bin/settings")
        .args(["get", "system", "screen_brightness"])
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let val = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<u32>()
        .ok()?;
    if val == 0 {
        return None;
    }
    Some(((val.saturating_mul(4095)) / 8191).clamp(15, 4095))
}

fn read_unlock_switch() -> Option<bool> {
    let output = std::process::Command::new("/system/bin/settings")
        .args([
            "get",
            "secure",
            "oplus_customize_fingerprint_unlock_switch",
        ])
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&output.stdout);
    logi!("switch raw={:?} rc={:?}", text.trim(), output.status.code());
    match text.trim() {
        "1" => Some(true),
        "0" => Some(false),
        _ => None,
    }
}

impl OplusFingerprint {
    fn unlock_switch_enabled(&self) -> bool {
        let cached = state_lock(&self.state).unlock_switch;
        if let Some(value) = cached {
            return value;
        }
        let value = read_unlock_switch();
        if let Some(value) = value {
            state_lock(&self.state).unlock_switch = Some(value);
        } else {
            logw!("switch read=failed");
        }
        value.unwrap_or(true)
    }

    fn set_unlock_switch(&self, enabled: bool) {
        let enrolling = {
            let mut state = state_lock(&self.state);
            state.unlock_switch = Some(enabled);
            state.operation == Operation::Enroll.code()
        };
        if !enabled && !enrolling {
            self.disarm_with("unlock-disabled");
        }
        logi!("switch enabled={enabled}");
    }

    fn arm(&self) {
        if !user_unlocked() {
            self.disarm_with("user-locked");
            return;
        }
        let (enrolling, already_armed) = {
            let state = state_lock(&self.state);
            (state.operation == Operation::Enroll.code(), state.armed)
        };
        if !enrolling {
            if !self.unlock_switch_enabled() {
                self.disarm_with("unlock-disabled");
                return;
            }
            if self.daemon.has_no_enrollments() {
                self.disarm_with("no-enrollment");
                return;
            }
        }
        if already_armed {
            return;
        }
        state_lock(&self.state).armed = true;
        let fod = self.daemon.fod();
        fod.on_ext_cmd(4, 1);
        fod.on_ext_cmd(1, 1);
        logi!("fod=on");
    }

    fn disarm(&self) {
        self.disarm_with("disarm");
    }

    fn disarm_with(&self, reason: &str) {
        state_lock(&self.state).armed = false;
        crate::touchfeature::set_touch_mode(0);
        let fod = self.daemon.fod();
        fod.on_ext_cmd(4, 0);
        fod.on_ext_cmd(1, 0);
        logi!("fod=off reason={reason}");
    }

    fn send_touch(&self, cmd: i32) {
        let Some(cb) = self.daemon.callback() else {
            logw!("touch tx={cmd} result=no-callback");
            return;
        };
        let binder = cb.as_binder();
        let result = binder.transact(CB_TX, FLAG_ONEWAY, |mut parcel| {
            parcel.write(&cmd)?;
            parcel.write(&8i32)?;
            parcel.write(&[0u8; 8][..])?;
            Ok(())
        });
        match result {
            Ok(_) => logd!("touch tx={cmd} result=ok"),
            Err(e) => logw!("touch tx={cmd} result=error err={e:?}"),
        }
    }

    fn touch_listener(&self) {
        let should_arm = {
            let mut state = state_lock(&self.state);
            state.listener = true;
            state.pending_down = false;
            state.physical_down = false;
            // Only arm immediately if an operation is active or the screen is off/doze (AOD/lockscreen).
            // Never arm while unlocked on SCREEN_ON with no active biometric operation.
            state.operation != 0 || state.screen_state != SCREEN_ON || !state.auth_done
        };
        if should_arm {
            self.arm();
        }
        logi!("listen=on armed={should_arm}");
    }

    fn screen_state(&self, value: i32) {
        let (reset_auth, should_arm, should_disarm, check_backlight) = {
            let mut state = state_lock(&self.state);
            let reset_auth =
                state.screen_state == SCREEN_ON && (value == SCREEN_OFF || value == SCREEN_DOZE);
            if reset_auth {
                if let Some(bl) = read_backlight().filter(|&v| v > 0) {
                    state.last_backlight = bl;
                }
                state.auth_done = false;
            }
            let prev = state.screen_state;
            state.screen_state = value;
            state.pending_down = false;
            state.physical_down = false;
            let active_op = state.operation != 0;
            let should_arm = active_op || (value != SCREEN_ON && state.listener);
            let should_disarm = value == SCREEN_ON && !active_op && state.auth_done;
            let check_backlight = value == SCREEN_ON && prev != SCREEN_ON && state.auth_done;
            (reset_auth, should_arm, should_disarm, check_backlight)
        };
        if should_arm {
            self.arm();
        } else if should_disarm {
            self.disarm_with("screen-on-idle");
        }
        if check_backlight {
            self.schedule_backlight_restore();
        }
        logi!("screen val={value} auth_reset={reset_auth} arm={should_arm} disarm={should_disarm}");
    }

    fn schedule_backlight_restore(&self) {
        if self
            .backlight_guard
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return;
        }
        let Some(instance) = global().cloned() else {
            self.backlight_guard.store(false, Ordering::SeqCst);
            return;
        };
        let _ = thread::Builder::new()
            .name("backlight-restore".into())
            .spawn(move || {
                for delay_ms in [150u64, 350u64] {
                    thread::sleep(Duration::from_millis(delay_ms));
                    let (screen_state, cached) = {
                        let state = state_lock(&instance.state);
                        (state.screen_state, state.last_backlight)
                    };
                    if screen_state == SCREEN_OFF {
                        break;
                    }
                    if read_backlight() == Some(0) {
                        let target = if cached > 0 {
                            cached
                        } else {
                            read_setting_backlight().unwrap_or(FALLBACK_BACKLIGHT)
                        };
                        if let Err(e) = std::fs::write(BACKLIGHT_BRIGHTNESS, target.to_string()) {
                            logw!("backlight=restore-error val={target} err={e}");
                        } else {
                            logi!("backlight=restore val={target}");
                        }
                    }
                }
                instance.backlight_guard.store(false, Ordering::SeqCst);
            });
    }

    fn authenticate_type(&self, auth_type: i32) {
        let should_arm = auth_type != 0;
        {
            let mut state = state_lock(&self.state);
            if should_arm {
                state.auth_done = false;
            }
            state.pending_down = false;
            state.physical_down = false;
        }
        if should_arm {
            self.arm();
        } else {
            self.disarm_with("auth-type-zero");
        }
        logi!("auth kind={auth_type}");
    }

    fn operation_start(&self, op: Operation) {
        let flush_down = {
            let mut state = state_lock(&self.state);
            state.operation = op.code();
            state.auth_done = false;
            state.listener = true;
            let flush_down = state.pending_down;
            state.pending_down = false;
            if flush_down {
                state.physical_down = true;
            }
            flush_down
        };
        self.arm();
        if flush_down {
            self.send_touch(TOUCH_DOWN);
        }
        logi!("session phase=begin kind={}", op.code());
    }

    fn touch_down(&self) {
        let send = {
            let mut state = state_lock(&self.state);
            if state.auth_done || state.physical_down {
                return;
            }
            state.physical_down = true;
            if !state.listener || state.operation == 0 {
                state.pending_down = true;
                false
            } else {
                state.pending_down = false;
                true
            }
        };
        if send {
            self.send_touch(TOUCH_DOWN);
        }
    }

    fn touch_up(&self) {
        let send = {
            let mut state = state_lock(&self.state);
            let was_down = state.physical_down;
            let auth_done = state.auth_done;
            state.physical_down = false;
            state.pending_down = false;
            state.listener && (was_down || auth_done)
        };
        if send {
            self.send_touch(TOUCH_UP);
        }
    }

    fn auth_success(&self) {
        {
            let mut state = state_lock(&self.state);
            state.auth_done = true;
            state.operation = 0;
            state.listener = false;
            state.pending_down = false;
        }
        self.disarm();
        self.schedule_backlight_restore();
        logi!("auth result=pass");
    }

    fn session_close(&self) {
        let auth_done = {
            let mut state = state_lock(&self.state);
            state.operation = 0;
            state.listener = false;
            state.pending_down = false;
            state.auth_done
        };
        self.disarm();
        logi!("session phase=end auth_done={auth_done}");
    }

    fn cancelled(&self) {
        let auth_done = {
            let mut state = state_lock(&self.state);
            state.operation = 0;
            state.listener = false;
            state.pending_down = false;
            state.physical_down = false;
            state.auth_done
        };
        self.disarm();
        logi!("session phase=abort auth_done={auth_done}");
    }

    fn no_enrollments(&self) {
        {
            let mut state = state_lock(&self.state);
            if state.operation == Operation::Enroll.code() {
                return;
            }
            state.operation = 0;
            state.listener = false;
            state.pending_down = false;
            state.physical_down = false;
        }
        self.disarm_with("no-enrollment");
        logi!("listen=off reason=no-enrollment");
    }

    fn panel_hbm_off(&self) {
        self.disarm();
        logi!("highlight=off");
    }

    fn panel_hbm_on(&self) {
        let allow = {
            let state = state_lock(&self.state);
            state.operation != 0 || state.screen_state != SCREEN_ON || !state.auth_done
        };
        if allow {
            self.arm();
            logi!("highlight=on");
        } else {
            logi!("highlight=skip reason=unlocked-idle");
        }
    }
}

pub fn on_operation_start(op: Operation) {
    if let Some(instance) = global() {
        instance.operation_start(op);
    }
}

pub fn on_touch_down() {
    if let Some(instance) = global() {
        instance.touch_down();
    }
}

pub fn on_touch_up() {
    if let Some(instance) = global() {
        instance.touch_up();
    }
}

pub fn on_auth_success() {
    if let Some(instance) = global() {
        instance.auth_success();
    }
}

pub fn on_session_close() {
    if let Some(instance) = global() {
        instance.session_close();
    }
}

pub fn on_cancelled() {
    if let Some(instance) = global() {
        instance.cancelled();
    }
}

pub fn on_no_enrollments() {
    if let Some(instance) = global() {
        instance.no_enrollments();
    }
}

fn write_ok(reply: &mut BorrowedParcel<'_>) -> Result<(), StatusCode> {
    reply.write(&Status::from(StatusCode::OK))
}

pub fn handle_send_fingerprint_cmd(
    data: &BorrowedParcel<'_>,
    reply: &mut BorrowedParcel<'_>,
) -> Result<(), StatusCode> {
    let cmd: i32 = data.read()?;
    let len: i32 = data.read()?;
    let array_len: i32 = data.read()?;
    let mut result = 0i32;

    match cmd {
        CMD_TOUCH_LISTENER => {
            if let Some(instance) = global() {
                instance.touch_listener();
            }
        }
        CMD_GET_ENROLL_TIMES => {
            result = -1;
        }
        CMD_SCREEN_STATE => {
            let value = if array_len >= 4 {
                data.read::<i32>()?
            } else {
                -1
            };
            if let Some(instance) = global() {
                instance.screen_state(value);
            }
        }
        CMD_AUTHENTICATE_TYPE => {
            let auth_type = if array_len >= 4 {
                data.read::<i32>()?
            } else {
                0
            };
            if let Some(instance) = global() {
                instance.authenticate_type(auth_type);
            }
        }
        CMD_UNLOCK_SWITCH => {
            let enabled = if array_len >= 1 {
                data.read::<u8>()? != 0
            } else {
                true
            };
            if let Some(instance) = global() {
                instance.set_unlock_switch(enabled);
            }
        }
        CMD_POST_AUTH => {
            logi!("postauth=unlock");
            if let Some(instance) = global() {
                instance.schedule_backlight_restore();
            }
        }
        other => {
            logi!("cmd id={other} result=ack");
        }
    }

    reply.write(&Status::from(StatusCode::OK))?;
    reply.write(&result)?;
    logd!("cmd id={cmd} txlen={len} argv={array_len} rc={result}");
    Ok(())
}

struct PanelFeature;

impl Remotable for PanelFeature {
    fn get_descriptor() -> &'static str {
        PANEL_DESCRIPTOR
    }

    fn on_transact(
        &self,
        code: TransactionCode,
        data: &BorrowedParcel<'_>,
        reply: &mut BorrowedParcel<'_>,
    ) -> Result<(), StatusCode> {
        match code {
            PANEL_GET_FEATURE => {
                let feature: i32 = data.read()?;
                let requested: i32 = data.read()?;
                match feature {
                    FEATURE_UDFPS_TYPE => {
                        write_ok(reply)?;
                        reply.write(&0i32)?;
                        reply.write(&[UDFPS_TYPE_VALUE][..])?;
                    }
                    FEATURE_COLOR_CALIBRATION => {
                        let valid = requested > 0 && requested <= 32;
                        let mut values = if valid {
                            vec![0i32; requested as usize]
                        } else {
                            Vec::new()
                        };
                        if !values.is_empty() {
                            values[0] = 1;
                        }
                        write_ok(reply)?;
                        reply.write(&(if valid { 0i32 } else { -1i32 }))?;
                        reply.write(&values)?;
                    }
                    FEATURE_AOD_SMOOTH => {
                        write_ok(reply)?;
                        reply.write(&0i32)?;
                        reply.write(&[AOD_SMOOTH_CAPS][..])?;
                    }
                    _ => {
                        let count = if requested > 0 && requested <= 32 {
                            requested as usize
                        } else {
                            0
                        };
                        let values = vec![0i32; count];
                        write_ok(reply)?;
                        reply.write(&-1i32)?;
                        reply.write(&values)?;
                    }
                }
                Ok(())
            }
            PANEL_SET_FEATURE => {
                let feature: i32 = data.read()?;
                let values: Vec<i32> = data.read()?;
                let value = values.first().copied().unwrap_or(-1);
                let result = match feature {
                    FEATURE_HBM_CONTROL => {
                        if let Some(instance) = global() {
                            if value != 0 {
                                instance.panel_hbm_on();
                            } else {
                                instance.panel_hbm_off();
                            }
                        }
                        0
                    }
                    FEATURE_AOD_COMPAT | FEATURE_AOD_SMOOTH => 0,
                    FEATURE_DOZE_STATE => {
                        if value == 0 || value == 1 {
                            crate::xiaomi_display::aod_doze();
                        }
                        0
                    }
                    _ => -1,
                };
                write_ok(reply)?;
                reply.write(&result)?;
                logd!("panel op=set feature={feature} val={value} rc={result}");
                Ok(())
            }
            AIDL_GET_VERSION => {
                write_ok(reply)?;
                reply.write(&1i32)
            }
            AIDL_GET_HASH => {
                write_ok(reply)?;
                reply.write(INTERFACE_HASH)
            }
            PANEL_GET_INFO => Err(StatusCode::UNKNOWN_TRANSACTION),
            _ => Err(StatusCode::UNKNOWN_TRANSACTION),
        }
    }

    fn on_dump(&self, _file: &std::fs::File, _args: &[&std::ffi::CStr]) -> Result<(), StatusCode> {
        Ok(())
    }

    fn get_class() -> InterfaceClass {
        InterfaceClass::new::<Binder<PanelFeature>>()
    }
}

fn prewarm_panel() {
    let Some(panel) = binder::get_service(PANEL_SERVICE) else {
        logw!("panel prewarm=proxy-error");
        return;
    };
    let result = panel.transact(PANEL_GET_FEATURE, 0, |mut parcel| {
        parcel.write(&FEATURE_UDFPS_TYPE)?;
        parcel.write(&1i32)?;
        Ok(())
    });
    match result {
        Ok(_) => logi!("panel prewarm feature=211 result=ok"),
        Err(e) => logw!("panel prewarm result=error err={e:?}"),
    }
}

fn register_panel_service() {
    let name = CString::new(PANEL_SERVICE).expect("panel service name");
    let existing = unsafe { sys::AServiceManager_checkService(name.as_ptr()) };
    if !existing.is_null() {
        unsafe { sys::AIBinder_decStrong(existing) };
        prewarm_panel();
        logi!("panel svc=present");
        return;
    }

    logw!("panel prewarm=unavailable");
    let binder = Binder::new(PanelFeature);
    match binder::add_service(PANEL_SERVICE, binder.as_binder()) {
        Ok(()) => logi!("panel svc=registered"),
        Err(e) => logw!("panel svc=error err={e:?}"),
    }
}

struct OplusTouch;

impl Remotable for OplusTouch {
    fn get_descriptor() -> &'static str {
        TOUCH_DESCRIPTOR
    }

    fn on_transact(
        &self,
        code: TransactionCode,
        data: &BorrowedParcel<'_>,
        reply: &mut BorrowedParcel<'_>,
    ) -> Result<(), StatusCode> {
        match code {
            TOUCH_INITIALIZE => {
                write_ok(reply)?;
                reply.write(&0i32)
            }
            TOUCH_IS_NODE_SUPPORT => {
                let _device_id: i32 = data.read()?;
                let node_flag: i32 = data.read()?;
                let supported = if node_flag == TOUCH_NODE_COORDINATE { 1i32 } else { 0i32 };
                write_ok(reply)?;
                reply.write(&supported)
            }
            TOUCH_READ_NODE_FILE => {
                let _device_id: i32 = data.read()?;
                let node_flag: i32 = data.read()?;
                let value = if node_flag == TOUCH_NODE_COORDINATE {
                    TOUCH_SINGLE_TAP_COORDS
                } else {
                    ""
                };
                write_ok(reply)?;
                reply.write(value)?;
                logd!("oplus-touch op=read node={node_flag}");
                Ok(())
            }
            TOUCH_WRITE_NODE_FILE
            | TOUCH_WRITE_BT_INFO
            | TOUCH_NOTIFY_CLIENT
            | TOUCH_REGISTER_EVENT_CALLBACK
            | TOUCH_UNREGISTER_EVENT_CALLBACK => {
                write_ok(reply)?;
                reply.write(&0i32)
            }
            TOUCH_WRITE_NODE_FILE_ONEWAY => Ok(()),
            AIDL_GET_VERSION => {
                write_ok(reply)?;
                reply.write(&TOUCH_INTERFACE_VERSION)
            }
            AIDL_GET_HASH => {
                write_ok(reply)?;
                reply.write(TOUCH_INTERFACE_HASH)
            }
            _ => Err(StatusCode::UNKNOWN_TRANSACTION),
        }
    }

    fn on_dump(&self, _file: &std::fs::File, _args: &[&std::ffi::CStr]) -> Result<(), StatusCode> {
        Ok(())
    }

    fn get_class() -> InterfaceClass {
        InterfaceClass::new::<Binder<OplusTouch>>()
    }
}

fn register_touch_service() {
    let name = CString::new(TOUCH_SERVICE).expect("touch service name");
    let existing = unsafe { sys::AServiceManager_checkService(name.as_ptr()) };
    if !existing.is_null() {
        unsafe { sys::AIBinder_decStrong(existing) };
        logi!("oplus-touch svc=present");
        return;
    }

    let binder = Binder::new(OplusTouch);
    match binder::add_service(TOUCH_SERVICE, binder.as_binder()) {
        Ok(()) => logi!("oplus-touch svc=registered"),
        Err(e) => logw!("oplus-touch svc=error err={e:?}"),
    }
}
