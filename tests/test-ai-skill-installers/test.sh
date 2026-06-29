#!/usr/bin/env bash
set -euo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$TEST_SCRIPT_DIR/../.." && pwd)"
WORK_DIR="${TEST_DIR:-$(mktemp -d)}"

compare_tree() {
  local source_dir="$1"
  local installed_dir="$2"
  local label="$3"

  find "$source_dir" -type f | sed "s#$source_dir/##" | sort > "$WORK_DIR/$label-source.txt"
  find "$installed_dir" -type f | sed "s#$installed_dir/##" | sort > "$WORK_DIR/$label-installed.txt"

  if ! diff -u "$WORK_DIR/$label-source.txt" "$WORK_DIR/$label-installed.txt"; then
    echo "FAILED: $label installed file manifest does not match source tree"
    exit 1
  fi
}

run_codex_local_install() {
  local target="$WORK_DIR/codex-local"
  CODEX_HOME="$target" "$REPO_ROOT/scripts/install-codex-skill.sh" > "$WORK_DIR/codex-local.log"
  compare_tree "$REPO_ROOT/codex-skills" "$target/skills" "codex-local"
}

run_claude_local_install() {
  local target="$WORK_DIR/claude-local"
  CLAUDE_HOME="$target" "$REPO_ROOT/scripts/install-claude-skill.sh" > "$WORK_DIR/claude-local.log"
  compare_tree "$REPO_ROOT/claude-plugins/skills" "$target/skills" "claude-local"
}

run_codex_remote_install() {
  local target="$WORK_DIR/codex-remote"
  local script="$WORK_DIR/install-codex-skill.sh"
  cp "$REPO_ROOT/scripts/install-codex-skill.sh" "$script"
  CODEX_HOME="$target" REQVIRE_REPO_RAW="file://$REPO_ROOT" bash "$script" > "$WORK_DIR/codex-remote.log"
  compare_tree "$REPO_ROOT/codex-skills" "$target/skills" "codex-remote"
}

run_claude_remote_install() {
  local target="$WORK_DIR/claude-remote"
  local script="$WORK_DIR/install-claude-skill.sh"
  cp "$REPO_ROOT/scripts/install-claude-skill.sh" "$script"
  CLAUDE_HOME="$target" REQVIRE_REPO_RAW="file://$REPO_ROOT" bash "$script" > "$WORK_DIR/claude-remote.log"
  compare_tree "$REPO_ROOT/claude-plugins/skills" "$target/skills" "claude-remote"
}

run_codex_local_install
run_claude_local_install
run_codex_remote_install
run_claude_remote_install

exit 0
