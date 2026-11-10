# MiOplusFingerprint-rs

This project is based on [hoshikv/mfp-daemon-rs](https://github.com/hoshikv/mfp-daemon-rs).

It is a native Rust implementation of the Android fingerprint AIDL HAL with an
Oplus/ColorOS compatibility layer for the optical under-display fingerprint. The
original daemon supplies the legacy vendor HAL integration, FOD engine, and
TouchFeature client. This project adds the Oplus session bridge, ColorOS touch
notifications, local HBM handling, and the panel-feature fallback used during
boot.

## Project structure

All tracked paths are listed below. Local build output (`target/`) and the
machine-specific `.cargo/config.toml` are ignored.

```text
.
├── .cargo
│   └── config.toml.example
├── aidl
│   └── vendor
│       └── xiaomi
│           └── hardware
│               └── fingerprintextension
│                   └── IXiaomiFingerprint.aidl
├── src
│   ├── bin
│   │   ├── ext_cmd.rs
│   │   └── test_client.rs
│   ├── gen
│   │   ├── android
│   │   │   └── hardware
│   │   │       ├── biometrics
│   │   │       │   ├── common
│   │   │       │   │   ├── AuthenticateReason.rs
│   │   │       │   │   ├── CommonProps.rs
│   │   │       │   │   ├── ComponentInfo.rs
│   │   │       │   │   ├── DisplayState.rs
│   │   │       │   │   ├── ICancellationSignal.rs
│   │   │       │   │   ├── OperationContext.rs
│   │   │       │   │   ├── OperationReason.rs
│   │   │       │   │   ├── SensorStrength.rs
│   │   │       │   │   └── WakeReason.rs
│   │   │       │   └── fingerprint
│   │   │       │       ├── AcquiredInfo.rs
│   │   │       │       ├── Error.rs
│   │   │       │       ├── FingerprintSensorType.rs
│   │   │       │       ├── IFingerprint.rs
│   │   │       │       ├── ISession.rs
│   │   │       │       ├── ISessionCallback.rs
│   │   │       │       ├── PointerContext.rs
│   │   │       │       ├── SensorLocation.rs
│   │   │       │       ├── SensorProps.rs
│   │   │       │       ├── SensorShape.rs
│   │   │       │       └── TouchDetectionParameters.rs
│   │   │       └── keymaster
│   │   │           ├── HardwareAuthenticatorType.rs
│   │   │           ├── HardwareAuthToken.rs
│   │   │           ├── SecurityLevel.rs
│   │   │           ├── Timestamp.rs
│   │   │           └── VerificationToken.rs
│   │   └── vendor
│   │       └── xiaomi
│   │           └── hardware
│   │               └── fingerprintextension
│   │                   └── IXiaomiFingerprint.rs
│   ├── fod.rs
│   ├── hal.rs
│   ├── lib.rs
│   ├── log.rs
│   ├── main.rs
│   ├── oplus_fingerprint.rs
│   ├── service.rs
│   └── touchfeature.rs
├── vendor
│   └── binder_ndk_sys
│       ├── src
│       │   ├── include_cpp
│       │   │   └── android
│       │   │       ├── binder_auto_utils.h
│       │   │       ├── binder_enums.h
│       │   │       ├── binder_interface_utils.h
│       │   │       ├── binder_internal_logging.h
│       │   │       ├── binder_parcel_utils.h
│       │   │       ├── binder_parcelable_utils.h
│       │   │       └── binder_to_string.h
│       │   ├── include_ndk
│       │   │   └── android
│       │   │       ├── binder_ibinder.h
│       │   │       ├── binder_ibinder_jni.h
│       │   │       ├── binder_parcel.h
│       │   │       ├── binder_parcel_jni.h
│       │   │       └── binder_status.h
│       │   ├── include_platform
│       │   │   └── android
│       │   │       ├── binder_ibinder_platform.h
│       │   │       ├── binder_libbinder.h
│       │   │       ├── binder_manager.h
│       │   │       ├── binder_parcel_platform.h
│       │   │       ├── binder_process.h
│       │   │       ├── binder_shell.h
│       │   │       └── binder_stability.h
│       │   ├── BinderBindings.hpp
│       │   ├── lib.rs
│       │   └── symbols.txt
│       ├── build.rs
│       ├── Cargo.toml
│       └── Cargo.toml.orig
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── LICENSE
└── README.md
```

`main()` performs this sequence:

1. Log `daemon=start`.
2. Load the legacy fingerprint HAL.
3. Start `Daemon`, including the FOD display, touch, and evdev monitors.
4. Start `OplusFingerprint`, which registers or prewarms the Oplus panel service.
5. Register `IFingerprint/default` as a persistent lazy service.
6. Register `IXiaomiFingerprint/default` when the HAL is available.
7. Log `daemon=ready`.

The daemon registers these services:

| Service | Direction | Purpose |
| --- | --- | --- |
| `android.hardware.biometrics.fingerprint.IFingerprint/default` | provided | Android fingerprint HAL |
| `vendor.xiaomi.hardware.fingerprintextension.IXiaomiFingerprint/default` | provided | Xiaomi `extCmd` |
| `vendor.oplus.hardware.displaypanelfeature.IDisplayPanelFeature/default` | provided when absent; consumed when present | Oplus panel features and boot-time fallback |
| `vendor.xiaomi.hw.touchfeature.ITouchFeature/default` | consumed | touch mode and authentication state |

## Fingerprint start authentication

Authentication starts through `ISession.authenticate()` or through an Oplus
preparation command:

```text
ISession.authenticate
  -> Job::Authenticate
  -> OplusFingerprint::on_operation_start(Auth)
       operation = 2
       auth_done = false
       listener = true
       flush a pending touch-down as 1201
       arm()
         -> FodEngine::on_ext_cmd(4, 1)
         -> FodEngine::on_ext_cmd(1, 1)
  -> HAL.authenticate(operationId)
```

`arm()` enables the FOD/HAL mode and the gxzw touch gate, then logs `fod=on`.
The touch mode is recalculated by `fod.rs`; it is sent through the vendor
TouchFeature service as mode `10`, or mode `35` for values `5` and `6`.

These Oplus commands also prepare the path:

| Command | Meaning | Effect |
| ---: | --- | --- |
| `1005` | authenticate type | clear auth state and `arm()` |
| `1008` | touch listener | set `listener = true` and `arm()` |
| `1011` | screen state | update screen state and `arm()` |
| `1024` | post-auth | log `postauth=unlock` |

When a real FOD press is detected, `fod.rs` performs the hardware sequence:

1. Require HAL state `1`, `3`, or `5`.
2. Turn on local HBM using the selected level.
3. Send HAL `extCmd 1 = 1`.
4. Send the touch coordinates through `extCmd 2` and `extCmd 3`.
5. Set TouchFeature authentication state to `0` using mode `41`.
6. Call `OplusFingerprint::on_touch_down()`.
7. Send callback transaction `1001`, command `1201` (`TOUCH_DOWN`).

The ColorOS UI uses the `1201` notification to show the fingerprint
authentication overlay and start its touch animation.

## Fingerprint stop authentication

The hide path depends on the result:

| Event | Behavior |
| --- | --- |
| authentication success, non-zero enrollment id | `auth_done = true`, `listener = false`, `disarm()` |
| authentication failure, enrollment id `0` | turn HBM off only; do not disarm yet |
| cancellation | reset FOD state and `disarm()` |
| session close | reset FOD state and `disarm()` |
| client death or HAL error | reset FOD state and `disarm()` |

`disarm()` sends:

```text
TouchFeature setModeValue(0, 10, 0)
FodEngine extCmd 4 = 0
FodEngine extCmd 1 = 0
log "fod=off"
```

After a successful authentication the listener is already disabled, so a
later finger-up does not send another `1202`. If the finger is lifted before
the authentication result, `on_touch_up()` sends transaction `1001`, command
`1202` (`TOUCH_UP`), and the ColorOS overlay receives the touch-up state.

A failed authentication deliberately leaves the session armed. The framework
can retry, close the session, or show the keyguard response; the next
cancel/close path performs the full disarm.

## Start enrollment and stop enrollment

Enrollment starts through `ISession.enroll(hat)`:

```text
ISession.enroll
  -> Job::Enroll
  -> OplusFingerprint::on_operation_start(Enroll)
       operation = 1
       auth_done = false
       listener = true
       flush a pending touch-down as 1201
       arm()
  -> HAL.enroll(token)
```

Enrollment stops through cancellation, session close, client death, or a HAL
error. Each stop path resets the FOD engine and calls `disarm()`.

Enrollment progress arrives as legacy notification type `3`:

```text
data[0..4] = enrollment_id (u32 little-endian)
data[4..8] = remaining     (u32 little-endian)
```

The daemon forwards `onEnrollmentProgress(enrollment_id, remaining)` to the
framework and turns local HBM off. A progress value of `remaining == 0` also
moves the global status back to idle. The session is still closed by the
framework, and that close performs the full `disarm()`.

## Screen-off fingerprint

The screen-off path is enabled only when:

```text
ro.hardware.fp.fod.touch.ctl.version = 2.0
```

At startup the FOD engine opens `/dev/mi_display/disp_feature`, turns local HBM
off, and starts three monitors:

- display and brightness events;
- `/sys/class/touch/touch_dev/fod_press_status`;
- the touch input evdev device for coordinates only.

Display and brightness events are registered with:

```c
ioctl(fd, 0x400c4407, { u64 cookie; u32 type })
```

using cookies `0` and `1 << 32`, and types `2` (display) and `5` (brightness).
A capture starts when:

```text
value != 0 && ((value & 3) == 3 || (value & 8) != 0)
```

and low brightness is reported when `(value & 4) != 0`. On capture start the
daemon sends the stored FOD coordinates and HAL `extCmd 10` with `6` for low
brightness or `1` otherwise. On capture stop it sends `extCmd 10 = 0`.

The authoritative press source is `fod_press_status`. It is polled every 20 ms,
and any value other than `'0'` is treated as pressed. Evdev is used only for
`ABS_MT_POSITION_X` and `ABS_MT_POSITION_Y`; `BTN_TOUCH` is a debug flag and
never starts authentication. The device is opened read-only and is never
grabbed.

Screen state arrives through Oplus command `1011`: `0` is screen off, `1` is
screen on, and `2` is doze. The bridge updates the state, resets authentication
when leaving screen-on, clears pending touch state, and re-arms the FOD path.

Local HBM is sent through ioctl `0x400c440e`:

| Value | State |
| ---: | --- |
| `0` | off |
| `1` | off for authentication stop |
| `2` | normal 1000-nit class |
| `3` | low-light level |
| `4` | mode-5 level |

HBM selection on finger-down is:

| Condition | Level |
| --- | ---: |
| `fp_mode == 3` and brightness `<= 411` | `3` |
| `fp_mode == 5` | `4` |
| other eligible modes (`1`, `3`) | `2` |

Eligible modes are `1`, `3`, and `5`; the check is:

```text
fp_mode < 6 && (1 << fp_mode & 0x2a) != 0
```

## ColorOS FOD animation (OnFpTouch)

The daemon does not draw the fingerprint animation and does not call
`OnFpTouch` directly. It supplies the ColorOS framework with the touch state
that drives `OnScreenFingerprintUiMech.onFpTouch(boolean)`.

The path is:

```text
FodEngine detects finger-down
  -> local HBM on
  -> HAL extCmd 1 / 2 / 3
  -> TouchFeature auth state 0
  -> OplusFingerprint::on_touch_down()
  -> callback transaction 1001, command 1201

FingerprintServiceProviderExtImpl.handleOnFingerprintCmd(1201)
  -> dispatchTouchDown()
  -> ColorOS OnFpTouch touch-down state

FodEngine detects finger-up
  -> HAL coordinates and extCmd 1 = 0
  -> local HBM off
  -> OplusFingerprint::on_touch_up()
  -> callback transaction 1001, command 1202

FingerprintServiceProviderExtImpl.handleOnFingerprintCmd(1202)
  -> dispatchTouchUp()
  -> ColorOS OnFpTouch touch-up state
```

`OnScreenFingerprintUiMech.onFpTouch(true)` sets `isTouchDownNow`, schedules
the touch animation, and can request display power while the device is
dreaming. On the up/hide path `hideUdfpsOverlay$1()` logs
`hideUdfpsOverlay onFpTouchUp` and calls `onFpTouchUp()` on the dream policy.
The visual animation remains entirely a ColorOS SystemUI concern.

## HBM disable on authentication failure or acquisition/enrollment progress

HBM is turned off immediately by `FodEngine::light_off()` on these events:

| Event | Hook | Reason |
| --- | --- | --- |
| raw acquired code `1..=5` | `on_acquired(code)` | `acquired` |
| enrollment progress | `on_enroll_progress()` | `enroll-progress` |
| authentication failure (`enrollment_id == 0`) | `on_auth_failed()` | `auth-failed` |
| vendor UI-ready timeout code `40` | `handle_ui_ready_timeout()` | HBM off |
| `extCmd 4` mode `0`, `2`, or `4` | `on_ext_cmd(4, ...)` | authentication stop |
| reset/cancel/close | `reset()` | full FOD reset |

`light_off()` only acts when `hbm_target` or `hbm_on` is set. It sends HBM
value `0`, clears both state flags, and logs one of:

```text
hbm=off reason=acquired
hbm=off reason=auth-failed
hbm=off reason=enroll-progress
```

It does not disarm touch mode, clear coordinates, or close the session. Full
cleanup remains in `FodEngine::reset()` and the Oplus `disarm()` path.

Exact scoping:

- `on_acquired()` only handles raw codes `1..=5`. Raw `0` is not included, and
  vendor acquired codes `>= 1000` are handled by the vendor/anti-mistouch path.
- `on_enroll_progress()` runs after the idle guard, so progress without an
  active global operation cannot turn off unrelated HBM.
- `on_auth_failed()` runs before the anti-mistouch suppression check, so a
  failed authentication always removes HBM.

## Oplus dimmfeature

The Oplus panel fallback prevents the boot-time dim-layer race. When
`IDisplayPanelFeature/default` is absent, the daemon registers a minimal
service that answers feature `211` with `0x410` (the UDFPS type used by the
ColorOS keyguard). It also handles HBM feature `22`, color calibration `179`,
AOD smooth capability `217`, and AOD compatibility `28`. If the real service is
already present, it is prewarmed instead. The fallback log is:

```text
panel prewarm=unavailable
panel svc=registered
```

The Oplus `sendFingerprintCmd` path uses transaction `1019`. Supported commands
are `1005`, `1008`, `1010` (`-1`), `1011`, and `1024`; other commands are
acknowledged and logged. Xiaomi `extCmd` conditions `1..8` drive the FOD state.
While the FOD engine is enabled, `extCmd 10` is generated internally and is not
forwarded from the extension service. The remaining Xiaomi commands are `11`
(OCP control), `12` (forwarded to the HAL), `13` (clear failed attempts), `14`
(anti-mistouch), `15` (low-brightness authentication), and `17` (add a failed
attempt).

Anti-mistouch uses vendor acquired code `28`; a matching failure is suppressed
after HBM is handled. Vendor code `40` is the UI-ready timeout. Lockout is five
failed attempts for 30 seconds, or twenty attempts for permanent lockout.

Current boundaries: one global callback and one active session; the legacy
vendor HAL is the only fingerprint backend; the FOD path targets the optical
`peridot` layout; Quick Launch and the FOD animation are SystemUI features, not
daemon UI. Enrollment pause/continue commands `1006` and `1007` are not
implemented and are treated as unknown commands.

Use this implementation only on devices you own and can recover. It talks
directly to a privileged Android fingerprint HAL.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
