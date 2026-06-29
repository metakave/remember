#!/bin/bash

# Exit on error
set -e

# Configure local writeable Cargo and Rustup paths for Vercel sandbox
export RUSTUP_HOME="/vercel/.rustup"
export CARGO_HOME="/vercel/.cargo"
export PATH="$CARGO_HOME/bin:/rust/bin:$PATH"

echo "=== System Information ==="
echo "User: $(whoami)"
echo "Home: $HOME"
echo "Env Path: $PATH"

echo "=== Checking existing Rust installation ==="
if command -v rustc &> /dev/null; then
    echo "rustc version: $(rustc --version)"
    echo "cargo version: $(cargo --version)"
fi

# Install rustup locally (overrides the global system read-only Rust if it exists)
echo "=== Installing local rustup ==="
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path

echo "=== Adding wasm32 compilation target ==="
rustup target add wasm32-unknown-unknown

echo "=== Downloading Trunk binary ==="
curl -L https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf -

echo "=== Building Frontend with Trunk ==="
./trunk build --release --working-directory frontend

echo "=== Frontend Build Successful! ==="
