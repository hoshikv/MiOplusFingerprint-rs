#![forbid(unsafe_code)]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#DisplayState : [i32; 5] {
    r#UNKNOWN = 0,
    r#LOCKSCREEN = 1,
    r#NO_UI = 2,
    r#SCREENSAVER = 3,
    r#AOD = 4,
  }
}
pub(crate) mod mangled {
 pub use super::r#DisplayState as _7_android_8_hardware_10_biometrics_6_common_12_DisplayState;
}
