NDK_DIR="${NDK_DIR:=`pwd`/android-ndk-r23b}"

mkdir -p .cargo
echo "[target.aarch64-linux-android]" > .cargo/config
echo "ar = \"$NDK_DIR/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar\"" >> .cargo/config
echo "linker = \"$NDK_DIR/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android30-clang\"" >> .cargo/config

if ! [ -d $NDK_DIR ]
then
	curl -o ndk.zip https://dl.google.com/android/repository/`basename $NDK_DIR`-linux.zip
	unzip ndk.zip
fi
set -x
export PATH="$NDK_DIR/toolchains/llvm/prebuilt/linux-x86_64/bin/:$PATH"
RUSTFLAGS="-L $NDK_DIR/toolchains/llvm/prebuilt/linux-x86_64/lib64/clang/12.0.8/lib/linux/aarch64/ -L deps/ -L $NDK_DIR/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/30" cargo build --target=aarch64-linux-android --release # -Zbuild-std

