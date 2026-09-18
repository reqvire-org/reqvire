#!/usr/bin/env bash
set -euo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$TEST_SCRIPT_DIR/../.." && pwd)"
WORK_DIR="${TEST_DIR:-$(mktemp -d)}"

check_traces_reference() {
  local skill_file="$1"
  local label="$2"
  local reference
  reference=$(sed -n '/^traces /p' "$skill_file")
  if [ -z "$reference" ]; then
    echo "FAILED: $label has no traces command reference"
    exit 1
  fi

  local args=()
  if [[ "$reference" == *--json* ]]; then
    args+=(--json)
  fi
  if ! (cd "$WORK_DIR" && "$REQVIRE_BIN" traces "${args[@]}") > "$WORK_DIR/$label-stdout.json" 2> "$WORK_DIR/$label-error.log"; then
    echo "FAILED: $label documented traces invocation is rejected: $reference"
    cat "$WORK_DIR/$label-error.log"
    exit 1
  fi
  if ! jq -e 'any(.files[]?.verifications[]?; .name == "Skill Reference Test")' "$WORK_DIR/$label-stdout.json" >/dev/null; then
    echo "FAILED: $label documented traces invocation did not return the fixture trace"
    exit 1
  fi
  if [[ "$reference" != *--output* ]]; then
    echo "FAILED: $label traces reference does not advertise optional file output"
    exit 1
  fi
  (cd "$WORK_DIR" && "$REQVIRE_BIN" traces --output "$label-file.json") > "$WORK_DIR/$label-file.log" 2>&1
  jq -S . "$WORK_DIR/$label-stdout.json" > "$WORK_DIR/$label-stdout-normalized.json"
  jq -S . "$WORK_DIR/$label-file.json" > "$WORK_DIR/$label-file-normalized.json"
  diff -u "$WORK_DIR/$label-stdout-normalized.json" "$WORK_DIR/$label-file-normalized.json"

  local flag_guidance
  flag_guidance=$(sed -n '/^\*\*Common flags:\*\*/,/^$/p' "$skill_file")
  for command in model containment resources traces; do
    if [[ "$flag_guidance" != *JSON-only* || "$flag_guidance" != *"\`$command\`"* ]]; then
      echo "FAILED: $label shared flag guidance does not explain JSON-only $command output"
      exit 1
    fi
  done

  # Reference pages are installed alongside SKILL.md and must use the same CLI.
  local unsupported_flags='((reqvire|"\$PWD")[[:space:]]+|^[[:space:]]*)(model|containment|resources|traces)[[:space:]][^`]*(--json|--mmd)'
  if grep -RnE --include='*.md' "$unsupported_flags" "$(dirname "$skill_file")" > "$WORK_DIR/$label-stale-references.log"; then
    echo "FAILED: $label installed references use unsupported JSON-only report flags"
    cat "$WORK_DIR/$label-stale-references.log"
    exit 1
  fi
}

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
  check_traces_reference "$target/skills/reqvire-syseng/SKILL.md" "codex-local"
}

run_claude_local_install() {
  local target="$WORK_DIR/claude-local"
  CLAUDE_HOME="$target" "$REPO_ROOT/scripts/install-claude-skill.sh" > "$WORK_DIR/claude-local.log"
  compare_tree "$REPO_ROOT/claude-plugins/skills" "$target/skills" "claude-local"
  check_traces_reference "$target/skills/syseng/SKILL.md" "claude-local"
}

run_codex_remote_install() {
  local target="$WORK_DIR/codex-remote"
  local script="$WORK_DIR/install-codex-skill.sh"
  cp "$REPO_ROOT/scripts/install-codex-skill.sh" "$script"
  CODEX_HOME="$target" REQVIRE_REPO_RAW="file://$REPO_ROOT" bash "$script" > "$WORK_DIR/codex-remote.log"
  compare_tree "$REPO_ROOT/codex-skills" "$target/skills" "codex-remote"
  check_traces_reference "$target/skills/reqvire-syseng/SKILL.md" "codex-remote"
}

run_claude_remote_install() {
  local target="$WORK_DIR/claude-remote"
  local script="$WORK_DIR/install-claude-skill.sh"
  cp "$REPO_ROOT/scripts/install-claude-skill.sh" "$script"
  CLAUDE_HOME="$target" REQVIRE_REPO_RAW="file://$REPO_ROOT" bash "$script" > "$WORK_DIR/claude-remote.log"
  compare_tree "$REPO_ROOT/claude-plugins/skills" "$target/skills" "claude-remote"
  check_traces_reference "$target/skills/syseng/SKILL.md" "claude-remote"
}

run_codex_local_install
run_claude_local_install
run_codex_remote_install
run_claude_remote_install

exit 0
