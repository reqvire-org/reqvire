# Element Manipulation

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
  * derivedFrom: [Element Manipulation Operations](../../UserRequirements.md#element-manipulation-operations)
---

### Create Element Operation

The system shall provide the capability to create new model elements by accepting a full element definition string in Markdown format, validating the element structure, and inserting it into the specified location if valid.

#### Details
When creating a new element, the system shall:
- Accept a string containing the full element definition in Markdown format (including ### header, metadata, relations, and content)
- Accept target location: file path and section name
- Accept optional index parameter for insertion position within section (0-based)
- Parse the element definition string to extract element structure
- Validate the element structure (proper subsections, valid relations, correct format)
- Verify the element name is unique within the target file
- Generate a unique element identifier based on file path and element name
- If validation passes, insert the element into the specified file and section:
  - If index is provided and valid, insert at that position within the section
  - If index is not provided or out of bounds, append to the end of the section
- If validation fails, reject the operation and report validation errors
- Maintain file structure and formatting after insertion

#### Relations
  * derivedFrom: [Element Manipulation File Persistence](#element-manipulation-file-persistence)
  * derivedFrom: [Structure of Markdown Documents](../../SpecificationsRequirements.md#structure-of-markdown-documents)
---

### Delete Element Operation

The system shall provide the capability to delete existing model elements while automatically removing or updating all relations that reference the deleted element.

#### Details
<details>
<summary>Delete Element Details</summary>

When deleting an element, the system shall:
- Remove the element and all its content from the source file
- Identify all relations pointing to the deleted element (incoming relations)
- Remove all relations that reference the deleted element from other elements
- Identify all relations from the deleted element (outgoing relations)
- Remove the complete element section including separators
- Maintain file structure and formatting after deletion
- Provide a report of all relations that were affected by the deletion

**Relation Handling:**
- All `derivedFrom` relations pointing to the deleted element shall be removed
- All `verifiedBy` relations pointing to the deleted element shall be removed
- All `verify` relations pointing to the deleted element shall be removed
- All `satisfiedBy` relations pointing to the deleted element shall be removed
- Relations from the deleted element are automatically removed with the element

</details>

#### Relations
  * derivedFrom: [Element Manipulation File Persistence](#element-manipulation-file-persistence)
---

### Move Element Operation

The system shall provide the capability to move existing model elements to different locations (file and/or section) while automatically updating all relations that reference the moved element.

#### Details
<details>
<summary>Move Element Details</summary>

When moving an element, the system shall:
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

**Relation Update Requirements:**
- All `derivedFrom` relations pointing to the moved element shall be updated to the new identifier
- All `verifiedBy` relations pointing to the moved element shall be updated to the new identifier
- All `verify` relations pointing to the moved element shall be updated to the new identifier
- All `satisfiedBy` relations pointing to the moved element shall be updated to the new identifier
- Relations within the moved element (outgoing relations) shall be preserved unchanged

**Identifier Update:**
- The element's identifier changes from `<old-file>#<element-name>` to `<new-file>#<element-name>`
- All references to the old identifier shall be updated to the new identifier

</details>

#### Relations
  * derivedFrom: [Element Manipulation File Persistence](#element-manipulation-file-persistence)
---

### Relation Consistency Maintenance

The system shall maintain bidirectional relation consistency when elements are manipulated, ensuring that forward and backward relations remain synchronized.

#### Details
<details>
<summary>Relation Consistency Details</summary>

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

</details>

#### Relations
  * derivedFrom: [Element Manipulation Operations](../../UserRequirements.md#element-manipulation-operations)
---
