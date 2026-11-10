use crate::fod::FodEngine;
use crate::hal::{self, FingerprintMsg, Hal};
use crate::mangled::_6_vendor_6_xiaomi_8_hardware_20_fingerprintextension_18_IXiaomiFingerprint as XiaomiTrait;
use crate::mangled::_7_android_8_hardware_9_keymaster_9_Timestamp as Timestamp;
use crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken as HardwareAuthToken;
use crate::mangled::_7_android_8_hardware_9_keymaster_25_HardwareAuthenticatorType as HardwareAuthenticatorType;
use crate::mangled::_7_android_8_hardware_10_biometrics_6_common_11_CommonProps as CommonProps;
use crate::mangled::_7_android_8_hardware_10_biometrics_6_common_13_ComponentInfo as ComponentInfo;
use crate::mangled::_7_android_8_hardware_10_biometrics_6_common_14_SensorStrength as SensorStrength;
use crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext as OperationContext;
use crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal as CancellationTrait;
use crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_5_Error as Error;
use crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_8_ISession as SessionTrait;
use crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_11_SensorProps as SensorProps;
use crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_11_SensorShape as SensorShape;
use crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_12_AcquiredInfo as AcquiredInfo;
use crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_12_IFingerprint as IFingerprintTrait;
use crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext as PointerContext;
use crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_SensorLocation as SensorLocation;
use crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_16_ISessionCallback as Cb;
use crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_21_FingerprintSensorType as FingerprintSensorType;
use binder::{BinderFeatures, DeathRecipient, IBinder, Interface, Strong};
use std::ffi::CString;
use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub const FINGERPRINT_SERVICE: &str =
    "android.hardware.biometrics.fingerprint.IFingerprint/default";
pub const XIAOMI_SERVICE: &str =
    "vendor.xiaomi.hardware.fingerprintextension.IXiaomiFingerprint/default";

const TOKEN_LEN: usize = 69;

const GS_IDLE: i32 = 0;
const GS_ENROLL: i32 = 2;
const GS_ENUMERATE: i32 = 5;
const GS_REMOVE: i32 = 6;
const GS_AUTHENTICATE: i32 = 8;

const SESSION_IDLE: i32 = 0;
const SESSION_CLOSED: i32 = 1;
const SESSION_GENERATE_CHALLENGE: i32 = 2;
const SESSION_REVOKE_CHALLENGE: i32 = 3;
const SESSION_ENROLL: i32 = 4;
const SESSION_AUTHENTICATE: i32 = 5;
const SESSION_DETECT_INTERACTION: i32 = 6;
const SESSION_ENUMERATE: i32 = 7;
const SESSION_REMOVE: i32 = 8;
const SESSION_GET_AUTHENTICATOR_ID: i32 = 9;
const SESSION_INVALIDATE_AUTHENTICATOR_ID: i32 = 10;
const SESSION_RESET_LOCKOUT: i32 = 11;

#[derive(Clone, Copy, PartialEq, Eq)]
enum LockoutState {
    None,
    Timed,
    Permanent,
}

struct Shared {
    callback: Option<Strong<dyn Cb>>,
    global_status: i32,
    failed_attempts: i32,
    remaining_attempts: i32,
    lockout: LockoutState,
    lockout_generation: u64,
    current_session_id: u64,
}

impl Default for Shared {
    fn default() -> Self {
        Self {
            callback: None,
            global_status: GS_IDLE,
            failed_attempts: 0,
            remaining_attempts: 5,
            lockout: LockoutState::None,
            lockout_generation: 0,
            current_session_id: 0,
        }
    }
}

struct SessionState {
    id: u64,
    status: AtomicI32,
    callback: Mutex<Option<Strong<dyn Cb>>>,
    death: Mutex<Option<DeathRecipient>>,
}

impl SessionState {
    fn new(id: u64, callback: Strong<dyn Cb>) -> Self {
        Self {
            id,
            status: AtomicI32::new(SESSION_IDLE),
            callback: Mutex::new(Some(callback)),
            death: Mutex::new(None),
        }
    }

    fn is_closed(&self) -> bool {
        self.status.load(Ordering::SeqCst) == SESSION_CLOSED
    }

    fn finish(&self) {
        if !self.is_closed() {
            self.status.store(SESSION_IDLE, Ordering::SeqCst);
        }
    }
}

enum Job {
    Notify(FingerprintMsg),
    GenerateChallenge(Arc<SessionState>),
    RevokeChallenge(Arc<SessionState>, u64),
    Enroll(Arc<SessionState>, [u8; TOKEN_LEN]),
    Authenticate(Arc<SessionState>, u64),
    Enumerate(Arc<SessionState>),
    Remove(Arc<SessionState>, Vec<u32>),
    GetAuthenticatorId(Arc<SessionState>),
    InvalidateAuthenticatorId(Arc<SessionState>),
    ResetLockout(Arc<SessionState>, [u8; TOKEN_LEN]),
    Cancel,
    ClientDied(Arc<SessionState>),
    AddFailedAttempts,
    ClearFailedAttempts,
}

pub struct Daemon {
    hal: Option<Arc<Hal>>,
    fod: Arc<FodEngine>,
    tx: Sender<Job>,
    shared: Mutex<Shared>,
    next_session_id: AtomicU64,
    sensor_type: i32,
}

impl Daemon {
    pub fn start(hal: Option<Arc<Hal>>, notify_rx: Receiver<FingerprintMsg>) -> Arc<Self> {
        let (tx, rx) = mpsc::channel();
        let fod = FodEngine::start(hal.clone());
        let daemon = Arc::new(Self {
            hal,
            fod,
            tx,
            shared: Mutex::new(Shared::default()),
            next_session_id: AtomicU64::new(1),
            sensor_type: hal::sensor_type_from_props(),
        });

        let notify_daemon = Arc::clone(&daemon);
        thread::Builder::new()
            .name("hal-notify".into())
            .spawn(move || {
                while let Ok(msg) = notify_rx.recv() {
                    if notify_daemon.tx.send(Job::Notify(msg)).is_err() {
                        break;
                    }
                }
            })
            .expect("spawn hal-notify");

        let worker_daemon = Arc::clone(&daemon);
        thread::Builder::new()
            .name("hal-worker".into())
            .spawn(move || worker_loop(worker_daemon, rx))
            .expect("spawn hal-worker");

        daemon
    }

    pub fn has_hal(&self) -> bool {
        self.hal.is_some()
    }

    fn shared(&self) -> std::sync::MutexGuard<'_, Shared> {
        self.shared.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub(crate) fn callback(&self) -> Option<Strong<dyn Cb>> {
        self.shared().callback.clone()
    }

    pub(crate) fn fod(&self) -> &Arc<FodEngine> {
        &self.fod
    }

    fn global_status(&self) -> i32 {
        self.shared().global_status
    }

    fn set_global_status(&self, status: i32) {
        self.shared().global_status = status;
    }

    fn next_session_id(&self) -> u64 {
        self.next_session_id.fetch_add(1, Ordering::SeqCst)
    }
}

fn worker_loop(daemon: Arc<Daemon>, rx: Receiver<Job>) {
    while let Ok(job) = rx.recv() {
        handle_job(&daemon, job);
    }
}

fn begin_request(
    daemon: &Daemon,
    state: &SessionState,
    session_status: i32,
    global_status: Option<i32>,
) -> bool {
    if state.is_closed() {
        return false;
    }
    state.status.store(session_status, Ordering::SeqCst);
    if let Some(status) = global_status {
        daemon.set_global_status(status);
    }
    true
}

fn handle_job(daemon: &Arc<Daemon>, job: Job) {
    match job {
        Job::Notify(msg) => dispatch_notify(daemon, &msg),
        Job::GenerateChallenge(state) => {
            if begin_request(daemon, &state, SESSION_GENERATE_CHALLENGE, None) {
                if let Some(hal) = daemon.hal.as_ref() {
                    hal.generate_challenge();
                }
                state.finish();
            }
        }
        Job::RevokeChallenge(state, challenge) => {
            if begin_request(daemon, &state, SESSION_REVOKE_CHALLENGE, None) {
                if let Some(hal) = daemon.hal.as_ref() {
                    hal.revoke_challenge(challenge);
                }
                state.finish();
            }
        }
        Job::Enroll(state, token) => {
            if begin_request(daemon, &state, SESSION_ENROLL, Some(GS_ENROLL)) {
                crate::oplus_fingerprint::on_operation_start(crate::oplus_fingerprint::Operation::Enroll);
                if let Some(hal) = daemon.hal.as_ref() {
                    hal.enroll(&token);
                }
                state.finish();
            }
        }
        Job::Authenticate(state, operation_id) => {
            if begin_request(daemon, &state, SESSION_AUTHENTICATE, Some(GS_AUTHENTICATE)) {
                crate::oplus_fingerprint::on_operation_start(crate::oplus_fingerprint::Operation::Auth);
                if let Some(hal) = daemon.hal.as_ref() {
                    hal.authenticate(operation_id);
                }
                state.finish();
            }
        }
        Job::Enumerate(state) => {
            if begin_request(daemon, &state, SESSION_ENUMERATE, Some(GS_ENUMERATE)) {
                if let Some(hal) = daemon.hal.as_ref() {
                    if hal.enumerate() != 0 {
                        daemon.set_global_status(GS_IDLE);
                    }
                }
                state.finish();
            }
        }
        Job::Remove(state, ids) => {
            if begin_request(daemon, &state, SESSION_REMOVE, Some(GS_REMOVE)) {
                if let Some(hal) = daemon.hal.as_ref() {
                    if hal.remove(&ids) != 0 {
                        daemon.set_global_status(GS_IDLE);
                    }
                }
                state.finish();
            }
        }
        Job::GetAuthenticatorId(state) => {
            if begin_request(daemon, &state, SESSION_GET_AUTHENTICATOR_ID, None) {
                if let Some(hal) = daemon.hal.as_ref() {
                    hal.get_authenticator_id();
                }
                state.finish();
            }
        }
        Job::InvalidateAuthenticatorId(state) => {
            if begin_request(daemon, &state, SESSION_INVALIDATE_AUTHENTICATOR_ID, None) {
                if let Some(hal) = daemon.hal.as_ref() {
                    hal.invalidate_authenticator_id(0);
                }
                state.finish();
            }
        }
        Job::ResetLockout(state, token) => {
            if begin_request(daemon, &state, SESSION_RESET_LOCKOUT, None) {
                if let Some(hal) = daemon.hal.as_ref() {
                    hal.reset_lockout(&token);
                }
                state.finish();
            }
        }
        Job::Cancel => {
            if let Some(hal) = daemon.hal.as_ref() {
                hal.cancel();
            }
            daemon.fod.reset("cancel");
            crate::oplus_fingerprint::on_cancelled();
        }
        Job::ClientDied(state) => {
            state.status.store(SESSION_CLOSED, Ordering::SeqCst);
            daemon.fod.reset("client died");
            crate::oplus_fingerprint::on_cancelled();
            let mut shared = daemon.shared();
            if shared.current_session_id == state.id {
                shared.callback = None;
                shared.global_status = GS_IDLE;
                shared.failed_attempts = 0;
                shared.remaining_attempts = 5;
                shared.lockout = LockoutState::None;
                shared.lockout_generation += 1;
            }
        }
        Job::AddFailedAttempts => add_failed_attempts(daemon),
        Job::ClearFailedAttempts => clear_failed_attempts(daemon),
    }
}

fn dispatch_notify(daemon: &Arc<Daemon>, msg: &FingerprintMsg) {
    match msg.typ {
        1 => {
            let raw = u32_at(&msg.data, 0);
            daemon.fod.on_acquired(raw);
            if raw >= 1000 {
                let vendor_code = (raw - 1000) as i32;
                if daemon.fod.anti_mistouch_should_report(vendor_code) {
                    daemon.fod.note_mistouch();
                    if let Some(cb) = daemon.callback() {
                        if let Err(e) = cb.onAcquired(AcquiredInfo::VENDOR, vendor_code) {
                            crate::logw!("callback=acquired result=error err={e:?}");
                        }
                    }
                    return;
                }
                if vendor_code == 40 {
                    daemon.fod.handle_ui_ready_timeout();
                    return;
                }
            }
            let (info, vendor_code) = match raw {
                0 => (AcquiredInfo::GOOD, 0),
                1 => (AcquiredInfo::PARTIAL, 0),
                2 => (AcquiredInfo::INSUFFICIENT, 0),
                3 => (AcquiredInfo::SENSOR_DIRTY, 0),
                4 => (AcquiredInfo::TOO_SLOW, 0),
                5 => (AcquiredInfo::TOO_FAST, 0),
                raw if raw >= 1000 => (AcquiredInfo::VENDOR, (raw - 1000) as i32),
                _ => (AcquiredInfo::INSUFFICIENT, 0),
            };
            if let Some(cb) = daemon.callback() {
                if let Err(e) = cb.onAcquired(info, vendor_code) {
                    crate::logw!("callback=acquired result=error err={e:?}");
                }
            }
        }
        3 => {
            if daemon.global_status() == GS_IDLE {
                crate::logd!("notify=enroll result=ignored reason=idle");
                return;
            }
            let enrollment_id = u32_at(&msg.data, 0) as i32;
            let remaining = u32_at(&msg.data, 4) as i32;
            daemon.fod.on_enroll_progress();
            if let Some(cb) = daemon.callback() {
                if let Err(e) = cb.onEnrollmentProgress(enrollment_id, remaining) {
                    crate::logw!("callback=enroll-progress result=error err={e:?}");
                }
            }
            if remaining == 0 {
                daemon.set_global_status(GS_IDLE);
            }
        }
        4 => {
            let ids = collect_ids(&msg.data);
            if let Some(cb) = daemon.callback() {
                if let Err(e) = cb.onEnrollmentsRemoved(&ids) {
                    crate::logw!("callback=enrollments-removed result=error err={e:?}");
                }
            }
            daemon.set_global_status(GS_IDLE);
        }
        5 => {
            if daemon.global_status() == GS_IDLE {
                crate::logd!("notify=auth result=ignored reason=idle");
                return;
            }
            let enrollment_id = u32_at(&msg.data, 0) as i32;
            let mistouch_reported = daemon.fod.take_mistouch_reported();
            if enrollment_id == 0 {
                daemon.fod.on_auth_failed();
                if !mistouch_reported {
                    if let Some(cb) = daemon.callback() {
                        if let Err(e) = cb.onAuthenticationFailed() {
                            crate::logw!("callback=auth-failed result=error err={e:?}");
                        }
                    }
                    if daemon.sensor_type & 0xfe == 2 {
                        add_failed_attempts(daemon);
                    }
                    crate::touchfeature::set_auth_state(1);
                }
            } else {
                daemon.fod.reset_anti_mistouch_count();
                let token = parse_legacy_token(&msg.data[4..4 + TOKEN_LEN]);
                if let Some(cb) = daemon.callback() {
                    if let Err(e) = cb.onAuthenticationSucceeded(enrollment_id, &token) {
                        crate::logw!("callback=auth-pass result=error err={e:?}");
                    }
                }
                reset_lockout_state(daemon, false);
                crate::touchfeature::set_auth_state(0);
                crate::oplus_fingerprint::on_auth_success();
                daemon.set_global_status(GS_IDLE);
            }
        }
        6 => {
            let ids = collect_ids(&msg.data);
            if let Some(cb) = daemon.callback() {
                if let Err(e) = cb.onEnrollmentsEnumerated(&ids) {
                    crate::logw!("callback=enrollments-list result=error err={e:?}");
                }
            }
            daemon.set_global_status(GS_IDLE);
        }
        7 => {
            if let Some(cb) = daemon.callback() {
                if let Err(e) = cb.onChallengeGenerated(u64_at(&msg.data, 0) as i64) {
                    crate::logw!("callback=challenge-gen result=error err={e:?}");
                }
            }
        }
        8 => {
            if let Some(cb) = daemon.callback() {
                if let Err(e) = cb.onChallengeRevoked(u64_at(&msg.data, 0) as i64) {
                    crate::logw!("callback=challenge-revoke result=error err={e:?}");
                }
            }
        }
        9 => {
            if let Some(cb) = daemon.callback() {
                if let Err(e) = cb.onAuthenticatorIdRetrieved(u64_at(&msg.data, 0) as i64) {
                    crate::logw!("callback=auth-id-get result=error err={e:?}");
                }
            }
        }
        10 => {
            if let Some(cb) = daemon.callback() {
                if let Err(e) = cb.onAuthenticatorIdInvalidated(u64_at(&msg.data, 0) as i64) {
                    crate::logw!("callback=auth-id-invalidate result=error err={e:?}");
                }
            }
        }
        11 => {
            reset_lockout_state(daemon, true);
        }
        -1 => {
            daemon.fod.reset("hal error");
            crate::oplus_fingerprint::on_cancelled();
            let raw = u32_at(&msg.data, 0);
            let (error, vendor_code) = match raw {
                1 => (Error::HW_UNAVAILABLE, 0),
                2 => (Error::UNABLE_TO_PROCESS, 0),
                3 => (Error::TIMEOUT, 0),
                4 => (Error::NO_SPACE, 0),
                5 => (Error::CANCELED, 0),
                6 => (Error::UNABLE_TO_REMOVE, 0),
                raw if raw >= 1000 => (Error::VENDOR, (raw - 1000) as i32),
                _ => {
                    crate::logw!("hal-error code={raw} result=unknown");
                    (Error::UNABLE_TO_PROCESS, 0)
                }
            };
            if error == Error::CANCELED {
                daemon.set_global_status(GS_IDLE);
            }
            if let Some(cb) = daemon.callback() {
                if let Err(e) = cb.onError(error, vendor_code) {
                    crate::logw!("callback=error result=error err={e:?}");
                }
            }
        }
        other => {
            crate::logw!("notify type={other} result=unknown");
        }
    }
}

fn add_failed_attempts(daemon: &Arc<Daemon>) {
    enum Action {
        None,
        Timed(u64),
        Permanent,
    }

    let action = {
        let mut shared = daemon.shared();
        shared.failed_attempts += 1;
        shared.remaining_attempts = (5 - shared.failed_attempts).max(0);
        if shared.lockout == LockoutState::Permanent {
            Action::None
        } else if shared.failed_attempts >= 20 {
            shared.lockout = LockoutState::Permanent;
            shared.lockout_generation += 1;
            Action::Permanent
        } else if shared.failed_attempts >= 5 && shared.lockout == LockoutState::None {
            shared.lockout = LockoutState::Timed;
            shared.lockout_generation += 1;
            Action::Timed(shared.lockout_generation)
        } else {
            Action::None
        }
    };

    match action {
        Action::None => {}
        Action::Timed(generation) => {
            if let Some(cb) = daemon.callback() {
                if let Err(e) = cb.onLockoutTimed(30_000) {
                    crate::logw!("callback=lockout-timed result=error err={e:?}");
                }
            }
            let daemon = Arc::clone(daemon);
            thread::Builder::new()
                .name("lockout-timer".into())
                .spawn(move || {
                    thread::sleep(Duration::from_secs(30));
                    let expired = {
                        let mut shared = daemon.shared();
                        if shared.lockout == LockoutState::Timed
                            && shared.lockout_generation == generation
                        {
                            shared.failed_attempts = 0;
                            shared.remaining_attempts = 5;
                            shared.lockout = LockoutState::None;
                            shared.lockout_generation += 1;
                            true
                        } else {
                            false
                        }
                    };
                    if expired {
                        if let Some(cb) = daemon.callback() {
                            if let Err(e) = cb.onLockoutCleared() {
                                crate::logw!("callback=lockout-cleared result=error err={e:?}");
                            }
                        }
                    }
                })
                .ok();
        }
        Action::Permanent => {
            if let Some(cb) = daemon.callback() {
                if let Err(e) = cb.onLockoutPermanent() {
                    crate::logw!("callback=lockout-permanent result=error err={e:?}");
                }
            }
            if let Some(hal) = daemon.hal.as_ref() {
                hal.cancel();
            }
        }
    }
}

fn clear_failed_attempts(daemon: &Daemon) {
    reset_lockout_state(daemon, false);
}

fn reset_lockout_state(daemon: &Daemon, notify: bool) {
    {
        let mut shared = daemon.shared();
        shared.failed_attempts = 0;
        shared.remaining_attempts = 5;
        shared.lockout = LockoutState::None;
        shared.lockout_generation += 1;
    }
    if notify {
        if let Some(cb) = daemon.callback() {
            if let Err(e) = cb.onLockoutCleared() {
                crate::logw!("callback=lockout-cleared result=error err={e:?}");
            }
        }
    }
}

fn collect_ids(data: &[u8; 80]) -> Vec<i32> {
    let mut ids = Vec::new();
    for offset in (0..20).step_by(4) {
        let id = u32_at(data, offset);
        if id != 0 {
            ids.push(id as i32);
        }
    }
    ids
}

fn u32_at(data: &[u8; 80], offset: usize) -> u32 {
    u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap())
}

fn u64_at(data: &[u8; 80], offset: usize) -> u64 {
    u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap())
}

fn parse_legacy_token(token: &[u8]) -> HardwareAuthToken {
    let challenge = u64::from_le_bytes(token[1..9].try_into().unwrap()) as i64;
    let user_id = u64::from_le_bytes(token[9..17].try_into().unwrap()) as i64;
    let authenticator_id = u64::from_le_bytes(token[17..25].try_into().unwrap()) as i64;
    let authenticator_type = u32::from_be_bytes(token[25..29].try_into().unwrap()) as i32;
    let timestamp = u64::from_be_bytes(token[29..37].try_into().unwrap()) as i64;
    HardwareAuthToken {
        challenge,
        userId: user_id,
        authenticatorId: authenticator_id,
        authenticatorType: HardwareAuthenticatorType(authenticator_type),
        timestamp: Timestamp {
            milliSeconds: timestamp,
        },
        mac: token[37..69].to_vec(),
    }
}

fn legacy_token(token: &HardwareAuthToken) -> [u8; TOKEN_LEN] {
    let mut out = [0u8; TOKEN_LEN];
    out[0] = 0;
    out[1..9].copy_from_slice(&(token.challenge as u64).to_le_bytes());
    out[9..17].copy_from_slice(&(token.userId as u64).to_le_bytes());
    out[17..25].copy_from_slice(&(token.authenticatorId as u64).to_le_bytes());
    out[25..29].copy_from_slice(&(token.authenticatorType.0 as u32).to_be_bytes());
    out[29..37].copy_from_slice(&(token.timestamp.milliSeconds as u64).to_be_bytes());
    let mac_len = token.mac.len().min(32);
    out[37..37 + mac_len].copy_from_slice(&token.mac[..mac_len]);
    out
}

#[allow(deprecated)]
fn sensor_props() -> SensorProps {
    let location = hal::parse_pair(
        hal::get_prop("persist.vendor.sys.fp.fod.location.X_Y"),
        (505, 2333),
    );
    let size = hal::parse_pair(
        hal::get_prop("persist.vendor.sys.fp.fod.size.width_height"),
        (210, 210),
    );
    let radius = (size.0 / 2).max(1);
    let sensor_type = FingerprintSensorType(hal::sensor_type_from_props() as i8);

    SensorProps {
        commonProps: CommonProps {
            sensorId: 5,
            sensorStrength: SensorStrength::STRONG,
            maxEnrollmentsPerUser: 5,
            componentInfo: vec![
                ComponentInfo {
                    componentId: "fingerprintSensor".into(),
                    hardwareVersion: "vendor/model/revision".into(),
                    firmwareVersion: "1.01".into(),
                    serialNumber: "00000001".into(),
                    softwareVersion: String::new(),
                },
                ComponentInfo {
                    componentId: "matchingAlgorithm".into(),
                    hardwareVersion: String::new(),
                    firmwareVersion: String::new(),
                    serialNumber: String::new(),
                    softwareVersion: "vendor/version/revision".into(),
                },
            ],
        },
        sensorType: sensor_type,
        sensorLocations: vec![SensorLocation {
            displayId: 0,
            sensorLocationX: location.0 + radius,
            sensorLocationY: location.1 + size.1 / 2,
            sensorRadius: radius,
            display: String::new(),
            sensorShape: SensorShape::CIRCLE,
        }],
        supportsNavigationGestures: false,
        supportsDetectInteraction: false,
        halHandlesDisplayTouches: true,
        halControlsIllumination: false,
        touchDetectionParameters: None,
    }
}

pub struct Fingerprint {
    daemon: Arc<Daemon>,
}

impl Interface for Fingerprint {}

impl Fingerprint {
    pub fn new(daemon: Arc<Daemon>) -> Self {
        Self { daemon }
    }
}

impl IFingerprintTrait for Fingerprint {
    fn r#getSensorProps(&self) -> binder::Result<Vec<SensorProps>> {
        Ok(vec![sensor_props()])
    }

    fn r#createSession(
        &self,
        sensor_id: i32,
        user_id: i32,
        callback: &Strong<dyn Cb>,
    ) -> binder::Result<Strong<dyn SessionTrait>> {
        if sensor_id < 0 || user_id < 0 {
            crate::logw!("session=create result=invalid sensor={sensor_id} user={user_id}");
        }

        crate::logi!("session=create sensor={sensor_id} user={user_id}");
        let path = format!("/data/vendor_de/{user_id}/fpdata");
        let id = self.daemon.next_session_id();
        let state = Arc::new(SessionState::new(id, callback.clone()));

        let mut recipient = DeathRecipient::new({
            let daemon = Arc::clone(&self.daemon);
            let state = Arc::clone(&state);
            move || {
                state.status.store(SESSION_CLOSED, Ordering::SeqCst);
                let _ = daemon.tx.send(Job::ClientDied(Arc::clone(&state)));
            }
        });
        let mut callback_binder = callback.as_binder();
        if callback_binder.link_to_death(&mut recipient).is_ok() {
            *state.death.lock().unwrap_or_else(|e| e.into_inner()) = Some(recipient);
        }

        {
            let mut shared = self.daemon.shared();
            shared.callback = Some(callback.clone());
            shared.current_session_id = id;
        }

        if let Some(hal) = self.daemon.hal.as_ref() {
            match CString::new(path.clone()) {
                Ok(path) => {
                    let ret = hal.set_active_group(user_id as u32, &path);
                    if ret != 0 {
                        crate::logw!("group=set user={user_id} path={path:?} rc={ret}");
                    }
                }
                Err(_) => crate::logw!("group=set result=invalid-path path={path:?}"),
            }
        }

        Ok(BnSession::new_binder(
            Session {
                daemon: Arc::clone(&self.daemon),
                state,
            },
            BinderFeatures::default(),
        ))
    }
}

struct Session {
    daemon: Arc<Daemon>,
    state: Arc<SessionState>,
}

impl Interface for Session {}

impl Session {
    fn cancellation_signal(&self) -> Strong<dyn CancellationTrait> {
        BnCancellationSignal::new_binder(
            CancellationSignal {
                daemon: Arc::clone(&self.daemon),
            },
            BinderFeatures::default(),
        )
    }

    fn send(&self, job: Job) {
        if let Err(e) = self.daemon.tx.send(job) {
            crate::logw!("job=session result=queue-error err={e}");
        }
    }

    fn reset_status_unless_closed(&self) {
        if !self.state.is_closed() {
            self.state.status.store(SESSION_IDLE, Ordering::SeqCst);
        }
    }
}

impl SessionTrait for Session {
    fn r#generateChallenge(&self) -> binder::Result<()> {
        if begin_request(&self.daemon, &self.state, SESSION_GENERATE_CHALLENGE, None) {
            self.send(Job::GenerateChallenge(Arc::clone(&self.state)));
        }
        Ok(())
    }

    fn r#revokeChallenge(&self, challenge: i64) -> binder::Result<()> {
        if begin_request(&self.daemon, &self.state, SESSION_REVOKE_CHALLENGE, None) {
            self.send(Job::RevokeChallenge(
                Arc::clone(&self.state),
                challenge as u64,
            ));
        }
        Ok(())
    }

    fn r#enroll(&self, hat: &HardwareAuthToken) -> binder::Result<Strong<dyn CancellationTrait>> {
        if begin_request(&self.daemon, &self.state, SESSION_ENROLL, Some(GS_ENROLL)) {
            self.send(Job::Enroll(Arc::clone(&self.state), legacy_token(hat)));
        }
        Ok(self.cancellation_signal())
    }

    fn r#authenticate(&self, operation_id: i64) -> binder::Result<Strong<dyn CancellationTrait>> {
        if begin_request(
            &self.daemon,
            &self.state,
            SESSION_AUTHENTICATE,
            Some(GS_AUTHENTICATE),
        ) {
            self.send(Job::Authenticate(
                Arc::clone(&self.state),
                operation_id as u64,
            ));
        }
        Ok(self.cancellation_signal())
    }

    fn r#detectInteraction(&self) -> binder::Result<Strong<dyn CancellationTrait>> {
        if begin_request(&self.daemon, &self.state, SESSION_DETECT_INTERACTION, None) {
            self.state.finish();
        }
        Ok(self.cancellation_signal())
    }

    fn r#enumerateEnrollments(&self) -> binder::Result<()> {
        if begin_request(
            &self.daemon,
            &self.state,
            SESSION_ENUMERATE,
            Some(GS_ENUMERATE),
        ) {
            self.send(Job::Enumerate(Arc::clone(&self.state)));
        }
        Ok(())
    }

    fn r#removeEnrollments(&self, enrollment_ids: &[i32]) -> binder::Result<()> {
        if begin_request(&self.daemon, &self.state, SESSION_REMOVE, Some(GS_REMOVE)) {
            let ids: Vec<u32> = enrollment_ids.iter().take(5).map(|&id| id as u32).collect();
            self.send(Job::Remove(Arc::clone(&self.state), ids));
        }
        Ok(())
    }

    fn r#getAuthenticatorId(&self) -> binder::Result<()> {
        if begin_request(
            &self.daemon,
            &self.state,
            SESSION_GET_AUTHENTICATOR_ID,
            None,
        ) {
            self.send(Job::GetAuthenticatorId(Arc::clone(&self.state)));
        }
        Ok(())
    }

    fn r#invalidateAuthenticatorId(&self) -> binder::Result<()> {
        if begin_request(
            &self.daemon,
            &self.state,
            SESSION_INVALIDATE_AUTHENTICATOR_ID,
            None,
        ) {
            self.send(Job::InvalidateAuthenticatorId(Arc::clone(&self.state)));
        }
        Ok(())
    }

    fn r#resetLockout(&self, hat: &HardwareAuthToken) -> binder::Result<()> {
        if begin_request(&self.daemon, &self.state, SESSION_RESET_LOCKOUT, None) {
            self.send(Job::ResetLockout(
                Arc::clone(&self.state),
                legacy_token(hat),
            ));
        }
        Ok(())
    }

    fn r#close(&self) -> binder::Result<()> {
        self.state.status.store(SESSION_CLOSED, Ordering::SeqCst);
        self.daemon.fod.reset("session close");
        crate::oplus_fingerprint::on_session_close();
        let callback = self
            .state
            .callback
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone();
        if let Some(cb) = callback {
            if let Err(e) = cb.onSessionClosed() {
                crate::logw!("callback=closed result=error err={e:?}");
            }
        }
        let _ = self
            .state
            .death
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .take();
        Ok(())
    }

    #[allow(deprecated)]
    fn r#onPointerDown(
        &self,
        pointer_id: i32,
        x: i32,
        y: i32,
        minor: f32,
        major: f32,
    ) -> binder::Result<()> {
        crate::logi!("pointer=down id={pointer_id} x={x} y={y} minor={minor} major={major}");
        self.reset_status_unless_closed();
        Ok(())
    }

    #[allow(deprecated)]
    fn r#onPointerUp(&self, pointer_id: i32) -> binder::Result<()> {
        crate::logi!("pointer=up id={pointer_id}");
        self.reset_status_unless_closed();
        Ok(())
    }

    fn r#onUiReady(&self) -> binder::Result<()> {
        crate::logi!("ui=ready");
        self.reset_status_unless_closed();
        Ok(())
    }

    fn r#authenticateWithContext(
        &self,
        operation_id: i64,
        _context: &OperationContext,
    ) -> binder::Result<Strong<dyn CancellationTrait>> {
        self.r#authenticate(operation_id)
    }

    fn r#enrollWithContext(
        &self,
        hat: &HardwareAuthToken,
        _context: &OperationContext,
    ) -> binder::Result<Strong<dyn CancellationTrait>> {
        self.r#enroll(hat)
    }

    fn r#detectInteractionWithContext(
        &self,
        _context: &OperationContext,
    ) -> binder::Result<Strong<dyn CancellationTrait>> {
        self.r#detectInteraction()
    }

    fn r#onPointerDownWithContext(&self, context: &PointerContext) -> binder::Result<()> {
        crate::logi!(
            "pointer=down id={} x={} y={} minor={} major={}",
            context.pointerId,
            context.x,
            context.y,
            context.minor,
            context.major
        );
        self.reset_status_unless_closed();
        Ok(())
    }

    fn r#onPointerUpWithContext(&self, context: &PointerContext) -> binder::Result<()> {
        crate::logi!("pointer=up id={}", context.pointerId);
        self.reset_status_unless_closed();
        Ok(())
    }

    fn r#onContextChanged(&self, _context: &OperationContext) -> binder::Result<()> {
        Ok(())
    }

    fn r#onPointerCancelWithContext(&self, _context: &PointerContext) -> binder::Result<()> {
        Ok(())
    }

    fn r#setIgnoreDisplayTouches(&self, _should_ignore: bool) -> binder::Result<()> {
        Ok(())
    }
}

struct CancellationSignal {
    daemon: Arc<Daemon>,
}

impl Interface for CancellationSignal {}

impl CancellationTrait for CancellationSignal {
    fn r#cancel(&self) -> binder::Result<()> {
        crate::logi!("session=cancel");
        if self.daemon.global_status() == GS_IDLE {
            crate::logw!("session=cancel result=idle");
            if let Some(cb) = self.daemon.callback() {
                if let Err(e) = cb.onError(Error::CANCELED, 0) {
                    crate::logw!("callback=error result=invoke-error err={e:?}");
                }
            }
        } else if let Err(e) = self.daemon.tx.send(Job::Cancel) {
            crate::logw!("job=cancel result=queue-error err={e}");
        }
        Ok(())
    }
}

pub struct XiaomiFingerprint {
    daemon: Arc<Daemon>,
}

impl Interface for XiaomiFingerprint {}

impl XiaomiFingerprint {
    pub fn new(daemon: Arc<Daemon>) -> Self {
        Self { daemon }
    }
}

impl XiaomiTrait for XiaomiFingerprint {
    fn r#extCmd(&self, cmd_id: i32, cmd_param: i32) -> binder::Result<i32> {
        let cmd = cmd_id as u32;
        let param = cmd_param as u32;
        let ret = match cmd {
            1..=8 => {
                self.daemon.fod.on_ext_cmd(cmd, param);
                0
            }
            10 => {
                if self.daemon.fod.is_enabled() {
                    crate::logd!("ui-ready=skip reason=fod-engine");
                    0
                } else {
                    self.daemon
                        .hal
                        .as_ref()
                        .map(|hal| hal.ext_cmd(cmd, param))
                        .unwrap_or(0)
                }
            }
            12 => self
                .daemon
                .hal
                .as_ref()
                .map(|hal| hal.ext_cmd(cmd, param))
                .unwrap_or(0),
            11 => {
                self.daemon.fod.cloud_update_ocp_control(param);
                0
            }
            13 => {
                let _ = self.daemon.tx.send(Job::ClearFailedAttempts);
                0
            }
            14 => {
                self.daemon.fod.cloud_update_anti_mistouch(param);
                0
            }
            15 => {
                crate::logd!("cfg=low-brightness-auth");
                0
            }
            17 => {
                let _ = self.daemon.tx.send(Job::AddFailedAttempts);
                0
            }
            _ => {
                crate::logw!("ext-cmd id={cmd} result=unknown");
                0
            }
        };
        Ok(ret)
    }
}

pub type BnFingerprint = crate::IFingerprint::BnFingerprint;
pub type BnSession = crate::ISession::BnSession;
pub type BnCancellationSignal = crate::ICancellationSignal::BnCancellationSignal;
pub type BnXiaomiFingerprint = crate::IXiaomiFingerprint::BnXiaomiFingerprint;
