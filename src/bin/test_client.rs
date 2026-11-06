use binder::{BinderFeatures, Interface, Strong};
use mfp_daemon::mangled::{
    _7_android_8_hardware_10_biometrics_11_fingerprint_11_SensorProps as SensorProps,
    _7_android_8_hardware_10_biometrics_11_fingerprint_12_AcquiredInfo as AcquiredInfo,
    _7_android_8_hardware_10_biometrics_11_fingerprint_12_IFingerprint as IFingerprint,
    _7_android_8_hardware_10_biometrics_11_fingerprint_16_ISessionCallback as ISessionCallback,
    _7_android_8_hardware_10_biometrics_11_fingerprint_5_Error as Error,
    _7_android_8_hardware_10_biometrics_11_fingerprint_8_ISession as ISession,
    _7_android_8_hardware_9_keymaster_17_HardwareAuthToken as HardwareAuthToken,
};
use mfp_daemon::service::FINGERPRINT_SERVICE;
use mfp_daemon::{loge, logi};
use mfp_daemon::ISessionCallback::BnSessionCallback;

struct Callback;

impl Interface for Callback {}

impl ISessionCallback for Callback {
    fn r#onChallengeGenerated(&self, challenge: i64) -> binder::Result<()> {
        logi!("test cb onChallengeGenerated({challenge})");
        Ok(())
    }
    fn r#onChallengeRevoked(&self, challenge: i64) -> binder::Result<()> {
        logi!("test cb onChallengeRevoked({challenge})");
        Ok(())
    }
    fn r#onAcquired(&self, info: AcquiredInfo, vendor_code: i32) -> binder::Result<()> {
        logi!("test cb onAcquired({info:?}, {vendor_code})");
        Ok(())
    }
    fn r#onError(&self, error: Error, vendor_code: i32) -> binder::Result<()> {
        logi!("test cb onError({error:?}, {vendor_code})");
        Ok(())
    }
    fn r#onEnrollmentProgress(&self, enrollment_id: i32, remaining: i32) -> binder::Result<()> {
        logi!("test cb onEnrollmentProgress({enrollment_id}, {remaining})");
        Ok(())
    }
    fn r#onAuthenticationSucceeded(
        &self,
        enrollment_id: i32,
        hat: &HardwareAuthToken,
    ) -> binder::Result<()> {
        logi!("test cb onAuthenticationSucceeded({enrollment_id}, {hat:?})");
        Ok(())
    }
    fn r#onAuthenticationFailed(&self) -> binder::Result<()> {
        logi!("test cb onAuthenticationFailed");
        Ok(())
    }
    fn r#onLockoutTimed(&self, duration_millis: i64) -> binder::Result<()> {
        logi!("test cb onLockoutTimed({duration_millis})");
        Ok(())
    }
    fn r#onLockoutPermanent(&self) -> binder::Result<()> {
        logi!("test cb onLockoutPermanent");
        Ok(())
    }
    fn r#onLockoutCleared(&self) -> binder::Result<()> {
        logi!("test cb onLockoutCleared");
        Ok(())
    }
    fn r#onInteractionDetected(&self) -> binder::Result<()> {
        logi!("test cb onInteractionDetected");
        Ok(())
    }
    fn r#onEnrollmentsEnumerated(&self, enrollment_ids: &[i32]) -> binder::Result<()> {
        logi!("test cb onEnrollmentsEnumerated({enrollment_ids:?})");
        Ok(())
    }
    fn r#onEnrollmentsRemoved(&self, enrollment_ids: &[i32]) -> binder::Result<()> {
        logi!("test cb onEnrollmentsRemoved({enrollment_ids:?})");
        Ok(())
    }
    fn r#onAuthenticatorIdRetrieved(&self, authenticator_id: i64) -> binder::Result<()> {
        logi!("test cb onAuthenticatorIdRetrieved({authenticator_id})");
        Ok(())
    }
    fn r#onAuthenticatorIdInvalidated(&self, new_authenticator_id: i64) -> binder::Result<()> {
        logi!("test cb onAuthenticatorIdInvalidated({new_authenticator_id})");
        Ok(())
    }
    fn r#onSessionClosed(&self) -> binder::Result<()> {
        logi!("test cb onSessionClosed");
        Ok(())
    }
}

fn props() -> Vec<SensorProps> {
    let fp: Strong<dyn IFingerprint> =
        binder::get_interface(FINGERPRINT_SERVICE).expect("fingerprint service");
    fp.getSensorProps().expect("getSensorProps")
}

fn main() {
    binder::ProcessState::start_thread_pool();

    let fp: Strong<dyn IFingerprint> =
        binder::get_interface(FINGERPRINT_SERVICE).expect("fingerprint service");
    logi!("test client props: {:?}", props());

    let cb = BnSessionCallback::new_binder(Callback, BinderFeatures::default());
    let session: Strong<dyn ISession> = match fp.createSession(5, 0, &cb) {
        Ok(session) => session,
        Err(e) => {
            loge!("createSession failed: {e:?}");
            return;
        }
    };
    logi!("test client session created");

    if let Err(e) = session.generateChallenge() {
        loge!("generateChallenge failed: {e:?}");
    }
    std::thread::sleep(std::time::Duration::from_millis(200));

    let hat = HardwareAuthToken::default();
    match session.enroll(&hat) {
        Ok(signal) => {
            logi!("test client enroll started");
            std::thread::sleep(std::time::Duration::from_millis(500));
            if let Err(e) = signal.cancel() {
                loge!("cancel failed: {e:?}");
            }
        }
        Err(e) => loge!("enroll failed: {e:?}"),
    }
    std::thread::sleep(std::time::Duration::from_millis(300));

    if let Err(e) = session.enumerateEnrollments() {
        loge!("enumerateEnrollments failed: {e:?}");
    }
    std::thread::sleep(std::time::Duration::from_millis(300));

    if let Err(e) = session.getAuthenticatorId() {
        loge!("getAuthenticatorId failed: {e:?}");
    }
    std::thread::sleep(std::time::Duration::from_millis(300));

    match session.authenticate(42) {
        Ok(signal) => {
            logi!("test client authenticate started");
            std::thread::sleep(std::time::Duration::from_millis(500));
            if let Err(e) = signal.cancel() {
                loge!("authenticate cancel failed: {e:?}");
            }
        }
        Err(e) => loge!("authenticate failed: {e:?}"),
    }
    std::thread::sleep(std::time::Duration::from_millis(300));

    if let Err(e) = session.resetLockout(&hat) {
        loge!("resetLockout failed: {e:?}");
    }
    std::thread::sleep(std::time::Duration::from_millis(200));

    if let Err(e) = session.invalidateAuthenticatorId() {
        loge!("invalidateAuthenticatorId failed: {e:?}");
    }
    std::thread::sleep(std::time::Duration::from_millis(300));

    if let Err(e) = session.removeEnrollments(&[0, 1]) {
        loge!("removeEnrollments failed: {e:?}");
    }
    std::thread::sleep(std::time::Duration::from_millis(300));

    if let Err(e) = session.onPointerDown(1, 610, 2438, 10.0, 10.0) {
        loge!("onPointerDown failed: {e:?}");
    }
    if let Err(e) = session.onPointerUp(1) {
        loge!("onPointerUp failed: {e:?}");
    }
    if let Err(e) = session.onUiReady() {
        loge!("onUiReady failed: {e:?}");
    }
    std::thread::sleep(std::time::Duration::from_millis(200));

    if let Err(e) = session.close() {
        loge!("close failed: {e:?}");
    }
    std::thread::sleep(std::time::Duration::from_millis(300));

    logi!("test client done");
}
