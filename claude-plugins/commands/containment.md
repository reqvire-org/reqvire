---
allowed-tools: Read, Bash(npx:*)
description: Analyze model's containment structure (folders, files, and elements) to understand organization and suggest improvements
model: claude-sonnet-4-5
---

# Containment View

Analyze the physical structure of your Reqvire model to understand how elements are organized across folders and files, identify organizational issues, and suggest improvements.

## Current Model Context

- Total elements: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '.global_counters.total_elements'`
- Total files: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '.global_counters.total_files'`

## Steps

1. **Get containment structure in JSON format:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" containment --json
   ```

   This provides the hierarchical structure showing:
   - Folder hierarchy
   - Files within each folder
   - Elements within each file (**filtered to show only top-level parents** - elements without hierarchical parent relations in same file)
   - Element types and identifiers

   **Note:** Element count in containment view does not represent total file size. Use `reqvire search --filter-file="path" --json` to get actual element counts per file.

2. **Get actual element counts per file:**
   ```bash
   # Count total elements per file
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '.files | to_entries[] | "\(.key): \(.value.total_elements) elements"'

   # Find files with many elements (>20)
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '.files | to_entries[] | select(.value.total_elements > 20) | "\(.key): \(.value.total_elements) elements"'

   # Find files with few elements (<3)
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '.files | to_entries[] | select(.value.total_elements < 3) | "\(.key): \(.value.total_elements) elements"'
   ```

3. **Analyze the structure:**
   - Identify files with too many elements (>20 suggests need for splitting)
   - Find files with too few elements (<3 suggests potential for consolidation)
   - Check if folder structure reflects logical organization
   - Verify naming conventions are consistent
   - Look for orphaned or misplaced files

4. **Generate text diagram for visualization:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" containment
   ```

   This creates a Mermaid graph showing the containment tree structure.

5. **Provide recommendations:**
   Based on the analysis, suggest:
   - Files that should be split (too many elements)
   - Files that could be merged (too few elements)
   - Better folder organization
   - Naming improvements
   - Structural refactoring using mv-file or mv commands

## Understanding Containment vs. Traceability

**Containment** (physical structure):
- How elements are organized in folders and files
- File system hierarchy
- Element grouping within files
- Use for: Reorganizing, refactoring, improving maintainability

**Traceability** (logical relationships):
- How elements relate through derivedFrom, verify, satisfiedBy, refinedBy relations
- Requirement flow-down and verification chains
- Use for: Understanding dependencies, impact analysis, coverage

## When to Use Containment View

Use containment analysis when:
- **Model organization needs assessment** - Check if file structure makes sense
- **Refactoring planning** - Decide which files to split, merge, or reorganize
- **Onboarding** - Help new team members understand model structure
- **Finding organizational issues** - Locate files that are too large, too small, or misplaced
- **Planning mv-file operations** - Understand current structure before reorganizing

## Analysis Questions to Ask

1. **Are files too large?**
   - Files with >20 elements are hard to navigate
   - Consider splitting by capability, subsystem, or level

2. **Are files too small?**
   - Files with 1-2 elements may indicate over-fragmentation
   - Consider using --squash to consolidate

3. **Does folder structure make sense?**
   - Are related requirements grouped together?
   - Do folder names reflect content?
   - Is hierarchy appropriate (not too deep, not too flat)?

4. **Are elements well-distributed?**
   - Look for imbalanced distribution across files
   - Check if related elements are in same file

5. **Is naming consistent?**
   - File naming conventions followed?
   - Folder names descriptive and standardized?

## Example Analysis Workflow

```bash
# 1. Get containment structure (folder/file hierarchy)
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" containment --json --output /tmp/containment.json

# 2. Get actual element counts per file
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '.files | to_entries[] | "\(.key): \(.value.total_elements) elements"' > /tmp/file_sizes.txt

# 3. Find files with many elements (candidates for splitting)
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '.files | to_entries[] | select(.value.total_elements > 20) | "\(.key): \(.value.total_elements) elements"'

# 4. Find files with few elements (candidates for consolidation)
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '.files | to_entries[] | select(.value.total_elements < 3) | "\(.key): \(.value.total_elements) elements"'

# 5. Visualize folder structure
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" containment

# 6. Plan refactoring based on findings
```

## Recommendations Format

Provide clear, actionable recommendations:

**Files to Split:**
- `requirements/LargeFile.md` (45 elements) → Consider splitting into:
  - `requirements/Capability1.md` (requirements for Capability 1)
  - `requirements/Capability2.md` (requirements for Capability 2)

**Files to Consolidate:**
- `temp/A.md` (2 elements) + `temp/B.md` (3 elements) → Use:
  ```bash
  npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-file "temp/B.md" "temp/A.md" --squash
  ```

**Folder Reorganization:**
- Move security-related files from `requirements/` to new `security/` folder

## Related Commands

- **Move file**: `/reqvire:mv-file` - Reorganize files based on analysis
- **Move element**: `/reqvire:mv` - Move individual elements between files
- **Analyze model**: `/analyze-model` - Check logical model structure and relationships
- **Search**: `reqvire search` - Find specific elements to understand placement
