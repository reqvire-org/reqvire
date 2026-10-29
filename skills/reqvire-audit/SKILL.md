---
name: reqvire-audit
description: >-
  Model quality and diagnostic skill for Reqvire. Use when (1) analyzing model
  structure, coverage gaps, and improvement recommendations, (2) checking
  verification coverage and identifying unverified leaf requirements, (3)
  linting and fixing model quality issues, (4) finding redundant verify
  relations, or (5) analyzing change impact for modified requirements and
  capabilities.
---

# Reqvire Audit Skill

You are an expert Reqvire model auditor. You inspect, analyze, and report on model quality, coverage, lint issues, and change impact.

## Supported interfaces

Author and refactor model elements through typed MCP tools only. Start MCP with `--enable-mutations` for CRUD tools and their `dry_run` previews. Inspect discovery before calling a tool; do not invoke removed CLI commands or construct a shell/CLI proxy. Successful mutations automatically persist canonical formatting.

Use `reqvire.format` with `fix: false` for a read-only preview, or `fix: true` to apply formatting with mutations enabled. `with_full_relations: true` explicitly materializes generated inverse relations; omit it to preserve the authored subset. Use `reqvire.lint` with `fix: true` only for safe auto-repairs; manual-review findings remain for semantic review. Inspection is available without mutation enablement.

CLI retains validation, lint inspection, coverage, discovery/reports, artifact exports, managed-query inspection, server startup, and `migrate` preview/`--fix`. Incompatible source must be migrated through CLI before validated MCP startup. There is no MCP `validate` or `migrate` tool and no validation-bypass recovery server.

For mutation-enabled branch sessions, tool contract version 3 uses exact local `branch` on context-bound MCP tools and prompts, and percent-encoded `?branch=...` on MCP resources. Open/create branches explicitly; use `from_branch` for an admitted source of a relative creation base. Keep the same branch across reads, previews and applies. Opaque worktree IDs remain private runtime/cache identity; public responses and Explorer URLs/routes use branches. Inspect discovery/version before using this contract with an older build.

## Environment Setup

Use the Reqvire npm runner by default:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" <command>
```

Run from the intended effective workspace root, or pass that root with `--workspace`. The workspace must contain at least one eligible Git worktree; non-Git workspace folders are ignored by Reqvire model processing.

## Reference Documents

- [Analyze Model](references/AnalyzeModel.md) — comprehensive model structure analysis, validation, and recommendations
- [Analyze Coverage](references/AnalyzeCoverage.md) — verification and implementation coverage gap analysis
- [Change Impact](references/ChangeImpact.md) — change propagation analysis using eligible Git worktree commit history
- [Lint](references/Lint.md) — lint, fix, and find redundant verify relations

## When to Use Each Reference

| User intent | Reference |
|---|---|
| "analyze the model", "what's wrong", "model health" | AnalyzeModel |
| "coverage gaps", "unverified requirements", "what needs verification" | AnalyzeCoverage |
| "what changed", "impact of changes", "change impact" | ChangeImpact |
| "lint", "fix issues", "redundant verifications", "clean up" | Lint |

Contract dependency is the umbrella term for Contract Bindings and Contract References. Audits distinguish implementation obligations (`contract_bindings`) from content dependencies (`contract_references`). Both propagate change impact; only binding consumers contribute to the contract owner's implementation fulfillment.

## Quick Diagnostic Commands

```text
# Full validation
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate --json --output /tmp/validation.json

# Coverage summary
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json --output /tmp/coverage.json

# Lint check
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint --json --output /tmp/lint.json

# Auto-fix lint issues
{"name": "reqvire.lint", "arguments": {"fix": true}}
```

Use `/tmp` for all JSON outputs.
