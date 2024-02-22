#!/usr/bin/env bash
rustup target add aarch64-linux-android

echo "Building for Android ARM64."
export RUSTFLAGS="-C target-feature=+crt-static"
cargo ndk -t aarch64-linux-android -p 33 build --release

echo "Find the binary at target/aarch64-linux-android/release/apbf"
