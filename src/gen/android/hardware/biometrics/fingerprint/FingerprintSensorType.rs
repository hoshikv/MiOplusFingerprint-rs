#![forbid(unsafe_code)]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#FingerprintSensorType : [i8; 6] {
    r#UNKNOWN = 0,
    r#REAR = 1,
    r#UNDER_DISPLAY_ULTRASONIC = 2,
    r#UNDER_DISPLAY_OPTICAL = 3,
    r#POWER_BUTTON = 4,
    r#HOME_BUTTON = 5,
  }
}
pub(crate) mod mangled {
 pub use super::r#FingerprintSensorType as _7_android_8_hardware_10_biometrics_11_fingerprint_21_FingerprintSensorType;
}
