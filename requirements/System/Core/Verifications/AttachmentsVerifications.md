# Elements

### Attach Command Verification

Verify attach command creates Attachments subsection and adds links.

#### Details
Test cases for file attachments:
- Create Attachments subsection if missing
- Add link with format `[filename](path)`
- Idempotent: duplicate attach doesn't create duplicate entry
- Many-to-many: same file attaches to multiple elements
- Dry-run mode makes no changes
- Validation passes after attach

Test cases for element attachments:
- Attach Refinement element by display name
- Auto-detect: file path takes priority over element name when file exists
- Element identifier format: `[Name](#id)` for same-file, `[Name](file.md#id)` for cross-file
- Only Refinement types allowed (constraint, behavior, specification)
- Error when attaching non-Refinement element (requirement, verification, etc.)
- Error when neither file nor element found

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [Attachment Commands](../../../Interfaces/CLI.md#attachment-commands)
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
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [Reserved Subsections Support](../StructureAndParsing.md#reserved-subsections-support)
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
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [CLI Search Command](../../../Interfaces/CLI.md#cli-search-command)
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
  * satisfiedBy: [test.sh](../../../../tests/test-change-impact-attachments/test.sh)
  * verify: [Change Impact Detection](../../Processing/ChangeImpact.md#change-impact-detection)
---

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
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [Attachment Target Validation](../StructureAndParsing.md#attachment-target-validation)
  * verify: [Reserved Subsections Support](../StructureAndParsing.md#reserved-subsections-support)
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
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [Attachment Target Validation](../StructureAndParsing.md#attachment-target-validation)
  * verify: [Reserved Subsections Support](../StructureAndParsing.md#reserved-subsections-support)
---

### Detach Command Verification

Verify detach command removes links and cleans up empty subsections.

#### Details
CRITICAL: Must verify detach triggers change impact.

Test cases for file attachments:
- Remove link from Attachments subsection
- Remove subsection when no attachments remain
- Detach from one element doesn't affect others
- Dry-run mode makes no changes
- Change impact analysis shows element as changed

Test cases for element attachments:
- Detach Refinement element by display name
- Auto-detect: match element name against existing attachments
- Works for both same-file and cross-file element attachments

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [Attachment Commands](../../../Interfaces/CLI.md#attachment-commands)
---

### Move Asset Command Verification

Verify mv-asset moves InternalPath files and updates all references across the model.

#### Details
Test cases for Attachments subsection updates:
- Find all elements with InternalPath attachment matching old path
- Update link text to new path
- Update link href to new path
- Handle relative path resolution from element's file location

Test cases for Relations updates:
- Find all satisfiedBy relations with InternalPath matching old path
- Find all satisfy relations with InternalPath matching old path
- Find all trace relations with InternalPath matching old path
- Update relation target to new path
- Handle relative path resolution from element's file location

Test cases for filesystem and reporting:
- Move/rename physical file on filesystem
- Report count of affected Attachments
- Report count of affected Relations
- Report list of modified specification files
- Dry-run mode: show changes without applying (file not moved)
- Validation passes after move

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [CLI Move Asset Command](../../../Interfaces/CLI.md#cli-move-asset-command)
---

### Remove Asset Command Verification

Verify rm-asset deletes InternalPath files and removes all references from the model.

#### Details
Test cases for Attachments subsection updates:
- Find all elements with InternalPath attachment matching path
- Remove attachment link from Attachments subsection
- Remove empty Attachments subsection if no attachments remain

Test cases for Relations updates:
- Find all satisfiedBy relations with InternalPath matching path
- Find all satisfy relations with InternalPath matching path
- Find all trace relations with InternalPath matching path
- Remove entire relation line from element

Test cases for filesystem and reporting:
- Delete physical file from filesystem
- Report count of removed Attachments
- Report count of removed Relations
- Report list of modified specification files
- Dry-run mode: show changes without applying (file not deleted)
- Validation passes after removal

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-assets/test.sh)
  * verify: [CLI Remove Asset Command](../../../Interfaces/CLI.md#cli-remove-asset-command)
---
