#!/usr/bin/env bash
set -euo pipefail

SCRIPT_PATH="${BASH_SOURCE[0]:-}"
if [ -n "$SCRIPT_PATH" ] && [ -f "$SCRIPT_PATH" ]; then
  REPO_ROOT="$(cd "$(dirname "$SCRIPT_PATH")/.." && pwd)"
  SOURCE_SKILLS_DIR="$REPO_ROOT/claude-plugins/skills"
else
  SOURCE_SKILLS_DIR=""
fi

REQVIRE_REPO_RAW="${REQVIRE_REPO_RAW:-https://raw.githubusercontent.com/reqvire-org/reqvire/main}"
SKILLS=(
  audit
  concept-authoring
  ontology-authoring
  syseng
)

CLAUDE_HOME="${CLAUDE_HOME:-$HOME/.claude}"
TARGET_SKILLS_DIR="$CLAUDE_HOME/skills"

mkdir -p "$TARGET_SKILLS_DIR"

install_local_skills() {
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

    echo "Installed Claude skill: $SKILL_NAME"
    echo "Location: $TARGET_SKILL_DIR"
  done
}

install_remote_skill_file() {
  local skill_name="$1"
  local relative_path="$2"
  local skill_dir="$3"
  local target_file="$skill_dir/$relative_path"

  mkdir -p "$(dirname "$target_file")"
  curl -fsSL "$REQVIRE_REPO_RAW/claude-plugins/skills/$skill_name/$relative_path" -o "$target_file"
}

install_remote_skills() {
  TMPDIR="$(mktemp -d)"
  cleanup() {
    rm -rf "$TMPDIR"
  }
  trap cleanup EXIT

  for SKILL_NAME in "${SKILLS[@]}"; do
    TARGET_SKILL_DIR="$TARGET_SKILLS_DIR/$SKILL_NAME"
    STAGED_SKILL_DIR="$TMPDIR/$SKILL_NAME"

    echo "Installing $SKILL_NAME to $TARGET_SKILL_DIR"
    mkdir -p "$STAGED_SKILL_DIR"
    install_remote_skill_file "$SKILL_NAME" "SKILL.md" "$STAGED_SKILL_DIR"

    case "$SKILL_NAME" in
      audit)
        install_remote_skill_file "$SKILL_NAME" "reference/AnalyzeCoverage.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/AnalyzeModel.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/ChangeImpact.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/Lint.md" "$STAGED_SKILL_DIR"
        ;;
      concept-authoring)
        install_remote_skill_file "$SKILL_NAME" "reference/ConceptAuthoring.md" "$STAGED_SKILL_DIR"
        ;;
      ontology-authoring)
        install_remote_skill_file "$SKILL_NAME" "reference/OntologyAuthoring.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/OntologyRefactoring.md" "$STAGED_SKILL_DIR"
        ;;
      syseng)
        install_remote_skill_file "$SKILL_NAME" "reference/AddCapability.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/AddRequirement.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/AddVerification.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/CapabilitySemanticContractRefactor.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/Collect.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/ConsolidateRequirements.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/Containment.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/ContainmentStructureRefactor.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/CreatingTasks.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/DesignDocOwnership.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/Link.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/Move.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/Remove.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/RenameElement.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/Setup.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/SpecificationLanguageCleanup.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/SpecificationsExtractionLogic.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/SubmodelRefactor.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/VerificationAlignment.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "reference/explore.md" "$STAGED_SKILL_DIR"
        ;;
    esac

    if [ -e "$TARGET_SKILL_DIR" ]; then
      echo "Removing existing skill at $TARGET_SKILL_DIR"
      rm -rf "$TARGET_SKILL_DIR"
    fi
    mv "$STAGED_SKILL_DIR" "$TARGET_SKILL_DIR"

    echo "Installed Claude skill: $SKILL_NAME"
    echo "Location: $TARGET_SKILL_DIR"
  done
}

if [ -n "$SOURCE_SKILLS_DIR" ] && [ -d "$SOURCE_SKILLS_DIR" ]; then
  install_local_skills
else
  install_remote_skills
fi
