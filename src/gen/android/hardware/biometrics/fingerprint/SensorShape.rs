#![forbid(unsafe_code)]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#SensorShape : [i8; 2] {
    r#SQUARE = 0,
    r#CIRCLE = 1,
  }
}
pub(crate) mod mangled {
 pub use super::r#SensorShape as _7_android_8_hardware_10_biometrics_11_fingerprint_11_SensorShape;
}
