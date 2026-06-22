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

## Environment Setup

Use the Reqvire npm runner by default:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" <command>
```

## Reference Documents

- [Analyze Model](reference/AnalyzeModel.md) — comprehensive model structure analysis, validation, and recommendations
- [Analyze Coverage](reference/AnalyzeCoverage.md) — verification and implementation coverage gap analysis
- [Change Impact](reference/ChangeImpact.md) — change propagation analysis using git commit history
- [Lint](reference/Lint.md) — lint, fix, and find redundant verify relations

## When to Use Each Reference

| User intent | Reference |
|---|---|
| "analyze the model", "what's wrong", "model health" | AnalyzeModel |
| "coverage gaps", "unverified requirements", "what needs verification" | AnalyzeCoverage |
| "what changed", "impact of changes", "change impact" | ChangeImpact |
| "lint", "fix issues", "redundant verifications", "clean up" | Lint |

## Quick Diagnostic Commands

```bash
# Full validation
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate --json --output /tmp/validation.json

# Coverage summary
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json --output /tmp/coverage.json

# Lint check
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint --json --output /tmp/lint.json

# Auto-fix lint issues
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" lint --fix
```

Use `/tmp` for all JSON outputs.
