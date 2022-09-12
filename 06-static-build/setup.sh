#! /bin/bash

rustup target add aarch64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-musl
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-musl

rustup update

pip3 install ziglang
cargo install cargo-zigbuild

if [[ "$OSTYPE" == "darwin"* ]]; then
    brew install FiloSottile/musl-cross/musl-cross
fi
