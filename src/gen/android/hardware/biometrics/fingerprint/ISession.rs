#![forbid(unsafe_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
use binder::declare_binder_interface;
declare_binder_interface! {
  ISession["android.hardware.biometrics.fingerprint.ISession"] {
    native: BnSession(on_transact),
    proxy: BpSession {
      cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
      cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
    },
    async: ISessionAsync,
    stability: binder::binder_impl::Stability::Vintf,
  }
}
pub trait ISession: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.biometrics.fingerprint.ISession" }
  fn r#generateChallenge(&self) -> binder::Result<()>;
  fn r#revokeChallenge(&self, _arg_challenge: i64) -> binder::Result<()>;
  fn r#enroll(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>;
  fn r#authenticate(&self, _arg_operationId: i64) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>;
  fn r#detectInteraction(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>;
  fn r#enumerateEnrollments(&self) -> binder::Result<()>;
  fn r#removeEnrollments(&self, _arg_enrollmentIds: &[i32]) -> binder::Result<()>;
  fn r#getAuthenticatorId(&self) -> binder::Result<()>;
  fn r#invalidateAuthenticatorId(&self) -> binder::Result<()>;
  fn r#resetLockout(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken) -> binder::Result<()>;
  fn r#close(&self) -> binder::Result<()>;
  #[deprecated = "use onPointerDownWithContext instead."]
  fn r#onPointerDown(&self, _arg_pointerId: i32, _arg_x: i32, _arg_y: i32, _arg_minor: f32, _arg_major: f32) -> binder::Result<()>;
  #[deprecated = "use onPointerUpWithContext instead."]
  fn r#onPointerUp(&self, _arg_pointerId: i32) -> binder::Result<()>;
  fn r#onUiReady(&self) -> binder::Result<()>;
  fn r#authenticateWithContext(&self, _arg_operationId: i64, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>;
  fn r#enrollWithContext(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>;
  fn r#detectInteractionWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>;
  fn r#onPointerDownWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::Result<()>;
  fn r#onPointerUpWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::Result<()>;
  fn r#onContextChanged(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<()>;
  fn r#onPointerCancelWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::Result<()>;
  fn r#setIgnoreDisplayTouches(&self, _arg_shouldIgnore: bool) -> binder::Result<()>;
  fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
    Ok(VERSION)
  }
  fn r#getInterfaceHash(&self) -> binder::Result<String> {
    Ok(HASH.into())
  }
  fn getDefaultImpl() -> ISessionDefaultRef where Self: Sized {
    DEFAULT_IMPL.lock().unwrap().clone()
  }
  fn setDefaultImpl(d: ISessionDefaultRef) -> ISessionDefaultRef where Self: Sized {
    std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
  }
}
pub trait ISessionAsync<P>: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.biometrics.fingerprint.ISession" }
  fn r#generateChallenge<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#revokeChallenge<'a>(&'a self, _arg_challenge: i64) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#enroll<'a>(&'a self, _arg_hat: &'a crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>>;
  fn r#authenticate<'a>(&'a self, _arg_operationId: i64) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>>;
  fn r#detectInteraction<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>>;
  fn r#enumerateEnrollments<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#removeEnrollments<'a>(&'a self, _arg_enrollmentIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#getAuthenticatorId<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#invalidateAuthenticatorId<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#resetLockout<'a>(&'a self, _arg_hat: &'a crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#close<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
  #[deprecated = "use onPointerDownWithContext instead."]
  fn r#onPointerDown<'a>(&'a self, _arg_pointerId: i32, _arg_x: i32, _arg_y: i32, _arg_minor: f32, _arg_major: f32) -> binder::BoxFuture<'a, binder::Result<()>>;
  #[deprecated = "use onPointerUpWithContext instead."]
  fn r#onPointerUp<'a>(&'a self, _arg_pointerId: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#onUiReady<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#authenticateWithContext<'a>(&'a self, _arg_operationId: i64, _arg_context: &'a crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>>;
  fn r#enrollWithContext<'a>(&'a self, _arg_hat: &'a crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken, _arg_context: &'a crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>>;
  fn r#detectInteractionWithContext<'a>(&'a self, _arg_context: &'a crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>>;
  fn r#onPointerDownWithContext<'a>(&'a self, _arg_context: &'a crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#onPointerUpWithContext<'a>(&'a self, _arg_context: &'a crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#onContextChanged<'a>(&'a self, _arg_context: &'a crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#onPointerCancelWithContext<'a>(&'a self, _arg_context: &'a crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#setIgnoreDisplayTouches<'a>(&'a self, _arg_shouldIgnore: bool) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
    Box::pin(async move { Ok(VERSION) })
  }
  fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
    Box::pin(async move { Ok(HASH.into()) })
  }
}
#[::async_trait::async_trait]
pub trait ISessionAsyncServer: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.biometrics.fingerprint.ISession" }
  async fn r#generateChallenge(&self) -> binder::Result<()>;
  async fn r#revokeChallenge(&self, _arg_challenge: i64) -> binder::Result<()>;
  async fn r#enroll(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>;
  async fn r#authenticate(&self, _arg_operationId: i64) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>;
  async fn r#detectInteraction(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>;
  async fn r#enumerateEnrollments(&self) -> binder::Result<()>;
  async fn r#removeEnrollments(&self, _arg_enrollmentIds: &[i32]) -> binder::Result<()>;
  async fn r#getAuthenticatorId(&self) -> binder::Result<()>;
  async fn r#invalidateAuthenticatorId(&self) -> binder::Result<()>;
  async fn r#resetLockout(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken) -> binder::Result<()>;
  async fn r#close(&self) -> binder::Result<()>;
  #[deprecated = "use onPointerDownWithContext instead."]
  async fn r#onPointerDown(&self, _arg_pointerId: i32, _arg_x: i32, _arg_y: i32, _arg_minor: f32, _arg_major: f32) -> binder::Result<()>;
  #[deprecated = "use onPointerUpWithContext instead."]
  async fn r#onPointerUp(&self, _arg_pointerId: i32) -> binder::Result<()>;
  async fn r#onUiReady(&self) -> binder::Result<()>;
  async fn r#authenticateWithContext(&self, _arg_operationId: i64, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>;
  async fn r#enrollWithContext(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>;
  async fn r#detectInteractionWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>;
  async fn r#onPointerDownWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::Result<()>;
  async fn r#onPointerUpWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::Result<()>;
  async fn r#onContextChanged(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<()>;
  async fn r#onPointerCancelWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::Result<()>;
  async fn r#setIgnoreDisplayTouches(&self, _arg_shouldIgnore: bool) -> binder::Result<()>;
}
impl BnSession {
  /// Create a new async binder service.
  pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn ISession>
  where
    T: ISessionAsyncServer + binder::Interface + Send + Sync + 'static,
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
    impl<T, R> ISession for Wrapper<T, R>
    where
      T: ISessionAsyncServer + Send + Sync + 'static,
      R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
    {
      fn r#generateChallenge(&self) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#generateChallenge())
      }
      fn r#revokeChallenge(&self, _arg_challenge: i64) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#revokeChallenge(_arg_challenge))
      }
      fn r#enroll(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
        self._rt.block_on(self._inner.r#enroll(_arg_hat))
      }
      fn r#authenticate(&self, _arg_operationId: i64) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
        self._rt.block_on(self._inner.r#authenticate(_arg_operationId))
      }
      fn r#detectInteraction(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
        self._rt.block_on(self._inner.r#detectInteraction())
      }
      fn r#enumerateEnrollments(&self) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#enumerateEnrollments())
      }
      fn r#removeEnrollments(&self, _arg_enrollmentIds: &[i32]) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#removeEnrollments(_arg_enrollmentIds))
      }
      fn r#getAuthenticatorId(&self) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#getAuthenticatorId())
      }
      fn r#invalidateAuthenticatorId(&self) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#invalidateAuthenticatorId())
      }
      fn r#resetLockout(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#resetLockout(_arg_hat))
      }
      fn r#close(&self) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#close())
      }
      fn r#onPointerDown(&self, _arg_pointerId: i32, _arg_x: i32, _arg_y: i32, _arg_minor: f32, _arg_major: f32) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#onPointerDown(_arg_pointerId, _arg_x, _arg_y, _arg_minor, _arg_major))
      }
      fn r#onPointerUp(&self, _arg_pointerId: i32) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#onPointerUp(_arg_pointerId))
      }
      fn r#onUiReady(&self) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#onUiReady())
      }
      fn r#authenticateWithContext(&self, _arg_operationId: i64, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
        self._rt.block_on(self._inner.r#authenticateWithContext(_arg_operationId, _arg_context))
      }
      fn r#enrollWithContext(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
        self._rt.block_on(self._inner.r#enrollWithContext(_arg_hat, _arg_context))
      }
      fn r#detectInteractionWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
        self._rt.block_on(self._inner.r#detectInteractionWithContext(_arg_context))
      }
      fn r#onPointerDownWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#onPointerDownWithContext(_arg_context))
      }
      fn r#onPointerUpWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#onPointerUpWithContext(_arg_context))
      }
      fn r#onContextChanged(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#onContextChanged(_arg_context))
      }
      fn r#onPointerCancelWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#onPointerCancelWithContext(_arg_context))
      }
      fn r#setIgnoreDisplayTouches(&self, _arg_shouldIgnore: bool) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#setIgnoreDisplayTouches(_arg_shouldIgnore))
      }
    }
    let wrapped = Wrapper { _inner: inner, _rt: rt };
    Self::new_binder(wrapped, features)
  }
}
pub trait ISessionDefault: Send + Sync {
  fn r#generateChallenge(&self) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#revokeChallenge(&self, _arg_challenge: i64) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#enroll(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#authenticate(&self, _arg_operationId: i64) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#detectInteraction(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#enumerateEnrollments(&self) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#removeEnrollments(&self, _arg_enrollmentIds: &[i32]) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getAuthenticatorId(&self) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#invalidateAuthenticatorId(&self) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#resetLockout(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#close(&self) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#onPointerDown(&self, _arg_pointerId: i32, _arg_x: i32, _arg_y: i32, _arg_minor: f32, _arg_major: f32) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#onPointerUp(&self, _arg_pointerId: i32) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#onUiReady(&self) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#authenticateWithContext(&self, _arg_operationId: i64, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#enrollWithContext(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#detectInteractionWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#onPointerDownWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#onPointerUpWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#onContextChanged(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#onPointerCancelWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#setIgnoreDisplayTouches(&self, _arg_shouldIgnore: bool) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
}
pub mod transactions {
  pub const r#generateChallenge: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
  pub const r#revokeChallenge: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
  pub const r#enroll: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
  pub const r#authenticate: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
  pub const r#detectInteraction: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
  pub const r#enumerateEnrollments: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
  pub const r#removeEnrollments: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
  pub const r#getAuthenticatorId: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
  pub const r#invalidateAuthenticatorId: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
  pub const r#resetLockout: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
  pub const r#close: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
  pub const r#onPointerDown: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
  pub const r#onPointerUp: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
  pub const r#onUiReady: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
  pub const r#authenticateWithContext: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 14;
  pub const r#enrollWithContext: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 15;
  pub const r#detectInteractionWithContext: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16;
  pub const r#onPointerDownWithContext: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 17;
  pub const r#onPointerUpWithContext: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 18;
  pub const r#onContextChanged: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 19;
  pub const r#onPointerCancelWithContext: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 20;
  pub const r#setIgnoreDisplayTouches: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 21;
  pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
  pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
}
pub type ISessionDefaultRef = Option<std::sync::Arc<dyn ISessionDefault>>;
use lazy_static::lazy_static;
lazy_static! {
  static ref DEFAULT_IMPL: std::sync::Mutex<ISessionDefaultRef> = std::sync::Mutex::new(None);
}
pub const VERSION: i32 = 3;
pub const HASH: &str = "637371b53fb7faf9bd43aa51b72c23852d6e6d96";
impl BpSession {
  fn build_parcel_generateChallenge(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_generateChallenge(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#generateChallenge();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_revokeChallenge(&self, _arg_challenge: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_challenge)?;
    Ok(aidl_data)
  }
  fn read_response_revokeChallenge(&self, _arg_challenge: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#revokeChallenge(_arg_challenge);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_enroll(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_hat)?;
    Ok(aidl_data)
  }
  fn read_response_enroll(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#enroll(_arg_hat);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_authenticate(&self, _arg_operationId: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_operationId)?;
    Ok(aidl_data)
  }
  fn read_response_authenticate(&self, _arg_operationId: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#authenticate(_arg_operationId);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_detectInteraction(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_detectInteraction(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#detectInteraction();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_enumerateEnrollments(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_enumerateEnrollments(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#enumerateEnrollments();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_removeEnrollments(&self, _arg_enrollmentIds: &[i32]) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_enrollmentIds)?;
    Ok(aidl_data)
  }
  fn read_response_removeEnrollments(&self, _arg_enrollmentIds: &[i32], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#removeEnrollments(_arg_enrollmentIds);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_getAuthenticatorId(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getAuthenticatorId(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#getAuthenticatorId();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_invalidateAuthenticatorId(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_invalidateAuthenticatorId(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#invalidateAuthenticatorId();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_resetLockout(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_hat)?;
    Ok(aidl_data)
  }
  fn read_response_resetLockout(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#resetLockout(_arg_hat);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_close(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_close(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#close();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_onPointerDown(&self, _arg_pointerId: i32, _arg_x: i32, _arg_y: i32, _arg_minor: f32, _arg_major: f32) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_pointerId)?;
    aidl_data.write(&_arg_x)?;
    aidl_data.write(&_arg_y)?;
    aidl_data.write(&_arg_minor)?;
    aidl_data.write(&_arg_major)?;
    Ok(aidl_data)
  }
  fn read_response_onPointerDown(&self, _arg_pointerId: i32, _arg_x: i32, _arg_y: i32, _arg_minor: f32, _arg_major: f32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#onPointerDown(_arg_pointerId, _arg_x, _arg_y, _arg_minor, _arg_major);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_onPointerUp(&self, _arg_pointerId: i32) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_pointerId)?;
    Ok(aidl_data)
  }
  fn read_response_onPointerUp(&self, _arg_pointerId: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#onPointerUp(_arg_pointerId);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_onUiReady(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_onUiReady(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#onUiReady();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_authenticateWithContext(&self, _arg_operationId: i64, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_operationId)?;
    aidl_data.write(_arg_context)?;
    Ok(aidl_data)
  }
  fn read_response_authenticateWithContext(&self, _arg_operationId: i64, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#authenticateWithContext(_arg_operationId, _arg_context);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_enrollWithContext(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_hat)?;
    aidl_data.write(_arg_context)?;
    Ok(aidl_data)
  }
  fn read_response_enrollWithContext(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#enrollWithContext(_arg_hat, _arg_context);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_detectInteractionWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_context)?;
    Ok(aidl_data)
  }
  fn read_response_detectInteractionWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#detectInteractionWithContext(_arg_context);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_onPointerDownWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_context)?;
    Ok(aidl_data)
  }
  fn read_response_onPointerDownWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#onPointerDownWithContext(_arg_context);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_onPointerUpWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_context)?;
    Ok(aidl_data)
  }
  fn read_response_onPointerUpWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#onPointerUpWithContext(_arg_context);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_onContextChanged(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_context)?;
    Ok(aidl_data)
  }
  fn read_response_onContextChanged(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#onContextChanged(_arg_context);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_onPointerCancelWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_context)?;
    Ok(aidl_data)
  }
  fn read_response_onPointerCancelWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#onPointerCancelWithContext(_arg_context);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_setIgnoreDisplayTouches(&self, _arg_shouldIgnore: bool) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_shouldIgnore)?;
    Ok(aidl_data)
  }
  fn read_response_setIgnoreDisplayTouches(&self, _arg_shouldIgnore: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ISession>::getDefaultImpl() {
        return _aidl_default_impl.r#setIgnoreDisplayTouches(_arg_shouldIgnore);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
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
impl ISession for BpSession {
  fn r#generateChallenge(&self) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_generateChallenge()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#generateChallenge, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_generateChallenge(_aidl_reply)
  }
  fn r#revokeChallenge(&self, _arg_challenge: i64) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_revokeChallenge(_arg_challenge)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#revokeChallenge, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_revokeChallenge(_arg_challenge, _aidl_reply)
  }
  fn r#enroll(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
    let _aidl_data = self.build_parcel_enroll(_arg_hat)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#enroll, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_enroll(_arg_hat, _aidl_reply)
  }
  fn r#authenticate(&self, _arg_operationId: i64) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
    let _aidl_data = self.build_parcel_authenticate(_arg_operationId)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#authenticate, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_authenticate(_arg_operationId, _aidl_reply)
  }
  fn r#detectInteraction(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
    let _aidl_data = self.build_parcel_detectInteraction()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#detectInteraction, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_detectInteraction(_aidl_reply)
  }
  fn r#enumerateEnrollments(&self) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_enumerateEnrollments()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#enumerateEnrollments, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_enumerateEnrollments(_aidl_reply)
  }
  fn r#removeEnrollments(&self, _arg_enrollmentIds: &[i32]) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_removeEnrollments(_arg_enrollmentIds)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#removeEnrollments, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_removeEnrollments(_arg_enrollmentIds, _aidl_reply)
  }
  fn r#getAuthenticatorId(&self) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_getAuthenticatorId()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getAuthenticatorId, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getAuthenticatorId(_aidl_reply)
  }
  fn r#invalidateAuthenticatorId(&self) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_invalidateAuthenticatorId()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#invalidateAuthenticatorId, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_invalidateAuthenticatorId(_aidl_reply)
  }
  fn r#resetLockout(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_resetLockout(_arg_hat)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#resetLockout, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_resetLockout(_arg_hat, _aidl_reply)
  }
  fn r#close(&self) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_close()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#close, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_close(_aidl_reply)
  }
  fn r#onPointerDown(&self, _arg_pointerId: i32, _arg_x: i32, _arg_y: i32, _arg_minor: f32, _arg_major: f32) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_onPointerDown(_arg_pointerId, _arg_x, _arg_y, _arg_minor, _arg_major)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#onPointerDown, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_onPointerDown(_arg_pointerId, _arg_x, _arg_y, _arg_minor, _arg_major, _aidl_reply)
  }
  fn r#onPointerUp(&self, _arg_pointerId: i32) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_onPointerUp(_arg_pointerId)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#onPointerUp, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_onPointerUp(_arg_pointerId, _aidl_reply)
  }
  fn r#onUiReady(&self) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_onUiReady()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#onUiReady, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_onUiReady(_aidl_reply)
  }
  fn r#authenticateWithContext(&self, _arg_operationId: i64, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
    let _aidl_data = self.build_parcel_authenticateWithContext(_arg_operationId, _arg_context)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#authenticateWithContext, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_authenticateWithContext(_arg_operationId, _arg_context, _aidl_reply)
  }
  fn r#enrollWithContext(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
    let _aidl_data = self.build_parcel_enrollWithContext(_arg_hat, _arg_context)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#enrollWithContext, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_enrollWithContext(_arg_hat, _arg_context, _aidl_reply)
  }
  fn r#detectInteractionWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> {
    let _aidl_data = self.build_parcel_detectInteractionWithContext(_arg_context)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#detectInteractionWithContext, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_detectInteractionWithContext(_arg_context, _aidl_reply)
  }
  fn r#onPointerDownWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_onPointerDownWithContext(_arg_context)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#onPointerDownWithContext, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_onPointerDownWithContext(_arg_context, _aidl_reply)
  }
  fn r#onPointerUpWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_onPointerUpWithContext(_arg_context)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#onPointerUpWithContext, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_onPointerUpWithContext(_arg_context, _aidl_reply)
  }
  fn r#onContextChanged(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_onContextChanged(_arg_context)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#onContextChanged, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_onContextChanged(_arg_context, _aidl_reply)
  }
  fn r#onPointerCancelWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_onPointerCancelWithContext(_arg_context)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#onPointerCancelWithContext, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_onPointerCancelWithContext(_arg_context, _aidl_reply)
  }
  fn r#setIgnoreDisplayTouches(&self, _arg_shouldIgnore: bool) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_setIgnoreDisplayTouches(_arg_shouldIgnore)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#setIgnoreDisplayTouches, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_setIgnoreDisplayTouches(_arg_shouldIgnore, _aidl_reply)
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
impl<P: binder::BinderAsyncPool> ISessionAsync<P> for BpSession {
  fn r#generateChallenge<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_generateChallenge() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#generateChallenge, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_generateChallenge(_aidl_reply)
      }
    )
  }
  fn r#revokeChallenge<'a>(&'a self, _arg_challenge: i64) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_revokeChallenge(_arg_challenge) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#revokeChallenge, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_revokeChallenge(_arg_challenge, _aidl_reply)
      }
    )
  }
  fn r#enroll<'a>(&'a self, _arg_hat: &'a crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>> {
    let _aidl_data = match self.build_parcel_enroll(_arg_hat) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#enroll, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_enroll(_arg_hat, _aidl_reply)
      }
    )
  }
  fn r#authenticate<'a>(&'a self, _arg_operationId: i64) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>> {
    let _aidl_data = match self.build_parcel_authenticate(_arg_operationId) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#authenticate, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_authenticate(_arg_operationId, _aidl_reply)
      }
    )
  }
  fn r#detectInteraction<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>> {
    let _aidl_data = match self.build_parcel_detectInteraction() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#detectInteraction, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_detectInteraction(_aidl_reply)
      }
    )
  }
  fn r#enumerateEnrollments<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_enumerateEnrollments() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#enumerateEnrollments, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_enumerateEnrollments(_aidl_reply)
      }
    )
  }
  fn r#removeEnrollments<'a>(&'a self, _arg_enrollmentIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_removeEnrollments(_arg_enrollmentIds) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#removeEnrollments, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_removeEnrollments(_arg_enrollmentIds, _aidl_reply)
      }
    )
  }
  fn r#getAuthenticatorId<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_getAuthenticatorId() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getAuthenticatorId, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getAuthenticatorId(_aidl_reply)
      }
    )
  }
  fn r#invalidateAuthenticatorId<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_invalidateAuthenticatorId() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#invalidateAuthenticatorId, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_invalidateAuthenticatorId(_aidl_reply)
      }
    )
  }
  fn r#resetLockout<'a>(&'a self, _arg_hat: &'a crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_resetLockout(_arg_hat) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#resetLockout, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_resetLockout(_arg_hat, _aidl_reply)
      }
    )
  }
  fn r#close<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_close() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#close, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_close(_aidl_reply)
      }
    )
  }
  fn r#onPointerDown<'a>(&'a self, _arg_pointerId: i32, _arg_x: i32, _arg_y: i32, _arg_minor: f32, _arg_major: f32) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_onPointerDown(_arg_pointerId, _arg_x, _arg_y, _arg_minor, _arg_major) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#onPointerDown, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_onPointerDown(_arg_pointerId, _arg_x, _arg_y, _arg_minor, _arg_major, _aidl_reply)
      }
    )
  }
  fn r#onPointerUp<'a>(&'a self, _arg_pointerId: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_onPointerUp(_arg_pointerId) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#onPointerUp, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_onPointerUp(_arg_pointerId, _aidl_reply)
      }
    )
  }
  fn r#onUiReady<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_onUiReady() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#onUiReady, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_onUiReady(_aidl_reply)
      }
    )
  }
  fn r#authenticateWithContext<'a>(&'a self, _arg_operationId: i64, _arg_context: &'a crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>> {
    let _aidl_data = match self.build_parcel_authenticateWithContext(_arg_operationId, _arg_context) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#authenticateWithContext, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_authenticateWithContext(_arg_operationId, _arg_context, _aidl_reply)
      }
    )
  }
  fn r#enrollWithContext<'a>(&'a self, _arg_hat: &'a crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken, _arg_context: &'a crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>> {
    let _aidl_data = match self.build_parcel_enrollWithContext(_arg_hat, _arg_context) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#enrollWithContext, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_enrollWithContext(_arg_hat, _arg_context, _aidl_reply)
      }
    )
  }
  fn r#detectInteractionWithContext<'a>(&'a self, _arg_context: &'a crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>>> {
    let _aidl_data = match self.build_parcel_detectInteractionWithContext(_arg_context) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#detectInteractionWithContext, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_detectInteractionWithContext(_arg_context, _aidl_reply)
      }
    )
  }
  fn r#onPointerDownWithContext<'a>(&'a self, _arg_context: &'a crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_onPointerDownWithContext(_arg_context) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#onPointerDownWithContext, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_onPointerDownWithContext(_arg_context, _aidl_reply)
      }
    )
  }
  fn r#onPointerUpWithContext<'a>(&'a self, _arg_context: &'a crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_onPointerUpWithContext(_arg_context) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#onPointerUpWithContext, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_onPointerUpWithContext(_arg_context, _aidl_reply)
      }
    )
  }
  fn r#onContextChanged<'a>(&'a self, _arg_context: &'a crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_onContextChanged(_arg_context) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#onContextChanged, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_onContextChanged(_arg_context, _aidl_reply)
      }
    )
  }
  fn r#onPointerCancelWithContext<'a>(&'a self, _arg_context: &'a crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_onPointerCancelWithContext(_arg_context) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#onPointerCancelWithContext, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_onPointerCancelWithContext(_arg_context, _aidl_reply)
      }
    )
  }
  fn r#setIgnoreDisplayTouches<'a>(&'a self, _arg_shouldIgnore: bool) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_setIgnoreDisplayTouches(_arg_shouldIgnore) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#setIgnoreDisplayTouches, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_setIgnoreDisplayTouches(_arg_shouldIgnore, _aidl_reply)
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
impl ISession for binder::binder_impl::Binder<BnSession> {
  fn r#generateChallenge(&self) -> binder::Result<()> { self.0.r#generateChallenge() }
  fn r#revokeChallenge(&self, _arg_challenge: i64) -> binder::Result<()> { self.0.r#revokeChallenge(_arg_challenge) }
  fn r#enroll(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> { self.0.r#enroll(_arg_hat) }
  fn r#authenticate(&self, _arg_operationId: i64) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> { self.0.r#authenticate(_arg_operationId) }
  fn r#detectInteraction(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> { self.0.r#detectInteraction() }
  fn r#enumerateEnrollments(&self) -> binder::Result<()> { self.0.r#enumerateEnrollments() }
  fn r#removeEnrollments(&self, _arg_enrollmentIds: &[i32]) -> binder::Result<()> { self.0.r#removeEnrollments(_arg_enrollmentIds) }
  fn r#getAuthenticatorId(&self) -> binder::Result<()> { self.0.r#getAuthenticatorId() }
  fn r#invalidateAuthenticatorId(&self) -> binder::Result<()> { self.0.r#invalidateAuthenticatorId() }
  fn r#resetLockout(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken) -> binder::Result<()> { self.0.r#resetLockout(_arg_hat) }
  fn r#close(&self) -> binder::Result<()> { self.0.r#close() }
  fn r#onPointerDown(&self, _arg_pointerId: i32, _arg_x: i32, _arg_y: i32, _arg_minor: f32, _arg_major: f32) -> binder::Result<()> { self.0.r#onPointerDown(_arg_pointerId, _arg_x, _arg_y, _arg_minor, _arg_major) }
  fn r#onPointerUp(&self, _arg_pointerId: i32) -> binder::Result<()> { self.0.r#onPointerUp(_arg_pointerId) }
  fn r#onUiReady(&self) -> binder::Result<()> { self.0.r#onUiReady() }
  fn r#authenticateWithContext(&self, _arg_operationId: i64, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> { self.0.r#authenticateWithContext(_arg_operationId, _arg_context) }
  fn r#enrollWithContext(&self, _arg_hat: &crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> { self.0.r#enrollWithContext(_arg_hat, _arg_context) }
  fn r#detectInteractionWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_10_biometrics_6_common_19_ICancellationSignal>> { self.0.r#detectInteractionWithContext(_arg_context) }
  fn r#onPointerDownWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::Result<()> { self.0.r#onPointerDownWithContext(_arg_context) }
  fn r#onPointerUpWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::Result<()> { self.0.r#onPointerUpWithContext(_arg_context) }
  fn r#onContextChanged(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext) -> binder::Result<()> { self.0.r#onContextChanged(_arg_context) }
  fn r#onPointerCancelWithContext(&self, _arg_context: &crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext) -> binder::Result<()> { self.0.r#onPointerCancelWithContext(_arg_context) }
  fn r#setIgnoreDisplayTouches(&self, _arg_shouldIgnore: bool) -> binder::Result<()> { self.0.r#setIgnoreDisplayTouches(_arg_shouldIgnore) }
  fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
  fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
}
fn on_transact(_aidl_service: &dyn ISession, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
  match _aidl_code {
    transactions::r#generateChallenge => {
      let _aidl_return = _aidl_service.r#generateChallenge();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#revokeChallenge => {
      let _arg_challenge: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#revokeChallenge(_arg_challenge);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#enroll => {
      let _arg_hat: crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#enroll(&_arg_hat);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#authenticate => {
      let _arg_operationId: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#authenticate(_arg_operationId);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#detectInteraction => {
      let _aidl_return = _aidl_service.r#detectInteraction();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#enumerateEnrollments => {
      let _aidl_return = _aidl_service.r#enumerateEnrollments();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#removeEnrollments => {
      let _arg_enrollmentIds: Vec<i32> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#removeEnrollments(&_arg_enrollmentIds);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getAuthenticatorId => {
      let _aidl_return = _aidl_service.r#getAuthenticatorId();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#invalidateAuthenticatorId => {
      let _aidl_return = _aidl_service.r#invalidateAuthenticatorId();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#resetLockout => {
      let _arg_hat: crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#resetLockout(&_arg_hat);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#close => {
      let _aidl_return = _aidl_service.r#close();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#onPointerDown => {
      let _arg_pointerId: i32 = _aidl_data.read()?;
      let _arg_x: i32 = _aidl_data.read()?;
      let _arg_y: i32 = _aidl_data.read()?;
      let _arg_minor: f32 = _aidl_data.read()?;
      let _arg_major: f32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#onPointerDown(_arg_pointerId, _arg_x, _arg_y, _arg_minor, _arg_major);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#onPointerUp => {
      let _arg_pointerId: i32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#onPointerUp(_arg_pointerId);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#onUiReady => {
      let _aidl_return = _aidl_service.r#onUiReady();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#authenticateWithContext => {
      let _arg_operationId: i64 = _aidl_data.read()?;
      let _arg_context: crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#authenticateWithContext(_arg_operationId, &_arg_context);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#enrollWithContext => {
      let _arg_hat: crate::mangled::_7_android_8_hardware_9_keymaster_17_HardwareAuthToken = _aidl_data.read()?;
      let _arg_context: crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#enrollWithContext(&_arg_hat, &_arg_context);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#detectInteractionWithContext => {
      let _arg_context: crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#detectInteractionWithContext(&_arg_context);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#onPointerDownWithContext => {
      let _arg_context: crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#onPointerDownWithContext(&_arg_context);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#onPointerUpWithContext => {
      let _arg_context: crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#onPointerUpWithContext(&_arg_context);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#onContextChanged => {
      let _arg_context: crate::mangled::_7_android_8_hardware_10_biometrics_6_common_16_OperationContext = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#onContextChanged(&_arg_context);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#onPointerCancelWithContext => {
      let _arg_context: crate::mangled::_7_android_8_hardware_10_biometrics_11_fingerprint_14_PointerContext = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#onPointerCancelWithContext(&_arg_context);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#setIgnoreDisplayTouches => {
      let _arg_shouldIgnore: bool = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#setIgnoreDisplayTouches(_arg_shouldIgnore);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
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
 pub use super::r#ISession as _7_android_8_hardware_10_biometrics_11_fingerprint_8_ISession;
}
