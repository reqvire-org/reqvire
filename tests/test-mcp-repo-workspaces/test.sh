#!/bin/bash
# Real Repo clients and local Git remotes; no network is used by this suite.
# Set REQVIRE_TEST_REPO_SOURCE to an installed Git checkout of Google's git-repo.
# Acceptance: Central branch selection, complete snapshots, owning-repository
# edits/commits, dependency-first pushes, and coherent Explorer publication.
set -uo pipefail
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
status=0
python3 "$TEST_SCRIPT_DIR/check_repo_workspaces.py" "$REQVIRE_BIN" "$TEST_DIR" > "$TEST_DIR/output/checks.txt" || status=1
diff -u "$TEST_SCRIPT_DIR/expected/checks.txt" "$TEST_DIR/output/checks.txt" || status=1
exit "$status"
