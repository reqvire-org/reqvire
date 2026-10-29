# Elements

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

### Format Duplicate Removal Behavior

Deduplication rules for format fix operations.

**Relations subsection:**
- Duplicate relations (same relation_type + same target) are removed, keeping first occurrence
- Different registered relation types are not duplicates solely because their targets agree; the complete candidate must still satisfy type compatibility and cycle rules.

**Contract Bindings subsection:**
- Duplicate contract_bindings (same target path or identifier) are removed, keeping first occurrence

**Shared duplicate classification:**
- Apply the classification referenced by the owning requirement. The shared writer deduplicates equivalent entries in valid formatting input. Invalid duplicate declarations in disk sources or mutation candidates remain validation errors; formatting does not bypass model admission.
- Cross-section duplicates are semantic errors and are not automatically removed. Report the shared validation diagnostic and preserve both declarations until the author chooses a valid repair.

#### Metadata
  * type: behavior
---
