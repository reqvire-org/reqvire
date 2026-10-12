# Reqvire Assistant Skills

The root `skills/` directory contains one collection for Codex, Claude Code, and
other agents supported by the [skills CLI](https://github.com/vercel-labs/skills).

| Skill | Use |
| --- | --- |
| `reqvire-syseng` | MBSE workflows, capabilities, requirements, contracts, verification, and implementation tasks |
| `reqvire-audit` | Validation, lint, coverage, change impact, and model review |
| `reqvire-ontology-authoring` | Structural OWL/Turtle vocabulary, semantic contracts, and ontology refactoring |
| `reqvire-concept-authoring` | Native SKOS schemes, concepts, mappings, and terminology |

## Install from main

```bash
npx skills add https://github.com/reqvire-org/reqvire/tree/main/skills
```

Choose the skills and target agents when prompted. Installation is scoped to the
current project by default. For all four skills globally in Codex and Claude Code:

```bash
npx skills add https://github.com/reqvire-org/reqvire/tree/main/skills --agent codex claude-code --skill '*' --global
```

For one project skill and one agent:

```bash
npx skills add https://github.com/reqvire-org/reqvire/tree/main/skills --agent claude-code --skill reqvire-syseng
```

The skills CLI manages installation directories and symlinks or copies. Use
`--copy` if your environment needs independent copies. To update, rerun the same
install command, then restart your assistant.

## Local checkout

Run from the repository root:

```bash
npx skills add ./skills --agent codex claude-code --skill '*'
```

## Migrating from the marketplace plugin

The Claude marketplace plugin and custom curl installers are deprecated. Open
Claude Code's `/plugin` manager, disable or uninstall the Reqvire plugin, then
install the shared skills with `--agent claude-code`. Skill names now use the
`reqvire-*` prefix. Existing direct-install users can install the shared packages
with the same stable names.

## Reqvire runtime

Skills provide instructions; installing them does not start Reqvire or enable
model mutations. Their default runtime command uses the Reqvire npm runner:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
```

Pin the runtime when reproducible behavior is required:

```bash
export REQVIRE_NPX_PACKAGE=@reqvire-org/reqvire@0.13.2
```

Author model changes through typed MCP tools with mutations enabled. Formatting
and lint repair use MCP; validation and migration remain CLI commands. Run
migration before MCP startup when the source is incompatible.
