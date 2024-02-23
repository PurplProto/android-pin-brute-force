#!/usr/bin/env bash

echo "Building for Linux x64."
echo -e "\e[31mThis is only for debugging purposes, it is not expected to run on this platform.\e[0m"

cargo build --target x86_64-unknown-linux-gnu

echo "Find the DEBUG binary at target/x86_64-unknown-linux-gnu/debug/apbf"
