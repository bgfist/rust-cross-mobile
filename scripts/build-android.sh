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

arch=aarch64
target=aarch64-linux-android
outdir=./app/android/jk-core-question-example/lib/src/main

# release模式下把符号都strip掉了，没法生成胶水代码，这里用debug模式生成吧
cargo ndk -t arm64-v8a -o $outdir/jniLibs build
cargo run --bin uniffi_bindgen generate --library ./target/$target/debug/libjk_core_question.so --language kotlin --out-dir $outdir/java

if [ "$mode" = "release" ]; then
    cargo ndk -t arm64-v8a -o $outdir/jniLibs build --release
fi
