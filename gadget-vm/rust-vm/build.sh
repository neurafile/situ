#!/bin/bash
# Build script for Rust VM to WASM

set -e

echo "Building Gadget VM to WASM..."

# Add cargo bin to PATH if not already there
export PATH="$HOME/.cargo/bin:$PATH"

# Ensure wasm32 target is installed
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Install wasm-bindgen-cli if not present
if ! command -v wasm-bindgen &> /dev/null; then
    echo "Installing wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli
fi

# Build
echo "Compiling Rust to WASM..."
cargo build --target wasm32-unknown-unknown --release

# Generate bindings
echo "Generating WASM bindings..."
wasm-bindgen \
    --target web \
    --out-dir ../js-bindings/wasm \
    --out-name gadget_vm \
    target/wasm32-unknown-unknown/release/gadget_vm.wasm

echo "Build complete! WASM files in js-bindings/wasm/"
