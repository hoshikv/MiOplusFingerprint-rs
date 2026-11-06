#![forbid(unsafe_code)]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#SecurityLevel : [i32; 3] {
    r#SOFTWARE = 0,
    r#TRUSTED_ENVIRONMENT = 1,
    r#STRONGBOX = 2,
  }
}
pub(crate) mod mangled {
 pub use super::r#SecurityLevel as _7_android_8_hardware_9_keymaster_13_SecurityLevel;
}
