# Setup Reqvire Environment

Install the shared Reqvire skills and verify the runtime for the chosen assistant.

## Install or refresh skills

Install from the repository's main branch:

```bash
npx skills add https://github.com/reqvire-org/reqvire/tree/main/skills
```

Choose the desired skills and agents when prompted. Project installation is the
default. For all shared skills globally in Codex and Claude Code:

```bash
npx skills add https://github.com/reqvire-org/reqvire/tree/main/skills --agent codex claude-code --skill '*' --global
```

To install one skill, add `--skill reqvire-syseng`; select one target with
`--agent codex` or `--agent claude-code`. From a checkout, use `npx skills add ./skills`.
Rerun the install command to refresh from main, then restart the assistant.

The Claude marketplace plugin and custom curl installers are deprecated. Existing
plugin users should disable or uninstall the Reqvire plugin through Claude Code's
`/plugin` manager before installing the shared collection. Use the stable
`reqvire-*` skill names rather than the old marketplace namespace.

## Verify the Reqvire runtime

Skills use the npm runner by default. Verify Node/npm and the Reqvire runner:

```bash
node --version
npm --version
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --version
```

For reproducible runtime behavior, set `REQVIRE_NPX_PACKAGE` before starting the assistant:

```bash
export REQVIRE_NPX_PACKAGE=@reqvire-org/reqvire@0.13.2
```

Use the intended workspace root for validation and MCP startup. Migration remains
CLI-only and precedes MCP startup when the model is incompatible:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mcp --enable-mutations
```

Installing skills does not start MCP or grant mutation permissions.

For a client-managed local project process, use stdio transport:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace /absolute/path/to/project mcp --transport stdio --enable-mutations
```

Use a stdio-capable client's command/args configuration and an absolute project
path when the launch directory is uncertain. HTTP remains the default. Stdio
retains validation, clean-worktree admission, ownership checks, and explicit
mutation/commit/GitHub enablement. HTTP listener/access flags are incompatible
with stdio. Protocol output is stdout; diagnostics are stderr; client EOF releases
the process's workers and locks while preserving pending work.

## Repository assistant instructions

When the user requests repository-specific setup, update the existing assistant
instructions file with the shared skill names and the supported operation boundary:

```markdown
Use `reqvire-syseng` for capabilities, requirements, contracts, verifications,
model refactoring, and implementation task planning. Use `reqvire-audit` for
model quality, coverage, lint, and change-impact review.

Use typed MCP tools for authoring and previews, with mutations enabled. Use
retained CLI validation and migration before MCP startup when required.
Validate after meaningful model changes and align verifications before implementation.
```

Preserve existing repository guidance and avoid duplicate instructions. Do not
change assistant configuration merely to install or refresh the skill files.
