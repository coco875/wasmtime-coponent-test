#!/bin/bash

# Script to update WIT files for all components
# Source file is loader/wit/world.wit

set -e

# Project root directory (one level above scripts folder)
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Source WIT file
SOURCE_WIT="$ROOT_DIR/loader/wit/world.wit"

# List of WIT file destinations to update
DESTINATIONS=(
    "$ROOT_DIR/components/rust-component/wit/world.wit"
    "$ROOT_DIR/components/test1-component/wit/world.wit"
    "$ROOT_DIR/components/test2-component/wit/world.wit"
    "$ROOT_DIR/components/js-component/world.wit"
    "$ROOT_DIR/components/python-component/world.wit"
)

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo "=== Updating WIT files ==="
echo ""

# Check that source file exists
if [ ! -f "$SOURCE_WIT" ]; then
    echo -e "${RED}Error: Source file $SOURCE_WIT does not exist${NC}"
    exit 1
fi

echo -e "${YELLOW}Source file:${NC} $SOURCE_WIT"
echo ""

# Copy WIT file to each destination
for dest in "${DESTINATIONS[@]}"; do
    if [ -f "$dest" ]; then
        # File exists, update it
        cp "$SOURCE_WIT" "$dest"
        echo -e "${GREEN}✓${NC} Updated: $dest"
    else
        # Create directory if needed
        mkdir -p "$(dirname "$dest")"
        cp "$SOURCE_WIT" "$dest"
        echo -e "${GREEN}✓${NC} Created: $dest"
    fi
done

echo ""
echo -e "${GREEN}=== Update complete ===${NC}"
