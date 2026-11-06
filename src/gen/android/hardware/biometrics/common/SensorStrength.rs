#![forbid(unsafe_code)]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#SensorStrength : [i8; 3] {
    r#CONVENIENCE = 0,
    r#WEAK = 1,
    r#STRONG = 2,
  }
}
pub(crate) mod mangled {
 pub use super::r#SensorStrength as _7_android_8_hardware_10_biometrics_6_common_14_SensorStrength;
}
