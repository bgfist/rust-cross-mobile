#!/usr/bin/env bash

arch=aarch64
outdir=ports/ohos/types

export CC="$HOME/${arch}-unknown-linux-ohos-clang.sh"
export CXX="$HOME/${arch}-unknown-linux-ohos-clang++.sh"
export AR="$HOME/Library/Huawei/Sdk/openharmony/9/native/llvm/bin/llvm-ar"
export RANLIB="$HOME/Library/Huawei/Sdk/openharmony/9/native/llvm/bin/llvm-ranlib"

npx napi build $outdir --target ${arch}-unknown-linux-ohos --release
mv $outdir/index.node $outdir/../libs/arm64-v8a/libjk_core_question.so

arch=x86_64
export CC="$HOME/${arch}-unknown-linux-ohos-clang.sh"
export CXX="$HOME/${arch}-unknown-linux-ohos-clang++.sh"
export AR="$HOME/Library/Huawei/Sdk/openharmony/9/native/llvm/bin/llvm-ar"
export RANLIB="$HOME/Library/Huawei/Sdk/openharmony/9/native/llvm/bin/llvm-ranlib"

npx napi build $outdir --target ${arch}-unknown-linux-ohos --release
mv $outdir/index.node $outdir/../libs/x86_64/libjk_core_question.so