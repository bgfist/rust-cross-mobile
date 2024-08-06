#!/usr/bin/env bash

arch=aarch64
target=aarch64-linux-android
outdir=./ports/android

cargo ndk -t arm64-v8a -o $outdir/jniLibs build --release
cargo run --bin uniffi_bindgen generate --library ./target/$target/release/libjk_core_question.so --language kotlin --out-dir $outdir