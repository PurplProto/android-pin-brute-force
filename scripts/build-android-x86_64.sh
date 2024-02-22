#!/usr/bin/env bash
rustup target add x86_64-linux-android

echo "Building for Android x86_64."
export RUSTFLAGS=""
cargo cargo ndk -t x86_64-linux-android -p 33 build --release

echo "Find the binary at target/x86_64-linux-android/release/apbf"
