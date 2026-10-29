#!/bin/bash
# Deletion and new links must preserve unrelated evidence targets and Git state.
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
output="$TEST_DIR/output"
root="$output/rm-artifact-links"
model=system-model/Interfaces/WebExplorer/Capabilities.md
checks=system-model/Verifications/Checks.md
artifacts=(Makefile LICENSE src/evidence.rs system-model/Interfaces/WebExplorer/src/evidence.rs NOTICE)
files=("$model" "$checks" "${artifacts[@]}")
targets='["../../../LICENSE","../../../Makefile","../../../src/evidence.rs"]'
mkdir -p "$root/$(dirname "$model")" "$root/$(dirname "$checks")" "$root/src" "$root/system-model/Interfaces/WebExplorer/src"
cp "$SCRIPT_DIR/fixtures/rm-artifact-links/Capabilities.md.txt" "$root/$model"
cp "$SCRIPT_DIR/fixtures/rm-artifact-links/Checks.md.txt" "$root/$checks"
printf 'all:\n\t@echo retained artifact\n' > "$root/Makefile"
printf 'Retained fixture license.\n' > "$root/LICENSE"
printf '// Retained implementation evidence.\n' > "$root/src/evidence.rs"
printf '// Local decoy, not the evidence target.\n' > "$root/system-model/Interfaces/WebExplorer/src/evidence.rs"
printf 'Unlinked root artifact for the link regression.\n' > "$root/NOTICE"
git -C "$root" init -q
git -C "$root" config user.name 'Artifact link regression'
git -C "$root" config user.email tests@example.com
git -C "$root" remote add origin https://example.com/model.git
git -C "$root" add .
git -C "$root" -c commit.gpgSign=false commit -qm Fixture
head=$(git -C "$root" rev-parse HEAD)
cp "$root/.git/index" "$output/rm-artifact-original.index"
status=0
: > "$output/rm-artifact-links-checks.txt"

snapshot() {
    local destination=$1 file
    for file in "${files[@]}"; do
        mkdir -p "$destination/$(dirname "$file")"
        cp "$root/$file" "$destination/$file"
    done
}

same_files() {
    local source=$1
    shift
    local file
    for file in "$@"; do cmp "$source/$file" "$root/$file"; done
}

run() {
    local label=$1 command=$2
    shift 2
    local executable=$REQVIRE_BIN
    case "$command" in add|rm|mv|rename|merge|mv-file|mv-folder|link|unlink|relink|mv-asset|rm-asset|format) executable=$MCP_OPERATION_BIN ;; esac
    (cd "$root" && timeout 40 "$executable" "$command" "$@" --json) \
        > "$output/rm-artifact-links-$label.json" 2> "$output/rm-artifact-links-$label.stderr"
}

check() {
    local name=$1
    shift
    # A background subshell keeps errexit active inside assertion functions;
    # waiting separately lets later independent assertions report their result.
    (set -e; "$@") > "$output/$name.log" 2>&1 &
    local pid=$! result=0
    wait "$pid" || result=$?
    local outcome=PASS
    if [[ "$result" != 0 ]]; then outcome=FAIL; cat "$output/$name.log" >&2; status=1; fi
    printf '%s %s\n' "$outcome" "$name" | tee -a "$output/rm-artifact-links-checks.txt"
}

artifact_targets() {
    jq -Rs '[scan("satisfiedBy: \\[[^\\]]+\\]\\(([^)]+)\\)") | .[0]] | unique' "$1"
}

preview_targets() {
    local file=$1 new=${2:-false}
    if [[ "$new" == true ]]; then [[ "$link_preview_status" == 0 ]]; else [[ "$preview_status" == 0 ]]; fi
    jq --arg path "$model" --argjson new "$new" --argjson targets "$targets" '
      .diffs | map(select(.file_path == $path)) | .[0].lines |
      def targets($color): [.[] | select(.color == $color) | .content |
        scan("satisfiedBy: \\[[^\\]]+\\]\\(([^)]+)\\)") | .[0]] | unique;
      targets("red") as $removed | targets("green") as $added |
      if $new then ($added - $removed == ["../../../NOTICE"] and $removed - $added == [])
      else
        ([.[] | select(.color != "red") | .content |
          scan("satisfiedBy: \\[[^\\]]+\\]\\(([^)]+)\\)") | .[0]] | unique) as $actual |
        ($removed == $added and ($actual - $targets == []))
      end
    ' "$file" > "$output/preview-targets.json"
    jq -e . "$output/preview-targets.json"
}

preview_keeps_files() { [[ "$preview_status" == 0 ]]; same_files "$before" "${files[@]}"; }
removed_verification() {
    [[ "$apply_status" == 0 ]]
    if grep -Fq 'Disposable Verification' "$root/$model"; then return 1; fi
    if grep -Fq 'Disposable Verification' "$root/$checks"; then return 1; fi
}
apply_targets() { artifact_targets "$root/$model" | jq -e --argjson targets "$targets" '. == $targets'; }
artifacts_head_index() {
    same_files "$before" "${artifacts[@]}"
    [[ $(git -C "$root" rev-parse HEAD) == "$head" ]]
    cmp "$root/.git/index" "$output/rm-artifact-original.index"
}
link_keeps_files() { [[ "$link_preview_status" == 0 ]]; same_files "$before_link" "${files[@]}"; }
linked_targets() {
    [[ "$link_status" == 0 ]]
    artifact_targets "$root/$model" | jq -e --argjson targets "$targets" '. == (($targets + ["../../../NOTICE"]) | sort)'
}

before="$output/rm-artifact-before"
snapshot "$before"
initial_status=0; run initial-validation validate || initial_status=$?
check rm-artifact-links-initial-valid test "$initial_status" = 0
preview_status=0; run preview rm 'Disposable Verification' --dry-run || preview_status=$?
check rm-artifact-links-preview-keeps-files preview_keeps_files
check rm-artifact-links-preview-keeps-targets preview_targets "$output/rm-artifact-links-preview.json"
apply_status=0; run apply rm 'Disposable Verification' || apply_status=$?
check rm-artifact-links-removes-verification removed_verification
check rm-artifact-links-apply-keeps-targets apply_targets
check rm-artifact-links-keeps-artifacts-head-index artifacts_head_index
validation_status=0; run persisted-validation validate || validation_status=$?
check rm-artifact-links-persisted-model-valid test "$validation_status" = 0
before_link="$output/rm-artifact-before-link"
snapshot "$before_link"
link_preview_status=0; run new-link-preview link 'Retained Requirement' satisfiedBy NOTICE --dry-run || link_preview_status=$?
check artifact-link-preview-keeps-files link_keeps_files
check artifact-link-preview-keeps-new-target preview_targets "$output/rm-artifact-links-new-link-preview.json" true
link_status=0; run new-link-apply link 'Retained Requirement' satisfiedBy NOTICE || link_status=$?
check artifact-link-apply-keeps-new-target linked_targets
linked_validation_status=0; run new-link-validation validate || linked_validation_status=$?
check artifact-link-persisted-model-valid test "$linked_validation_status" = 0
exit "$status"
