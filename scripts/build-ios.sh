#!/usr/bin/env bash

arch=aarch64
target=${arch}-apple-ios
outdir=./ports/ios

# release模式下把符号都strip掉了，没法生成胶水代码，这里用debug模式生成吧
cargo build --target $target
cargo run --bin uniffi_bindgen generate --library ./target/$target/debug/libjk_core_question.a --language swift --out-dir $outdir

cargo build --target $target --release
cp ./target/$target/release/libjk_core_question.a $outdir

# 生成的静态文件strip后会小很多
strip ./target/$target/release/libjk_core_question.a