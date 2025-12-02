# Model Refactoring & Optimization

Use this reference when reorganizing the model structure without changing requirements intent. The goal is better organization, traceability, and maintainability.

**Key principle**: The system behavior specification remains unchanged - only the model structure improves.

## Refactoring Activities

- Extracting inline constraints/specifications into dedicated elements
- Converting attachments to `satisfiedBy` relations where appropriate
- Merging duplicate/overlapping requirements
- Splitting mixed-type requirements (user vs system)
- Moving elements between files for better organization
- Adding missing relations (satisfiedBy, derivedFrom)
- Removing redundant verify relations
- Consolidating scattered specifications
- **Reordering elements**: Parents at top, children at bottom (follow derivedFrom hierarchy)

## Workflow

### Step 1: Audit Specifications Without Relations

Find specifications not linked to any requirement:

```bash
reqvire search --filter-type='specification' --not-have-relations='satisfy' --short --json
```

These need to be linked via `satisfiedBy` from appropriate requirements.

### Step 2: Find Requirements Asking for Specifications

Search for patterns like:
- "following clearly defined specifications"
- "adhering to precondition rules"
- "shall support distinct states"
- "shall implement constraints"
- "shall enforce standardized policies"

These requirements should have `satisfiedBy` relations to the specifications they ask for.

### Step 3: Convert Attachments to satisfiedBy Where Appropriate

For each specification attachment, ask:
- Does this requirement *define* this specification? → Use `satisfiedBy`
- Does this requirement *reference* or *depend on* this specification? → Keep as `Attachment`

### Step 4: Consolidate Constraints

Find a root requirement that asks for constraints to be defined. Add `satisfiedBy` relations to all constraint elements.

### Step 5: Validate After Each Change

```bash
reqvire validate
reqvire lint --fix
```

## Consolidating Specifications

### Extract inline constraints into Constraints.md

Find requirements with hardcoded limits and extract them as constraint elements:

1. Create constraint elements in `requirements/Specifications/Constraints.md`
2. Attach the constraint to requirements that reference the limit
3. Link constraint to parent requirement via `satisfiedBy`

**Examples of hardcoded limits to extract:**

*Web App:*
- "Session expires after 30 minutes of inactivity"
- "Maximum file upload size: 10 MB"
- "Password must be 8-128 characters"

*API:*
- "Rate limit: 100 requests per minute"
- "Maximum payload size: 1 MB"
- "Token expiration: 24 hours"

*Database:*
- "Connection pool maximum: 20 connections"
- "Query timeout: 30 seconds"

### Extract inline specifications

Find requirements with detailed specifications in Details section:

1. Create specification elements in `requirements/Specifications/*Specifications.md`
2. Use `satisfiedBy` relation from the requirement

## Identifying Missing Relations

Look for specification elements with empty relations. For each:

1. Find which requirement asks for this specification to be defined
2. Change the attachment to a `satisfiedBy` relation on that requirement
3. Keep attachments on other requirements that just reference (don't define) the specification

## Merging Duplicate Requirements

When you find overlapping requirements, use `reqvire merge`:

```bash
# Preview the merge
reqvire merge "Primary Requirement" "Duplicate Requirement" --dry-run

# Execute the merge
reqvire merge "Primary Requirement" "Duplicate Requirement"

# Merge multiple sources at once
reqvire merge "Main Feature" "Feature Part A" "Feature Part B"
```

The merge command:
1. Consolidates content from duplicate into primary's Details section
2. Merges all relations (verifiedBy, derivedFrom, etc.) with deduplication
3. Updates all references pointing to duplicate to point to primary
4. Deletes the duplicate element

**Type compatibility:**
- Requirements merge with requirements (all subtypes compatible)
- Verifications merge with verifications
- Refinements (constraint, behavior, specification) merge with each other

**When to merge vs when to link:**
- **Merge**: Elements express the same capability (duplicates)
- **Link (derivedFrom)**: Elements have hierarchical relationship (parent/child)
- **Link (trace)**: Elements are related but represent distinct capabilities

**Post-merge cleanup**: If the merged result needs content restructuring (removing "Merged Details" artifacts), use `/reqvire:consolidate` to read, fix, and override with clean content.

## When to Split Requirements (using derivedFrom)

**1. Type separation** - Don't mix requirement types:
- User requirements (stakeholder needs) should not contain system requirements (technical details)
- Split when a requirement mixes "what users need" with "how the system implements it"

**2. Change impact & containment:**
- Scope isolation - Changes shouldn't require re-verification of unrelated aspects
- Independent verification - Each child can be verified separately
- Different ownership - Parts owned by different teams
- Risk isolation - Separate high-risk from stable parts

**3. Granularity:**
- Atomic testability - Each requirement maps to clear pass/fail verification
- Single responsibility - One requirement = one clear purpose
- Stop splitting when further decomposition adds overhead without value

## Example: Before and After

### Before (Attachment):
```markdown
### API Authorization Specification

The system shall implement API Access Authorization following clearly defined specifications.

#### Metadata
  * type: user-requirement

#### Attachments
  * [Authorization System Specification](../Specifications/AuthSpecifications.md#authorization-system-specification)
```

### After (satisfiedBy):
```markdown
### API Authorization Specification

The system shall implement API Access Authorization following clearly defined specifications.

#### Metadata
  * type: user-requirement

#### Relations
  * satisfiedBy: [Authorization System Specification](../Specifications/AuthSpecifications.md#authorization-system-specification)
```

### Constraint Consolidation:

Root requirement with satisfiedBy:
```markdown
### Operational Constraints

The system shall implement operational constraints and rate limits.

#### Metadata
  * type: user-requirement

#### Relations
  * satisfiedBy: [Rate Limits](../Specifications/Constraints.md#rate-limits)
  * satisfiedBy: [Session Limits](../Specifications/Constraints.md#session-limits)
```

Referencing requirement (keeps attachment):
```markdown
### Add IP to Whitelist

The system shall allow adding IPs to whitelist.

#### Attachments
  * [Environment Limits](../Specifications/Constraints.md#environment-limits)
```

## Git Philosophy

**IMPORTANT**: When removing or changing requirements:
- **DELETE** deprecated requirements completely
- **REMOVE** broken relations
- **USE** git history to track changes (commit messages explain rationale)
- **NEVER** keep "DEPRECATED" notes or "Previous behavior" documentation
- Clean specifications are more valuable than inline deprecation notes
