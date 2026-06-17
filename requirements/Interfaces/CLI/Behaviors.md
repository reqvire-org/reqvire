# Elements

### Attachment Input Auto-Detection Behavior

When attaching or detaching via CLI commands, the system shall resolve attachment targets as refinement-element identifiers.

#### Details
The resolution follows this order:

1. **Identifier Parse**: Parse target input as an element identifier (for example `file.md#refinement-id`).
2. **Identifier Normalization**: Resolve and normalize identifier to full internal identifier format.
3. **Type Validation**: Confirm target element type is a Refinement type (`constraint`, `behavior`, `specification`).
4. **Error Handling**: If parsing or resolution fails, report a clear error message indicating that a refinement identifier target is required.

#### Metadata
  * type: behavior
---

### Change Propagation Behavior

How changes propagate through model based on relation types.

#### Details
Propagation categories and relation impact rules are defined by the Reqvire relation and change-impact ontologies.

**Attachment Impact:**
- Content changes → propagate impact
- Path renames → track but no impact

#### Metadata
  * type: behavior
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

