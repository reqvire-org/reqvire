import type {
  DetailAttachmentItem,
  DetailConceptReferenceItem,
  DetailMetaBadge,
  DetailRelationItem,
  DesignSystemColorToken,
  FileBrowserBreadcrumb,
  FileBrowserItem,
  IconName,
  OntologyDetailNode,
  PaneLegendRow,
  PaneNotationLegendRow,
  PaneSummaryItem,
  ShellNavigationItem,
} from "@ds";

export type ShowcaseMockViewId = "model" | "traces" | "ontologies" | "search" | "coverage";

export interface ShowcaseMockView {
  id: ShowcaseMockViewId;
  hash: string;
  label: string;
  desc: string;
  icon: IconName;
}

export interface ShowcaseTreeRow {
  id: string;
  label: string;
  icon: IconName;
  count?: number;
  depth: number;
  kind: "folder" | "file" | "element";
  expandable?: boolean;
  open?: boolean;
  selected?: boolean;
}

export const FULL_APP_MOCKS: readonly ShowcaseMockView[] = [
  {
    id: "model",
    hash: "#/model",
    label: "Model View",
    desc: "File tree plus element grid. The primary workspace.",
    icon: "folder",
  },
  {
    id: "traces",
    hash: "#/traces",
    label: "Traces View",
    desc: "Coverage Sankey for requirements to verifications.",
    icon: "activity",
  },
  {
    id: "ontologies",
    hash: "#/ontologies",
    label: "Ontologies View",
    desc: "RDF and SHACL class hierarchy with node inspector.",
    icon: "globe",
  },
  {
    id: "search",
    hash: "#/search",
    label: "Search View",
    desc: "Full-text element search with live filtering.",
    icon: "search",
  },
  {
    id: "coverage",
    hash: "#/coverage",
    label: "Coverage View",
    desc: "Coverage KPIs, evidence bars, capability rollups, and gaps.",
    icon: "pie-chart",
  },
];

export const SHELL_NAVIGATION: ShellNavigationItem[] = [
  { value: "model", label: "Model", icon: "folder", badge: "640" },
  { value: "traces", label: "Traces", icon: "activity" },
  { value: "ontologies", label: "Ontologies", icon: "globe", badge: "14" },
  { value: "search", label: "Search", icon: "search" },
];

export const SHELL_ACTIONS = [
  { id: "help", label: "Open help", icon: "help-circle" },
  { id: "theme", label: "Toggle theme", icon: "sun" },
  { id: "settings", label: "Workspace settings", icon: "settings" },
] as const satisfies readonly { id: string; label: string; icon: IconName; active?: boolean }[];

export const PANE_SUMMARY_ITEMS = [
  { label: "Elements", value: "640" },
  { label: "Relations", value: "1,090" },
  { label: "Files", value: "38" },
  { label: "Coverage", value: "86%" },
] as const satisfies readonly PaneSummaryItem[];

export const PANE_FILTER_ROWS = [
  { id: "requirements", icon: "box", label: "Requirements", count: 128 },
  { id: "verifications", icon: "check", label: "Verifications", count: 74 },
  { id: "resources", icon: "file-text", label: "Evidence files", count: 38 },
] as const satisfies readonly { id: string; icon: IconName; label: string; count: number }[];

export const PANE_LEGEND_ROWS = [
  { id: "capability", label: "Capabilities", colorToken: "--capability" },
  { id: "requirement", label: "Requirements", colorToken: "--requirement" },
  { id: "verification", label: "Verifications", colorToken: "--verification" },
  { id: "trace", label: "Trace relation", colorToken: "--edge-trace", line: true },
] as const satisfies readonly PaneLegendRow[];

export const PANE_NOTATION_ROWS = [
  { symbol: "D/R", label: "Domain and range" },
  { symbol: "SH", label: "SHACL overlay" },
  { symbol: "INV", label: "Inverse property" },
] as const satisfies readonly PaneNotationLegendRow[];

export const FILE_BROWSER_BREADCRUMBS = [
  { path: "", label: "Project" },
  { path: "requirements", label: "requirements" },
] as const satisfies readonly FileBrowserBreadcrumb[];

export const FILE_BROWSER_ITEMS = [
  {
    kind: "folder",
    id: "requirements-capabilities",
    name: "Capabilities",
    path: "requirements/Capabilities",
    displayPath: "requirements/Capabilities",
    elementCount: 8,
    childCount: 2,
  },
  {
    kind: "folder",
    id: "requirements-functional",
    name: "Functional",
    path: "requirements/Functional",
    displayPath: "requirements/Functional",
    elementCount: 5,
    childCount: 3,
  },
  {
    kind: "file",
    id: "requirements-capabilities-md",
    name: "Capabilities.md",
    path: "requirements/Capabilities.md",
    displayPath: "requirements/Capabilities.md",
    elementCount: 3,
    childCount: 0,
    selected: true,
    contentHref: "#/content/requirements/Capabilities.md",
  },
  {
    kind: "file",
    id: "requirements-contracts-md",
    name: "Contracts.md",
    path: "requirements/Contracts.md",
    displayPath: "requirements/Contracts.md",
    elementCount: 6,
    childCount: 0,
    contentHref: "#/content/requirements/Contracts.md",
  },
] as const satisfies readonly FileBrowserItem[];

export const MODEL_TREE_ROWS: ShowcaseTreeRow[] = [
  {
    id: "requirements-folder",
    label: "requirements/",
    icon: "folder-open",
    count: 3,
    depth: 0,
    kind: "folder",
    expandable: true,
    open: true,
  },
  {
    id: "system-requirements-file",
    label: "SystemRequirements.md",
    icon: "file-text",
    count: 12,
    depth: 1,
    kind: "file",
    expandable: true,
    open: true,
  },
  {
    id: "REQ-DET-042",
    label: "Traceability Coverage Requirement",
    icon: "box",
    depth: 2,
    kind: "element",
    selected: true,
  },
  {
    id: "VER-DET-010",
    label: "Coverage Export Verification",
    icon: "check",
    depth: 2,
    kind: "element",
  },
  {
    id: "ontology-folder",
    label: "ontologies/",
    icon: "folder",
    count: 2,
    depth: 0,
    kind: "folder",
    expandable: true,
  },
];

export const WORKSPACE_STATS = [
  { label: "Requirements", value: 128, token: "--requirement" },
  { label: "Verified", value: "86%", token: "--verification" },
  { label: "Open gaps", value: 12, token: "--accent" },
] as const satisfies readonly { label: string; value: string | number; token: DesignSystemColorToken }[];

export const DETAIL_META_BADGES: DetailMetaBadge[] = [
  { key: "owner", value: "systems", provenance: "explicit" },
  { key: "priority", value: "high", provenance: "explicit" },
  { key: "lifecycle", value: "approved", provenance: "inherited" },
];

export const DETAIL_RELATIONS: DetailRelationItem[] = [
  {
    id: "rel-verifies",
    label: "verifiedBy",
    target: {
      id: "VER-DET-010",
      label: "Coverage Export Verification",
      kind: "element",
      elementType: "test-verification",
      typeFamily: "verification",
      href: "#/model/VER-DET-010",
      external: false,
    },
  },
  {
    id: "rel-derived",
    label: "derivedFrom",
    target: {
      id: "CAP-DET-001",
      label: "Project traceability capability",
      kind: "element",
      elementType: "capability",
      typeFamily: "capability",
      href: "#/model/CAP-DET-001",
      external: false,
    },
  },
  {
    id: "rel-source",
    label: "specifiedIn",
    target: {
      id: "requirements/SystemRequirements.md",
      label: "SystemRequirements.md",
      kind: "resource",
      href: "#/content/requirements/SystemRequirements.md",
      external: false,
    },
  },
];

export const DETAIL_ATTACHMENTS: DetailAttachmentItem[] = [
  {
    id: "att-report",
    targetId: "reports/coverage-summary.md",
    kind: "resource",
    label: "Coverage summary report",
    href: "#/content/reports/coverage-summary.md",
    external: false,
  },
  {
    id: "att-graph",
    targetId: "visualization/traceability-graph.json",
    kind: "resource",
    label: "Traceability graph export",
    href: "#/content/visualization/traceability-graph.json",
    external: false,
  },
];

export const DETAIL_CONCEPT_REFERENCES: DetailConceptReferenceItem[] = [
  {
    id: "concept-traceability",
    label: "Traceability",
    iri: "https://reqvire.dev/ontology#Traceability",
    ontologyNodeId: "https://reqvire.dev/ontology#Traceability",
    ontologyLabel: "Traceability",
  },
  {
    id: "concept-verification",
    label: "Verification evidence",
    iri: "https://reqvire.dev/ontology#VerificationEvidence",
    ontologyNodeId: "https://reqvire.dev/ontology#VerificationEvidence",
    ontologyLabel: "VerificationEvidence",
  },
];

export const ONTOLOGY_REQUIREMENT_NODE: OntologyDetailNode = {
  id: "https://reqvire.dev/ontology#Requirement",
  label: "Requirement",
  semantic_type: "class",
  full_uri: "https://reqvire.dev/ontology#Requirement",
  comment: "A normative statement that constrains or specifies expected system behavior.",
  rdf_types: ["owl:Class", "rdfs:Class"],
  badges: [
    { kind: "class", symbol: "CLS", label: "Class" },
    { kind: "shape", symbol: "SH", label: "Validated by SHACL" },
  ],
  slot_facets: [
    {
      slot_label: "verifiedBy",
      slot_iri: "https://reqvire.dev/ontology#verifiedBy",
      slot_kind: "object-property",
      target_class_label: "Verification",
      target_class_iri: "https://reqvire.dev/ontology#Verification",
      source_shape_label: "RequirementShape",
      source_shape_iri: "https://reqvire.dev/shapes#RequirementShape",
      facets: [
        { name: "minCount", value: "1" },
        { name: "severity", value: "Violation" },
      ],
    },
  ],
  constructs: [
    {
      id: "requirement-subclass",
      kind: "subclass-inclusion",
      label: "Requirement is a model element",
      subject: "Requirement",
      predicate: "rdfs:subClassOf",
      object: "ModelElement",
      source: {
        source_name: "core ontology",
        file_path: "ontologies/core.ttl",
        line_number: 42,
        kind: "ttl",
      },
    },
  ],
  literal_values: [
    { predicate: "skos:prefLabel", value: "Requirement" },
    { predicate: "reqvire:displayOrder", value: "20" },
  ],
  constraints: [
    { property: "sh:targetClass", value: "reqvire:Requirement" },
  ],
  sources: [
    {
      source_name: "core ontology",
      file_path: "ontologies/core.ttl",
      line_number: 42,
      kind: "ttl",
      link: "#/content/ontologies/core.ttl",
    },
    {
      source_name: "requirement shape",
      file_path: "ontologies/shapes.ttl",
      line_number: 88,
      kind: "shacl",
      link: "#/content/ontologies/shapes.ttl",
    },
  ],
};

export const ONTOLOGY_COMPACT_NODE: OntologyDetailNode = {
  id: "https://reqvire.dev/ontology#CoverageReport",
  label: "CoverageReport",
  semantic_type: "class",
  full_uri: "https://reqvire.dev/ontology#CoverageReport",
  comment: "Compact showcase node for the inline ontology detail pattern.",
  rdf_types: ["owl:Class"],
  badges: [
    { kind: "class", symbol: "CLS", label: "Class" },
  ],
  sources: [
    {
      source_name: "report ontology",
      file_path: "ontologies/reports.ttl",
      line_number: 24,
      kind: "ttl",
      link: "#/content/ontologies/reports.ttl",
    },
  ],
};

export const ONTOLOGY_NODES: OntologyDetailNode[] = [
  ONTOLOGY_REQUIREMENT_NODE,
  ONTOLOGY_COMPACT_NODE,
  {
    id: "https://reqvire.dev/ontology#Traceability",
    label: "Traceability",
    semantic_type: "class",
    full_uri: "https://reqvire.dev/ontology#Traceability",
    comment: "A semantic concept for navigable relationships between model elements.",
    rdf_types: ["owl:Class"],
    sources: [
      {
        source_name: "traceability ontology",
        file_path: "ontologies/traceability.ttl",
        line_number: 18,
        kind: "ttl",
        link: "#/content/ontologies/traceability.ttl",
      },
    ],
  },
  {
    id: "https://reqvire.dev/ontology#VerificationEvidence",
    label: "VerificationEvidence",
    semantic_type: "class",
    full_uri: "https://reqvire.dev/ontology#VerificationEvidence",
    comment: "A semantic concept for evidence that supports verification claims.",
    rdf_types: ["owl:Class"],
    sources: [
      {
        source_name: "verification ontology",
        file_path: "ontologies/verification.ttl",
        line_number: 31,
        kind: "ttl",
        link: "#/content/ontologies/verification.ttl",
      },
    ],
  },
  {
    id: "https://reqvire.dev/ontology#verifiedBy",
    label: "verifiedBy",
    semantic_type: "object-property",
    full_uri: "https://reqvire.dev/ontology#verifiedBy",
    domain: [
      {
        label: "Requirement",
        iri: "https://reqvire.dev/ontology#Requirement",
        kind: "class",
      },
    ],
    range: [
      {
        label: "Verification",
        iri: "https://reqvire.dev/ontology#Verification",
        kind: "class",
      },
    ],
  },
  {
    id: "https://reqvire.dev/ontology#specifiedIn",
    label: "specifiedIn",
    semantic_type: "object-property",
    full_uri: "https://reqvire.dev/ontology#specifiedIn",
    domain: [
      {
        label: "Requirement",
        iri: "https://reqvire.dev/ontology#Requirement",
        kind: "class",
      },
    ],
    range: [
      {
        label: "Resource",
        iri: "https://reqvire.dev/ontology#Resource",
        kind: "class",
      },
    ],
  },
];

export const CODE_SAMPLE = `export function coverageFor(requirement) {
  const verifications = requirement.relations.verifiedBy;
  return verifications.length > 0 ? "covered" : "gap";
}`;
