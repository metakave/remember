#!/bin/bash

# Exit on error
set -e

# Configure local writeable Cargo and Rustup paths for Vercel sandbox
export RUSTUP_HOME="/vercel/.rustup"
export CARGO_HOME="/vercel/.cargo"
export PATH="$CARGO_HOME/bin:$PATH"
export RUSTUP_INIT_SKIP_PATH_CHECK="yes"

echo "=== System Information ==="
ARCH=$(uname -m)
echo "Architecture: $ARCH"
echo "User: $(whoami)"
echo "Home: $HOME"

# Install rustup locally (forces setup of stable toolchain in writeable directory)
echo "=== Installing local rustup ==="
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path --default-toolchain stable

# Source cargo env to load rustc/cargo into the path
if [ -f "$CARGO_HOME/env" ]; then
    source "$CARGO_HOME/env"
fi

echo "=== Verifying Rust Installation ==="
rustc --version
cargo --version

echo "=== Adding wasm32 compilation target ==="
rustup target add wasm32-unknown-unknown

echo "=== Downloading Trunk binary ==="
if [ "$ARCH" = "x86_64" ]; then
    TRUNK_URL="https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz"
elif [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
    TRUNK_URL="https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-aarch64-unknown-linux-gnu.tar.gz"
else
    echo "Unsupported architecture: $ARCH"
    exit 1
fi

curl -L "$TRUNK_URL" | tar -xzf -

echo "=== Building Frontend with Trunk ==="
./trunk build --release --working-directory frontend

echo "=== Frontend Build Successful! ==="
