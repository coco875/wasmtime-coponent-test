#!/bin/bash
set -e

# Generate bindings if not exists
if [ ! -f "mod_world.c" ]; then
    wit-bindgen c ./world.wit --out-dir .
fi

echo "Compiling C component..."

# Check which clang to use
CLANG="clang"
# You might need to specify a path to clang with wasm support if system clang doesn't have it
# CLANG="/opt/wasi-sdk/bin/clang"

# Compile to core wasm module
# We need to link against mod_world_component_type.o to embed the interface
$CLANG --target=wasm32-wasi \
    -mexec-model=reactor \
    -o main.core.wasm \
    main.c mod_world.c mod_world_component_type.o \
    -Wl,--export-dynamic

echo "Creating component..."
# Create the component from the core module
# Note: If we use libc/WASI (printf), the resulting component will import `wasi_snapshot_preview1`.
# To make it fully standalone or use standard WASI interfaces, we might need an adapter.
# For now, we just wrap it.
wasm-tools component new main.core.wasm -o c-component.wasm

if [ -f "c-component.wasm" ]; then
    echo "✓ Build successful: c-component.wasm"
else
    echo "✗ Build failed"
    exit 1
fi
