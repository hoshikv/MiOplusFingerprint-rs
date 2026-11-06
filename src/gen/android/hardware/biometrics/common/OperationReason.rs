#![forbid(unsafe_code)]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#OperationReason : [i8; 3] {
    r#UNKNOWN = 0,
    r#BIOMETRIC_PROMPT = 1,
    r#KEYGUARD = 2,
  }
}
pub(crate) mod mangled {
 pub use super::r#OperationReason as _7_android_8_hardware_10_biometrics_6_common_15_OperationReason;
}
