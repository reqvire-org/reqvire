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

CLI retains workspace preparation and management, validation, lint inspection, coverage, discovery/reports, artifact exports, managed-query inspection, server startup, and `migrate` preview/`--fix`. Incompatible source must be migrated through CLI before validated MCP startup. There is no MCP validate or model-syntax migration tool and no validation-bypass recovery server. Workspace configuration conversion uses the separate typed reqvire.workspace.migrate operation in an admitted context.

For mutation-enabled branch sessions, tool contract version 5 uses exact local `branch` on context-bound MCP tools and prompts, and percent-encoded `?branch=...` on MCP resources. Open/create branches explicitly; use `from_branch` for an admitted source of a relative creation base. The same operations apply to single repositories and declared workspace compositions. Keep the same branch across reads, previews and applies. Worktrees and opaque IDs remain private implementation details. Inspect discovery/version before using this contract with an older build.

## Environment Setup

For a multi-repository composition, use one reqvire-workspace.toml for all repository declarations; native includes, fragments and overlays are unsupported. Without a manifest, use the normal single-repository workflow. Inspect configured repositories_dir, refs and exact pins with workspace status or reqvire.workspace_status before analysis.

Use the Reqvire npm runner by default:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" <command>
```

Run from the intended effective workspace root, or pass that root with `--workspace`. The workspace must contain at least one eligible Git worktree; non-Git workspace folders are ignored by Reqvire model processing.

For change impact analysis, ordinary Git mode requires one eligible worktree. A declared workspace uses the root branch's tracked reqvire-workspace.toml and exact component pins. The owning root may contain no model elements. Required objects must be local; explicit workspace sync prepares checkouts. A baseline predating conversion may use versioned root-owned XML through the shared legacy reader. Omitted baselines use actual PR context; otherwise ask for the intended baseline. Explicit HEAD reviews pending changes. See Change Impact before choosing a ref.

## Reference Documents

- [Analyze Model](references/AnalyzeModel.md) — comprehensive model structure analysis, validation, and recommendations
- [Analyze Coverage](references/AnalyzeCoverage.md) — verification and implementation coverage gap analysis
- [Change Impact](references/ChangeImpact.md) — change propagation analysis using ordinary Git or complete repository compositions
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
