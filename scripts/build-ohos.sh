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
outdir=./app/ohos/jk_core_question_example/jk_core_question/types

export CC="$HOME/${arch}-unknown-linux-ohos-clang.sh"
export CXX="$HOME/${arch}-unknown-linux-ohos-clang++.sh"
export AR="$HOME/Library/Huawei/Sdk/openharmony/9/native/llvm/bin/llvm-ar"
export RANLIB="$HOME/Library/Huawei/Sdk/openharmony/9/native/llvm/bin/llvm-ranlib"

npx napi build $outdir --target ${arch}-unknown-linux-ohos $1
rm -f $outdir/index.node
mv ./target/${arch}-unknown-linux-ohos/$mode/libjk_core_question.so $outdir/../libs/arm64-v8a/libjk_core_question.so

# arch=x86_64
# export CC="$HOME/${arch}-unknown-linux-ohos-clang.sh"
# export CXX="$HOME/${arch}-unknown-linux-ohos-clang++.sh"
# export AR="$HOME/Library/Huawei/Sdk/openharmony/9/native/llvm/bin/llvm-ar"
# export RANLIB="$HOME/Library/Huawei/Sdk/openharmony/9/native/llvm/bin/llvm-ranlib"

# cargo build --target ${arch}-unknown-linux-ohos $1
# mv ./target/${arch}-unknown-linux-ohos/$mode/libjk_core_question.so $outdir/../libs/x86_64/libjk_core_question.so