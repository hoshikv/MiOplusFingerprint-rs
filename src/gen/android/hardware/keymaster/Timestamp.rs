#![forbid(unsafe_code)]
#[derive(Debug)]
pub struct r#Timestamp {
  pub r#milliSeconds: i64,
}
impl Default for r#Timestamp {
  fn default() -> Self {
    Self {
      r#milliSeconds: 0,
    }
  }
}
impl binder::Parcelable for r#Timestamp {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#milliSeconds)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#milliSeconds = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#Timestamp);
binder::impl_deserialize_for_parcelable!(r#Timestamp);
impl binder::binder_impl::ParcelableMetadata for r#Timestamp {
  fn get_descriptor() -> &'static str { "android.hardware.keymaster.Timestamp" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#Timestamp as _7_android_8_hardware_9_keymaster_9_Timestamp;
}
