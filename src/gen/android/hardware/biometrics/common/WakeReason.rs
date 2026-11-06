#![forbid(unsafe_code)]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#WakeReason : [i32; 10] {
    r#UNKNOWN = 0,
    r#POWER_BUTTON = 1,
    r#GESTURE = 2,
    r#WAKE_KEY = 3,
    r#WAKE_MOTION = 4,
    r#LID = 5,
    r#DISPLAY_GROUP_ADDED = 6,
    r#TAP = 7,
    r#LIFT = 8,
    r#BIOMETRIC = 9,
  }
}
pub(crate) mod mangled {
 pub use super::r#WakeReason as _7_android_8_hardware_10_biometrics_6_common_10_WakeReason;
}
