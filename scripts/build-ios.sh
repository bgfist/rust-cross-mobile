#!/usr/bin/env bash

arch=aarch64
target=${arch}-apple-ios
outdir=./ports/ios

cargo build --target $target --release
cargo run --bin uniffi_bindgen generate --library ./target/$target/release/libjk_core_question.a --language swift --out-dir $outdir
cp ./target/$target/release/libjk_core_question.a $outdir