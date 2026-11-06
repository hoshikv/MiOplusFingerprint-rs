#![forbid(unsafe_code)]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#HardwareAuthenticatorType : [i32; 4] {
    r#NONE = 0,
    r#PASSWORD = 1,
    r#FINGERPRINT = 2,
    r#ANY = -1,
  }
}
pub(crate) mod mangled {
 pub use super::r#HardwareAuthenticatorType as _7_android_8_hardware_9_keymaster_25_HardwareAuthenticatorType;
}
