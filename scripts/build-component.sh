#!/bin/bash

# Script to build a specific Rust component

set -e

# Project root directory
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Arguments
COMPONENT_NAME="${1:-rust-component}"
MODE="${2:-debug}"

echo -e "${YELLOW}=== Building $COMPONENT_NAME ===${NC}"
echo -e "Mode: $MODE"
echo ""

COMPONENT_DIR="$ROOT_DIR/components/$COMPONENT_NAME"

if [ ! -d "$COMPONENT_DIR" ]; then
    echo -e "${RED}Error: Component $COMPONENT_NAME does not exist${NC}"
    echo "Available components:"
    ls -1 "$ROOT_DIR/components/"
    exit 1
fi

cd "$COMPONENT_DIR"

# Detect component type and build accordingly
if [ -f "Cargo.toml" ]; then
    # Rust component
    cargo_name=$(grep '^name = ' Cargo.toml | head -1 | sed 's/name = "\(.*\)"/\1/' | tr '-' '_')
    
    if [ "$MODE" = "release" ]; then
        cargo component build --release
        WASM_PATH="target/wasm32-wasip1/release/${cargo_name}.wasm"
    else
        cargo component build
        WASM_PATH="target/wasm32-wasip1/debug/${cargo_name}.wasm"
    fi
    
    # Determine output name
    case "$COMPONENT_NAME" in
        "test1-component") OUTPUT_NAME="test1" ;;
        "test2-component") OUTPUT_NAME="test2" ;;
        *) OUTPUT_NAME="$COMPONENT_NAME" ;;
    esac
    
    if [ -f "$WASM_PATH" ]; then
        cp "$WASM_PATH" "$ROOT_DIR/loader/${OUTPUT_NAME}.wasm"
        echo -e "${GREEN}✓${NC} Built: loader/${OUTPUT_NAME}.wasm"
    else
        echo -e "${RED}✗${NC} Error: Wasm file not found"
        exit 1
    fi

elif [ -f "app.js" ]; then
    # JavaScript component
    if ! command -v jco &> /dev/null; then
        echo -e "${RED}✗${NC} jco is not installed. Install with: npm install -g @bytecodealliance/jco"
        exit 1
    fi
    
    jco componentize app.js --wit world.wit --world-name example --out "${COMPONENT_NAME}.wasm"
    mv "${COMPONENT_NAME}.wasm" "$ROOT_DIR/loader/${COMPONENT_NAME}.wasm"
    echo -e "${GREEN}✓${NC} Built: loader/${COMPONENT_NAME}.wasm"

elif [ -f "app.py" ]; then
    # Python component
    if ! command -v componentize-py &> /dev/null; then
        echo -e "${RED}✗${NC} componentize-py is not installed. Install with: pip install componentize-py"
        exit 1
    fi
    
    componentize-py -d world.wit -w example componentize app -o "${COMPONENT_NAME}.wasm"
    mv "${COMPONENT_NAME}.wasm" "$ROOT_DIR/loader/${COMPONENT_NAME}.wasm"
    echo -e "${GREEN}✓${NC} Built: loader/${COMPONENT_NAME}.wasm"

else
    echo -e "${RED}✗${NC} Unknown component type"
    exit 1
fi
