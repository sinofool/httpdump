#!/bin/bash

cargo clean && cargo build --target wasm32-wasi --release || exit 1
cp target/wasm32-wasi/release/authfilter.wasm $NGINX_HTML/authfilter.wasm || exit 2

SHA=$(sha256sum -b target/wasm32-wasi/release/authfilter.wasm)
sed -i "s/sha256:.*$/sha256: ${SHA:0:64}/" authfilter.yaml || exit 3

kubectl apply -f authfilter.yaml

