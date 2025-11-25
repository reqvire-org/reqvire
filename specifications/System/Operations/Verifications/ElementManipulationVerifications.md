# Elements

### CLI Add Element Test

The test shall verify that the `add` command creates new elements from stdin or inline string input, validates structure, supports index-based insertion, and outputs git-style diffs.

#### Details
**Test Setup:**
- Prepare test model with existing files and sections
- Prepare valid element markdown strings
- Prepare invalid element markdown strings
- Set up test environment with stdin/tty detection

**Test Steps - Stdin Input:**
1. Pipe element markdown to `reqvire add <file> <section>`
2. Verify element is read from stdin
3. Verify element is validated and inserted
4. Verify git-style diff is output showing changes
5. Verify file is modified on disk

**Test Steps - Inline String:**
1. Run `reqvire add <file> <section> "### Element..."`
2. Verify element is read from inline argument
3. Verify element is validated and inserted
4. Verify git-style diff is output

**Test Steps - Index Insertion:**
1. Run `reqvire add <file> <section> 0 < element.md`
2. Verify element is inserted at index 0
3. Run without index and verify append to end
4. Run with out-of-bounds index and verify append

**Test Steps - Dry Run:**
1. Run `reqvire add --dry-run <file> <section> < element.md`
2. Verify git-style diff is shown
3. Verify no changes are applied to files

**Test Steps - JSON Output:**
1. Run `reqvire add --json <file> <section> < element.md`
2. Verify JSON output with element details
3. Verify changes are applied

**Test Steps - Validation Errors:**
1. Provide invalid element (duplicate name, malformed)
2. Verify error is reported
3. Verify file is not modified
4. Verify exit code is non-zero

**Success Criteria:**
- Reads from stdin when piped
- Reads from inline argument when provided
- Validates before insertion
- Inserts at specified index or appends
- Shows git-style diff by default
- Supports --dry-run preview
- Supports --json output
- Reports validation errors
- Returns correct exit codes

#### Metadata
  * type: test-verification

#### Relations
  * verify: [CLI Add Element Command](../../../Interfaces/CLI.md#cli-add-element-command)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-manipulation/test.sh)
---

### Target Location Validation Test

The test shall verify that target file path validation and auto-creation work correctly, enforcing safety constraints for gitignore, reqvireignore, and path depth limits.

#### Details
**Test Setup:**
- Create `.gitignore` with exclusion patterns (e.g., `**/build/**`, `temp-*.md`)
- Create `.reqvireignore` with exclusion patterns (e.g., `**/draft-*.md`)
- Prepare valid target paths within depth limits
- Prepare invalid target paths (ignored paths, excessive depth)

**Test Steps - Path Validation:**
1. Attempt to create element in path excluded by `.gitignore`
2. Verify operation is rejected with appropriate error message
3. Attempt to create element in path excluded by `.reqvireignore`
4. Verify operation is rejected with appropriate error message
5. Attempt to create element in path nested more than 10 subdirectories deep
6. Verify operation is rejected with depth limit error
7. Attempt to create element in valid path
8. Verify operation succeeds

**Test Steps - Auto-Creation:**
1. Create element in non-existent file with valid path
2. Verify file is created with proper structure (`# Elements` header)
3. Verify existing file content is preserved when adding elements

**Success Criteria:**
- Paths excluded by `.gitignore` are rejected
- Paths excluded by `.reqvireignore` are rejected
- Paths exceeding 10 subdirectory depth are rejected
- Error messages clearly indicate which constraint was violated
- Valid paths are accepted
- Non-existent files are created with proper structure
- Created files follow Reqvire markdown conventions

**Test Coverage:**
- Gitignore pattern exclusion
- Reqvireignore pattern exclusion
- Path depth limit (exactly 10, 11, 15 subdirectories)
- Auto-create file
- Valid path variations

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Target Location Validation and Auto-Creation](../ElementManipulation.md#target-location-validation-and-auto-creation)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-target-location-validation/test.sh)
---

### File Persistence Test

The test shall verify that element manipulation operations are persisted to source files in storage and that only modified files are flushed.

#### Details
**Test Setup:**
- Create a test model with multiple files and elements
- Record initial file timestamps and content
- Prepare element manipulation operations that affect specific files

**Test Steps:**
1. Perform element manipulation operation (create/delete/move)
2. Verify that modified files are written to disk
3. Verify that file content on disk matches the in-memory model state
4. Verify that only files affected by the operation were rewritten
5. Verify that unmodified files remain unchanged (check timestamps)
6. Read files from disk and parse to confirm changes persisted
7. Test file I/O error handling (simulate write failures)

**Success Criteria:**
- All changes are persisted to disk after manipulation completes
- File content on disk matches in-memory model state exactly
- Only modified files are written (optimization verified)
- Unmodified files have unchanged timestamps
- File format and structure are maintained
- Errors are handled gracefully with appropriate reporting

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Element Manipulation File Persistence](../ElementManipulation.md#element-manipulation-file-persistence)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-file-persistence/test.sh)
---

### CLI Move Element Test

The test shall verify that the `mv` command relocates elements, updates all relations, supports index-based insertion, and outputs git-style diffs.

#### Details
**Test Setup:**
- Create test model with multiple files and sections
- Create elements with incoming and outgoing relations
- Document expected relation updates
- Prepare target locations

**Test Steps - Basic Move:**
1. Run `reqvire mv <element-name> <target-file> <section>`
2. Verify element is removed from source
3. Verify element is inserted into target
4. Verify all incoming relations are updated
5. Verify git-style diff shows all affected files
6. Verify identifier change is reported

**Test Steps - Index Insertion:**
1. Run `reqvire mv <element-name> <file> <section> 0`
2. Verify element is inserted at index 0 in target
3. Run without index and verify append to end
4. Run with out-of-bounds index and verify append

**Test Steps - Dry Run:**
1. Run `reqvire mv --dry-run <element-name> <file> <section>`
2. Verify git-style diff is shown for all affected files
3. Verify no changes are applied
4. Verify relation updates are previewed

**Test Steps - JSON Output:**
1. Run `reqvire mv --json <element-name> <file> <section>`
2. Verify JSON output with relation updates
3. Verify old → new identifier mapping
4. Verify changes are applied

**Test Steps - Error Cases:**
1. Try to move non-existent element by name
2. Try to move to invalid location
3. Try to move with duplicate name at target
4. Verify errors are reported
5. Verify exit code is non-zero

**Success Criteria:**
- Moves element to target location
- Updates all incoming relations
- Preserves element content and outgoing relations
- Inserts at specified index or appends
- Shows git-style diff for all affected files
- Reports identifier change
- Supports --dry-run preview
- Supports --json output with relation mapping
- Reports validation and location errors
- Returns correct exit codes

#### Metadata
  * type: test-verification

#### Relations
  * verify: [CLI Move Element Command](../../../Interfaces/CLI.md#cli-move-element-command)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-manipulation/test.sh)
---

### CLI Move File Test

The test shall verify that the `mv-file` command moves entire specification files with all their elements to a new location, updates all relations referencing elements in the moved file, and outputs git-style diffs.

#### Details
**Test Setup:**
- Create test model with multiple specification files
- Create files with multiple elements
- Create elements in other files with relations pointing to elements in the file to be moved
- Document expected relation updates
- Prepare target file paths

**Test Steps - Basic Move File:**
1. Run `reqvire mv-file <source-file> <target-file>`
2. Verify source file is removed from filesystem
3. Verify target file is created with all elements from source
4. Verify all elements are preserved with identical content, metadata, and relations
5. Verify all incoming relations (from other files) are updated to reference the new file location
6. Verify element identifiers are updated to reflect new file path
7. Verify git-style diff shows source file deletion and target file creation
8. Verify git-style diff shows all affected files with relation updates

**Test Steps - Subdirectory Execution:**
1. Navigate to a subdirectory of the git repository
2. Run `reqvire mv-file specifications/File.md specifications/NewFile.md`
3. Verify paths are resolved relative to current working directory
4. Verify target file is created at `<cwd>/specifications/NewFile.md`
5. Verify source file at `<cwd>/specifications/File.md` is removed
6. Verify all relations are updated correctly

**Test Steps - Dry Run:**
1. Run `reqvire mv-file --dry-run <source-file> <target-file>`
2. Verify git-style diff is shown for all affected files
3. Verify no changes are applied to filesystem
4. Verify relation updates are previewed
5. Verify file move is previewed (deletion + creation)

**Test Steps - JSON Output:**
1. Run `reqvire mv-file --json <source-file> <target-file>`
2. Verify JSON output with list of moved elements
3. Verify old → new identifier mappings for all elements
4. Verify list of affected files with relation updates
5. Verify changes are applied

**Test Steps - Error Cases:**
1. Try to move non-existent source file
2. Try to move to a target file that already exists
3. Try to move file outside subdirectory scope when running from subdirectory
4. Verify errors are reported clearly
5. Verify exit code is non-zero
6. Verify no changes are applied on error

**Success Criteria:**
- Source file is deleted from filesystem
- Target file is created with all elements
- All element content, metadata, and outgoing relations are preserved
- All incoming relations are updated to new file location
- Element identifiers are updated (file path component changes)
- Shows git-style diff for source deletion, target creation, and all affected files
- Paths are resolved relative to current working directory
- Supports --dry-run preview
- Supports --json output with element mappings
- Reports errors for non-existent files and existing targets
- Returns correct exit codes
- Works correctly when executed from subdirectories

**Test Coverage:**
- Move file with single element
- Move file with multiple elements
- Move file with elements that have incoming relations from other files
- Move file with elements that have outgoing relations to other files
- Move file with bidirectional relations (verify/verifiedBy, derive/derivedFrom)
- Execute from git repository root
- Execute from subdirectory (relative path resolution)
- Dry run mode
- JSON output mode
- Error: non-existent source file
- Error: target file already exists

#### Metadata
  * type: test-verification

#### Relations
  * verify: [CLI Move File Command](../../../Interfaces/CLI.md#cli-move-file-command)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-manipulation/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-subdirectory-functionality/test.sh)
---

### CLI Remove Element Test

The test shall verify that the `rm` command deletes elements, removes relations, and outputs git-style diffs showing all affected files.

#### Details
**Test Setup:**
- Create test model with multiple elements
- Create elements with incoming relations (derivedFrom, verifiedBy, etc.)
- Document expected relation removals

**Test Steps - Basic Deletion:**
1. Run `reqvire rm <element-name>`
2. Verify element is deleted from file
3. Verify all incoming relations are removed
4. Verify git-style diff shows all affected files
5. Verify changes are applied

**Test Steps - Dry Run:**
1. Run `reqvire rm --dry-run <element-name>`
2. Verify git-style diff is shown
3. Verify no changes are applied to files
4. Verify affected files are listed

**Test Steps - JSON Output:**
1. Run `reqvire rm --json <element-name>`
2. Verify JSON output with removed relations
3. Verify changes are applied

**Test Steps - Error Cases:**
1. Try to remove non-existent element by name
2. Verify error is reported
3. Verify exit code is non-zero

**Success Criteria:**
- Deletes element from file
- Removes all incoming relations
- Shows git-style diff for all affected files
- Supports --dry-run preview
- Supports --json output with relation details
- Reports errors for non-existent elements
- Returns correct exit codes

#### Metadata
  * type: test-verification

#### Relations
  * verify: [CLI Remove Element Command](../../../Interfaces/CLI.md#cli-remove-element-command)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-manipulation/test.sh)
---

### Move Element Test

The test shall verify that existing model elements can be moved to different locations, all relations referencing the moved element are automatically updated, target locations are created if needed, and empty source files are removed when no elements remain.

#### Details
**Test Setup:**
- Create a test model with multiple files
- Create an element to be moved with incoming and outgoing relations
- Document all relations pointing to the element
- Create a file with only one element for empty file cleanup testing
- Prepare non-existent target locations for auto-creation testing

**Test Steps:**
1. Move the element to the target location
2. Verify the element was removed from the source location
3. Verify the element was inserted into the target location
4. Verify all element content, metadata, and outgoing relations are preserved
5. Verify the element identifier was updated to reflect the new location
6. Verify all incoming relations were updated to reference the new identifier
7. Validate the model after element move
8. Check that no dangling relations exist

**Test Steps - Auto-Creation:**
1. Move element to non-existent target file
2. Verify target file is created with proper structure

**Test Steps - Empty Source File Cleanup:**
1. Move the only element from a file to another location
2. Verify the element is moved successfully
3. Verify the source file is deleted from the filesystem
4. Verify the file deletion is reported in the operation output
5. Move an element leaving other elements in the source file
6. Verify the source file is NOT deleted (still contains elements)

**Success Criteria:**
- Element is removed from source location
- Element is inserted into target location
- Element content and metadata are preserved
- Element identifier is updated correctly
- All incoming relations (derivedFrom, verifiedBy, verify, satisfiedBy) are updated to the new identifier
- Outgoing relations (relations from the moved element to other elements) are preserved unchanged
- File structure remains valid in both source and target files
- Model validation passes
- Non-existent target files are created with proper structure
- Empty source files are deleted after move
- Source files with remaining elements are preserved
- File creation and deletion are reported

**Test Coverage:**
- Move element within the same file (different position)
- Move element to a different file
- Move element to specific index in target file (0, middle, end)
- Move element without index (defaults to end of target file)
- Move element with out-of-bounds index (appends to end)
- Move element with `derivedFrom` relations pointing to it
- Move element with `verifiedBy` relations pointing to it
- Move element with `verify` relations pointing to it
- Move element with `satisfiedBy` relations pointing to it
- Move element with multiple types of incoming relations
- Move to non-existent target file (auto-create)
- Move last element from file (triggers source file deletion)
- Move element leaving other elements (source file preserved)

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Move Element Operation](../ElementManipulation.md#move-element-operation)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-manipulation/test.sh)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-empty-file-cleanup/test.sh)
---

### CLI Rename Element Test

The test shall verify that the `rename` command renames elements, updates all relations, and outputs git-style diffs.

#### Details
**Test Setup:**
- Create test model with elements
- Create elements with incoming and outgoing relations
- Document expected relation updates

**Test Steps - Basic Rename:**
1. Run `reqvire rename <current-name> <new-name>`
2. Verify element heading is updated in markdown file
3. Verify all incoming relations are updated with new identifier
4. Verify git-style diff shows all affected files
5. Verify identifier change is reported (old → new)

**Test Steps - Dry Run:**
1. Run `reqvire rename --dry-run <current-name> <new-name>`
2. Verify git-style diff is shown
3. Verify no changes are applied
4. Verify relation updates are previewed

**Test Steps - JSON Output:**
1. Run `reqvire rename --json <current-name> <new-name>`
2. Verify JSON output with relation updates
3. Verify old → new identifier mapping
4. Verify changes are applied

**Test Steps - Error Cases:**
1. Try to rename non-existent element
2. Try to rename to an existing element name
3. Verify errors are reported
4. Verify exit code is non-zero

**Success Criteria:**
- Updates element heading text
- Updates all incoming relations
- Preserves element content and outgoing relations
- Shows git-style diff for all affected files
- Reports identifier change
- Supports --dry-run preview
- Supports --json output
- Reports validation errors
- Returns correct exit codes

#### Metadata
  * type: test-verification

#### Relations
  * verify: [CLI Rename Element Command](../../../Interfaces/CLI.md#cli-rename-element-command)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-manipulation/test.sh)
---

### Relation Consistency Test

The test shall verify that bidirectional relation consistency is maintained when elements are manipulated.

#### Details
<details>
<summary>Test Criteria</summary>

**Test Setup:**
- Create a test model with elements having bidirectional relations
- Set up test cases for derivedFrom/derive, verifiedBy/verify relations

**Test Steps:**
1. Create an element with relations and verify bidirectional consistency
2. Delete an element and verify both forward and backward relations are removed
3. Move an element and verify both forward and backward relations are updated
4. Validate model consistency after each manipulation operation

**Success Criteria:**
- After element creation: bidirectional relations are properly established
- After element deletion: both forward and backward relations are removed
- After element move: both forward and backward relations are updated
- Model validation passes after each operation
- No dangling or inconsistent relations exist

**Test Coverage:**
- Verify derivedFrom/derive relation consistency
- Verify verifiedBy/verify relation consistency
- Test consistency after create operations
- Test consistency after delete operations
- Test consistency after move operations

</details>

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Relation Consistency Maintenance](../ElementManipulation.md#relation-consistency-maintenance)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-relation-consistency/test.sh)
---

### Delete Element Test

The test shall verify that existing model elements can be deleted, all relations referencing the deleted element are automatically removed, and empty files are removed when no elements remain.

#### Details
**Test Setup:**
- Create a test model with multiple elements
- Create an element to be deleted with incoming and outgoing relations
- Document all relations pointing to the element (derivedFrom, verifiedBy, verify, satisfiedBy)
- Create a file with only one element for empty file cleanup testing

**Test Steps:**
1. Delete the target element
2. Verify the element was completely removed from the file
3. Verify all relations pointing to the deleted element were removed from other elements
4. Verify the file structure and formatting remain intact
5. Validate the model after element deletion
6. Check that no dangling relations exist

**Test Steps - Empty File Cleanup:**
1. Delete the only element in a file
2. Verify the element is removed
3. Verify the file is deleted from the filesystem
4. Verify the file deletion is reported in the operation output
5. Delete an element leaving other elements in the file
6. Verify the file is NOT deleted (still contains elements)

**Success Criteria:**
- Element is completely removed from the source file
- All incoming relations (relations from other elements to the deleted element) are removed
- File structure remains valid
- Model validation passes
- No dangling relations remain in the model
- Files containing only the deleted element are removed
- Files with remaining elements are preserved
- File deletion is reported when it occurs

**Test Coverage:**
- Delete element with `derivedFrom` relations pointing to it
- Delete element with `verifiedBy` relations pointing to it
- Delete element with `verify` relations pointing to it
- Delete element with `satisfiedBy` relations pointing to it
- Delete element with multiple types of incoming relations
- Delete last element in file (triggers file deletion)
- Delete element leaving other elements (file preserved)

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Delete Element Operation](../ElementManipulation.md#delete-element-operation)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-manipulation/test.sh)
---

### Create Element Test

The test shall verify that new model elements can be created from a full Markdown definition string after validation, and that invalid element definitions are rejected with appropriate error reporting.

#### Details
**Test Setup:**
- Prepare a test model with existing files
- Prepare valid element definition strings in Markdown format (with ### header, metadata, relations, content)
- Prepare invalid element definition strings (malformed structure, duplicate names, invalid relations)
- Identify target file path

**Test Steps:**
1. Attempt to create an element with a valid definition string
2. Verify the element was parsed correctly from the string
3. Verify the element structure was validated (subsections, relations, format)
4. Verify the element name was checked for global uniqueness
5. Verify the element was inserted into the correct file
6. Verify the element has proper Markdown structure in the file
7. Validate the model after element creation

**Test Steps for Invalid Cases:**
1. Attempt to create an element with an invalid definition string
2. Verify the validation detects the structural errors
3. Verify the operation is rejected before insertion
4. Verify appropriate validation error messages are reported
5. Verify the target file remains unchanged

**Test Steps for Relation Validation:**
1. Create element with valid relations (targets exist in model)
2. Verify relations are normalized to git-root-relative format
3. Verify element is created successfully
4. Attempt to create element with non-existent relation target
5. Verify operation is rejected with clear error message
6. Verify error message indicates which relation target was not found
7. Verify target file remains unchanged after rejection

**Test Coverage:**
- Valid element with all subsections (metadata, relations, details)
- Valid element with minimal structure (only ### header and content)
- Valid element inserted at specific index (0, middle, end)
- Valid element inserted without index (defaults to end)
- Valid element inserted with out-of-bounds index (appends to end)
- Invalid element with duplicate name (violates global uniqueness)
- Invalid element with malformed subsections
- Invalid element with invalid relations
- Invalid element with missing ### header
- **Valid element with relations to existing elements**
- **Invalid element with relation to non-existent element**
- **Relations specified as relative paths (../File.md#element)**
- **Relations specified as repo-relative paths (specifications/File.md#element)**
- **Relations specified as same-file references (#element)**
- **External link relations (http://, https://) are allowed**

**Success Criteria:**
- Valid element definitions are accepted and created
- Element is parsed from the Markdown string correctly
- Element structure validation runs before insertion
- Global uniqueness is enforced
- Element inserted at specified index position within file
- Element appended to end when index not provided or out of bounds
- Invalid element definitions are rejected
- Validation errors are reported clearly
- Failed operations do not modify target files
- Model validation passes after successful creation
- **Relation targets are validated to exist in the model**
- **Non-existent relation targets cause rejection**
- **Relation paths are normalized to git-root-relative format**
- **External links (http://, https://) bypass validation**

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Create Element Operation](../ElementManipulation.md#create-element-operation)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-manipulation/test.sh)
---

### Move File Squash Test

The test shall verify that the `mv-file --squash` command moves all elements from a source file to an existing target file's first section, updates all relations, and removes the source file.

#### Details
**Test Setup:**
- Create a source file with multiple elements
- Create a target file that already exists with its own elements and sections
- Create elements in other files with relations pointing to elements in the source file
- Document expected relation updates and element placement

**Test Steps - Squash to Existing File:**
1. Run `reqvire mv-file --squash <source-file> <existing-target-file>`
2. Verify source file is removed from filesystem
3. Verify all elements from source file are added to target file's first section
4. Verify elements from source file are appended to the first level-2 section (##) in target file
5. Verify original target file elements remain unchanged
6. Verify element ordering from source file is preserved when inserted
7. Verify all element identifiers are updated to reflect new file path
8. Verify all incoming relations (from other files) are updated to reference the new file location
9. Verify git-style diff shows source file deletion and target file modification
10. Verify git-style diff shows all affected files with relation updates

**Test Steps - Error Without Squash Flag:**
1. Run `reqvire mv-file <source-file> <existing-target-file>` (without --squash)
2. Verify operation fails with clear error message
3. Verify error message indicates target file already exists
4. Verify no changes are applied to any files
5. Verify exit code is non-zero

**Test Steps - Squash with Dry Run:**
1. Run `reqvire mv-file --squash --dry-run <source-file> <existing-target-file>`
2. Verify git-style diff is shown for all affected files
3. Verify no changes are applied to filesystem
4. Verify preview shows elements being added to target file's first section

**Test Steps - Squash with JSON Output:**
1. Run `reqvire mv-file --squash --json <source-file> <existing-target-file>`
2. Verify JSON output with list of moved elements
3. Verify old → new identifier mappings for all elements
4. Verify list of affected files with relation updates
5. Verify changes are applied

**Success Criteria:**
- Source file is deleted from filesystem
- All elements from source are added to target file's first section
- Target file's existing elements remain unchanged
- Element ordering from source file is preserved
- All incoming relations are updated to new file location
- Element identifiers are updated (file path component changes)
- Shows git-style diff for source deletion, target modification, and all affected files
- Supports --dry-run preview
- Supports --json output with element mappings
- Returns correct exit codes
- Without --squash flag, existing target file causes error

**Test Coverage:**
- Squash file with single element to existing target
- Squash file with multiple elements to existing target
- Squash file with elements that have incoming relations from other files
- Squash file with elements that have outgoing relations to other files
- Verify elements are placed in target file's first section
- Verify target file's existing content is preserved
- Dry run mode with --squash
- JSON output mode with --squash
- Error: target file exists without --squash flag

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Move File Operation](../ElementManipulation.md#move-file-operation)
  * satisfiedBy: [test.sh](../../../../tests/test-crud-mv-file-squash/test.sh)
---
