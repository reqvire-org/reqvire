# Elements

### Document Structure Specification

Canonical model document structure.

#### Details
- Preserve a valid multi-element `# Elements` document or single-element `# Element` document. Single-element files retain exactly one element and cannot gain a sibling during normalization.
- In `# Elements` documents, emit H3 element headings and H4 reserved subsections. In `# Element` documents, retain H2 metadata, optional relations and the named body; headings within that body remain its content. Normalize structural spacing and separators for the selected form. Preserve ordinary page text and eligible non-reserved content instead of introducing legacy `## Elements` sections.
- Preserve Markdown code fences and their bytes; literal headings inside them are not model structure.
- Use the same document writer for default formatting and mutation persistence. Formatting does not migrate invalid legacy headers or infer repairs for invalid models; known migrations remain separate.

#### Metadata
  * type: specification
---

### Format Consistency Specification

Rules for detecting and fixing formatting inconsistencies in requirements documents.

#### Details
**Excess Whitespace:**
- Detect and fix excess whitespace after element headers, subsection headers, and relation identifiers
- Maintain consistent formatting across all requirements documents

**Inconsistent Newlines:**
- Detect and fix excess or missing newlines before element headers and subsection headers
- Normalize to exactly two newlines before subsections (e.g., "#### Details")

**Missing Separators:**
- Detect consecutive element sections that lack a separator line (---) between them
- Insert separator to maintain consistent visual separation
- Normalize consecutive separators to single separators

**Reserved Subsections:**
- Identify and fix inconsistent indentation and bullet types in relation lists
- Ensure consistent indentation in relation lists (2-space format)
- Normalize relation entries to proper 2-space indentation format

**Output Formatting:**
- Display changes with sequential line numbering that reflects final file positions
- Provide context lines with proper line number continuity

**Semantic Repair Boundary:**
- Consume the prose-link classification and guidance referenced by the owning requirement. Formatting must not silently move an invalid prose link into a relation, Contract Binding, Contract Reference, or Concept Reference, because the valid repair depends on author intent and source/target compatibility.
- Preserve the shared diagnostic and repair examples when reporting such a violation. Formatting normalization must not make the invalid dependency appear valid by changing its label or choosing a relation automatically.

#### Metadata
  * type: specification
---

### Full Relations Insertion Contract Specification

#### Details
Auto-generated relations are inverse relations created by the parser during model loading but not persisted to files by default. See Relation Types Specification for opposite relation pairs.

When the core full-relations option is enabled, exposed as MCP `with_full_relations: true`:
- All relations from the model registry are written to the Relations subsection
- Relations are sorted according to the Relation Ordering Normalization requirement

#### Metadata
  * type: specification
---

### Relation Ordering Specification

Contract for deterministic authored relation ordering.

#### Details
1. For concept and concept-scheme elements, order relation families by priority: `derivedFrom`, `broader`, `narrower`, `related`, `exactMatch`, `closeMatch`, then all remaining compatible relation types.
2. For other element types, all relation types share the same priority.
3. Within equal priority, sort alphabetically by relation type name; within the same type, sort alphabetically by resolved target identifier, file path or URL according to target kind.
4. Apply the same ordering to authored and explicitly materialized generated relations. The default formatter preserves the authored relation subset.
5. Priority affects serialization only; it does not convert concept associations into taxonomy or change relation semantics.

#### Metadata
  * type: specification
---
