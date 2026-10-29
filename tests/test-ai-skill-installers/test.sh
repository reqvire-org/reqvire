#!/usr/bin/env bash
set -euo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$TEST_SCRIPT_DIR/../.." && pwd)"
WORK_DIR="${TEST_DIR:-$(mktemp -d)}"


# Pin the external installer for reproducible package acceptance checks.
bash "$TEST_SCRIPT_DIR/check_skills.sh" "$REPO_ROOT" "$WORK_DIR"
