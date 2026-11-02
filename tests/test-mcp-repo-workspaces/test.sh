#!/bin/bash
# Native TOML workspaces and real local Git remotes; no Repo/Python bootstrap.
# Acceptance: Central branch selection, complete snapshots, owning-repository
# edits/commits, dependency-first pushes, and coherent Explorer publication.
set -uo pipefail
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
status=0

if [[ "${REQVIRE_REPO_CHECKS:-all}" == composition ]]; then
    source "$TEST_SCRIPT_DIR/check_composition.sh"
    run_composition_cases
    diff -u "$TEST_SCRIPT_DIR/expected/composition-checks.txt" "$TEST_DIR/output/composition-checks.txt" || status=1
    exit "$status"
fi

source "$TEST_SCRIPT_DIR/check_repo_workspaces.sh"
run_repo_workspace_cases
if [[ "${REQVIRE_REPO_CHECKS:-all}" == workspace ]]; then
    diff -u <(head -n 15 "$TEST_SCRIPT_DIR/expected/checks.txt") "$TEST_DIR/output/checks.txt" || status=1
    exit "$status"
fi
diff -u "$TEST_SCRIPT_DIR/expected/checks.txt" "$TEST_DIR/output/checks.txt" || status=1
source "$TEST_SCRIPT_DIR/check_change_impact.sh"
run_impact_cases
diff -u "$TEST_SCRIPT_DIR/expected/impact-checks.txt" "$TEST_DIR/output/impact-checks.txt" || status=1
source "$TEST_SCRIPT_DIR/check_composition.sh"
run_composition_cases
diff -u "$TEST_SCRIPT_DIR/expected/composition-checks.txt" "$TEST_DIR/output/composition-checks.txt" || status=1
exit "$status"
