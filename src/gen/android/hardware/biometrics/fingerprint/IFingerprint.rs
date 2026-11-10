#![forbid(unsafe_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
use binder::declare_binder_interface;
declare_binder_interface! {
  IFingerprint["android.hardware.biometrics.fingerprint.IFingerprint"] {
    native: BnFingerprint(on_transact),
    proxy: BpFingerprint {
      cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
      cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
    },
    async: IFingerprintAsync,
    stability: binder::binder_impl::Stability::Vintf,
  }
}
pub trait IFingerprint: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.biometrics.fingerprint.IFingerprint" }
  fn r#getSensorProps(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_11_SensorProps>>;
  fn r#createSession(&self, _arg_sensorId: i32, _arg_userId: i32, _arg_cb: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_16_ISessionCallback>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_8_ISession>>;
  fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
    Ok(VERSION)
  }
  fn r#getInterfaceHash(&self) -> binder::Result<String> {
    Ok(HASH.into())
  }
  fn getDefaultImpl() -> IFingerprintDefaultRef where Self: Sized {
    DEFAULT_IMPL.lock().unwrap().clone()
  }
  fn setDefaultImpl(d: IFingerprintDefaultRef) -> IFingerprintDefaultRef where Self: Sized {
    std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
  }
}
pub trait IFingerprintAsync<P>: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.biometrics.fingerprint.IFingerprint" }
  fn r#getSensorProps<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_11_SensorProps>>>;
  fn r#createSession<'a>(&'a self, _arg_sensorId: i32, _arg_userId: i32, _arg_cb: &'a binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_16_ISessionCallback>) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_8_ISession>>>;
  fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
    Box::pin(async move { Ok(VERSION) })
  }
  fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
    Box::pin(async move { Ok(HASH.into()) })
  }
}
#[::async_trait::async_trait]
pub trait IFingerprintAsyncServer: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.biometrics.fingerprint.IFingerprint" }
  async fn r#getSensorProps(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_11_SensorProps>>;
  async fn r#createSession(&self, _arg_sensorId: i32, _arg_userId: i32, _arg_cb: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_16_ISessionCallback>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_8_ISession>>;
}
impl BnFingerprint {
  /// Create a new async binder service.
  pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IFingerprint>
  where
    T: IFingerprintAsyncServer + binder::Interface + Send + Sync + 'static,
    R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
  {
    struct Wrapper<T, R> {
      _inner: T,
      _rt: R,
    }
    impl<T, R> binder::Interface for Wrapper<T, R> where T: binder::Interface, R: Send + Sync {
      fn as_binder(&self) -> binder::SpIBinder { self._inner.as_binder() }
      fn dump(&self, _file: &std::fs::File, _args: &[&std::ffi::CStr]) -> std::result::Result<(), binder::StatusCode> { self._inner.dump(_file, _args) }
    }
    impl<T, R> IFingerprint for Wrapper<T, R>
    where
      T: IFingerprintAsyncServer + Send + Sync + 'static,
      R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
    {
      fn r#getSensorProps(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_11_SensorProps>> {
        self._rt.block_on(self._inner.r#getSensorProps())
      }
      fn r#createSession(&self, _arg_sensorId: i32, _arg_userId: i32, _arg_cb: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_16_ISessionCallback>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_8_ISession>> {
        self._rt.block_on(self._inner.r#createSession(_arg_sensorId, _arg_userId, _arg_cb))
      }
    }
    let wrapped = Wrapper { _inner: inner, _rt: rt };
    Self::new_binder(wrapped, features)
  }
}
pub trait IFingerprintDefault: Send + Sync {
  fn r#getSensorProps(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_11_SensorProps>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#createSession(&self, _arg_sensorId: i32, _arg_userId: i32, _arg_cb: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_16_ISessionCallback>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_8_ISession>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
}
pub mod transactions {
  pub const r#getSensorProps: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
  pub const r#createSession: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
  pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
  pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
}
pub type IFingerprintDefaultRef = Option<std::sync::Arc<dyn IFingerprintDefault>>;
use lazy_static::lazy_static;
lazy_static! {
  static ref DEFAULT_IMPL: std::sync::Mutex<IFingerprintDefaultRef> = std::sync::Mutex::new(None);
}
pub const VERSION: i32 = 3;
pub const HASH: &str = "637371b53fb7faf9bd43aa51b72c23852d6e6d96";
impl BpFingerprint {
  fn build_parcel_getSensorProps(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getSensorProps(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_11_SensorProps>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IFingerprint>::getDefaultImpl() {
        return _aidl_default_impl.r#getSensorProps();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_11_SensorProps> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_createSession(&self, _arg_sensorId: i32, _arg_userId: i32, _arg_cb: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_16_ISessionCallback>) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_sensorId)?;
    aidl_data.write(&_arg_userId)?;
    aidl_data.write(_arg_cb)?;
    Ok(aidl_data)
  }
  fn read_response_createSession(&self, _arg_sensorId: i32, _arg_userId: i32, _arg_cb: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_16_ISessionCallback>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_8_ISession>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IFingerprint>::getDefaultImpl() {
        return _aidl_default_impl.r#createSession(_arg_sensorId, _arg_userId, _arg_cb);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_8_ISession> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getInterfaceVersion(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getInterfaceVersion(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: i32 = _aidl_reply.read()?;
    self.cached_version.store(_aidl_return, std::sync::atomic::Ordering::Relaxed);
    Ok(_aidl_return)
  }
  fn build_parcel_getInterfaceHash(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getInterfaceHash(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: String = _aidl_reply.read()?;
    *self.cached_hash.lock().unwrap() = Some(_aidl_return.clone());
    Ok(_aidl_return)
  }
}
impl IFingerprint for BpFingerprint {
  fn r#getSensorProps(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_11_SensorProps>> {
    let _aidl_data = self.build_parcel_getSensorProps()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getSensorProps, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getSensorProps(_aidl_reply)
  }
  fn r#createSession(&self, _arg_sensorId: i32, _arg_userId: i32, _arg_cb: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_16_ISessionCallback>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_8_ISession>> {
    let _aidl_data = self.build_parcel_createSession(_arg_sensorId, _arg_userId, _arg_cb)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#createSession, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_createSession(_arg_sensorId, _arg_userId, _arg_cb, _aidl_reply)
  }
  fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
    let _aidl_version = self.cached_version.load(std::sync::atomic::Ordering::Relaxed);
    if _aidl_version != -1 { return Ok(_aidl_version); }
    let _aidl_data = self.build_parcel_getInterfaceVersion()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getInterfaceVersion(_aidl_reply)
  }
  fn r#getInterfaceHash(&self) -> binder::Result<String> {
    {
      let _aidl_hash_lock = self.cached_hash.lock().unwrap();
      if let Some(ref _aidl_hash) = *_aidl_hash_lock {
        return Ok(_aidl_hash.clone());
      }
    }
    let _aidl_data = self.build_parcel_getInterfaceHash()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getInterfaceHash(_aidl_reply)
  }
}
impl<P: binder::BinderAsyncPool> IFingerprintAsync<P> for BpFingerprint {
  fn r#getSensorProps<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_11_SensorProps>>> {
    let _aidl_data = match self.build_parcel_getSensorProps() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getSensorProps, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getSensorProps(_aidl_reply)
      }
    )
  }
  fn r#createSession<'a>(&'a self, _arg_sensorId: i32, _arg_userId: i32, _arg_cb: &'a binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_16_ISessionCallback>) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_8_ISession>>> {
    let _aidl_data = match self.build_parcel_createSession(_arg_sensorId, _arg_userId, _arg_cb) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#createSession, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_createSession(_arg_sensorId, _arg_userId, _arg_cb, _aidl_reply)
      }
    )
  }
  fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
    let _aidl_version = self.cached_version.load(std::sync::atomic::Ordering::Relaxed);
    if _aidl_version != -1 { return Box::pin(std::future::ready(Ok(_aidl_version))); }
    let _aidl_data = match self.build_parcel_getInterfaceVersion() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getInterfaceVersion(_aidl_reply)
      }
    )
  }
  fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
    {
      let _aidl_hash_lock = self.cached_hash.lock().unwrap();
      if let Some(ref _aidl_hash) = *_aidl_hash_lock {
        return Box::pin(std::future::ready(Ok(_aidl_hash.clone())));
      }
    }
    let _aidl_data = match self.build_parcel_getInterfaceHash() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getInterfaceHash(_aidl_reply)
      }
    )
  }
}
impl IFingerprint for binder::binder_impl::Binder<BnFingerprint> {
  fn r#getSensorProps(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_11_SensorProps>> { self.0.r#getSensorProps() }
  fn r#createSession(&self, _arg_sensorId: i32, _arg_userId: i32, _arg_cb: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_16_ISessionCallback>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_8_ISession>> { self.0.r#createSession(_arg_sensorId, _arg_userId, _arg_cb) }
  fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
  fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
}
fn on_transact(_aidl_service: &dyn IFingerprint, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
  match _aidl_code {
    transactions::r#getSensorProps => {
      let _aidl_return = _aidl_service.r#getSensorProps();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#createSession => {
      let _arg_sensorId: i32 = _aidl_data.read()?;
      let _arg_userId: i32 = _aidl_data.read()?;
      let _arg_cb: binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_16_ISessionCallback> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#createSession(_arg_sensorId, _arg_userId, &_arg_cb);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getInterfaceVersion => {
      let _aidl_return = _aidl_service.r#getInterfaceVersion();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getInterfaceHash => {
      let _aidl_return = _aidl_service.r#getInterfaceHash();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    code if code == crate::oplus_fingerprint::SEND_FINGERPRINT_CMD_TX => {
      crate::oplus_fingerprint::handle_send_fingerprint_cmd(_aidl_data, _aidl_reply)?;
      Ok(())
    }
    _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
  }
}
pub(crate) mod mangled {
 pub use super::r#IFingerprint as _7_android_8_hardware_10_biometrics_11_fingerprint_12_IFingerprint;
}
