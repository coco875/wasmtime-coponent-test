#!/bin/bash
# Build this JavaScript component
cd "$(dirname "${BASH_SOURCE[0]}")"

# Check if jco is installed
if ! command -v jco &> /dev/null; then
    echo "✗ jco is not installed. Install with: npm install -g @bytecodealliance/jco"
    exit 1
fi

# Build with jco
jco componentize app.js --wit world.wit --world-name example --out js-component.wasm

if [ -f "js-component.wasm" ]; then
    mv js-component.wasm ../../loader/js-component.wasm
    echo "✓ js-component built"
else
    echo "✗ Build failed"
    exit 1
fi
