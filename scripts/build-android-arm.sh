#!/usr/bin/env bash
rustup target add armv7-linux-androideabi

echo "Building for Android ARM."
export RUSTFLAGS="-C target-feature=+crt-static"
cargo cargo ndk -t armv7-linux-androideabi -p 33 build --release

echo "Find the binary at target/armv7-linux-androideabi/release/apbf"
