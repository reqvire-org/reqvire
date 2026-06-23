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
- Different relation types to same target are NOT duplicates (e.g., `derivedFrom: A` and `trace: A` are both kept)

**Contract Bindings subsection:**
- Duplicate contract_bindings (same target path or identifier) are removed, keeping first occurrence

**Cross-section duplicates are NOT removed** - these require user decision and are reported by validation.

#### Metadata
  * type: behavior
---
