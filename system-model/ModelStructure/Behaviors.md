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

Describes how workspace scope detection and enforcement works when Reqvire starts from a directory that may be a Git root, a child of a Git root, or a parent of Git repositories.

#### Details
**Detection Steps:**

1. **Establish effective workspace root**: Use the process working directory after startup workspace selection has been applied.
2. **Discover eligible Git worktrees**: Detect the containing Git worktree when the workspace root is inside one, and detect descendant Git worktrees under the workspace root.
3. **Collect Git metadata**: Record revision, branch, remote, dirty-state, and source-control metadata for eligible worktrees.
4. **Limit file processing**: Only process specification files, local assets, implementation files, and evidence artifacts that are both inside the effective workspace root and inside an eligible Git worktree.
5. **Ignore non-Git folders**: Ignore every workspace-root descendant folder that is outside all eligible Git worktrees.
6. **Normalize paths**: Store identifiers, InternalPath targets, diagnostics, reports, exports, and consumer records as workspace-root-relative paths.
7. **Validate references**: References to elements outside the workspace root or outside all eligible Git worktrees generate missing target errors.

**When run from a Git root:**
- Process files under that directory.
- Workspace-relative identifiers match the legacy single-repository path shape.

**When run from a child of a Git root:**
- Process files under the child directory only.
- Parent repository content is outside the Reqvire workspace unless the process is started from a higher directory.
- Identifiers do not include the child directory prefix because the child directory is the workspace root.

**When run from a parent of one or more Git repositories:**
- Process files under descendant Git repositories only.
- Ignore non-Git folders under the parent workspace root.
- Nested repository directories are ordinary workspace subdirectories for Reqvire addressing.
- Git metadata for nested repositories may be reported separately, but it does not change identifier roots.

**When run from a non-Git directory with no descendant Git repositories:**
- No files are eligible for model parsing or local artifact inclusion.
- Commands that require a model report an empty or missing model according to their normal diagnostics.

This behavior enables focused work on specific areas of large models while maintaining reference integrity.

#### Metadata
  * type: behavior
---
