#!/usr/bin/env bash
# Source from an owning suite and invoke with its profile and expected manifest.
run_element_selection_checks() {
    local profile="$1" expected="$2" status=0
    local selection_scripts
    selection_scripts="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    python3 "$selection_scripts/element_selection_checks.py" \
        --binary "${REAL_REQVIRE_BIN:-$REQVIRE_BIN}" \
        --workspace "$TEST_DIR" --profile "$profile" \
        > "$TEST_DIR/output/element-selection.txt" || status=1
    diff -u "$expected" "$TEST_DIR/output/element-selection.txt" || status=1
    return "$status"
}
