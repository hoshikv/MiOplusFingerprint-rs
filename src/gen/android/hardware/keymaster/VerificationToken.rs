#![forbid(unsafe_code)]
#[derive(Debug)]
pub struct r#VerificationToken {
  pub r#challenge: i64,
  pub r#timestamp: crate::mangled::_7_android_8_hardware_9_keymaster_9_Timestamp,
  pub r#securityLevel: crate::mangled::_7_android_8_hardware_9_keymaster_13_SecurityLevel,
  pub r#mac: Vec<u8>,
}
impl Default for r#VerificationToken {
  fn default() -> Self {
    Self {
      r#challenge: 0,
      r#timestamp: Default::default(),
      r#securityLevel: crate::mangled::_7_android_8_hardware_9_keymaster_13_SecurityLevel::SOFTWARE,
      r#mac: Default::default(),
    }
  }
}
impl binder::Parcelable for r#VerificationToken {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#challenge)?;
      subparcel.write(&self.r#timestamp)?;
      subparcel.write(&self.r#securityLevel)?;
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
        self.r#timestamp = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#securityLevel = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#mac = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#VerificationToken);
binder::impl_deserialize_for_parcelable!(r#VerificationToken);
impl binder::binder_impl::ParcelableMetadata for r#VerificationToken {
  fn get_descriptor() -> &'static str { "android.hardware.keymaster.VerificationToken" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#VerificationToken as _7_android_8_hardware_9_keymaster_17_VerificationToken;
}
