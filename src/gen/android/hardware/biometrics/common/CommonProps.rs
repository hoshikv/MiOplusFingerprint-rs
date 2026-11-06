#![forbid(unsafe_code)]
#[derive(Debug)]
pub struct r#CommonProps {
  pub r#sensorId: i32,
  pub r#sensorStrength: crate::mangled::_7_android_8_hardware_10_biometrics_6_common_14_SensorStrength,
  pub r#maxEnrollmentsPerUser: i32,
  pub r#componentInfo: Vec<crate::mangled::_7_android_8_hardware_10_biometrics_6_common_13_ComponentInfo>,
}
impl Default for r#CommonProps {
  fn default() -> Self {
    Self {
      r#sensorId: 0,
      r#sensorStrength: crate::mangled::_7_android_8_hardware_10_biometrics_6_common_14_SensorStrength::CONVENIENCE,
      r#maxEnrollmentsPerUser: 0,
      r#componentInfo: Default::default(),
    }
  }
}
impl binder::Parcelable for r#CommonProps {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#sensorId)?;
      subparcel.write(&self.r#sensorStrength)?;
      subparcel.write(&self.r#maxEnrollmentsPerUser)?;
      subparcel.write(&self.r#componentInfo)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#sensorId = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#sensorStrength = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#maxEnrollmentsPerUser = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#componentInfo = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#CommonProps);
binder::impl_deserialize_for_parcelable!(r#CommonProps);
impl binder::binder_impl::ParcelableMetadata for r#CommonProps {
  fn get_descriptor() -> &'static str { "android.hardware.biometrics.common.CommonProps" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#CommonProps as _7_android_8_hardware_10_biometrics_6_common_11_CommonProps;
}
