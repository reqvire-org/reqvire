export type MockElement = {
  id: string;
  name: string;
  element_type: string;
  type_family: string;
  file_path: string;
  line_number: number;
  content: string;
  metadata: Record<string, string>;
  governance: Record<string, string>;
};

export type MockRelation = {
  id: string;
  source_id: string;
  target_id: string;
  relation_type: string;
};

export const MOCK_ELEMENTS: MockElement[] = [
  {
    id: "CAP-001",
    name: "System Modeling Capability",
    element_type: "capability",
    type_family: "capability",
    file_path: "requirements/Capabilities.md",
    line_number: 3,
    content:
      "As a **System Engineer**, I want a well-defined Reqvire model structure, so that I can manage system requirements and traceability across all project phases.",
    metadata: { status: "active" },
    governance: { owner: "systems-team" },
  },
  {
    id: "REQ-001",
    name: "Model Structure Specification",
    element_type: "requirement",
    type_family: "requirement",
    file_path: "requirements/SystemRequirements.md",
    line_number: 12,
    content:
      "The system shall provide a structured model format supporting requirements, capabilities, verifications, and traceability links.",
    metadata: { status: "draft", priority: "high" },
    governance: { owner: "systems-team" },
  },
  {
    id: "REQ-002",
    name: "Traceability Coverage Requirement",
    element_type: "requirement",
    type_family: "requirement",
    file_path: "requirements/SystemRequirements.md",
    line_number: 28,
    content:
      "The system shall maintain bidirectional traceability between requirements and their verification artifacts.",
    metadata: { status: "active", priority: "high" },
    governance: { owner: "systems-team" },
  },
  {
    id: "VER-001",
    name: "Model Structure Test",
    element_type: "test-verification",
    type_family: "verification",
    file_path: "requirements/Verifications/ModelStructure.md",
    line_number: 5,
    content: "Verify that the model parser correctly extracts all element types from a representative Markdown document.",
    metadata: { status: "active", method: "test" },
    governance: {},
  },
  {
    id: "SPEC-001",
    name: "Containment Specification",
    element_type: "specification",
    type_family: "refinement",
    file_path: "requirements/Specifications.md",
    line_number: 8,
    content: "Defines containment rules for nested model elements within source files.",
    metadata: { status: "active" },
    governance: {},
  },
  {
    id: "ONT-001",
    name: "Element Type Ontology",
    element_type: "ontology",
    type_family: "ontology",
    file_path: "ontologies/ElementTypes.ttl",
    line_number: 1,
    content: "RDF/SHACL ontology defining the element type vocabulary for Reqvire models.",
    metadata: {},
    governance: {},
  },
];

export const MOCK_RELATIONS: MockRelation[] = [
  { id: "r1", source_id: "REQ-001", target_id: "CAP-001", relation_type: "specifiedBy" },
  { id: "r2", source_id: "VER-001", target_id: "REQ-001", relation_type: "verifiedBy" },
  { id: "r3", source_id: "REQ-002", target_id: "REQ-001", relation_type: "derivedFrom" },
  { id: "r4", source_id: "SPEC-001", target_id: "REQ-001", relation_type: "refinedBy" },
];

export const MOCK_FILES = [
  { id: "f1", name: "Capabilities.md", path: "requirements/Capabilities.md", element_count: 4 },
  { id: "f2", name: "SystemRequirements.md", path: "requirements/SystemRequirements.md", element_count: 12 },
  { id: "f3", name: "Verifications/ModelStructure.md", path: "requirements/Verifications/ModelStructure.md", element_count: 3 },
  { id: "f4", name: "Specifications.md", path: "requirements/Specifications.md", element_count: 8 },
  { id: "f5", name: "ElementTypes.ttl", path: "ontologies/ElementTypes.ttl", element_count: 1 },
];
