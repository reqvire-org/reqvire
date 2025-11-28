# Elements

### Dry-Run Mode Behavior

Preview mode behavior for modification commands.

#### Details
- Show changes without applying by default
- Require --fix flag to apply changes
- Show diff output for changes
- Exit 0 for successful preview

#### Metadata
  * type: behavior
---

### Element Ordering Behavior

Ordering algorithm for elements within specification files during formatting and file persistence operations.

#### Details
**Ordering Principle:**
Elements shall be ordered so that parent elements appear before their children (file-local derivedFrom hierarchy).

**Algorithm:**
1. **Identify Root Elements**: Elements with no file-local `derivedFrom` relations (parents are in other files or none)
2. **Group by Hierarchy**: Group elements by their file-local parent chains
3. **Sort Root Groups**: Sort root element groups alphabetically by root element name
4. **Order Within Groups**: Within each group, order elements topologically:
   - Parent element first
   - Direct children next (sorted alphabetically among siblings)
   - Grandchildren after their parents (recursively)

**Example:**
```
# Before (unordered):
- Child B (derivedFrom: Parent A)
- Grandchild Z (derivedFrom: Child A)
- Parent A
- Grandchild M (derivedFrom: Child A)
- Child A (derivedFrom: Parent A)
- Standalone Element

# After (ordered):
- Parent A              <- Root, alphabetically first among roots with children
- Child A               <- Child of Parent A, alphabetically first sibling
- Grandchild M          <- Child of Child A, alphabetically first grandchild
- Grandchild Z          <- Child of Child A, alphabetically second grandchild
- Child B               <- Child of Parent A, alphabetically second sibling
- Standalone Element    <- Root with no children, alphabetically after Parent A
```

**Scope:**
- Only considers `derivedFrom` relations pointing to elements in the same file
- Cross-file relations do not affect ordering
- Elements without file-local hierarchy remain as independent roots

#### Metadata
  * type: behavior
---

### File Persistence Behavior

How element manipulation operations persist changes to files:
- Track modified files during operations
- Write only modified files to storage
- Maintain file format and structure
- Handle I/O errors with reporting

**Synchronization:**
- On-disk matches in-memory after success
- No partial changes on error

#### Metadata
  * type: behavior
---

### Target Location Constraint

Validation rules for target paths in element operations.

**Path Validation:**
- Path not excluded by .gitignore/.reqvireignore
- Maximum 10 subdirectory nesting depth
- Path must be accessible and writable

**Auto-creation:**
- Create missing target files with `# Elements` header

**Error Reporting:**
- Clear message indicating which constraint was violated

#### Metadata
  * type: constraint
---
