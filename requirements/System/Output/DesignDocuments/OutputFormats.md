# Elements

### JSON Output Structure

Standard JSON output structure for CLI commands that support the `--json` flag.

#### Details
JSON output conventions:

**Structure:**
- Root object with semantic field names (not abbreviated)
- Arrays for collections (elements, relations, files)
- Nested objects for hierarchical data
- Consistent field naming using snake_case

**Common Fields:**
- `identifier`: Full element identifier (file#fragment)
- `name`: Display name of element
- `type`: Element type string
- `file_path`: Relative path from git root
- `relations`: Array of relation objects with `type` and `target` fields
- `attachments`: Array of attachment strings (file paths or element identifiers)

**Error Handling:**
- Error responses include `error` field with message
- Successful responses omit error field entirely
- Exit code accompanies JSON (0=success, non-zero=error)

#### Metadata
  * type: specification
---

### Short Mode Behavior

Behavior when `--short` flag is provided to CLI commands.

#### Details
Short mode reduces output verbosity for quick scanning:

**Text Output (--short without --json):**
- One line per element: `[type] identifier - name`
- Omit detailed content, relations, and metadata
- Suitable for piping to other tools

**JSON Output (--short with --json):**
- Omit verbose fields: `content`, `page_content`, `attachments`
- Omit computed fields: `element_count`, `total_elements`, `global_counters`
- Retain: `identifier`, `name`, `type`, `file_path`
- Retain: `relations` (for traceability)

**Rationale:**
- Reduces output size for large models
- Faster parsing by downstream tools
- Maintains essential traceability information

#### Metadata
  * type: behavior
---

### Text Output Formatting

Human-readable text output conventions for CLI commands.

#### Details
Default text output (when neither `--json` nor other format flags specified):

**Hierarchical Display:**
- Group elements by file, then by section
- Use indentation to show containment
- Display element name with type indicator

**Element Information:**
- Full element name and identifier
- Element type in brackets: `[requirement]`, `[test-verification]`
- Relations listed with target identifiers
- Attachments listed as file paths or element names

**Formatting:**
- Color output when terminal supports it (errors in red, warnings in yellow)
- Git-style diff format for change previews
- Line numbers for file references

**Consistency:**
- Deterministic ordering (alphabetical by identifier)
- Consistent spacing and alignment
- No trailing whitespace

#### Metadata
  * type: specification
---
