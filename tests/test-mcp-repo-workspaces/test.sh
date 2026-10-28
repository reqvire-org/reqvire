#!/bin/bash
# Real Repo clients and local Git remotes. Tool bootstrap is isolated in TEST_DIR;
# feature checks use only local fixtures and never depend on another checkout.
# Acceptance: Central branch selection, complete snapshots, owning-repository
# edits/commits, dependency-first pushes, and coherent Explorer publication.
set -uo pipefail
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
status=0

prepare_repo_tool() {
    local destination="$TEST_DIR/repo-tool"
    git clone -q --branch v2.65 --single-branch \
        https://gerrit.googlesource.com/git-repo "$destination" || return
    export PATH="$destination:$PATH"
}

if ! prepare_repo_tool; then
    echo 'Failed to prepare the temporary Repo tool fixture.' >&2
    exit 2
fi

python3 "$TEST_SCRIPT_DIR/check_repo_workspaces.py" "$REQVIRE_BIN" "$TEST_DIR" > "$TEST_DIR/output/checks.txt" || status=1
source "$TEST_SCRIPT_DIR/check_native_repo.sh"
run_native_repo_cases
diff -u "$TEST_SCRIPT_DIR/expected/checks.txt" "$TEST_DIR/output/checks.txt" || status=1
exit "$status"
