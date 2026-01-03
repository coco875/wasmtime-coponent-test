#!/bin/bash

# Script to generate bindings from WIT files

set -e

# Project root directory
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Generating WIT bindings ===${NC}"
echo ""

# Rust components
RUST_COMPONENTS=(
    "rust-component"
    "test1-component"
    "test2-component"
)

for component in "${RUST_COMPONENTS[@]}"; do
    COMPONENT_DIR="$ROOT_DIR/components/$component"
    
    if [ -d "$COMPONENT_DIR" ]; then
        echo -e "${YELLOW}→${NC} Generating bindings for $component..."
        cd "$COMPONENT_DIR"
        
        if cargo component build 2>/dev/null; then
            echo -e "${GREEN}✓${NC} Bindings generated for $component"
        else
            echo -e "${RED}✗${NC} Error generating bindings for $component"
        fi
    fi
done

# Python component
PYTHON_DIR="$ROOT_DIR/components/python-component"
if [ -d "$PYTHON_DIR" ]; then
    echo -e "${YELLOW}→${NC} Generating bindings for python-component..."
    cd "$PYTHON_DIR"
    
    if command -v componentize-py &> /dev/null; then
        componentize-py -d world.wit -w example bindings . 2>/dev/null && \
            echo -e "${GREEN}✓${NC} Bindings generated for python-component" || \
            echo -e "${RED}✗${NC} Error generating bindings for python-component"
    else
        echo -e "${YELLOW}⚠${NC} componentize-py not installed, skipping python-component"
    fi
fi

echo ""
echo -e "${GREEN}=== Generation complete ===${NC}"
