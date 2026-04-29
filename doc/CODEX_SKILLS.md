# Codex Skills for Reqvire

This repository includes a Codex skill package for Reqvire MBSE workflows.

## Included Skill

- `codex-skills/reqvire-syseng`

Use this skill for:
- requirements and specifications work
- verification modeling and coverage checks
- model exploration and impact analysis
- MBSE-first change workflows in Reqvire

The skill uses the Reqvire npm package by default:

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
1. Resolves target path as `$CODEX_HOME/skills/reqvire-syseng`
2. Removes any existing `reqvire-syseng` at that location
3. Copies `codex-skills/reqvire-syseng` from this repo into the global skills folder

## Manual Install (Alternative)

```bash
export CODEX_HOME="${CODEX_HOME:-$HOME/.codex}"
mkdir -p "$CODEX_HOME/skills"
rm -rf "$CODEX_HOME/skills/reqvire-syseng"
cp -R codex-skills/reqvire-syseng "$CODEX_HOME/skills/reqvire-syseng"
```

## Update After Repo Changes

Re-run:

```bash
./scripts/install-codex-skill.sh
```

This overwrites the global copy with the latest version from the repository.
