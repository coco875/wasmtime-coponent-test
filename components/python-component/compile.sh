#!/bin/bash
# Build this Python component
cd "$(dirname "${BASH_SOURCE[0]}")"

# Check if componentize-py is installed
if ! command -v componentize-py &> /dev/null; then
    echo "✗ componentize-py is not installed. Install with: pip install componentize-py"
    exit 1
fi

# Build with componentize-py
componentize-py -d world.wit -w example componentize app -o python-component.wasm

if [ -f "python-component.wasm" ]; then
    mv python-component.wasm ../../loader/python-component.wasm
    echo "✓ python-component built"
else
    echo "✗ Build failed"
    exit 1
fi
