#!/usr/bin/env bash
set -euo pipefail

SCRIPT_PATH="${BASH_SOURCE[0]:-}"
if [ -n "$SCRIPT_PATH" ] && [ -f "$SCRIPT_PATH" ]; then
  REPO_ROOT="$(cd "$(dirname "$SCRIPT_PATH")/.." && pwd)"
  SOURCE_SKILLS_DIR="$REPO_ROOT/codex-skills"
else
  SOURCE_SKILLS_DIR=""
fi
REQVIRE_REPO_RAW="${REQVIRE_REPO_RAW:-https://raw.githubusercontent.com/reqvire-org/reqvire/main}"
SKILLS=(
  reqvire-audit
  reqvire-concept-authoring
  reqvire-ontology-authoring
  reqvire-syseng
)

CODEX_HOME="${CODEX_HOME:-$HOME/.codex}"
TARGET_SKILLS_DIR="$CODEX_HOME/skills"

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

    echo "Installed Codex skill: $SKILL_NAME"
    echo "Location: $TARGET_SKILL_DIR"
  done
}

install_remote_skill_file() {
  local skill_name="$1"
  local relative_path="$2"
  local skill_dir="$3"
  local target_file="$skill_dir/$relative_path"

  mkdir -p "$(dirname "$target_file")"
  curl -fsSL "$REQVIRE_REPO_RAW/codex-skills/$skill_name/$relative_path" -o "$target_file"
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
      reqvire-audit)
        install_remote_skill_file "$SKILL_NAME" "references/AnalyzeCoverage.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/AnalyzeModel.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/ChangeImpact.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/Lint.md" "$STAGED_SKILL_DIR"
        ;;
      reqvire-concept-authoring)
        install_remote_skill_file "$SKILL_NAME" "agents/openai.yaml" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/ConceptAuthoring.md" "$STAGED_SKILL_DIR"
        ;;
      reqvire-ontology-authoring)
        install_remote_skill_file "$SKILL_NAME" "agents/openai.yaml" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/OntologyAuthoring.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/OntologyRefactoring.md" "$STAGED_SKILL_DIR"
        ;;
      reqvire-syseng)
        install_remote_skill_file "$SKILL_NAME" "references/AddCapability.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/AddRequirement.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/AddVerification.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/CapabilitySemanticContractRefactor.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/Collect.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/ConsolidateRequirements.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/Containment.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/ContainmentStructureRefactor.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/CreatingTasks.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/DesignDocOwnership.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/Link.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/Move.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/Remove.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/RenameElement.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/Setup.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/SpecificationLanguageCleanup.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/SpecificationsExtractionLogic.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/SubmodelRefactor.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/VerificationAlignment.md" "$STAGED_SKILL_DIR"
        install_remote_skill_file "$SKILL_NAME" "references/explore.md" "$STAGED_SKILL_DIR"
        ;;
    esac

    if [ -e "$TARGET_SKILL_DIR" ]; then
      echo "Removing existing skill at $TARGET_SKILL_DIR"
      rm -rf "$TARGET_SKILL_DIR"
    fi
    mv "$STAGED_SKILL_DIR" "$TARGET_SKILL_DIR"

    echo "Installed Codex skill: $SKILL_NAME"
    echo "Location: $TARGET_SKILL_DIR"
  done
}

if [ -d "$SOURCE_SKILLS_DIR" ]; then
  install_local_skills
else
  install_remote_skills
fi
