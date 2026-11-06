#![forbid(unsafe_code)]
#[derive(Debug)]
pub struct r#SensorLocation {
  #[deprecated = "use the display field instead. This field was never used."]
  pub r#displayId: i32,
  pub r#sensorLocationX: i32,
  pub r#sensorLocationY: i32,
  pub r#sensorRadius: i32,
  pub r#display: String,
  pub r#sensorShape: crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_11_SensorShape,
}
impl Default for r#SensorLocation {
  fn default() -> Self {
    Self {
      r#displayId: 0,
      r#sensorLocationX: 0,
      r#sensorLocationY: 0,
      r#sensorRadius: 0,
      r#display: "".into(),
      r#sensorShape: crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_11_SensorShape::CIRCLE,
    }
  }
}
impl binder::Parcelable for r#SensorLocation {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#displayId)?;
      subparcel.write(&self.r#sensorLocationX)?;
      subparcel.write(&self.r#sensorLocationY)?;
      subparcel.write(&self.r#sensorRadius)?;
      subparcel.write(&self.r#display)?;
      subparcel.write(&self.r#sensorShape)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#displayId = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#sensorLocationX = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#sensorLocationY = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#sensorRadius = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#display = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#sensorShape = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#SensorLocation);
binder::impl_deserialize_for_parcelable!(r#SensorLocation);
impl binder::binder_impl::ParcelableMetadata for r#SensorLocation {
  fn get_descriptor() -> &'static str { "android.hardware.biometrics.fingerprint.SensorLocation" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#SensorLocation as _7_android_8_hardware_10_biometrics_11_fingerprint_14_SensorLocation;
}
