use binder::BinderFeatures;
use mfp_daemon::service::{
    self, BnFingerprint, BnXiaomiFingerprint, Daemon, Fingerprint, XiaomiFingerprint,
};
use mfp_daemon::{hal, loge, logi};
use std::sync::Arc;
use std::sync::mpsc;

fn main() {
    logi!("fingerprint AIDL HAL starting");

    binder::ProcessState::set_thread_pool_max_thread_count(0);

    let (notify_tx, notify_rx) = mpsc::channel();
    hal::set_notify_sender(notify_tx.clone());
    let hal = hal::Hal::open().map(Arc::new);
    drop(notify_tx);
    let daemon = Daemon::start(hal, notify_rx);

    let fingerprint =
        BnFingerprint::new_binder(Fingerprint::new(daemon.clone()), BinderFeatures::default());
    if let Err(e) =
        binder::register_lazy_service(service::FINGERPRINT_SERVICE, fingerprint.as_binder())
    {
        loge!("failed to register {}: {e:?}", service::FINGERPRINT_SERVICE);
        return;
    }

    binder::force_lazy_services_persist(true);

    if daemon.has_hal() {
        let xiaomi = BnXiaomiFingerprint::new_binder(
            XiaomiFingerprint::new(daemon.clone()),
            BinderFeatures::default(),
        );
        if let Err(e) = binder::add_service(service::XIAOMI_SERVICE, xiaomi.as_binder()) {
            loge!("failed to register {}: {e:?}", service::XIAOMI_SERVICE);
        }
    }

    logi!("fingerprint AIDL HAL ready");
    binder::ProcessState::join_thread_pool();
}
