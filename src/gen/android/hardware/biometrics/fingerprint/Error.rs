#![forbid(unsafe_code)]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#Error : [i8; 10] {
    r#UNKNOWN = 0,
    r#HW_UNAVAILABLE = 1,
    r#UNABLE_TO_PROCESS = 2,
    r#TIMEOUT = 3,
    r#NO_SPACE = 4,
    r#CANCELED = 5,
    r#UNABLE_TO_REMOVE = 6,
    r#VENDOR = 7,
    r#BAD_CALIBRATION = 8,
    r#POWER_PRESS = 9,
  }
}
pub(crate) mod mangled {
 pub use super::r#Error as _7_android_8_hardware_10_biometrics_11_fingerprint_5_Error;
}
