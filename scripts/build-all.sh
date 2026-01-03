#!/bin/bash

# Script to build all WebAssembly components

set -e

# Project root directory
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Build mode (debug or release)
MODE="${1:-debug}"

echo -e "${BLUE}=== Building all components ===${NC}"
echo -e "${YELLOW}Mode:${NC} $MODE"
echo ""

# Function to build a Rust component
build_rust_component() {
    local name="$1"
    local output_name="$2"
    local component_dir="$ROOT_DIR/components/$name"
    
    if [ ! -d "$component_dir" ]; then
        echo -e "${RED}✗${NC} Component $name not found"
        return 1
    fi
    
    echo -e "${YELLOW}→${NC} Building $name..."
    cd "$component_dir"
    
    # Read package name from Cargo.toml
    local cargo_name=$(grep '^name = ' Cargo.toml | head -1 | sed 's/name = "\(.*\)"/\1/' | tr '-' '_')
    
    if [ "$MODE" = "release" ]; then
        cargo component build --release
        local wasm_path="target/wasm32-wasip1/release/${cargo_name}.wasm"
    else
        cargo component build
        local wasm_path="target/wasm32-wasip1/debug/${cargo_name}.wasm"
    fi
    
    # Copy wasm file to loader
    if [ -f "$wasm_path" ]; then
        cp "$wasm_path" "$ROOT_DIR/loader/${output_name}.wasm"
        echo -e "${GREEN}✓${NC} $name built → loader/${output_name}.wasm"
    else
        echo -e "${RED}✗${NC} Wasm file not found: $wasm_path"
        return 1
    fi
}

# Function to build a JavaScript component
build_js_component() {
    local name="$1"
    local output_name="$2"
    local component_dir="$ROOT_DIR/components/$name"
    
    if [ ! -d "$component_dir" ]; then
        echo -e "${RED}✗${NC} Component $name not found"
        return 1
    fi
    
    echo -e "${YELLOW}→${NC} Building $name..."
    cd "$component_dir"
    
    # Check if jco is installed
    if ! command -v jco &> /dev/null; then
        echo -e "${RED}✗${NC} jco is not installed. Install with: npm install -g @bytecodealliance/jco"
        return 1
    fi
    
    # Build with jco
    jco componentize app.js --wit world.wit --world-name mod-world --out "${output_name}.wasm"
    
    if [ -f "${output_name}.wasm" ]; then
        mv "${output_name}.wasm" "$ROOT_DIR/loader/${output_name}.wasm"
        echo -e "${GREEN}✓${NC} $name built → loader/${output_name}.wasm"
    else
        echo -e "${RED}✗${NC} Failed to build $name"
        return 1
    fi
}

# Function to build a Python component
build_python_component() {
    local name="$1"
    local output_name="$2"
    local component_dir="$ROOT_DIR/components/$name"
    
    if [ ! -d "$component_dir" ]; then
        echo -e "${RED}✗${NC} Component $name not found"
        return 1
    fi
    
    echo -e "${YELLOW}→${NC} Building $name..."
    cd "$component_dir"
    
    # Check if componentize-py is installed
    if ! command -v componentize-py &> /dev/null; then
        echo -e "${RED}✗${NC} componentize-py is not installed. Install with: pip install componentize-py"
        return 1
    fi
    
    # Build with componentize-py
    componentize-py -d world.wit -w mod-world componentize app -o "${output_name}.wasm"
    
    if [ -f "${output_name}.wasm" ]; then
        mv "${output_name}.wasm" "$ROOT_DIR/loader/${output_name}.wasm"
        echo -e "${GREEN}✓${NC} $name built → loader/${output_name}.wasm"
    else
        echo -e "${RED}✗${NC} Failed to build $name"
        return 1
    fi
}

# Build Rust components
build_rust_component "rust-component" "rust-component"
build_rust_component "test1-component" "test1"
build_rust_component "test2-component" "test2"

# Build JavaScript component
build_js_component "js-component" "js-component" || true

# Build Python component
build_python_component "python-component" "python-component" || true

echo ""
echo -e "${GREEN}=== Build complete ===${NC}"
