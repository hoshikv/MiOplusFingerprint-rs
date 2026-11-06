use mfp_daemon::mangled::_6_vendor_6_xiaomi_8_hardware_20_fingerprintextension_18_IXiaomiFingerprint as XiaomiTrait;
use mfp_daemon::service::XIAOMI_SERVICE;

fn main() {
    let mut args = std::env::args().skip(1);
    let cmd: i32 = args.next().and_then(|v| v.parse().ok()).unwrap_or(0);
    let param: i32 = args.next().and_then(|v| v.parse().ok()).unwrap_or(0);
    binder::ProcessState::start_thread_pool();
    let service: binder::Strong<dyn XiaomiTrait> =
        binder::get_interface(XIAOMI_SERVICE).expect("xiaomi fingerprint service");
    match service.r#extCmd(cmd, param) {
        Ok(ret) => println!("extCmd({cmd}, {param}) = {ret}"),
        Err(e) => println!("extCmd({cmd}, {param}) failed: {e:?}"),
    }
}
