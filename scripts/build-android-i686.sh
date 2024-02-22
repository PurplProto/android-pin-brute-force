#!/usr/bin/env bash
rustup target add i686-linux-android

echo "Building for Android i686"
export RUSTFLAGS="-C target-feature=+crt-static"
cargo cargo ndk -t i686-linux-android -p 33 build --release

echo "Find the binary at target/i686-linux-android/release/apbf"
