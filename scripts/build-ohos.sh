#!/usr/bin/env bash

arch=aarch64

export CC="$HOME/${arch}-unknown-linux-ohos-clang.sh"
export CXX="$HOME/${arch}-unknown-linux-ohos-clang++.sh"
export AR="$HOME/Library/Huawei/Sdk/openharmony/9/native/llvm/bin/llvm-ar"
export RANLIB="$HOME/Library/Huawei/Sdk/openharmony/9/native/llvm/bin/llvm-ranlib"

cargo build --target ${arch}-unknown-linux-ohos --release