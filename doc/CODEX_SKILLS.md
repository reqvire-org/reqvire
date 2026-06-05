# Codex Skills for Reqvire

This repository includes Codex skill packages for Reqvire MBSE workflows.

## Included Skills

- `codex-skills/reqvire-syseng`
  - Use for requirements and specifications work, verification modeling and coverage checks, model exploration and impact analysis, and MBSE-first change workflows in Reqvire.
- `codex-skills/reqvire-ontology-authoring`
  - Use for competency-question-driven creation, extension, and validation of Reqvire ontology elements for IT engineering, systems engineering, MBSE, and system-of-interest models with OWL/Turtle vocabulary, capability attachment context, semantic-contract and semantic-query-contract boundaries, ontology hierarchy, domain/range/property modeling, individuals, axioms, and semantic export readiness.

The skills use the Reqvire npm package by default:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
```

To pin the Reqvire version used by the skill, set:

```bash
export REQVIRE_NPX_PACKAGE=@reqvire-org/reqvire@0.13.2
```

## Install Globally (User Machine)

Global Codex skills are loaded from `$CODEX_HOME/skills`.

If `CODEX_HOME` is not set, the default is `~/.codex`.

Run from this repository root:

```bash
./scripts/install-codex-skill.sh
```

What this does:
1. Resolves target path as `$CODEX_HOME/skills`
2. Removes existing global copies of each skill found under `codex-skills`
3. Copies each repo skill into the global skills folder

## Manual Install (Alternative)

```bash
export CODEX_HOME="${CODEX_HOME:-$HOME/.codex}"
mkdir -p "$CODEX_HOME/skills"
rm -rf "$CODEX_HOME/skills/reqvire-syseng"
cp -R codex-skills/reqvire-syseng "$CODEX_HOME/skills/reqvire-syseng"
rm -rf "$CODEX_HOME/skills/reqvire-ontology-authoring"
cp -R codex-skills/reqvire-ontology-authoring "$CODEX_HOME/skills/reqvire-ontology-authoring"
```

## Update After Repo Changes

Re-run:

```bash
./scripts/install-codex-skill.sh
```

This overwrites the global copy with the latest version from the repository.
