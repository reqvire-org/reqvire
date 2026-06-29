# Elements


### Model Diagram Generation

System shall provide CLI command to generate model structure diagrams.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Structure Exploration](Capabilities.md#model-structure-exploration)

---

### Model Filtering Capability

System shall support filtering model diagrams from a specific root element using element name.

#### Details
- Use --from <NAME> flag to specifiedBy starting element
- Match by element name (not identifier)
- Build nested structure from that element

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Diagram Generation](#model-diagram-generation)

---

### Forward Relation Traversal

System shall traverse only forward relations when filtering from a root element.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Filtering Capability](#model-filtering-capability)

---

### Default Model Roots

System shall filter to ontology roots, concept roots, and capability roots when no --from filter is specified.

Ontology roots, concept roots, and capability roots are ontology, concept-scheme, and capability elements without rendered hierarchical parent relations.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Filtering Capability](#model-filtering-capability)

---


### Default JSON Output Format

System shall generate structured JSON output by default.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Structure Exploration](Capabilities.md#model-structure-exploration)

---

### Output Format Flag Removal

System shall reject removed model output-format flags instead of retaining compatibility aliases.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Structure Exploration](Capabilities.md#model-structure-exploration)

---

### JSON Output Format

System shall generate structured JSON output with model-centric nested structure.

#### Details
- Elements array with identifier, name, element_type, file_path, section, section_index
- Relations nested inside each element with full target details
- Three target types supported:
  - **Element**: nested recursively with full details
  - **File**: `{"path": "src/file.rs", "type": "file"}`
  - **External**: `{"url": "https://example.com", "type": "external"}`
- Metadata includes total_elements, total_relations (no duplicates), filtered_from
- Cycle detection prevents infinite recursion

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Diagram Generation](#model-diagram-generation)

---
