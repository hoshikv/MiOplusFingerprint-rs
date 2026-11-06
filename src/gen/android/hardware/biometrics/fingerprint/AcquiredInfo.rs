#![forbid(unsafe_code)]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#AcquiredInfo : [i8; 15] {
    r#UNKNOWN = 0,
    r#GOOD = 1,
    r#PARTIAL = 2,
    r#INSUFFICIENT = 3,
    r#SENSOR_DIRTY = 4,
    r#TOO_SLOW = 5,
    r#TOO_FAST = 6,
    r#VENDOR = 7,
    r#START = 8,
    r#TOO_DARK = 9,
    r#TOO_BRIGHT = 10,
    r#IMMOBILE = 11,
    r#RETRYING_CAPTURE = 12,
    r#LIFT_TOO_SOON = 13,
    r#POWER_PRESS = 14,
  }
}
pub(crate) mod mangled {
 pub use super::r#AcquiredInfo as _7_android_8_hardware_10_biometrics_11_fingerprint_12_AcquiredInfo;
}
