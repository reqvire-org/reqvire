# Attachments

This document specifies requirements for the Attachments subsection feature, enabling requirements to link external documents.

---

## Default

### Attachments Subsection Support
The system shall support an Attachments subsection for linking external documents to requirements.

#### Details
- Support markdown link syntax: `* [path](path)`
- Link text equals path (git-root-relative)
- Many-to-many relationship (multiple requirements can link same document)
- Never parse attachment files (treat as opaque)
- Auto-cleanup: remove subsection when empty

#### Relations
* derivedFrom: [Reserved Subsections Support](ModelManagement.md#reserved-subsections-support)

---

### Attachment Parsing
The system shall parse Attachments subsections from requirement documents.

#### Details
Parse markdown bullet list items in Attachments subsection. Extract file paths from markdown links where link text equals href. Normalize paths to git-root-relative format using the requirement file's directory as base.

#### Relations
* derivedFrom: [Attachments Subsection Support](#attachments-subsection-support)

---

### Attachment Validation
The system shall validate attachment file references during Pass 2 validation.

#### Details
- Verify file existence on filesystem
- Validate markdown link format: `[path](path)`
- Detect duplicate attachments within element
- Report missing attachment targets as validation errors

#### Relations
* derivedFrom: [Attachments Subsection Support](#attachments-subsection-support)

---

### Attachment Change Impact
The system shall include attachments in change impact analysis.

#### Details
All attachment operations must trigger change impact:
- Attach operation triggers change impact
- Detach operation triggers change impact (CRITICAL)
- Move attachment operation triggers change impact
- Remove attachment operation triggers change impact

This is achieved by including attachment paths in the element's content hash calculation.

#### Relations
* derivedFrom: [Attachments Subsection Support](#attachments-subsection-support)
* derivedFrom: [Change Impact Analysis](ChangeImpact.md#change-impact-analysis)

---

### Attach Command
The system shall provide CLI command to attach external documents to elements.

#### Details
Syntax: `reqvire attach <attachment-path> <element-name> [--dry-run]`

Behavior:
- Create Attachments subsection if doesn't exist
- Add link to subsection with format `* [path](path)`
- Skip if already attached (idempotent)
- Support many-to-many (same file to multiple elements)
- Mark element file as modified
- Support dry-run mode for preview

#### Relations
* derivedFrom: [Attachments Subsection Support](#attachments-subsection-support)

---

### Detach Command
The system shall provide CLI command to detach external documents from elements.

#### Details
Syntax: `reqvire detach <element-name> <attachment-path> [--dry-run]`

Behavior:
- Remove link from Attachments subsection
- Remove subsection if no attachments remain
- Trigger change impact on element (CRITICAL)
- Mark element file as modified
- Support dry-run mode for preview

#### Relations
* derivedFrom: [Attachments Subsection Support](#attachments-subsection-support)

---

### Move Attachment Command
The system shall provide CLI command to move/rename attachments.

#### Details
Syntax: `reqvire mv-attachment <old-path> <new-path> [--dry-run]`

Behavior:
- Update ALL references across all elements
- Update both link text and href (text = path)
- Report affected elements
- Mark all affected element files as modified
- Support dry-run mode for preview

#### Relations
* derivedFrom: [Attachments Subsection Support](#attachments-subsection-support)

---

### Remove Attachment Command
The system shall provide CLI command to remove attachment files.

#### Details
Syntax: `reqvire rm-attachment <attachment-path> [--dry-run]`

Behavior:
- Delete physical file from filesystem
- Detach from ALL elements
- Remove empty Attachments subsections
- Report affected elements
- Mark all affected element files as modified
- Support dry-run mode for preview

#### Relations
* derivedFrom: [Attachments Subsection Support](#attachments-subsection-support)

---

### Attachment Search Filters
The system shall support searching by attachments.

#### Details
Two search filters:
- `--has-attachments`: Filter elements with Attachments subsection
- `--filter-attachment <glob>`: Filter by attachment path pattern (supports glob patterns like `*.pdf`, `docs/*`)

#### Relations
* derivedFrom: [Attachments Subsection Support](#attachments-subsection-support)

---

### Attachment Output Rendering
The system shall render Attachments subsection in all output formats.

#### Details
Support rendering in:
- Markdown output: preserve original format
- HTML export: render as clickable links
- JSON serialization: include attachments array with file_path field

Format: List of markdown links with consistent indentation

#### Relations
* derivedFrom: [Attachments Subsection Support](#attachments-subsection-support)

---
