/*
 * Project Store schema — TypeScript mirror of the Rust `ExplorerProjectStore`
 * defined in crates/reqvire-core/src/html/store.rs (owned by Task-50).
 *
 * This file is the authoritative frontend view of the seed contract. Keep it
 * in sync with store.rs. The schema version string below must match
 * `SCHEMA_VERSION` in store.rs; loadStore.ts diagnoses mismatches.
 *
 * Opaque report projections (`submodels`, `traces`, `coverage`, `ontology`,
 * `knowledge_graph`) are typed as `unknown` here on purpose: their internal
 * shapes are produced by existing report generators and are consumed by the
 * dedicated view modules, which will narrow them as those views are built out.
 * Store consumers must tolerate unknown future fields (forward-compatible).
 */

/** Must match `SCHEMA_VERSION` in crates/reqvire-core/src/html/store.rs. */
export const EXPECTED_SCHEMA_VERSION = "2026-06-30.project-store.v4";

export interface ProjectStoreProject {
  name: string;
  root_label: string;
  workspace_root: string;
  eligible_git_worktrees: ProjectStoreGitWorktree[];
  repository?: string | null;
  branch?: string | null;
}

export interface ProjectStoreGitWorktree {
  root: string;
  workspace_relative_root: string;
  head?: string | null;
  dirty: boolean;
}

export interface ProjectStoreFolder {
  path: string;
  parent: string | null;
  children: string[];
}

export interface ProjectStoreFile {
  path: string;
  display_path: string;
  markdown_content: string;
  parent_folder: string;
  element_ids: string[];
  resource_ids: string[];
}

export interface ProjectStoreResource {
  id: string;
  kind: string;
  target: string;
  display: string;
  file_path: string | null;
  source_text?: string | null;
  external_url: string | null;
  referring_element_ids: string[];
  relation_types: string[];
}

export interface ProjectStoreElement {
  id: string;
  name: string;
  element_type: string;
  type_family: string;
  file_path: string;
  line_number: number;
  source_anchor: string;
  content: string;
  metadata: Record<string, string>;
  governance: Record<string, string>;
}

export interface ProjectStoreRelation {
  id: string;
  source_id: string;
  target_id: string;
  target_kind: string;
  relation_type: string;
  canonical_relation_type: string;
  source_relation_types: string[];
  authored: boolean;
  generated_opposite: boolean;
  resource_id: string | null;
}

export interface ProjectStoreContractBindingEntry {
  id: string;
  source_id: string;
  target: string;
  target_kind: string;
  resource_id: string | null;
  content_hash: string | null;
}

export interface ProjectStoreConceptReference {
  id: string;
  source_id: string;
  target_element_id: string;
  label: string;
  iri: string;
  line_number: number;
}

export interface ProjectStoreThesaurusUsage {
  id: string;
  label: string;
  type: string;
}

export interface ProjectStoreThesaurusScheme {
  id: string;
  element_id: string;
  label: string;
  definition: string;
  source_href: string;
  source_label: string;
  concept_base: string;
  concept_prefix: string;
  top_concept_ids: string[];
}

export interface ProjectStoreThesaurusConcept {
  id: string;
  element_id: string;
  label: string;
  scheme_id: string;
  scheme_element_id: string;
  scheme_label: string;
  parent_id: string | null;
  child_ids: string[];
  definition: string;
  alt_labels: string[];
  scope_note: string;
  example_values: string[];
  related_ids: string[];
  exact_match_ids: string[];
  close_match_ids: string[];
  used_by: ProjectStoreThesaurusUsage[];
  maps_to: ProjectStoreThesaurusUsage[];
  source_href: string;
  source_label: string;
}

export interface ProjectStoreThesaurus {
  schemes: ProjectStoreThesaurusScheme[];
  concepts: ProjectStoreThesaurusConcept[];
}

export interface ProjectStoreSearchDocument {
  id: string;
  kind: string;
  title: string;
  route: string;
  text: string;
}

export interface ProjectStoreSummaries {
  elements: number;
  files: number;
  folders: number;
  resources: number;
  relations: number;
  contract_bindings: number;
  concept_refs: number;
  ontology_blocks: number;
  shape_blocks: number;
}

export interface ProjectStoreRoute {
  id: string;
  pattern: string;
  title: string;
}

export interface ProjectStoreRoutes {
  canonical: ProjectStoreRoute[];
}

/*
 * Report projections.
 *
 * These mirror the existing Rust report generators surfaced through the Project
 * Store. They are intentionally partial/forward-compatible (consumers tolerate
 * extra and missing fields) since the underlying reports evolve independently.
 */
export interface CoverageSummary {
  total_requirements_in_scope?: number;
  total_leaf_requirements?: number;
  covered_requirements?: number;
  uncovered_requirements?: number;
  verified_leaf_requirements?: number;
  unverified_leaf_requirements?: number;
  leaf_requirements_coverage_percentage?: number;
  implementation_coverage_percentage?: number;
  total_verifications?: number;
  total_test_verifications?: number;
  satisfied_test_verifications?: number;
  unsatisfied_test_verifications?: number;
  test_verifications_satisfaction_percentage?: number;
  orphaned_verifications?: number;
  orphaned_verifications_percentage?: number;
  verification_types?: Record<string, number>;
  coverage_sources?: Record<string, number>;
}

export interface CoverageProjection {
  summary?: CoverageSummary;
  [section: string]: unknown;
}

export interface TraceVerification {
  identifier: string;
  name: string;
  file: string;
  type?: string;
  directly_verified_count?: number;
  total_requirements_in_tree?: number;
  directly_verified_requirements?: string[];
  trace_tree?: TraceTree;
}

export interface TraceTree {
  requirements: TraceRequirementNode[];
}

export interface TraceRequirementNode {
  id: string;
  name: string;
  type: string;
  is_directly_verified: boolean;
  children: TraceRequirementNode[];
}

export interface TraceFileEntry {
  verifications: TraceVerification[];
}

export interface TracesProjection {
  files: Record<string, TraceFileEntry>;
}

export interface OntologyProjection {
  summary?: {
    ontology_blocks?: number;
    shape_blocks?: number;
    total_blocks?: number;
    total_quads?: number;
  };
  graph_layers?: OntologyGraphLayer[];
  declarations?: Record<string, OntologyTermDeclaration[]>;
  shape_references?: { element_identifier: string; iri: string; kind: string }[];
  blocks?: { file_path: string; kind: string; line_number: number }[];
  diagnostics?: unknown[];
  projection?: OntologyConstructProjectionGraph;
  graph_data?: OntologyGraphData;
  external_sources?: ExternalOntologySource[];
  external_materialization?: "none" | "used_subset" | string;
  external_counts?: OntologyExternalCounts;
  ttl_href?: string;
}

export interface OntologyGraphLayer {
  role: string;
  graph_iri: string;
  included: boolean;
  source: string;
}

export interface OntologyTermDeclaration {
  iri: string;
  role: string;
  element_identifier: string;
  external?: boolean;
  materialized_in_used_subset?: boolean;
}

export interface ExternalOntologySource {
  owner_identifier: string;
  owner_name: string;
  prefix: string;
  namespace: string;
  resource?: string | null;
  source: string;
  format: string;
  line_number: number;
}

export interface OntologyExternalCounts {
  declared_external_source_count?: number;
  visible_external_source_count?: number;
  used_external_source_count?: number;
  available_external_term_declaration_count?: number;
  visible_external_term_declaration_count?: number;
  materialized_external_term_count?: number;
  raw_external_triple_count?: number;
  materialized_external_triple_count?: number;
}

export interface OntologyProjectionTerm {
  kind: "iri" | "blank-node" | "literal" | string;
  value: string;
  label: string;
}

export interface OntologyProjectionSource {
  source_block: string;
  source_element_identifier: string;
  source_name: string;
  file_path: string;
  line_number: number;
  block_kind: string;
}

export interface OntologyProjectionEvidence {
  source: OntologyProjectionSource;
  subject: OntologyProjectionTerm;
  predicate: OntologyProjectionTerm;
  object: OntologyProjectionTerm;
}

export interface OntologyProjectionProvenance {
  derivation_mode: string;
  source: OntologyProjectionSource;
  pattern_contract_iri?: string;
  evidence?: OntologyProjectionEvidence[];
}

export interface OntologyConstructMember {
  sequence_index: number;
  term: OntologyProjectionTerm;
  source: OntologyProjectionSource;
}

export interface OntologySymbol {
  concept_name: string;
  raw_unicode_code_point: string;
  rendered_unicode_character: string;
  tooltip: string;
  accessible_label: string;
}

export interface OntologyConstruct {
  id: string;
  family: string;
  kind: string;
  subject: OntologyProjectionTerm;
  predicate?: OntologyProjectionTerm;
  object?: OntologyProjectionTerm;
  property?: OntologyProjectionTerm;
  members?: OntologyConstructMember[];
  property_characteristic?: string;
  restriction_kind?: string;
  class_expression_kind?: string;
  shape_overlay_kind?: string;
  symbol?: OntologySymbol;
  provenance: OntologyProjectionProvenance;
}

export interface OntologyConstructProjection {
  id: string;
  family: string;
  derivation_mode: string;
  pattern_contract_iri?: string;
  construct_ids: string[];
}

export interface OntologyConstructProjectionGraph {
  id: string;
  derivation_mode: string;
  projections: OntologyConstructProjection[];
  constructs: OntologyConstruct[];
  symbols?: OntologySymbol[];
}

export interface OntologyGraphData {
  nodes?: OntologyGraphNode[];
  edges?: OntologyGraphEdge[];
}

export interface OntologyGraphNode {
  id: string;
  label: string;
  type?: string;
  node_type?: string;
  semantic_type: string;
  layer: "authored" | "concepts" | "reqvire-context" | "external-source";
  source_kind: "ontology" | "shape" | "concepts" | "model-context" | "external-ontology";
  full_uri: string;
  ontology_document?: string;
  scheme_iri?: string;
  scheme_label?: string;
  comment: string;
  rdf_types: string[];
  type_evidence: OntologyGraphTypeEvidence[];
  sources: OntologyGraphSource[];
  constraints: { property: string; value: string }[];
  badges: { kind: string; symbol: string; code_point: string; label: string }[];
  equivalence_group: string;
  inverse_properties: string[];
  property_chains: { id: string; members: string[]; source: string }[];
  domain: { label: string; iri: string; kind: string }[];
  range: { label: string; iri: string; kind: string }[];
  literal_values: { predicate: string; value: string; source: OntologyGraphSource }[];
  slot_facets: OntologyGraphSlotFacet[];
  constructs: OntologyGraphConstructDetail[];
}

export interface OntologyGraphEdge {
  source: string;
  target: string;
  label: string;
  layer: "authored" | "concepts" | "reqvire-context" | "external-source";
  source_kind: "ontology" | "shape" | "concepts" | "model-context" | "external-ontology";
}

export interface OntologyGraphSource {
  source: string;
  source_name: string;
  file_path: string;
  line_number: number;
  kind: string;
  link: string;
}

export interface OntologyGraphTypeEvidence {
  iri: string;
  label: string;
  source: OntologyGraphSource;
}

export interface OntologyGraphSlotFacet {
  slot_label: string;
  slot_iri: string;
  slot_kind: string;
  target_class_label: string;
  target_class_iri: string;
  source_shape_label: string;
  source_shape_iri: string;
  source: OntologyGraphSource;
  facets: { name: string; value: string }[];
}

export interface OntologyGraphConstructDetail {
  id: string;
  family: string;
  kind: string;
  label: string;
  subject: string;
  predicate: string;
  object: string;
  property: string;
  members: string[];
  source: OntologyGraphSource;
  badge?: { kind: string; symbol: string; code_point: string; label: string };
}

export interface KnowledgeGraphNode {
  id: string;
  identifier: string;
  label: string;
  type?: string;
  node_type?: string;
  element_type: string;
  file_path: string;
  line_number?: number;
  link?: string;
  description?: string;
  metadata?: KnowledgeGraphFact[];
  governance?: KnowledgeGraphFact[];
  outgoing?: KnowledgeGraphFact[];
  incoming?: KnowledgeGraphFact[];
  contract_bindings?: KnowledgeGraphFact[];
  concept_references?: KnowledgeGraphFact[];
}

export interface KnowledgeGraphFact {
  name: string;
  value: string;
  link?: string;
  kind?: string;
}

export interface KnowledgeGraphSubmodel {
  root_id: string;
  root_name: string;
  root_type: string;
  requirement_count: number;
}

export interface KnowledgeGraphProjection {
  summary?: {
    elements?: number;
    relations?: number;
    contract_bindings?: number;
    concept_references?: number;
    resources?: number;
    submodels?: number;
  };
  nodes?: KnowledgeGraphNode[];
  edges?: { source: string; target: string; label: string; kind: string; authored?: boolean }[];
  submodels?: KnowledgeGraphSubmodel[];
}

export interface ExplorerProjectStore {
  schema_version: string;
  project: ProjectStoreProject;
  folders: ProjectStoreFolder[];
  files: ProjectStoreFile[];
  resources: ProjectStoreResource[];
  elements: ProjectStoreElement[];
  relations: ProjectStoreRelation[];
  contract_bindings: ProjectStoreContractBindingEntry[];
  concept_refs: ProjectStoreConceptReference[];
  thesaurus: ProjectStoreThesaurus;
  /** Opaque: capability-rooted submodel report projection. */
  submodels: unknown;
  /** Verification trace report projection. */
  traces: TracesProjection;
  /** Coverage report projection. */
  coverage: CoverageProjection;
  /** Ontology projection + SHACL/Turtle evidence. */
  ontology: OntologyProjection;
  /** Graph-ready nodes/edges projection. */
  knowledge_graph: KnowledgeGraphProjection;
  search: ProjectStoreSearchDocument[];
  summaries: ProjectStoreSummaries;
  routes: ProjectStoreRoutes;
}

/** Global loaded from the generated `assets/project-store.js` data asset. */
declare global {
  interface Window {
    reqvireProjectStore?: unknown;
  }
}
