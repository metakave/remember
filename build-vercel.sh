#!/bin/bash

# Exit on error
set -e

echo "=== Installing Rust toolchain ==="
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

echo "=== Adding wasm32 compilation target ==="
rustup target add wasm32-unknown-unknown

echo "=== Downloading Trunk binary ==="
curl -L https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf -

echo "=== Building Frontend with Trunk ==="
./trunk build --release --working-directory frontend

echo "=== Frontend Build Successful! ==="
