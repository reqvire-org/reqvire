# Elements

### Create Element Override Behavior

When the create element operation is invoked with override mode:
1. The system shall extract the element name from the markdown input (### Element Name pattern)
2. If an element with that name exists in the model, the system shall remove it first
3. The system shall then add the new element content to the target file
4. The operation shall be reported as "Update" rather than "Add"

#### Metadata
  * type: behavior

#### Relations
  * satisfy: [Create Element Operation](ElementManipulation.md#create-element-operation)
---

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

### Format Duplicate Removal Behavior

Deduplication rules for format fix operations.

**Relations subsection:**
- Duplicate relations (same relation_type + same target) are removed, keeping first occurrence
- Different relation types to same target are NOT duplicates (e.g., `derivedFrom: A` and `trace: A` are both kept)

**Attachments subsection:**
- Duplicate attachments (same target path or identifier) are removed, keeping first occurrence

**Cross-section duplicates are NOT removed** - these require user decision and are reported by validation.

#### Metadata
  * type: behavior

#### Relations
  * satisfy: [Format Duplicate Removal](Formatting.md#format-duplicate-removal)
---

### Merge Content Transformation Behavior

Content transformation rules for the merge elements operation.

#### Details
**Content Merging:**
1. For each source element in order:
   - If source has main content (before any #### subsection): append to target's `#### Details`
   - If source has `#### Details`: create `#### Merged Details (Source Name)` with that content
2. Preserve target's original content structure
3. Merged content appears after target's existing Details section (or creates one)

**Relation Deduplication:**
- Relations are deduplicated by (relation_type, target) pair
- First occurrence is kept (target's relations take precedence)
- Different relation types to same target are NOT duplicates

**Attachment Deduplication:**
- Attachments are deduplicated by target identifier/path
- First occurrence is kept (target's attachments take precedence)

**Cross-Section Duplicate Check:**
- Before merge completes, validate no target appears in both Relations AND Attachments
- If cross-section duplicate detected, abort merge with error listing duplicates
- User must resolve by removing one of the duplicates before retrying

**Relation Redirection:**
- Find all elements with relations pointing to source elements
- Update those relations to point to target element's identifier
- This includes both forward and backward relations

#### Metadata
  * type: behavior

#### Relations
  * satisfy: [Merge Element Operation](ElementManipulation.md#merge-element-operation)
---
