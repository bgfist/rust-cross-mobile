#!/usr/bin/env bash

arch=aarch64
target=aarch64-linux-android
outdir=./ports/android

# release模式下把符号都strip掉了，没法生成胶水代码，这里用debug模式生成吧
cargo ndk -t arm64-v8a -o $outdir/jniLibs build
cargo run --bin uniffi_bindgen generate --library ./target/$target/debug/libjk_core_question.so --language kotlin --out-dir $outdir

cargo ndk -t arm64-v8a -o $outdir/jniLibs build --release
