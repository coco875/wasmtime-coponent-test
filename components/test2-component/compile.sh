#!/bin/bash
# Build this component
cd "$(dirname "${BASH_SOURCE[0]}")"
cargo component build "$@"

# Copy to loader (wasm name is based on Cargo package name)
if [ "$1" = "--release" ]; then
    cp target/wasm32-wasip1/release/rust_component.wasm ../../loader/test2.wasm
else
    cp target/wasm32-wasip1/debug/rust_component.wasm ../../loader/test2.wasm
fi
echo "✓ test2-component built"