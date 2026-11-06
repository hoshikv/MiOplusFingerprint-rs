#![forbid(unsafe_code)]
#[derive(Debug)]
pub struct r#TouchDetectionParameters {
  pub r#targetSize: f32,
  pub r#minOverlap: f32,
}
impl Default for r#TouchDetectionParameters {
  fn default() -> Self {
    Self {
      r#targetSize: 1.000000f32,
      r#minOverlap: 0.000000f32,
    }
  }
}
impl binder::Parcelable for r#TouchDetectionParameters {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#targetSize)?;
      subparcel.write(&self.r#minOverlap)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#targetSize = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#minOverlap = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#TouchDetectionParameters);
binder::impl_deserialize_for_parcelable!(r#TouchDetectionParameters);
impl binder::binder_impl::ParcelableMetadata for r#TouchDetectionParameters {
  fn get_descriptor() -> &'static str { "android.hardware.biometrics.fingerprint.TouchDetectionParameters" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#TouchDetectionParameters as _7_android_8_hardware_10_biometrics_11_fingerprint_24_TouchDetectionParameters;
}
