use crate::service::Daemon;
use crate::{logd, logi, logw};
use binder::binder_impl::{
    Binder, BorrowedParcel, IBinderInternal, InterfaceClass, Remotable, TransactionCode, FLAG_ONEWAY,
};
use binder::{Interface, Status, StatusCode};
use binder_ndk_sys as sys;
use std::ffi::CString;
use std::sync::{Arc, Mutex, OnceLock};

pub const SEND_FINGERPRINT_CMD_TX: TransactionCode = 1019;

const CB_TX: TransactionCode = 1001;
const TOUCH_DOWN: i32 = 1201;
const TOUCH_UP: i32 = 1202;

const CMD_AUTHENTICATE_TYPE: i32 = 1005;
const CMD_TOUCH_LISTENER: i32 = 1008;
const CMD_GET_ENROLL_TIMES: i32 = 1010;
const CMD_SCREEN_STATE: i32 = 1011;
const CMD_POST_AUTH: i32 = 1024;

const SCREEN_OFF: i32 = 0;
const SCREEN_ON: i32 = 1;
const SCREEN_DOZE: i32 = 2;

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

const UDFPS_TYPE_VALUE: i32 = 0x410;
const AOD_SMOOTH_CAPS: i32 = 0xF;
const INTERFACE_HASH: &str = "86d5512f98a34fe15c70d7bb459d3f7e853bcde3";

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
        }
    }
}

pub struct OplusFingerprint {
    daemon: Arc<Daemon>,
    state: Mutex<State>,
}

static INSTANCE: OnceLock<Arc<OplusFingerprint>> = OnceLock::new();

pub fn start(daemon: Arc<Daemon>) -> Arc<OplusFingerprint> {
    let instance = Arc::new(OplusFingerprint {
        daemon,
        state: Mutex::new(State::default()),
    });
    let _ = INSTANCE.set(Arc::clone(&instance));
    register_panel_service();
    instance
}

fn global() -> Option<&'static Arc<OplusFingerprint>> {
    INSTANCE.get()
}

fn state_lock(state: &Mutex<State>) -> std::sync::MutexGuard<'_, State> {
    state.lock().unwrap_or_else(|e| e.into_inner())
}

impl OplusFingerprint {
    fn arm(&self) {
        let fod = self.daemon.fod();
        fod.on_ext_cmd(4, 1);
        fod.on_ext_cmd(1, 1);
        logi!("fod=on");
    }

    fn disarm(&self) {
        crate::touchfeature::set_touch_mode(0);
        let fod = self.daemon.fod();
        fod.on_ext_cmd(4, 0);
        fod.on_ext_cmd(1, 0);
        logi!("fod=off");
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
        {
            let mut state = state_lock(&self.state);
            state.listener = true;
            state.auth_done = false;
            state.pending_down = false;
            state.physical_down = false;
        }
        self.arm();
        logi!("listen=on");
    }

    fn screen_state(&self, value: i32) {
        let reset_auth = {
            let mut state = state_lock(&self.state);
            let reset_auth =
                state.screen_state == SCREEN_ON && (value == SCREEN_OFF || value == SCREEN_DOZE);
            state.screen_state = value;
            if reset_auth {
                state.auth_done = false;
            }
            state.pending_down = false;
            state.physical_down = false;
            reset_auth
        };
        self.arm();
        logi!("screen val={value} auth_reset={reset_auth}");
    }

    fn authenticate_type(&self, auth_type: i32) {
        {
            let mut state = state_lock(&self.state);
            state.auth_done = false;
            state.pending_down = false;
            state.physical_down = false;
        }
        self.arm();
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

    fn panel_hbm_off(&self) {
        self.disarm();
        logi!("highlight=off");
    }

    fn panel_hbm_on(&self) {
        self.arm();
        logi!("highlight=on");
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
        CMD_POST_AUTH => {
            logi!("postauth=unlock");
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
                logd!("panel op=get feature={feature} n={requested}");
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
