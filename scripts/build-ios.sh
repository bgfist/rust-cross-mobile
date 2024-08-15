#!/usr/bin/env bash

mode=debug

if [ $# -eq 0 ]; then
    echo "[Build Mode]: Debug"
elif [ $# -eq 1 ]; then
    if [ "$1" = "--release" ]; then
        mode=release
        echo "[Build Mode]: Release"
    else
        echo "Error: The argument must be --release"
        exit 1
    fi
else
    echo "Error: Too many arguments. Only --release is accepted"
fi

outdir=./ports/ios
libname=jk_core_question
static_libname=lib$libname.a
dy_libname=lib$libname.dylib

arch=aarch64
target=${arch}-apple-ios
cargo build --target $target $1
# 生成的静态文件strip后会小很多
strip ./target/$target/$mode/$static_libname

arch2=aarch64
target2=${arch2}-apple-ios-sim
cargo build --target $target2 $1
# 生成的静态文件strip后会小很多
strip ./target/$target2/$mode/$static_libname

arch3=x86_64
target3=${arch3}-apple-ios
cargo build --target $target3 $1
# 生成的静态文件strip后会小很多
strip ./target/$target3/$mode/$static_libname

framework_path=$outdir/JkCoreQuestion/RustFramework.xcframework
universal_path=$framework_path/ios-arm64_x86_64-simulator/$static_libname
lipo ./target/$target2/$mode/$static_libname ./target/$target3/$mode/$static_libname -create -output $universal_path

if [ "$mode" = "release" ]; then
    # release模式下把符号都strip掉了，没法生成胶水代码，这里用debug模式生成吧
    cargo build --target $target
fi
cargo run --bin uniffi_bindgen generate --library ./target/$target/debug/$dy_libname --language swift --out-dir $outdir

outdir_h=$outdir/${libname}FFI.h
outdir_modulemap=$outdir/${libname}FFI.modulemap
outdir_swift=$outdir/${libname}.swift

cp $outdir_h $framework_path/ios-arm64/Headers/
cp $outdir_modulemap $framework_path/ios-arm64/Headers/module.modulemap
cp ./target/$target/$mode/$static_libname $framework_path/ios-arm64/

cp $outdir_h $framework_path/ios-arm64_x86_64-simulator/Headers/
cp $outdir_modulemap $framework_path/ios-arm64_x86_64-simulator/Headers/module.modulemap

cp $outdir_swift $framework_path/../Sources/JkCoreQuestion/

# clean
rm $outdir_h $outdir_modulemap $outdir_swift
