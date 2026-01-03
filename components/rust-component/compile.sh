#!/bin/bash
# Build this component
cd "$(dirname "${BASH_SOURCE[0]}")"
cargo component build "$@"

# Copy to loader
if [ "$1" = "--release" ]; then
    cp target/wasm32-wasip1/release/rust_component.wasm ../../loader/rust-component.wasm
else
    cp target/wasm32-wasip1/debug/rust_component.wasm ../../loader/rust-component.wasm
fi
echo "✓ rust-component built"