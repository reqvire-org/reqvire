# ElementManipulation

## Element Manipulation

### Element Manipulation File Persistence

The system shall persist all element manipulation operations to the source files in storage, synchronizing changes from the in-memory model to the file system to ensure data durability.

#### Details
When element manipulation operations are performed, the system shall:
- Track which files have been modified during manipulation operations
- Flush modified files to storage after manipulation operations complete
- Update only the files that were actually modified (optimization)
- Ensure file content on disk reflects the current state of the in-memory model
- Maintain file format and structure during write operations
- Handle file I/O errors gracefully with appropriate error reporting

**Optimization Strategy:**
- The system may maintain a list of modified files during manipulation operations
- Only files marked as modified need to be written to storage
- Unmodified files shall not be rewritten to avoid unnecessary I/O operations

**Synchronization Guarantee:**
- After a manipulation operation completes successfully, all changes shall be persisted to disk
- The on-disk representation shall match the in-memory model state
- No changes shall be lost due to lack of persistence

#### Relations
  * derivedFrom: [Element Manipulation Operations](../ModelManagement.md#element-manipulation-operations)
  * satisfiedBy: [graph_registry.rs](../../core/src/graph_registry.rs)
  * satisfiedBy: [crud.rs](../../core/src/crud.rs)
---

### Target Location Validation and Auto-Creation

The system shall validate target file paths for element manipulation operations and automatically create files and sections when they do not exist, subject to path safety constraints.

#### Details
When validating and preparing target locations, the system shall:

**Path Validation:**
- Verify the target file path is not excluded by `.gitignore` patterns
- Verify the target file path is not excluded by `.reqvireignore` patterns
- Verify the file path nesting depth does not exceed 10 subdirectories from the git repository root
- Reject operations with invalid paths and provide clear error messages

**Auto-Creation:**
- If the target file does not exist and the path is valid, create the file with proper structure:
  - Add level 1 header based on filename (e.g., `# Requirements` for `Requirements.md`)
  - Add level 2 section header if section name is provided
- If the target file exists but the specified section does not exist, add the section header
- Ensure created files and sections follow Reqvire markdown structure conventions

**Error Handling:**
- Report error if path would be ignored by `.gitignore` or `.reqvireignore`
- Report error if path nesting exceeds 10 subdirectories
- Report error if file path is invalid or inaccessible
- Provide specific error message indicating which constraint was violated

#### Relations
  * derivedFrom: [Element Manipulation Operations](../ModelManagement.md#element-manipulation-operations)
  * derivedFrom: [Ignore Files Integration](../ReqvireTool/Configuration.md#ignore-files-integration)
  * derivedFrom: [Git Repository as Project Root](../ModelManagement.md#git-repository-as-project-root)
  * satisfiedBy: [utils.rs](../../core/src/utils.rs)
  * satisfiedBy: [graph_registry.rs](../../core/src/graph_registry.rs)
---

### Create Element Operation

The system shall provide the capability to create new model elements by accepting a full element definition string in Markdown format, validating the element structure and relations, and inserting it into the specified location if valid.

#### Details
When creating a new element, the system shall:
- Accept a string containing the full element definition in Markdown format (including ### header, metadata, relations, and content)
- Accept target location: file path and section name
- Accept optional index parameter for insertion position within section (0-based)
- Validate the target location using path validation rules
- Create target file and/or section if they do not exist (subject to validation constraints)
- Parse the element definition string to extract element structure
- Validate the element structure (proper subsections, valid relations, correct format)
- Verify the element name is unique within the target file
- Generate a unique element identifier based on file path and element name
- **Validate and normalize all relations in the element:**
  - Parse relation targets from the markdown (may be relative paths or repo-relative paths)
  - Normalize relation targets to be relative to the git repository root
  - Validate that each relation target element exists in the model
  - Reject the operation if any relation target does not exist
  - Provide clear error messages indicating which relation target was not found
- If validation passes, insert the element into the specified file and section:
  - If index is provided and valid, insert at that position within the section
  - If index is not provided or out of bounds, append to the end of the section
- If validation fails, reject the operation and report validation errors
- Maintain file structure and formatting after insertion

**Relation Validation Rules:**
- Relation targets may be specified as:
  - Relative paths from the target file location (e.g., `../UserReqs.md#requirement`)
  - Paths relative to git repository root (e.g., `specifications/UserReqs.md#requirement`)
  - Same-file references (e.g., `#other-requirement`)
- All relation targets must be normalized to git repository root relative format before insertion
- All relation targets must reference existing elements in the model
- External links (http://, https://, etc.) are allowed and not validated

#### Relations
  * derivedFrom: [Element Manipulation File Persistence](#element-manipulation-file-persistence)
  * derivedFrom: [Target Location Validation and Auto-Creation](#target-location-validation-and-auto-creation)
  * derivedFrom: [Structure and Addressing in Markdown Documents](../ModelStructure.md#structure-and-addressing-in-markdown-documents)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)
  * satisfiedBy: [graph_registry.rs](../../core/src/graph_registry.rs)
  * satisfiedBy: [crud.rs](../../core/src/crud.rs)
  * satisfiedBy: [diff.rs](../../core/src/diff.rs)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * satisfiedBy: [utils.rs](../../core/src/utils.rs)
---

### Delete Element Operation

The system shall provide the capability to delete existing model elements while automatically removing or updating all relations that reference the deleted element, and removing empty files when no elements remain.

#### Details
When deleting an element, the system shall:
- Remove the element and all its content from the source file
- Identify all relations pointing to the deleted element (incoming relations)
- Remove all relations that reference the deleted element from other elements
- Identify all relations from the deleted element (outgoing relations)
- Remove the complete element section including separators
- Maintain file structure and formatting after deletion
- Provide a report of all relations that were affected by the deletion

**Empty File Cleanup:**
- After deleting the element, check if the source file contains any remaining elements
- If no elements remain and all sections are empty (only page content, headers, or whitespace), remove the file from the filesystem
- If the file is removed, report the file deletion in the operation output

**Relation Handling:**
- All `derivedFrom` relations pointing to the deleted element shall be removed
- All `verifiedBy` relations pointing to the deleted element shall be removed
- All `verify` relations pointing to the deleted element shall be removed
- All `satisfiedBy` relations pointing to the deleted element shall be removed
- Relations from the deleted element are automatically removed with the element

#### Relations
  * derivedFrom: [Element Manipulation File Persistence](#element-manipulation-file-persistence)
  * satisfiedBy: [graph_registry.rs](../../core/src/graph_registry.rs)
  * satisfiedBy: [crud.rs](../../core/src/crud.rs)
  * satisfiedBy: [diff.rs](../../core/src/diff.rs)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
---

### Move Element Operation

The system shall provide the capability to move existing model elements to different locations (file and/or section) while automatically updating all relations that reference the moved element, creating target locations if needed, and removing empty source files when no elements remain.

#### Details
When moving an element, the system shall:
- Validate the target location using path validation rules
- Create target file and/or section if they do not exist (subject to validation constraints)
- Remove the element from the source location (file and section)
- Accept optional index parameter for insertion position within target section (0-based)
- Insert the element into the target location (file and section):
  - If index is provided and valid, insert at that position within the target section
  - If index is not provided or out of bounds, append to the end of the target section
- Preserve all element content, metadata, and relations
- Update the element's identifier to reflect the new location
- Identify all relations pointing to the moved element (incoming relations)
- Update all relations that reference the moved element with the new identifier
- Maintain file structure and formatting in both source and target files
- Ensure the element name is unique within the target file
- Provide a report of all relations that were updated

**Empty Source File Cleanup:**
- After moving the element, check if the source file contains any remaining elements
- If no elements remain and all sections are empty (only page content, headers, or whitespace), remove the source file from the filesystem
- If the file is removed, report the file deletion in the operation output

**Relation Update Requirements:**
- All relations (both forward and backward) pointing to the moved element shall be updated to the new identifier
- Relations within the moved element (outgoing relations) shall be preserved unchanged

**Identifier Update:**
- The element's identifier changes from `<old-file>#<element-name>` to `<new-file>#<element-name>`
- All references to the old identifier shall be updated to the new identifier

#### Relations
  * derivedFrom: [Element Manipulation File Persistence](#element-manipulation-file-persistence)
  * derivedFrom: [Target Location Validation and Auto-Creation](#target-location-validation-and-auto-creation)
  * satisfiedBy: [graph_registry.rs](../../core/src/graph_registry.rs)
  * satisfiedBy: [crud.rs](../../core/src/crud.rs)
  * satisfiedBy: [diff.rs](../../core/src/diff.rs)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
---

### Move File Operation

The system shall provide the capability to move entire specification files with all their elements to a new location in the repository while updating all relation references throughout the model.

#### Details
When moving a file, the system shall:
- Accept source file path (relative to git repository root)
- Accept target file path (relative to git repository root)
- Accept optional squashing flag
- Validate both source and target paths
- Move the physical file from source to target location
- Update all element identifiers within the file to reflect the new file path
- Update all relation references (both forward and backward) throughout the model that point to any element in the moved file
- Preserve all file content, structure, and formatting

The system shall reject the operation with a clear error message if:
- The source file does not exist
- The target file already exists (unless --squash flag is provided)
- The source or target paths fail validation

**Squash Mode Behavior:**
When the --squash flag is provided and the target file already exists, the system shall:
- Move all elements from the source file to the target file's first section
- Remove the source file after all elements have been successfully moved
- Preserve element ordering from the source file when inserting into target section

#### Relations
  * derivedFrom: [Element Manipulation File Persistence](#element-manipulation-file-persistence)
  * derivedFrom: [Target Location Validation and Auto-Creation](#target-location-validation-and-auto-creation)
---

### Relation Consistency Maintenance

The system shall maintain bidirectional relation consistency when elements are manipulated, ensuring that forward and backward relations remain synchronized.

#### Details
When manipulating elements, the system shall ensure:
- If element A derives from element B, then B must have a derive relation to A
- If element A is verified by verification V, then V must have a verify relation to A
- When an element is deleted, both forward and backward relations are removed
- When an element is moved, both forward and backward relations are updated
- After any manipulation operation, the model remains in a valid state with no dangling relations

**Validation:**
- The system shall validate relation consistency after each manipulation operation
- The system shall report any inconsistencies detected during manipulation
- The system shall prevent operations that would leave the model in an inconsistent state

#### Relations
  * derivedFrom: [Element Manipulation Operations](../ModelManagement.md#element-manipulation-operations)
  * satisfiedBy: [graph_registry.rs](../../core/src/graph_registry.rs)
---

### Rename Element Operation

The system shall provide the capability to rename existing model elements by changing their heading text while updating all relation references and the registry.

#### Details
When renaming an element, the system shall:
- Accept the current element name and the new element name
- Validate that the current element exists in the model registry
- Validate that the new name is globally unique in the model registry
- Update the element's heading text in the markdown file
- Update all relation references (both forward and backward) to use the new element identifier
- Update the element identifier in the registry

The system shall reject the operation with a clear error message if:
- The element does not exist
- The new name conflicts with an existing element

#### Relations
  * derivedFrom: [Element Manipulation File Persistence](#element-manipulation-file-persistence)
  * satisfiedBy: [graph_registry.rs](../../core/src/graph_registry.rs)
  * satisfiedBy: [crud.rs](../../core/src/crud.rs)
  * satisfiedBy: [diff.rs](../../core/src/diff.rs)
---
