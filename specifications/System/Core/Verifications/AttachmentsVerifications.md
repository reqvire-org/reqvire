# Elements

### Attachments Subsection Parsing Verification

Verify the system correctly parses Attachments subsections including both file paths and element identifiers.

#### Details
Test cases for file paths:
- Parse markdown links in Attachments subsection
- Extract paths where link text equals href
- Normalize paths to git-root-relative
- Handle multiple attachments in single element
- Reject links where text ≠ href (for file paths)

Test cases for element identifiers:
- Parse markdown links to Refinement elements (constraint, behavior, specification)
- Normalize element identifiers like relation targets
- Support full identifier format `file.md#element-name`
- Support same-file format `#element-name`
- Handle mixed file path and element identifier attachments

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Reserved Subsections Support](../StructureAndParsing.md#reserved-subsections-support)
  * verify: [Attachment Target Validation](../StructureAndParsing.md#attachment-target-validation)
  * satisfiedBy: [test.sh](../../../../tests/test-attachments/test.sh)
---

### Attachments Validation Verification

Verify the system validates attachment targets including file existence and element identifier validity.

#### Details
Test cases for file paths:
- Validation passes when attachment files exist
- Validation fails for missing attachment files
- Error message includes element identifier and missing path
- Validation occurs in Pass 2

Test cases for element identifiers:
- Accept Refinement element identifiers (constraint, behavior, specification)
- Reject non-Refinement element identifiers (requirement, user-requirement, verification)
- Error message indicates expected Refinement type
- Validation fails for non-existent element identifiers

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Reserved Subsections Support](../StructureAndParsing.md#reserved-subsections-support)
  * verify: [Attachment Target Validation](../StructureAndParsing.md#attachment-target-validation)
  * satisfiedBy: [test.sh](../../../../tests/test-attachments/test.sh)
---

### Attachments Change Impact Verification

Verify Refinement element operations are tracked in change impact analysis.

#### Details
Test cases for Refinement element content changes:
- Refinement element content change is detected in change-impact report
- Same-file, cross-file, and cross-directory Refinement references are validated

Test cases for Refinement element mv operations:
- mv to different file: relocation reported, attachment identifiers updated automatically
- mv to different directory: relocation reported, attachment identifiers updated automatically
- Behavior matches relation target relocation handling

Test cases for Refinement element rm operations:
- rm of attached Refinement element: validation fails with clear error about broken attachment

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Change Impact Detection](../../Processing/ChangeImpact.md#change-impact-detection)
  * satisfiedBy: [test.sh](../../../../tests/test-change-impact-attachments/test.sh)
---

### Attach Command Verification

Verify attach command creates Attachments subsection and adds links.

#### Details
Test cases:
- Create Attachments subsection if missing
- Add link with format `[path](path)`
- Idempotent: duplicate attach doesn't create duplicate entry
- Many-to-many: same file attaches to multiple elements
- Dry-run mode makes no changes
- Validation passes after attach

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Attachment Commands](../../../Interfaces/CLI.md#attachment-commands)
  * satisfiedBy: [test.sh](../../../../tests/test-attachments/test.sh)
---

### Detach Command Verification

Verify detach command removes links and cleans up empty subsections.

#### Details
CRITICAL: Must verify detach triggers change impact.

Test cases:
- Remove link from Attachments subsection
- Remove subsection when no attachments remain
- Detach from one element doesn't affect others
- Dry-run mode makes no changes
- Change impact analysis shows element as changed

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Attachment Commands](../../../Interfaces/CLI.md#attachment-commands)
  * satisfiedBy: [test.sh](../../../../tests/test-attachments/test.sh)
---

### Move Attachment Command Verification

Verify mv-attachment updates all references across elements.

#### Details
Test cases:
- Update ALL elements with attachment
- Update both link text and href
- Link text equals path after move
- Report all affected elements
- Dry-run mode makes no changes
- Validation passes after move

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Attachment Commands](../../../Interfaces/CLI.md#attachment-commands)
  * satisfiedBy: [test.sh](../../../../tests/test-attachments/test.sh)
---

### Remove Attachment Command Verification

Verify rm-attachment deletes file and detaches from all elements.

#### Details
Test cases:
- Delete physical file from filesystem
- Detach from ALL elements
- Remove empty Attachments subsections
- Report all affected elements
- Dry-run mode makes no changes (file not deleted)
- Validation passes after removal

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Attachment Commands](../../../Interfaces/CLI.md#attachment-commands)
  * satisfiedBy: [test.sh](../../../../tests/test-attachments/test.sh)
---

### Attachment Identifier CRUD Verification

Verify that moving or renaming Refinement elements updates attachment identifiers throughout the model.

#### Details
Test cases for move operations:
- Moving a Refinement element updates all attachment identifiers referencing it
- All files with referencing attachments are modified
- Attachment format remains valid after move

Test cases for rename operations:
- Renaming a Refinement element updates all attachment identifiers referencing it
- All files with referencing attachments are modified
- Attachment format remains valid after rename

Test cases for consistency:
- Behavior matches relation target updates (same update logic)
- Validation passes after CRUD operations
- No orphaned or broken attachment references

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Attachment Identifier Updates](../ModelManagement.md#attachment-identifier-updates)
---

### Attachment Search Filters Verification

Verify search filters correctly find elements by attachments.

#### Details
Test cases:
- `--has-attachments` finds only elements with attachments
- `--filter-attachment` with glob pattern matches correctly
- Pattern `*.pdf` matches PDF files only
- Pattern `docs/*` matches all files in docs directory
- No false positives or false negatives

#### Metadata
  * type: test-verification

#### Relations
  * verify: [CLI Search Command](../../../Interfaces/CLI.md#cli-search-command)
  * satisfiedBy: [test.sh](../../../../tests/test-attachments/test.sh)
---

### Attachment Output Rendering Verification

Verify attachments render correctly in all output formats.

#### Details
Test cases for file paths:
- Markdown output preserves format
- HTML export renders clickable links
- JSON includes attachments array
- JSON file_path field contains git-root-relative path
- Consistent indentation in markdown

Test cases for element identifiers:
- JSON includes element identifiers in attachments array as strings
- Element identifier format: `"file.md#element-name"`
- HTML export renders clickable links to Refinement elements
- Mixed file path and element identifier attachments display correctly

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Reserved Subsections Support](../StructureAndParsing.md#reserved-subsections-support)
  * satisfiedBy: [test.sh](../../../../tests/test-attachments/test.sh)
---
