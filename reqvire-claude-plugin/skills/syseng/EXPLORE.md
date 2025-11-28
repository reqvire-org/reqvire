# Exploring and Understanding the Model

Use this reference when you need to understand the model structure, search for requirements, or browse the specifications.

## Quick Model Overview

```bash
# Get model summary with element counts
reqvire search --short --json | jq '.summary'

# List all files with elements
reqvire search --short --json | jq '.files[].file_path'

# Check model health
reqvire validate
reqvire coverage
```

## Searching Elements

### By Type
```bash
# Find all user requirements
reqvire search --filter-type="user-requirement" --short

# Find all system requirements
reqvire search --filter-type="system-requirement" --short

# Find all verifications
reqvire search --filter-type="verification" --short

# Find specifications/constraints/behaviors
reqvire search --filter-type="specification" --short
reqvire search --filter-type="constraint" --short
```

### By File Location
```bash
# Elements in specific folder
reqvire search --filter-file="requirements/System/**" --short

# Elements in specific file
reqvire search --filter-file="**/UserStories.md" --short
```

### By Name or Content
```bash
# Find by name pattern
reqvire search --filter-name=".*Authentication.*" --short

# Find by content pattern
reqvire search --filter-content="SHALL.*validate" --short
```

### By Relations
```bash
# Find unverified requirements
reqvire search --filter-type="requirement" --not-have-relations="verifiedBy" --short

# Find unsatisfied verifications (test type without implementation)
reqvire search --filter-type="test-verification" --not-have-relations="satisfiedBy" --short

# Find elements with specific relations
reqvire search --have-relations="verifiedBy,satisfiedBy" --short

# Find specifications not linked to any requirement
reqvire search --filter-type="specification" --not-have-relations="satisfy" --short
```

### By Attachments
```bash
# Find elements with attachments
reqvire search --has-attachments --short

# Find elements with specific attachment type
reqvire search --filter-attachment="*.pdf" --short
reqvire search --filter-attachment="**/DesignDocuments/**" --short
```

## Understanding Traceability

```bash
# See verification traces (upward from verifications to root requirements)
reqvire traces

# See verification coverage report
reqvire coverage

# See all files referenced by model (implementations, design docs)
reqvire resources
```

## Understanding Element Details

When you find an element of interest:
1. Read the full element content (not just --short output)
2. Check for **attachments** - they contain critical details
3. Follow relations to understand context:
   - `derivedFrom` → parent requirements (why this exists)
   - `satisfiedBy` → implementations (how it's fulfilled)
   - `verifiedBy` → verifications (how it's tested)

## Containment View

```bash
# See physical file/folder structure
reqvire containment

# JSON format for programmatic analysis
reqvire containment --json
```

## Common Analysis Patterns

### Find gaps in verification
```bash
# Unverified leaf requirements
reqvire search --filter-type="requirement" --not-have-relations="verifiedBy" --short

# Check coverage percentage
reqvire coverage --json | jq '.summary'
```

### Find orphaned elements
```bash
# Specifications not satisfying any requirement
reqvire search --filter-type="specification" --not-have-relations="satisfy" --short --json

# Verifications not verifying any requirement
reqvire search --filter-type="verification" --not-have-relations="verify" --short
```

### Understand a feature area
```bash
# Start from user requirement, follow derivations
reqvire model --from "Feature Name"

# See all elements in feature's folder
reqvire search --filter-file="**/FeatureName/**" --short
```
