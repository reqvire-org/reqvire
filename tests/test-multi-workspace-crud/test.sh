#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "❌ FAILED: $1"
  if [ -n "${2:-}" ] && [ -f "$2" ]; then
    echo ""
    cat "$2"
  fi
  exit 1
}

init_multi_workspace_repos() {
  local workspace="$1"

  for repo in "$workspace/repo-a" "$workspace/repo-b"; do
    (cd "$repo" && git init >/dev/null 2>&1)
    (cd "$repo" && git config --local user.email "test@example.com" >/dev/null 2>&1)
    (cd "$repo" && git config --local user.name "Test User" >/dev/null 2>&1)
    (cd "$repo" && git add . >/dev/null 2>&1)
    (cd "$repo" && git commit -m "Initial commit" >/dev/null 2>&1)
  done
}

REQVIRE_BIN="${REQVIRE_BIN:-reqvire}"
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTSIDE_DIR="$(mktemp -d -t reqvire-multi-crud-outside-XXXXXX)"
CRUD_WORKSPACE="$(mktemp -d -t reqvire-multi-workspace-crud-XXXXXX)"

mkdir -p "$TEST_DIR/output"
cp -a "$TEST_SCRIPT_DIR/fixtures/." "$CRUD_WORKSPACE/"
init_multi_workspace_repos "$CRUD_WORKSPACE"
mkdir -p "$CRUD_WORKSPACE/not-a-repo"

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$CRUD_WORKSPACE" validate) > "$TEST_DIR/output/initial-validate.txt" 2>&1 \
  || fail "cross-repo CRUD fixture should validate before mutations" "$TEST_DIR/output/initial-validate.txt"

set +e
(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$CRUD_WORKSPACE" add not-a-repo/Added.md < "$CRUD_WORKSPACE/add-elements/RepoBAddedRequirement.md") > "$TEST_DIR/output/add-non-git.txt" 2>&1
ADD_NON_GIT_EXIT=$?
set -e
if [ "$ADD_NON_GIT_EXIT" -eq 0 ]; then
  fail "add should reject targets outside eligible Git worktrees" "$TEST_DIR/output/add-non-git.txt"
fi
if ! grep -q "outside eligible Git worktrees" "$TEST_DIR/output/add-non-git.txt"; then
  fail "add non-Git target error should mention eligible Git worktrees" "$TEST_DIR/output/add-non-git.txt"
fi
[ ! -f "$CRUD_WORKSPACE/not-a-repo/Added.md" ] \
  || fail "add rejection must not create a file under not-a-repo"

set +e
(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$CRUD_WORKSPACE" mv "Repo A Movable Requirement" not-a-repo/Moved.md) > "$TEST_DIR/output/mv-non-git.txt" 2>&1
MV_NON_GIT_EXIT=$?
set -e
if [ "$MV_NON_GIT_EXIT" -eq 0 ]; then
  fail "mv should reject targets outside eligible Git worktrees" "$TEST_DIR/output/mv-non-git.txt"
fi
if ! grep -q "outside eligible Git worktrees" "$TEST_DIR/output/mv-non-git.txt"; then
  fail "mv non-Git target error should mention eligible Git worktrees" "$TEST_DIR/output/mv-non-git.txt"
fi
[ ! -f "$CRUD_WORKSPACE/not-a-repo/Moved.md" ] \
  || fail "mv rejection must not create a file under not-a-repo"

set +e
(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$CRUD_WORKSPACE" mv-file repo-a/specifications/FileMove.md not-a-repo/FileMove.md) > "$TEST_DIR/output/mv-file-non-git.txt" 2>&1
MV_FILE_NON_GIT_EXIT=$?
set -e
if [ "$MV_FILE_NON_GIT_EXIT" -eq 0 ]; then
  fail "mv-file should reject targets outside eligible Git worktrees" "$TEST_DIR/output/mv-file-non-git.txt"
fi
if ! grep -q "outside eligible Git worktrees" "$TEST_DIR/output/mv-file-non-git.txt"; then
  fail "mv-file non-Git target error should mention eligible Git worktrees" "$TEST_DIR/output/mv-file-non-git.txt"
fi
[ ! -f "$CRUD_WORKSPACE/not-a-repo/FileMove.md" ] \
  || fail "mv-file rejection must not create a file under not-a-repo"

set +e
(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$CRUD_WORKSPACE" mv-folder repo-a/specifications/FolderMove not-a-repo/FolderMove) > "$TEST_DIR/output/mv-folder-non-git.txt" 2>&1
MV_FOLDER_NON_GIT_EXIT=$?
set -e
if [ "$MV_FOLDER_NON_GIT_EXIT" -eq 0 ]; then
  fail "mv-folder should reject targets outside eligible Git worktrees" "$TEST_DIR/output/mv-folder-non-git.txt"
fi
if ! grep -q "outside eligible Git worktrees" "$TEST_DIR/output/mv-folder-non-git.txt"; then
  fail "mv-folder non-Git target error should mention eligible Git worktrees" "$TEST_DIR/output/mv-folder-non-git.txt"
fi
[ ! -d "$CRUD_WORKSPACE/not-a-repo/FolderMove" ] \
  || fail "mv-folder rejection must not create a folder under not-a-repo"

set +e
(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$CRUD_WORKSPACE" mv-asset repo-a/docs/asset-to-move.txt not-a-repo/asset-to-move.txt) > "$TEST_DIR/output/mv-asset-non-git.txt" 2>&1
MV_ASSET_NON_GIT_EXIT=$?
set -e
if [ "$MV_ASSET_NON_GIT_EXIT" -eq 0 ]; then
  fail "mv-asset should reject targets outside eligible Git worktrees" "$TEST_DIR/output/mv-asset-non-git.txt"
fi
if ! grep -q "outside eligible Git worktrees" "$TEST_DIR/output/mv-asset-non-git.txt"; then
  fail "mv-asset non-Git target error should mention eligible Git worktrees" "$TEST_DIR/output/mv-asset-non-git.txt"
fi
[ ! -f "$CRUD_WORKSPACE/not-a-repo/asset-to-move.txt" ] \
  || fail "mv-asset rejection must not create a file under not-a-repo"

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$CRUD_WORKSPACE" add repo-b/specifications/Added.md < "$CRUD_WORKSPACE/add-elements/RepoBAddedRequirement.md") > "$TEST_DIR/output/add.txt" 2>&1 \
  || fail "add should create an element in repo B from a multi-repo workspace root" "$TEST_DIR/output/add.txt"

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$CRUD_WORKSPACE" mv "Repo A Movable Requirement" repo-b/specifications/MovedElement.md) > "$TEST_DIR/output/mv.txt" 2>&1 \
  || fail "mv should move an element from repo A to repo B" "$TEST_DIR/output/mv.txt"

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$CRUD_WORKSPACE" rename "Repo B Rename Requirement" "Repo B Renamed Requirement") > "$TEST_DIR/output/rename.txt" 2>&1 \
  || fail "rename should update an element in repo B while preserving references" "$TEST_DIR/output/rename.txt"

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$CRUD_WORKSPACE" link "Repo B Link Source Requirement" satisfiedBy repo-a/docs/evidence.txt) > "$TEST_DIR/output/link.txt" 2>&1 \
  || fail "link should add a relation from repo B to a repo A path" "$TEST_DIR/output/link.txt"

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$CRUD_WORKSPACE" search --json --filter-name "Repo B Link Source Requirement") > "$TEST_DIR/output/link-search.json" 2>&1 \
  || fail "search after cross-repo link should run" "$TEST_DIR/output/link-search.json"
jq -e '.files[].elements[] | select(.name == "Repo B Link Source Requirement") | .relations[]? | select(.relation_type == "satisfiedBy" and .target.target == "repo-a/docs/evidence.txt")' "$TEST_DIR/output/link-search.json" >/dev/null \
  || fail "link should persist a workspace-root-relative repo A target" "$TEST_DIR/output/link-search.json"

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$CRUD_WORKSPACE" unlink "Repo B Link Source Requirement" repo-a/docs/evidence.txt) > "$TEST_DIR/output/unlink.txt" 2>&1 \
  || fail "unlink should remove a relation from repo B to a repo A path" "$TEST_DIR/output/unlink.txt"

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$CRUD_WORKSPACE" relink "Repo B Relink Requirement" satisfiedBy repo-b/docs/old-evidence.txt repo-a/docs/evidence.txt) > "$TEST_DIR/output/relink.txt" 2>&1 \
  || fail "relink should replace a repo B path with a repo A path" "$TEST_DIR/output/relink.txt"

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$CRUD_WORKSPACE" merge "Repo B Merge Target Requirement" "Repo A Merge Source Requirement") > "$TEST_DIR/output/merge.txt" 2>&1 \
  || fail "merge should merge a repo A element into a repo B target" "$TEST_DIR/output/merge.txt"

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$CRUD_WORKSPACE" mv-file repo-a/specifications/FileMove.md repo-b/specifications/FileMovedFromA.md) > "$TEST_DIR/output/mv-file.txt" 2>&1 \
  || fail "mv-file should move a file from repo A to repo B" "$TEST_DIR/output/mv-file.txt"

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$CRUD_WORKSPACE" mv-folder repo-a/specifications/FolderMove repo-b/specifications/FolderMovedFromA) > "$TEST_DIR/output/mv-folder.txt" 2>&1 \
  || fail "mv-folder should move a folder from repo A to repo B" "$TEST_DIR/output/mv-folder.txt"

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$CRUD_WORKSPACE" mv-asset repo-a/docs/asset-to-move.txt repo-b/docs/asset-moved-from-a.txt) > "$TEST_DIR/output/mv-asset.txt" 2>&1 \
  || fail "mv-asset should move an asset from repo A to repo B and rewrite references" "$TEST_DIR/output/mv-asset.txt"

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$CRUD_WORKSPACE" rm-asset repo-b/docs/asset-remove.txt) > "$TEST_DIR/output/rm-asset.txt" 2>&1 \
  || fail "rm-asset should remove an asset in repo B and clean references" "$TEST_DIR/output/rm-asset.txt"

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$CRUD_WORKSPACE" rm "Repo A Removable Requirement") > "$TEST_DIR/output/rm.txt" 2>&1 \
  || fail "rm should remove an element from repo A in a multi-repo workspace" "$TEST_DIR/output/rm.txt"

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$CRUD_WORKSPACE" validate) > "$TEST_DIR/output/final-validate.txt" 2>&1 \
  || fail "cross-repo CRUD workspace should validate after all mutations" "$TEST_DIR/output/final-validate.txt"

(cd "$OUTSIDE_DIR" && "$REQVIRE_BIN" --workspace "$CRUD_WORKSPACE" search --json) > "$TEST_DIR/output/final-search.json" 2>&1 \
  || fail "search after all cross-repo CRUD mutations should run" "$TEST_DIR/output/final-search.json"

[ -f "$CRUD_WORKSPACE/repo-b/specifications/Added.md" ] \
  || fail "add should create repo-b/specifications/Added.md"
[ -f "$CRUD_WORKSPACE/repo-b/specifications/MovedElement.md" ] \
  || fail "mv should create repo-b/specifications/MovedElement.md"
[ ! -f "$CRUD_WORKSPACE/repo-a/specifications/FileMove.md" ] \
  || fail "mv-file should remove repo-a/specifications/FileMove.md"
[ -f "$CRUD_WORKSPACE/repo-b/specifications/FileMovedFromA.md" ] \
  || fail "mv-file should create repo-b/specifications/FileMovedFromA.md"
[ ! -d "$CRUD_WORKSPACE/repo-a/specifications/FolderMove" ] \
  || fail "mv-folder should remove repo-a/specifications/FolderMove"
[ -f "$CRUD_WORKSPACE/repo-b/specifications/FolderMovedFromA/Nested.md" ] \
  || fail "mv-folder should create repo-b/specifications/FolderMovedFromA/Nested.md"
[ ! -f "$CRUD_WORKSPACE/repo-a/docs/asset-to-move.txt" ] \
  || fail "mv-asset should remove repo-a/docs/asset-to-move.txt"
[ -f "$CRUD_WORKSPACE/repo-b/docs/asset-moved-from-a.txt" ] \
  || fail "mv-asset should create repo-b/docs/asset-moved-from-a.txt"
[ ! -f "$CRUD_WORKSPACE/repo-b/docs/asset-remove.txt" ] \
  || fail "rm-asset should remove repo-b/docs/asset-remove.txt"

jq -e '.files[].elements[] | select(.identifier == "repo-b/specifications/Added.md#repo-b-added-requirement")' "$TEST_DIR/output/final-search.json" >/dev/null \
  || fail "add result should be indexed under repo B" "$TEST_DIR/output/final-search.json"
jq -e '.files[].elements[] | select(.identifier == "repo-b/specifications/MovedElement.md#repo-a-movable-requirement")' "$TEST_DIR/output/final-search.json" >/dev/null \
  || fail "mv result should be indexed under repo B" "$TEST_DIR/output/final-search.json"
jq -e '.files[].elements[] | select(.name == "Repo B Renamed Requirement")' "$TEST_DIR/output/final-search.json" >/dev/null \
  || fail "rename result should be indexed with the new name" "$TEST_DIR/output/final-search.json"
if jq -e '.files[].elements[] | select(.name == "Repo B Rename Requirement" or .name == "Repo A Removable Requirement" or .name == "Repo A Merge Source Requirement")' "$TEST_DIR/output/final-search.json" >/dev/null; then
  fail "renamed, removed, and merged source elements should not remain indexed" "$TEST_DIR/output/final-search.json"
fi
jq -e '.files[].elements[] | select(.name == "Repo B Relink Requirement") | .relations[]? | select(.relation_type == "satisfiedBy" and .target.target == "repo-a/docs/evidence.txt")' "$TEST_DIR/output/final-search.json" >/dev/null \
  || fail "relink result should target repo A evidence" "$TEST_DIR/output/final-search.json"
if jq -e '.files[].elements[] | select(.name == "Repo B Link Source Requirement") | .relations[]? | select(.relation_type == "satisfiedBy")' "$TEST_DIR/output/final-search.json" >/dev/null; then
  fail "unlink result should remove the temporary cross-repo evidence relation" "$TEST_DIR/output/final-search.json"
fi
jq -e '.files[].elements[] | select(.name == "Repo B Asset Requirement") | .relations[]? | select(.relation_type == "satisfiedBy" and .target.target == "repo-b/docs/asset-moved-from-a.txt")' "$TEST_DIR/output/final-search.json" >/dev/null \
  || fail "mv-asset result should rewrite repo B requirement to moved asset path" "$TEST_DIR/output/final-search.json"
if jq -e '.files[].elements[] | select(.name == "Repo B Remove Asset Requirement") | .relations[]? | select(.relation_type == "satisfiedBy")' "$TEST_DIR/output/final-search.json" >/dev/null; then
  fail "rm-asset result should remove the deleted asset relation" "$TEST_DIR/output/final-search.json"
fi
grep -q "The system shall merge this source requirement from repo A into a repo B target." "$CRUD_WORKSPACE/repo-b/specifications/B.md" \
  || fail "merge should preserve source content under the repo B target" "$CRUD_WORKSPACE/repo-b/specifications/B.md"
