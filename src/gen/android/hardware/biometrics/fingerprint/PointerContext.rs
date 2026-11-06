#![forbid(unsafe_code)]
#[derive(Debug)]
pub struct r#PointerContext {
  pub r#pointerId: i32,
  pub r#x: f32,
  pub r#y: f32,
  pub r#minor: f32,
  pub r#major: f32,
  pub r#orientation: f32,
  pub r#isAod: bool,
  pub r#time: i64,
  pub r#gestureStart: i64,
}
impl Default for r#PointerContext {
  fn default() -> Self {
    Self {
      r#pointerId: -1,
      r#x: 0.000000f32,
      r#y: 0.000000f32,
      r#minor: 0.000000f32,
      r#major: 0.000000f32,
      r#orientation: 0.000000f32,
      r#isAod: false,
      r#time: 0,
      r#gestureStart: 0,
    }
  }
}
impl binder::Parcelable for r#PointerContext {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#pointerId)?;
      subparcel.write(&self.r#x)?;
      subparcel.write(&self.r#y)?;
      subparcel.write(&self.r#minor)?;
      subparcel.write(&self.r#major)?;
      subparcel.write(&self.r#orientation)?;
      subparcel.write(&self.r#isAod)?;
      subparcel.write(&self.r#time)?;
      subparcel.write(&self.r#gestureStart)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#pointerId = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#x = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#y = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#minor = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#major = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#orientation = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#isAod = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#time = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#gestureStart = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#PointerContext);
binder::impl_deserialize_for_parcelable!(r#PointerContext);
impl binder::binder_impl::ParcelableMetadata for r#PointerContext {
  fn get_descriptor() -> &'static str { "android.hardware.biometrics.fingerprint.PointerContext" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#PointerContext as _7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext;
}
