package vendor.xiaomi.hardware.fingerprintextension;

@VintfStability
interface IXiaomiFingerprint {
  int extCmd(int cmdId, int cmdParam);
}
