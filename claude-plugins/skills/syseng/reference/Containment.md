# Containment Analysis

Analyze the physical structure of a Reqvire model to understand how elements are organized across folders and files, identify organizational issues, and suggest improvements.

This reference is for **read-only inspection**. For executing a containment refactor (moving files, reorganizing structure), see [ContainmentStructureRefactor](ContainmentStructureRefactor.md).

## Steps

1. **Check current model context:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '{total_elements: .global_counters.total_elements, total_files: .global_counters.total_files}'
   ```

2. **Get containment structure in JSON format:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" containment --json
   ```

   This provides the hierarchical structure showing:
   - Folder hierarchy
   - Files within each folder
   - Elements within each file (filtered to show only top-level parents)
   - Element types and identifiers

   **Note:** Element count in containment view does not represent total file size. Use `reqvire search --filter-file="path" --json` to get actual element counts per file.

3. **Get actual element counts per file:**
   ```bash
   # Count total elements per file
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '.files | to_entries[] | "\(.key): \(.value.total_elements) elements"'

   # Find files with many elements (>20)
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '.files | to_entries[] | select(.value.total_elements > 20) | "\(.key): \(.value.total_elements) elements"'

   # Find files with few elements (<3)
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json | jq -r '.files | to_entries[] | select(.value.total_elements < 3) | "\(.key): \(.value.total_elements) elements"'
   ```

4. **Analyze the structure:**
   - Identify files with too many elements (>20 suggests need for splitting)
   - Find files with too few elements (<3 suggests potential for consolidation)
   - Check if folder structure reflects logical organization
   - Verify naming conventions are consistent
   - Look for orphaned or misplaced files

5. **Generate text diagram for visualization:**
   ```bash
   npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" containment
   ```

   This creates a Mermaid graph showing the containment tree structure.

6. **Provide recommendations:**
   Based on the analysis, suggest:
   - Files that should be split (too many elements)
   - Files that could be merged (too few elements)
   - Better folder organization
   - Naming improvements
   - Structural refactoring using mv, mv-file, or mv-folder commands

## Understanding Containment vs. Traceability

**Containment** (physical structure):
- How elements are organized in folders and files
- File system hierarchy
- Element grouping within files
- Use for: Reorganizing, refactoring, improving maintainability

**Traceability** (logical relationships):
- How elements relate through derivedFrom, verify, satisfiedBy, definedBy relations
- Requirement flow-down and verification chains
- Use for: Understanding dependencies, impact analysis, coverage

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

## Recommendations Format

Provide clear, actionable recommendations:

**Files to Split:**
- `system-model/LargeFile.md` (45 elements) → Consider splitting into:
  - `system-model/Capability1.md` (requirements for Capability 1)
  - `system-model/Capability2.md` (requirements for Capability 2)

**Files to Consolidate:**
- `temp/A.md` (2 elements) + `temp/B.md` (3 elements) → Use:
  ```bash
  npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-file "temp/B.md" "temp/A.md" --squash
  ```

**Folder Reorganization:**
- Move security-related files from `system-model/` to new `security/` folder
  ```bash
  npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" mv-folder "system-model/SecurityDrafts" "system-model/Security"
  ```

## When to Use Containment Analysis

Use containment analysis when:
- **Model organization needs assessment** - Check if file structure makes sense
- **Refactoring planning** - Decide which files to split, merge, or reorganize
- **Onboarding** - Help new team members understand model structure
- **Finding organizational issues** - Locate files that are too large, too small, or misplaced
- **Planning mv-file or mv-folder operations** - Understand current structure before reorganizing

For executing the refactor, see [ContainmentStructureRefactor](ContainmentStructureRefactor.md).
