#! /bin/bash

function build {
    cargo build \
        --release \
        --target "$1"-unknown-linux-musl
}

function zigbuild {
    cargo zigbuild \
        --release \
        --target "$1"-unknown-linux-gnu
}

set -x

build aarch64
build x86_64
zigbuild aarch64
zigbuild x86_64