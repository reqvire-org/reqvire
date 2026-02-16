---
name: reqvire-syseng
description: MBSE and requirements engineering workflow for Reqvire. Use when exploring a Reqvire model, adding or refactoring requirements/specifications/verifications, running impact/coverage analysis, and generating implementation tasks with full traceability.
---

# Reqvire System Engineering Skill

Use this skill for any requirements, specifications, verifications, or system-model work in Reqvire.

## When To Use

- Exploring and understanding existing Reqvire models
- Adding features in MBSE order
- Refactoring or consolidating requirements/specifications
- Analyzing change impact and verification coverage
- Generating implementation tasks from model changes

## Core Rules

1. Work from repository root.
2. Prefer `reqvire` CLI commands over manual markdown edits.
3. MBSE-first sequence is mandatory:
   - Requirements
   - Refinements (specifications/constraints/behaviors)
   - Verifications
   - Code implementation links (`satisfiedBy`)
4. Validate after meaningful changes:
   - `reqvire validate`
   - `reqvire lint`
   - `reqvire coverage`
5. Use `reqvire collect` when implementing or reviewing requirements with trace dependencies.

## Quick Start: Common Workflows

1. Check tool availability:
   - `reqvire --version`
2. Explore current model:
   - `reqvire search --short --json | jq '.summary'`
   - `reqvire model`
3. Apply change using reqvire commands (`add`, `link`, `unlink`, `mv`, `rename`, `rm`).
4. Run validation and coverage.
5. Summarize what changed and what still needs action.

## Command Reference

```bash
# Explore
reqvire search --short --json | jq '.summary'
reqvire model
reqvire collect "Requirement Name" --json

# Validate quality
reqvire validate
reqvire lint
reqvire coverage

# Impact analysis
reqvire change-impact --git-commit="$(git merge-base main HEAD)"
```

## Validation & Quality Checklist

Run these after each meaningful change:

```bash
reqvire validate
reqvire lint
reqvire coverage
```

## References

Load only the reference file needed for the current task:

- `references/explore.md` - model exploration and advanced search patterns
- `references/AddFeature.md` - MBSE feature creation workflow
- `references/ConsolidateRequirements.md` - model refactoring and cleanup
- `references/CreatingTasks.md` - generate implementation tasks from requirement changes
- `references/SpecificationsExtractionLogic.md` - extract technical specs from requirements
