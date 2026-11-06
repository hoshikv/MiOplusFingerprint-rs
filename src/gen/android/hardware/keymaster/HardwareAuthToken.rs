#![forbid(unsafe_code)]
#[derive(Debug)]
pub struct r#HardwareAuthToken {
  pub r#challenge: i64,
  pub r#userId: i64,
  pub r#authenticatorId: i64,
  pub r#authenticatorType: crate::mangled::_7_android_8_hardware_9_keymaster_25_HardwareAuthenticatorType,
  pub r#timestamp: crate::mangled::_7_android_8_hardware_9_keymaster_9_Timestamp,
  pub r#mac: Vec<u8>,
}
impl Default for r#HardwareAuthToken {
  fn default() -> Self {
    Self {
      r#challenge: 0,
      r#userId: 0,
      r#authenticatorId: 0,
      r#authenticatorType: crate::mangled::_7_android_8_hardware_9_keymaster_25_HardwareAuthenticatorType::NONE,
      r#timestamp: Default::default(),
      r#mac: Default::default(),
    }
  }
}
impl binder::Parcelable for r#HardwareAuthToken {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#challenge)?;
      subparcel.write(&self.r#userId)?;
      subparcel.write(&self.r#authenticatorId)?;
      subparcel.write(&self.r#authenticatorType)?;
      subparcel.write(&self.r#timestamp)?;
      subparcel.write(&self.r#mac)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#challenge = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#userId = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#authenticatorId = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#authenticatorType = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#timestamp = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#mac = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#HardwareAuthToken);
binder::impl_deserialize_for_parcelable!(r#HardwareAuthToken);
impl binder::binder_impl::ParcelableMetadata for r#HardwareAuthToken {
  fn get_descriptor() -> &'static str { "android.hardware.keymaster.HardwareAuthToken" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#HardwareAuthToken as _7_android_8_hardware_9_keymaster_17_HardwareAuthToken;
}
