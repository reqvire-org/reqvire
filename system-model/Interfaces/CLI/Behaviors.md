# Elements

### Change Propagation Behavior

How changes propagate through model based on relation types.

#### Details
Propagation categories and relation impact rules are defined by the Reqvire relation and change-impact ontologies.

**Contract Bindings Impact:**
- Content changes → propagate impact
- Path renames → track but no impact

#### Metadata
  * type: behavior
---

### Contract Bindings Input Auto-Detection Behavior

When bindContract or removing contract binding via CLI commands, the system shall resolve contract_bindings targets as contract-element identifiers.

#### Details
The source requirement and existing contract target use the bound Existing Element Selection Specification. Targets accept exact names and canonical identifiers while retaining already supported contract-identifier normalization. Conflicting name/identifier matches fail as ambiguous rather than receiving identifier precedence.

After selection, require a compatible `source`, `constraint`, `behavior`, `specification`, `state` or `input-output` contract with the ownership and scope required by the Contract Bindings contracts. Paths and external URLs do not become reusable contracts. Preserve link/unlink detection, idempotency and atomic rejection behavior.

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
