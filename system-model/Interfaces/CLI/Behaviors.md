# Elements

### Change Propagation Behavior

How changes propagate through model based on relation types.

#### Details
Propagation categories and relation impact rules are defined by the Reqvire relation and change-impact ontologies.

**Contract Bindings Impact:**
- Content changes → propagate impact
- Path renames → track but no impact



**Comparison invocation:**
The change-impact command accepts `--git-commit` and the optional `--base-manifest-commit` under the shared baseline-selection contract. Forward the effective workspace and selected manifest to shared composition reconstruction and reporting; do not implement adapter-local component selection. Preserve text output by default and JSON output with `--json`.

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
- Omit verbose fields: `content`, `page_content`, `contract_bindings`
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
