import { useEffect, useRef } from "react";
import {
  Box,
  Grid,
  Text,
} from "@radix-ui/themes";
import { useStore } from "../store/StoreContext";
import type { ExplorerViewProps } from "../components/ExplorerViewProps";
import type {
  OntologyGraphData,
  OntologyConstruct,
  OntologyProjectionSource,
  OntologyProjectionTerm,
} from "../store/types";
import { ViewFrame } from "./ViewFrame";

declare global {
  interface Window {
    filterOntologyGraph?: (query: string) => void;
    focusOntologyNode?: (nodeId: string) => void;
    clearOntologySelection?: () => void;
    fitOntologyGraph?: () => void;
    resetOntologyGraphLayout?: () => void;
  }
}

interface OntologyTermNode {
  id: string;
  label: string;
  role: string;
  termKind: string;
  declarations: string[];
  constructIds: string[];
  sources: OntologyProjectionSource[];
  literals: OntologyLiteralEvidence[];
}

interface OntologyConstructEdge {
  id: string;
  source: string;
  target: string;
  label: string;
  constructId: string;
}

interface OntologyLiteralEvidence {
  predicate: string;
  value: string;
  source: OntologyProjectionSource;
}

interface OntologyExplorerModel {
  terms: OntologyTermNode[];
  constructs: OntologyConstruct[];
  edges: OntologyConstructEdge[];
}

function termId(term: OntologyProjectionTerm | undefined): string | null {
  if (!term || term.kind === "literal") return null;
  return term.value;
}

function termLabel(term: OntologyProjectionTerm | undefined): string {
  return term?.label || term?.value || "";
}

function isDatatypeIri(value: string): boolean {
  return (
    value.startsWith("http://www.w3.org/2001/XMLSchema#") ||
    value.startsWith("https://www.w3.org/2001/XMLSchema#") ||
    value.startsWith("xsd:")
  );
}

function roleRank(role: string): number {
  const ranks: Record<string, number> = {
    resource: 0,
    class: 1,
    datatype: 2,
    property: 2,
    "named-individual": 3,
    "object-property": 4,
    "datatype-property": 4,
    "node-shape": 5,
    "property-shape": 5,
    restriction: 6,
    "class-expression": 6,
  };
  return ranks[role] ?? 0;
}

function upgradeRole(current: string, candidate: string): string {
  if (!current || current === "resource") return candidate;
  if (candidate === "resource") return current;
  if (current === "property" && candidate.endsWith("property")) return candidate;
  if (candidate === "class" && current !== "resource") return current;
  return roleRank(candidate) > roleRank(current) ? candidate : current;
}

function uniqueSources(sources: OntologyProjectionSource[]): OntologyProjectionSource[] {
  const seen = new Set<string>();
  return sources.filter((source) => {
    const key = [
      source.source_element_identifier,
      source.file_path,
      source.line_number,
      source.block_kind,
    ].join("|");
    if (seen.has(key)) return false;
    seen.add(key);
    return true;
  });
}

function addTerm(
  terms: Map<string, OntologyTermNode>,
  term: OntologyProjectionTerm | undefined,
  roleHint: string,
  constructId: string | null,
  source?: OntologyProjectionSource,
) {
  const id = termId(term);
  if (!id || !term) return;
  const role = isDatatypeIri(id) ? "datatype" : roleHint;
  const existing =
    terms.get(id) ??
    ({
      id,
      label: termLabel(term),
      role: role || "resource",
      termKind: term.kind,
      declarations: [],
      constructIds: [],
      sources: [],
      literals: [],
    } satisfies OntologyTermNode);
  existing.label = existing.label || termLabel(term);
  existing.role = upgradeRole(existing.role, role || "resource");
  if (constructId && !existing.constructIds.includes(constructId)) {
    existing.constructIds.push(constructId);
  }
  if (source) existing.sources.push(source);
  terms.set(id, existing);
}

function addLiteral(
  terms: Map<string, OntologyTermNode>,
  owner: OntologyProjectionTerm | undefined,
  predicate: OntologyProjectionTerm | undefined,
  literal: OntologyProjectionTerm | undefined,
  source: OntologyProjectionSource,
) {
  const ownerId = termId(owner);
  if (!ownerId || literal?.kind !== "literal") return;
  const node = terms.get(ownerId);
  if (!node) return;
  node.literals.push({
    predicate: termLabel(predicate),
    value: literal.value,
    source,
  });
}

function addEdge(
  edges: OntologyConstructEdge[],
  construct: OntologyConstruct,
  source: OntologyProjectionTerm | undefined,
  target: OntologyProjectionTerm | undefined,
  label: string,
) {
  const sourceId = termId(source);
  const targetId = termId(target);
  if (!sourceId || !targetId || sourceId === targetId) return;
  edges.push({
    id: `${construct.id}:${edges.length}`,
    source: sourceId,
    target: targetId,
    label,
    constructId: construct.id,
  });
}

function declarationEntries(
  declarations: Record<string, { role: string; element_identifier: string }[]> | undefined,
) {
  return Object.entries(declarations ?? {}).flatMap(([iri, entries]) =>
    (entries ?? []).map((entry) => ({ iri, ...entry })),
  );
}

export function buildOntologyExplorerModel(
  declarations: Record<string, { role: string; element_identifier: string }[]> | undefined,
  constructs: OntologyConstruct[] | undefined,
): OntologyExplorerModel {
  const terms = new Map<string, OntologyTermNode>();
  const edges: OntologyConstructEdge[] = [];

  for (const declaration of declarationEntries(declarations)) {
    addTerm(
      terms,
      { kind: "iri", value: declaration.iri, label: declaration.iri.split(/[#/:]/).pop() ?? declaration.iri },
      declaration.role || "resource",
      null,
    );
    const node = terms.get(declaration.iri);
    if (node && !node.declarations.includes(declaration.element_identifier)) {
      node.declarations.push(declaration.element_identifier);
    }
  }

  for (const construct of constructs ?? []) {
    const source = construct.provenance?.source;
    switch (construct.kind) {
      case "property-domain":
        addTerm(terms, construct.subject, "property", construct.id, source);
        addTerm(terms, construct.object, "class", construct.id, source);
        addEdge(edges, construct, construct.subject, construct.object, "domain");
        break;
      case "property-range":
      {
        const propertyRole =
          construct.object && isDatatypeIri(construct.object.value)
            ? "datatype-property"
            : "object-property";
        addTerm(terms, construct.subject, propertyRole, construct.id, source);
        addTerm(
          terms,
          construct.object,
          construct.object && isDatatypeIri(construct.object.value) ? "datatype" : "class",
          construct.id,
          source,
        );
        if (construct.object?.kind === "literal" && source) {
          addLiteral(terms, construct.subject, construct.predicate, construct.object, source);
        } else {
          addEdge(edges, construct, construct.subject, construct.object, "range");
        }
        break;
      }
      case "subclass-inclusion":
        addTerm(terms, construct.subject, "class", construct.id, source);
        addTerm(terms, construct.object, "class", construct.id, source);
        addEdge(edges, construct, construct.subject, construct.object, "subclass");
        break;
      case "membership":
        addTerm(terms, construct.subject, "named-individual", construct.id, source);
        addTerm(terms, construct.object, "class", construct.id, source);
        addEdge(edges, construct, construct.subject, construct.object, "member-of");
        break;
      case "inverse-property":
        addTerm(terms, construct.subject, "property", construct.id, source);
        addTerm(terms, construct.object, "property", construct.id, source);
        addEdge(edges, construct, construct.subject, construct.object, "inverse");
        break;
      case "disjointness":
        addTerm(terms, construct.subject, "class", construct.id, source);
        addTerm(terms, construct.object, "class", construct.id, source);
        addEdge(edges, construct, construct.subject, construct.object, "disjoint");
        break;
      case "equivalence-group":
        addTerm(terms, construct.subject, "class", construct.id, source);
        addTerm(terms, construct.object, "class", construct.id, source);
        for (const member of construct.members ?? []) {
          addTerm(terms, member.term, "class", construct.id, member.source);
          addEdge(edges, construct, construct.subject, member.term, "equivalent");
        }
        addEdge(edges, construct, construct.subject, construct.object, "equivalent");
        break;
      case "property-chain":
        addTerm(terms, construct.subject, "property", construct.id, source);
        for (const member of construct.members ?? []) {
          addTerm(terms, member.term, "property", construct.id, member.source);
          addEdge(edges, construct, construct.subject, member.term, "chain");
        }
        break;
      case "property-characteristic":
        addTerm(terms, construct.subject, "property", construct.id, source);
        break;
      case "restriction":
        addTerm(terms, construct.subject, "restriction", construct.id, source);
        addTerm(terms, construct.property, "property", construct.id, source);
        addTerm(terms, construct.object, "class", construct.id, source);
        addEdge(edges, construct, construct.subject, construct.property, "on-property");
        addEdge(edges, construct, construct.subject, construct.object, construct.restriction_kind || "restriction");
        break;
      case "class-expression":
        addTerm(terms, construct.subject, "class-expression", construct.id, source);
        for (const member of construct.members ?? []) {
          addTerm(terms, member.term, "class", construct.id, member.source);
          addEdge(edges, construct, construct.subject, member.term, construct.class_expression_kind || "member");
        }
        break;
      case "shape-overlay":
        addTerm(terms, construct.subject, construct.shape_overlay_kind || "node-shape", construct.id, source);
        addTerm(terms, construct.property, "property", construct.id, source);
        addTerm(terms, construct.object, "class", construct.id, source);
        addEdge(edges, construct, construct.subject, construct.object, construct.shape_overlay_kind || "shape");
        if (construct.object?.kind === "literal" && source) {
          addLiteral(terms, construct.subject, construct.predicate, construct.object, source);
        }
        break;
      default:
        addTerm(terms, construct.subject, "resource", construct.id, source);
        addTerm(terms, construct.property, "property", construct.id, source);
        addTerm(terms, construct.object, "resource", construct.id, source);
        addEdge(edges, construct, construct.subject, construct.object, construct.kind);
        if (construct.object?.kind === "literal" && source) {
          addLiteral(terms, construct.subject, construct.predicate, construct.object, source);
        }
    }

    for (const evidence of construct.provenance?.evidence ?? []) {
      addLiteral(terms, evidence.subject, evidence.predicate, evidence.object, evidence.source);
    }
  }

  for (const node of terms.values()) {
    node.constructIds.sort();
    node.declarations.sort();
    node.sources = uniqueSources(node.sources).sort((a, b) =>
      `${a.file_path}:${a.line_number}`.localeCompare(`${b.file_path}:${b.line_number}`),
    );
  }

  return {
    terms: [...terms.values()].sort((a, b) =>
      roleRank(a.role) - roleRank(b.role) || a.label.localeCompare(b.label),
    ),
    constructs: [...(constructs ?? [])].sort((a, b) => a.id.localeCompare(b.id)),
    edges,
  };
}

export function OntologiesView(_: Partial<ExplorerViewProps> = {}) {
  const { store } = useStore();
  const onto = store.ontology ?? {};
  const graphData = onto.graph_data;
  if (graphData && (graphData.nodes?.length ?? 0) > 0 && onto.graph_renderer?.js) {
    return (
      <OntologyCommittedRenderer
        graphData={graphData}
        css={onto.graph_renderer.css ?? ""}
        js={onto.graph_renderer.js}
        ttlHref={onto.ttl_href}
        summary={onto.summary}
      />
    );
  }
  return (
    <MissingCanonicalOntologyGraph
      hasGraphData={Boolean(graphData && (graphData.nodes?.length ?? 0) > 0)}
    />
  );
}

function MissingCanonicalOntologyGraph({ hasGraphData }: { hasGraphData: boolean }) {
  return (
    <ViewFrame testId="ontologies">
      <Grid columns={{ initial: "1fr", lg: "minmax(0, 1fr) 390px" }} className="graph-route">
        <Box className="graph-canvas-wrap">
          <div className="graph-render-notice">
            {hasGraphData
              ? "Ontology graph renderer assets were not exported."
              : "Ontology graph data was not exported."}
          </div>
        </Box>
        <Box className="graph-sidebar">
          <div className="graph-inspector-header">
            <Text as="div" size="3" weight="bold">
              Node Inspector
            </Text>
          </div>
          <div className="graph-inspector-body">
            <Text size="2" color="gray" className="italic">
              The canonical ontology graph requires Project Store graph data and renderer assets.
            </Text>
          </div>
        </Box>
      </Grid>
    </ViewFrame>
  );
}

function OntologyCommittedRenderer({
  graphData,
  css,
  js,
  ttlHref,
  summary,
}: {
  graphData: OntologyGraphData;
  css: string;
  js: string;
  ttlHref?: string;
  summary?: {
    ontology_blocks?: number;
    shape_blocks?: number;
    total_blocks?: number;
    total_quads?: number;
  };
}) {
  const mountRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    const mount = mountRef.current;
    if (!mount) return undefined;
    const script = document.createElement("script");
    script.type = "module";
    script.textContent = `const ontologyGraphData = ${JSON.stringify(graphData).replace(
      /</g,
      "\\u003c",
    )};\n${js}`;
    mount.appendChild(script);
    return () => {
      script.remove();
      delete (window as typeof window & {
        filterOntologyGraph?: unknown;
        focusOntologyNode?: unknown;
        clearOntologySelection?: unknown;
        fitOntologyGraph?: unknown;
        resetOntologyGraphLayout?: unknown;
      }).filterOntologyGraph;
      delete (window as typeof window & { focusOntologyNode?: unknown }).focusOntologyNode;
      delete (window as typeof window & { clearOntologySelection?: unknown }).clearOntologySelection;
      delete (window as typeof window & { fitOntologyGraph?: unknown }).fitOntologyGraph;
      delete (window as typeof window & { resetOntologyGraphLayout?: unknown }).resetOntologyGraphLayout;
    };
  }, [graphData, js]);

  return (
    <ViewFrame testId="ontologies">
      <style>{css}</style>
      <style>{`
        .ontology-page.graph-route{display:grid!important;}
        .ontology-graph-panel{display:block!important;background:var(--reqvire-surface-base);}
        .ontology-graph-canvas,.ontology-graph-sidebar,.ontology-inspector-body{background:var(--reqvire-surface-base)!important;}
        .ontology-graph-canvas{height:100%!important;min-height:0!important;}
        .ontology-inspector-header{box-sizing:border-box!important;min-height:46px!important;height:auto!important;padding:10px 14px!important;border-bottom:1px solid var(--reqvire-surface-border)!important;background:var(--reqvire-surface-base)!important;color:#172027!important;justify-content:flex-start!important;text-align:left!important;}
        .ontology-inspector-header h2{font-size:14px!important;line-height:1.3!important;color:#172027!important;}
        .ontology-sidebar-summary{box-sizing:border-box!important;width:100%!important;flex-wrap:wrap!important;gap:2px 0!important;overflow:visible!important;background:var(--reqvire-surface-muted)!important;white-space:normal!important;}
        .ontology-summary-entry + .ontology-summary-entry::before{padding:0 5px!important;}
      `}</style>
      <div ref={mountRef} className="ontology-page graph-route">
        <section className="ontology-graph-panel graph-canvas-wrap" aria-label="Ontology graph explorer">
          <div className="ontology-graph-canvas">
            <div id="ontology-graph-container" role="img" aria-label="Ontology and SHACL relationship graph" />
          </div>
        </section>
        <aside className="ontology-graph-sidebar graph-sidebar">
            <div className="ontology-search-panel">
              <input
                id="ontology-graph-search"
                type="search"
                placeholder="Search kind, domain/range, sources, SHACL, badges"
                className="ontology-graph-search"
                onInput={(event) => window.filterOntologyGraph?.(event.currentTarget.value)}
              />
              <ul id="ontology-graph-results" className="ontology-graph-results" />
            </div>
            <div className="ontology-inspector-header">
              <h2 id="ontology-inspector-title">Node Inspector</h2>
              <button id="ontology-inspector-clear" type="button" onClick={() => window.clearOntologySelection?.()} aria-label="Clear selection">
                x
              </button>
            </div>
            <div id="ontology-inspector-body" className="ontology-inspector-body">
              <p className="text-gray-500 italic m-0">
                Search or select a graph node to inspect URI, RDF type, comments, and SHACL constraints.
              </p>
            </div>
            <div className="ontology-sidebar-summary" aria-label="Ontology graph summary">
              <span className="ontology-summary-entry">Ont <strong>{summary?.ontology_blocks ?? 0}</strong></span>
              <span className="ontology-summary-entry">Shapes <strong>{summary?.shape_blocks ?? 0}</strong></span>
              <span className="ontology-summary-entry">Quads <strong>{summary?.total_quads ?? 0}</strong></span>
              <span className="ontology-summary-entry">Blocks <strong>{summary?.total_blocks ?? 0}</strong></span>
              {ttlHref && (
                <a href={ttlHref} className="ontology-summary-entry ontology-footer-download" title="Download ontologies.ttl">
                  Download .ttl
                </a>
              )}
            </div>
        </aside>
      </div>
    </ViewFrame>
  );
}
