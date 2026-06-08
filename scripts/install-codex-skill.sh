#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SOURCE_SKILLS_DIR="$REPO_ROOT/codex-skills"

CODEX_HOME="${CODEX_HOME:-$HOME/.codex}"
TARGET_SKILLS_DIR="$CODEX_HOME/skills"

if [ ! -d "$SOURCE_SKILLS_DIR" ]; then
  echo "Source skills directory not found: $SOURCE_SKILLS_DIR" >&2
  exit 1
fi

mkdir -p "$TARGET_SKILLS_DIR"

for SOURCE_SKILL_DIR in "$SOURCE_SKILLS_DIR"/*; do
  if [ ! -d "$SOURCE_SKILL_DIR" ]; then
    continue
  fi

  SKILL_NAME="$(basename "$SOURCE_SKILL_DIR")"
  TARGET_SKILL_DIR="$TARGET_SKILLS_DIR/$SKILL_NAME"

  if [ -e "$TARGET_SKILL_DIR" ]; then
    echo "Removing existing skill at $TARGET_SKILL_DIR"
    rm -rf "$TARGET_SKILL_DIR"
  fi

  echo "Installing $SKILL_NAME to $TARGET_SKILL_DIR"
  cp -R "$SOURCE_SKILL_DIR" "$TARGET_SKILL_DIR"

  echo "Installed Codex skill: $SKILL_NAME"
  echo "Location: $TARGET_SKILL_DIR"
done
