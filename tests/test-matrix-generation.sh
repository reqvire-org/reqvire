#!/bin/bash

# Test script for the traceability matrix generation functionality
set -e

# Get the root directory of the repository
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Build the reqflow binary
echo "Building reqflow binary..."
cd "$ROOT_DIR"
cargo build

# Create a temporary directory for the test
TEST_DIR="$ROOT_DIR/tests/test-matrix"
mkdir -p "$TEST_DIR/output"

# Generate markdown traceability matrix
echo "Generating markdown traceability matrix..."
"$ROOT_DIR/target/debug/reqflow" --generate-matrix

# Generate JSON traceability matrix
echo "Generating JSON traceability matrix..."
"$ROOT_DIR/target/debug/reqflow" --generate-matrix --json

echo "Traceability matrix generation test completed successfully!"