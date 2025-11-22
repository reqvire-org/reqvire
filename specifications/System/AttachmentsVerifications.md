# Requirements

This document specifies verifications for the Attachments feature requirements.

### Attachments Subsection Parsing Verification

Verify the system correctly parses Attachments subsections.

#### Details
Test cases:
- Parse markdown links in Attachments subsection
- Extract paths where link text equals href
- Normalize paths to git-root-relative
- Handle multiple attachments in single element
- Reject links where text ≠ href

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Reserved Subsections Support](Subsections.md#reserved-subsections-support)
  * satisfiedBy: [test.sh](../../tests/test-attachments/test.sh)
---

### Attachments Validation Verification

Verify the system validates attachment file existence.

#### Details
Test cases:
- Validation passes when attachment files exist
- Validation fails for missing attachment files
- Error message includes element identifier and missing path
- Validation occurs in Pass 2

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Reserved Subsections Support](Subsections.md#reserved-subsections-support)
  * satisfiedBy: [test.sh](../../tests/test-attachments/test.sh)
---

### Attachments Change Impact Verification

Verify attach/detach operations trigger change impact analysis.

#### Details
CRITICAL: Detach operation must trigger change impact.

Test cases:
- Attach operation marks element as changed
- Detach operation marks element as changed
- mv-attachment operation marks affected elements as changed
- rm-attachment operation marks affected elements as changed
- Element hash changes when attachments modified
- Change impact propagates through derivedFrom relations

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Requirements Change Propagation](ChangeImpact.md#requirements-change-propagation)
  * satisfiedBy: [test.sh](../../tests/test-attachments/test.sh)
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
  * verify: [Attachment Commands](../Interfaces/CLI.md#attachment-commands)
  * satisfiedBy: [test.sh](../../tests/test-attachments/test.sh)
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
  * verify: [Attachment Commands](../Interfaces/CLI.md#attachment-commands)
  * satisfiedBy: [test.sh](../../tests/test-attachments/test.sh)
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
  * verify: [Attachment Commands](../Interfaces/CLI.md#attachment-commands)
  * satisfiedBy: [test.sh](../../tests/test-attachments/test.sh)
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
  * verify: [Attachment Commands](../Interfaces/CLI.md#attachment-commands)
  * satisfiedBy: [test.sh](../../tests/test-attachments/test.sh)
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
  * verify: [CLI Search Command](../Interfaces/CLI.md#cli-search-command)
  * satisfiedBy: [test.sh](../../tests/test-attachments/test.sh)
---

### Attachment Output Rendering Verification

Verify attachments render correctly in all output formats.

#### Details
Test cases:
- Markdown output preserves format
- HTML export renders clickable links
- JSON includes attachments array
- JSON file_path field contains git-root-relative path
- Consistent indentation in markdown

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Reserved Subsections Support](Subsections.md#reserved-subsections-support)
  * satisfiedBy: [test.sh](../../tests/test-attachments/test.sh)
---
