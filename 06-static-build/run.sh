#! /bin/bash

TARGET_CC=x86_64-linux-musl-gcc \
cargo run \
    --release \
    --target x86_64-unknown-linux-musl