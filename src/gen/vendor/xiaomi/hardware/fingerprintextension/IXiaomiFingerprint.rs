#![forbid(unsafe_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
use binder::declare_binder_interface;
declare_binder_interface! {
  IXiaomiFingerprint["vendor.xiaomi.hardware.fingerprintextension.IXiaomiFingerprint"] {
    native: BnXiaomiFingerprint(on_transact),
    proxy: BpXiaomiFingerprint {
      cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
      cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
    },
    async: IXiaomiFingerprintAsync,
    stability: binder::binder_impl::Stability::Vintf,
  }
}
pub trait IXiaomiFingerprint: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "vendor.xiaomi.hardware.fingerprintextension.IXiaomiFingerprint" }
  fn r#extCmd(&self, _arg_cmdId: i32, _arg_cmdParam: i32) -> binder::Result<i32>;
  fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
    Ok(VERSION)
  }
  fn r#getInterfaceHash(&self) -> binder::Result<String> {
    Ok(HASH.into())
  }
  fn getDefaultImpl() -> IXiaomiFingerprintDefaultRef where Self: Sized {
    DEFAULT_IMPL.lock().unwrap().clone()
  }
  fn setDefaultImpl(d: IXiaomiFingerprintDefaultRef) -> IXiaomiFingerprintDefaultRef where Self: Sized {
    std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
  }
}
pub trait IXiaomiFingerprintAsync<P>: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "vendor.xiaomi.hardware.fingerprintextension.IXiaomiFingerprint" }
  fn r#extCmd<'a>(&'a self, _arg_cmdId: i32, _arg_cmdParam: i32) -> binder::BoxFuture<'a, binder::Result<i32>>;
  fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
    Box::pin(async move { Ok(VERSION) })
  }
  fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
    Box::pin(async move { Ok(HASH.into()) })
  }
}
#[::async_trait::async_trait]
pub trait IXiaomiFingerprintAsyncServer: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "vendor.xiaomi.hardware.fingerprintextension.IXiaomiFingerprint" }
  async fn r#extCmd(&self, _arg_cmdId: i32, _arg_cmdParam: i32) -> binder::Result<i32>;
}
impl BnXiaomiFingerprint {
  /// Create a new async binder service.
  pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IXiaomiFingerprint>
  where
    T: IXiaomiFingerprintAsyncServer + binder::Interface + Send + Sync + 'static,
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
    impl<T, R> IXiaomiFingerprint for Wrapper<T, R>
    where
      T: IXiaomiFingerprintAsyncServer + Send + Sync + 'static,
      R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
    {
      fn r#extCmd(&self, _arg_cmdId: i32, _arg_cmdParam: i32) -> binder::Result<i32> {
        self._rt.block_on(self._inner.r#extCmd(_arg_cmdId, _arg_cmdParam))
      }
    }
    let wrapped = Wrapper { _inner: inner, _rt: rt };
    Self::new_binder(wrapped, features)
  }
}
pub trait IXiaomiFingerprintDefault: Send + Sync {
  fn r#extCmd(&self, _arg_cmdId: i32, _arg_cmdParam: i32) -> binder::Result<i32> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
}
pub mod transactions {
  pub const r#extCmd: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
  pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
  pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
}
pub type IXiaomiFingerprintDefaultRef = Option<std::sync::Arc<dyn IXiaomiFingerprintDefault>>;
use lazy_static::lazy_static;
lazy_static! {
  static ref DEFAULT_IMPL: std::sync::Mutex<IXiaomiFingerprintDefaultRef> = std::sync::Mutex::new(None);
}
pub const VERSION: i32 = 1;
pub const HASH: &str = "1ed45089cc89154986c15fc1c591d72e9ac7ae0d";
impl BpXiaomiFingerprint {
  fn build_parcel_extCmd(&self, _arg_cmdId: i32, _arg_cmdParam: i32) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_cmdId)?;
    aidl_data.write(&_arg_cmdParam)?;
    Ok(aidl_data)
  }
  fn read_response_extCmd(&self, _arg_cmdId: i32, _arg_cmdParam: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IXiaomiFingerprint>::getDefaultImpl() {
        return _aidl_default_impl.r#extCmd(_arg_cmdId, _arg_cmdParam);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: i32 = _aidl_reply.read()?;
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
impl IXiaomiFingerprint for BpXiaomiFingerprint {
  fn r#extCmd(&self, _arg_cmdId: i32, _arg_cmdParam: i32) -> binder::Result<i32> {
    let _aidl_data = self.build_parcel_extCmd(_arg_cmdId, _arg_cmdParam)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#extCmd, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_extCmd(_arg_cmdId, _arg_cmdParam, _aidl_reply)
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
impl<P: binder::BinderAsyncPool> IXiaomiFingerprintAsync<P> for BpXiaomiFingerprint {
  fn r#extCmd<'a>(&'a self, _arg_cmdId: i32, _arg_cmdParam: i32) -> binder::BoxFuture<'a, binder::Result<i32>> {
    let _aidl_data = match self.build_parcel_extCmd(_arg_cmdId, _arg_cmdParam) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#extCmd, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_extCmd(_arg_cmdId, _arg_cmdParam, _aidl_reply)
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
impl IXiaomiFingerprint for binder::binder_impl::Binder<BnXiaomiFingerprint> {
  fn r#extCmd(&self, _arg_cmdId: i32, _arg_cmdParam: i32) -> binder::Result<i32> { self.0.r#extCmd(_arg_cmdId, _arg_cmdParam) }
  fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
  fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
}
fn on_transact(_aidl_service: &dyn IXiaomiFingerprint, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
  match _aidl_code {
    transactions::r#extCmd => {
      let _arg_cmdId: i32 = _aidl_data.read()?;
      let _arg_cmdParam: i32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#extCmd(_arg_cmdId, _arg_cmdParam);
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
    _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
  }
}
pub(crate) mod mangled {
 pub use super::r#IXiaomiFingerprint as _6_vendor_6_xiaomi_8_hardware_20_fingerprintextension_18_IXiaomiFingerprint;
}
