#!/bin/bash

# Script to build Java component with TeaVM

set -e

# Project directory
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$DIR"

echo "Building Java component with TeaVM..."

# Build with Gradle
./gradlew generateWasmGC

# Copy output
if [ -f "build/generated/teavm/wasm-gc/java-component.wasm" ]; then
    echo "✓ Build successful!"
    echo "  Output: build/generated/teavm/wasm-gc/java-component.wasm"
else
    echo "✗ Build failed!"
    exit 1
fi
