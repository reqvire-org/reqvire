#!/bin/bash
set -euo pipefail
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
bash "$TEST_SCRIPT_DIR/check_build.sh" "$TEST_DIR/output"
diff -u "$TEST_SCRIPT_DIR/expected/checks.txt" "$TEST_DIR/output/checks.txt"
bash "$TEST_SCRIPT_DIR/check_pipeline.sh" "$TEST_DIR/output/pipeline"
diff -u "$TEST_SCRIPT_DIR/expected/pipeline-checks.txt" "$TEST_DIR/output/pipeline/checks.txt"
