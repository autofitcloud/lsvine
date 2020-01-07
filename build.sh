#!/bin/bash
# https://github.com/rustwasm/wasm-pack/blob/51e6351c28fbd40745719e6d4a7bf26dadd30c85/.travis.yml#L74-L91

# clear
rm dist/* -rf
rm target/* -rf

# version
LSVINE_VERSION=v0.2.0

# ubuntu et al
TARGET=x86_64-unknown-linux-musl
rustup target add $TARGET
cargo build --release --target $TARGET --locked
tar -C target/x86_64-unknown-linux-musl/release/ -czf dist/lsvine-$LSVINE_VERSION-$TARGET.tar.gz lsvine

# mac et al (need to run on a mac, or check https://github.com/rustwasm/wasm-pack/blob/51e6351c28fbd40745719e6d4a7bf26dadd30c85/.travis.yml#L74-L91)
#MACOSX_DEPLOYMENT_TARGET=10.7
#TARGET=x86_64-apple-darwin
#rustup target add $TARGET
#cargo build --release --target $TARGET --locked
#tar -czf dist/lsvine-LSVINE_VERSION-$TARGET.tar.gz target/release/lsvine
