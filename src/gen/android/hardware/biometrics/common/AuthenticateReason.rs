#![forbid(unsafe_code)]
#[derive(Debug)]
pub enum r#AuthenticateReason {
  VendorAuthenticateReason(crate::mangled::_7_android_8_hardware_10_biometrics_6_common_18_AuthenticateReason_6_Vendor),
  FaceAuthenticateReason(crate::mangled::_7_android_8_hardware_10_biometrics_6_common_18_AuthenticateReason_4_Face),
  FingerprintAuthenticateReason(crate::mangled::_7_android_8_hardware_10_biometrics_6_common_18_AuthenticateReason_11_Fingerprint),
}
impl Default for r#AuthenticateReason {
  fn default() -> Self {
    Self::VendorAuthenticateReason(Default::default())
  }
}
impl binder::Parcelable for r#AuthenticateReason {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    match self {
      Self::VendorAuthenticateReason(v) => {
        parcel.write(&0i32)?;
        parcel.write(v)
      }
      Self::FaceAuthenticateReason(v) => {
        parcel.write(&1i32)?;
        parcel.write(v)
      }
      Self::FingerprintAuthenticateReason(v) => {
        parcel.write(&2i32)?;
        parcel.write(v)
      }
    }
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    let tag: i32 = parcel.read()?;
    match tag {
      0 => {
        let value: crate::mangled::_7_android_8_hardware_10_biometrics_6_common_18_AuthenticateReason_6_Vendor = parcel.read()?;
        *self = Self::VendorAuthenticateReason(value);
        Ok(())
      }
      1 => {
        let value: crate::mangled::_7_android_8_hardware_10_biometrics_6_common_18_AuthenticateReason_4_Face = parcel.read()?;
        *self = Self::FaceAuthenticateReason(value);
        Ok(())
      }
      2 => {
        let value: crate::mangled::_7_android_8_hardware_10_biometrics_6_common_18_AuthenticateReason_11_Fingerprint = parcel.read()?;
        *self = Self::FingerprintAuthenticateReason(value);
        Ok(())
      }
      _ => {
        Err(binder::StatusCode::BAD_VALUE)
      }
    }
  }
}
binder::impl_serialize_for_parcelable!(r#AuthenticateReason);
binder::impl_deserialize_for_parcelable!(r#AuthenticateReason);
impl binder::binder_impl::ParcelableMetadata for r#AuthenticateReason {
  fn get_descriptor() -> &'static str { "android.hardware.biometrics.common.AuthenticateReason" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub mod r#Vendor {
  #[derive(Debug)]
  pub struct r#Vendor {
    pub r#extension: binder::ParcelableHolder,
  }
  impl Default for r#Vendor {
    fn default() -> Self {
      Self {
        r#extension: binder::ParcelableHolder::new(binder::binder_impl::Stability::Vintf),
      }
    }
  }
  impl binder::Parcelable for r#Vendor {
    fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
      parcel.sized_write(|subparcel| {
        subparcel.write(&self.r#extension)?;
        Ok(())
      })
    }
    fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
      parcel.sized_read(|subparcel| {
        if subparcel.has_more_data() {
          self.r#extension = subparcel.read()?;
        }
        Ok(())
      })
    }
  }
  binder::impl_serialize_for_parcelable!(r#Vendor);
  binder::impl_deserialize_for_parcelable!(r#Vendor);
  impl binder::binder_impl::ParcelableMetadata for r#Vendor {
    fn get_descriptor() -> &'static str { "android.hardware.biometrics.common.AuthenticateReason.Vendor" }
    fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
  }
}
pub mod r#Fingerprint {
  #![allow(non_upper_case_globals)]
  use binder::declare_binder_enum;
  declare_binder_enum! {
    r#Fingerprint : [i32; 1] {
      r#UNKNOWN = 0,
    }
  }
}
pub mod r#Face {
  #![allow(non_upper_case_globals)]
  use binder::declare_binder_enum;
  declare_binder_enum! {
    r#Face : [i32; 11] {
      r#UNKNOWN = 0,
      r#STARTED_WAKING_UP = 1,
      r#PRIMARY_BOUNCER_SHOWN = 2,
      r#ASSISTANT_VISIBLE = 3,
      r#ALTERNATE_BIOMETRIC_BOUNCER_SHOWN = 4,
      r#NOTIFICATION_PANEL_CLICKED = 5,
      r#OCCLUDING_APP_REQUESTED = 6,
      r#PICK_UP_GESTURE_TRIGGERED = 7,
      r#QS_EXPANDED = 8,
      r#SWIPE_UP_ON_BOUNCER = 9,
      r#UDFPS_POINTER_DOWN = 10,
    }
  }
}
pub mod r#Tag {
  #![allow(non_upper_case_globals)]
  use binder::declare_binder_enum;
  declare_binder_enum! {
    r#Tag : [i32; 3] {
      r#vendorAuthenticateReason = 0,
      r#faceAuthenticateReason = 1,
      r#fingerprintAuthenticateReason = 2,
    }
  }
}
pub(crate) mod mangled {
 pub use super::r#AuthenticateReason as _7_android_8_hardware_10_biometrics_6_common_18_AuthenticateReason;
 pub use super::r#Vendor::r#Vendor as _7_android_8_hardware_10_biometrics_6_common_18_AuthenticateReason_6_Vendor;
 pub use super::r#Fingerprint::r#Fingerprint as _7_android_8_hardware_10_biometrics_6_common_18_AuthenticateReason_11_Fingerprint;
 pub use super::r#Face::r#Face as _7_android_8_hardware_10_biometrics_6_common_18_AuthenticateReason_4_Face;
 pub use super::r#Tag::r#Tag as _7_android_8_hardware_10_biometrics_6_common_18_AuthenticateReason_3_Tag;
}
