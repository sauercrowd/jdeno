NDK_DIR=android-ndk-r23b
if ! [ -d $NDK_DIR ]
then
	curl -o ndk.zip https://dl.google.com/android/repository/$NDK_DIR-linux.zip
	unzip ndk.zip
fi
set -x
export PATH="$NDK_DIR/toolchains/llvm/prebuilt/linux-x86_64/bin/:$PATH"
export CC=aarch64-linux-android30-clang
RUSTFLAGS="-L $NDK_DIR/toolchains/llvm/prebuilt/linux-x86_64/lib64/clang/12.0.8/lib/linux/aarch64/ -L deps/ -L $NDK_DIR/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/30" cargo build --target=aarch64-linux-android --release # -Zbuild-std


