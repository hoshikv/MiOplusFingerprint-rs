#![forbid(unsafe_code)]
#[derive(Debug)]
pub struct r#SensorProps {
  pub r#commonProps: crate::mangled::_7_android_8_hardware_10_biometrics_6_common_11_CommonProps,
  pub r#sensorType: crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_21_FingerprintSensorType,
  pub r#sensorLocations: Vec<crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_SensorLocation>,
  pub r#supportsNavigationGestures: bool,
  pub r#supportsDetectInteraction: bool,
  pub r#halHandlesDisplayTouches: bool,
  pub r#halControlsIllumination: bool,
  pub r#touchDetectionParameters: Option<crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_24_TouchDetectionParameters>,
}
impl Default for r#SensorProps {
  fn default() -> Self {
    Self {
      r#commonProps: Default::default(),
      r#sensorType: crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_21_FingerprintSensorType::UNKNOWN,
      r#sensorLocations: Default::default(),
      r#supportsNavigationGestures: false,
      r#supportsDetectInteraction: false,
      r#halHandlesDisplayTouches: false,
      r#halControlsIllumination: false,
      r#touchDetectionParameters: Default::default(),
    }
  }
}
impl binder::Parcelable for r#SensorProps {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#commonProps)?;
      subparcel.write(&self.r#sensorType)?;
      subparcel.write(&self.r#sensorLocations)?;
      subparcel.write(&self.r#supportsNavigationGestures)?;
      subparcel.write(&self.r#supportsDetectInteraction)?;
      subparcel.write(&self.r#halHandlesDisplayTouches)?;
      subparcel.write(&self.r#halControlsIllumination)?;
      subparcel.write(&self.r#touchDetectionParameters)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#commonProps = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#sensorType = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#sensorLocations = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#supportsNavigationGestures = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#supportsDetectInteraction = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#halHandlesDisplayTouches = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#halControlsIllumination = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#touchDetectionParameters = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#SensorProps);
binder::impl_deserialize_for_parcelable!(r#SensorProps);
impl binder::binder_impl::ParcelableMetadata for r#SensorProps {
  fn get_descriptor() -> &'static str { "android.hardware.biometrics.fingerprint.SensorProps" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#SensorProps as _7_android_8_hardware_10_biometrics_11_fingerprint_11_SensorProps;
}
