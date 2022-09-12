# Static builds

Creating a "static binary" makes it nearly trivial to deploy an application to
any Linux host. Your program will be stand-alone and not require any runtime 
dependencies.

Cross-compilation is somewhat of a dark art, though. 

## Setup

Setting means running the `setup.sh` script. It will download the standard 
library and for musl libc on AMD64/x86_64 and ARM8/aarch64.

If you're running Linux, you may need to install extra packages.


## Linker

To compile to other platforms, we need a linker. This is a program that knows
how to weave the symbols from each of our crates together, using the platform's
conventions so that they can be loaded into RAM when the program is executed.

### rust-lld

If the Fates are willing, we will be able to use Rust's own linker (`rust-lld`)
to do this work.

You also need to provide some configuration to `cargo`, by adding these sections
to `.cargo/config`. 

```toml
[target.x86_64-unknown-linux-musl]
linker = "rust-lld"
rustflags = ["-C", "target-feature=+crt-static"]

[target.aarch64-unknown-linux-musl]
linker = "rust-lld"
rustflags = ["-C", "target-feature=+crt-static"]
```

### GCC

On Linux, you'll require a toolchain for ARM.

For Debian-based distributions, incl. Ubuntu:

```bash
sudo apt-get install gcc-aarch64-linux-gnu
```

Your cargo config (`.cargo/config.toml`) becomes this:

```toml
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
rustflags = ["-C", "target-feature=+crt-static"]

[target.aarch64-unknown-linux-musl]
linker = "aarch64-linux-musl-gcc"
rustflags = ["-C", "target-feature=+crt-static"]
```