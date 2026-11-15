NDK=/opt/android-ndk/toolchains/llvm/prebuilt/linux-x86_64

CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$NDK/bin/aarch64-linux-android34-clang" \
BINDGEN_EXTRA_CLANG_ARGS_aarch64_linux_android="--target=aarch64-linux-android34 --sysroot=$NDK/sysroot -isystem $NDK/sysroot/usr/include/c++/v1" \
cargo build --release --target aarch64-linux-android
