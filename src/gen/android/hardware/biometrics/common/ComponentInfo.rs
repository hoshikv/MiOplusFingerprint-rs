#![forbid(unsafe_code)]
#[derive(Debug)]
pub struct r#ComponentInfo {
  pub r#componentId: String,
  pub r#hardwareVersion: String,
  pub r#firmwareVersion: String,
  pub r#serialNumber: String,
  pub r#softwareVersion: String,
}
impl Default for r#ComponentInfo {
  fn default() -> Self {
    Self {
      r#componentId: Default::default(),
      r#hardwareVersion: Default::default(),
      r#firmwareVersion: Default::default(),
      r#serialNumber: Default::default(),
      r#softwareVersion: Default::default(),
    }
  }
}
impl binder::Parcelable for r#ComponentInfo {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#componentId)?;
      subparcel.write(&self.r#hardwareVersion)?;
      subparcel.write(&self.r#firmwareVersion)?;
      subparcel.write(&self.r#serialNumber)?;
      subparcel.write(&self.r#softwareVersion)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#componentId = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#hardwareVersion = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#firmwareVersion = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#serialNumber = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#softwareVersion = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#ComponentInfo);
binder::impl_deserialize_for_parcelable!(r#ComponentInfo);
impl binder::binder_impl::ParcelableMetadata for r#ComponentInfo {
  fn get_descriptor() -> &'static str { "android.hardware.biometrics.common.ComponentInfo" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#ComponentInfo as _7_android_8_hardware_10_biometrics_6_common_13_ComponentInfo;
}
