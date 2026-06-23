# Elements

### Contract Bindings Identifier CRUD Update Behavior

When a Contract element is moved or renamed through CRUD operations, all contract_bindings identifiers referencing that element must be updated using the same update mechanism as relation target updates.

#### Details
The update process follows these steps:

1. **Identify affected contract_bindings**: Find all elements that have contract_bindings identifiers pointing to the affected Contract element
2. **Update identifier paths**: For each affected contract_bindings:
   - **On move**: Update the file path portion of the identifier to reflect the new location
   - **On rename**: Update the element name portion of the identifier (fragment) to reflect the new name
3. **Preserve link text**: The display text of the markdown link is preserved
4. **File persistence**: Modified files are written back to disk with updated contract_bindings

This behavior mirrors the existing relation target update behavior used when moving or renaming elements, ensuring consistency across the model.

#### Metadata
  * type: behavior
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

### Requirement Governance Metadata Inheritance Behavior

When requirement governance metadata is missing from a requirement element, the model resolves effective governance metadata from the nearest requirement ancestor or from the governance metadata defaults.

#### Details
Resolution follows this order for each governance metadata key independently:

1. **Explicit value**: Use the value authored on the current element in `#### Metadata`.
2. **Inherited value**: If the current element omits the key, walk requirement hierarchy ancestors through `derivedFrom` / `derive` relations and use the nearest explicit value.
3. **Default value**: If no ancestor defines the key, use the default value from the Requirement Governance Metadata Specification.

Contract elements do not participate in governance metadata inheritance as metadata authors. When model evidence needs governance context for a contract element, the context shall be resolved from the contract's directly owning requirement.

The model shall retain enough source information for consumers to distinguish explicit, inherited, and default effective values.

Formatting and rewrite operations shall not insert inherited or default values into Markdown source. Only explicit metadata authored by the user, or explicitly changed by a mutation operation, shall be persisted.

#### Metadata
  * type: behavior
---

### Subdirectory Auto-Detection Behavior

Describes how subdirectory scope detection and enforcement works.

#### Details
**Detection Steps:**

1. **Detect git root**: Run `git rev-parse --show-toplevel` to find repository root
2. **Determine relative scope**: Calculate current working directory path relative to git root
3. **Limit file processing**: Only process specification files within the current subdirectory
4. **Validate references**: References to elements outside the subdirectory scope generate missing target errors

**When run from git root:**
- Process all files in the repository
- No scope limitations apply

This behavior enables focused work on specific areas of large models while maintaining reference integrity.

#### Metadata
  * type: behavior
---
