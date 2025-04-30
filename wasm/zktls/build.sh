#! /usr/bin/env bash

set -e

wasm-pack build --target web --release || exit 1
wasm-pack pack || exit 1
mv pkg/zktls-0.1.1.tgz pkg/zktls-web.tgz

wasm-pack build --target nodejs --release || exit 1
wasm-pack pack || exit 1
mv pkg/zktls-0.1.1.tgz pkg/zktls-nodejs.tgz

wasm-pack build --target bundler --release || exit 1
wasm-pack pack || exit 1
mv pkg/zktls-0.1.1.tgz pkg/zktls-bundler.tgz
