import { useEffect, useState } from "preact/hooks";
import { type TargetedSubmitEvent } from "preact";
import { Button } from "../../components/core/Button";
import { Icon } from "../../components/core/Icon";
import { ElementIcon } from "../../components/data/ElementIcon";
import { TokenSwatch } from "../../components/data/TokenVisual";
import { ToggleRow } from "../../components/controls/ToggleRow";
import { PaneControlSection, PaneFilterGroup, PaneFilterGrid, PaneSearchForm } from "../side-pane/PaneControls";
import { PaneActionRow, PaneGhostLink } from "../side-pane/PaneActionRow";
import { PaneLegend } from "../side-pane/PaneLegend";
import { PaneNotationLegend } from "../side-pane/PaneNotationLegend";
import { PaneSelection } from "../side-pane/PaneSelection";
import { PaneSummary, type PaneSummaryItem } from "../side-pane/PaneSummary";
import type { DesignSystemColorToken } from "../../palette";
import type { OntologyGraphNode, KnowledgeGraphNode } from "./types";

interface GraphElement { id: string; name: string; element_type: string; type_family: string; }
export interface ModelGraphControlsProps {
  selectedNodeId: string | null;
  nodes: KnowledgeGraphNode[];
  elementById: (id: string) => GraphElement | undefined;
  onOpenElement: (id: string) => void;
  onClear: () => void;
  options: { type: string; family: string; count: number }[];
  activeTypes: Set<string>;
  onResetFilters: () => void;
  onToggleType: (type: string) => void;
  summaryItems: PaneSummaryItem[];
}
export function ModelGraphControls({ selectedNodeId, nodes, elementById, onOpenElement,
  onClear, options, activeTypes, onResetFilters, onToggleType, summaryItems }: ModelGraphControlsProps) {
    return (
      <>
        <PaneControlSection aria-label="Graph controls">
          <KnowledgeGraphSelectedElementLink
            selectedNodeId={selectedNodeId}
            nodes={nodes}
            elementById={elementById}
            onOpenElement={onOpenElement}
            onClear={onClear}
          />
          <PaneFilterGroup label="Show">
            <Button size="sm" onClick={onResetFilters}>
              Reset filters
            </Button>
            {options.map((option) => (
              <ToggleRow
                key={option.type}
                label={humanize(option.type)}
                on={activeTypes.has(option.type)}
                variant="filter"
                icon={<ElementIcon type={option.type} family={option.family} size="sm" />}
                meta={formatCompactCount(option.count)}
                onToggle={() => onToggleType(option.type)}
              />
            ))}
          </PaneFilterGroup>
        </PaneControlSection>
        <PaneSummary items={summaryItems} placement="footer" />
      </>
    );

}

export const ONTOLOGY_CONSTRUCT_FILTERS = [
  ["domain-range", "Domain/range", "D/R"],
  ["subclass", "Subclass", "⊆"],
  ["membership", "Membership", "∈"],
  ["disjoint", "Disjoint", "⟂"],
  ["equivalence", "Equivalence", "⇔"],
  ["inverse", "Inverse", "⟲"],
  ["property-chain", "Property chain", "∘"],
  ["property-characteristic", "Property char.", "→"],
  ["restriction", "Restriction", "∀"],
  ["class-expression", "Class expr.", "∩"],
  ["shape-overlay", "SHACL overlay", "SH"],
] as const;

export const ONTOLOGY_ORIGIN_FILTERS = [
  ["authored", "Defined", "authored"],
  ["registry", "Registry", "registry"],
  ["construct", "Constructs", "construct"],
] as const;

export const ONTOLOGY_LAYER_FILTERS = [
  ["layer-authored", "Ontologies", "authored"],
  ["layer-concepts", "Concepts", "concepts"],
  ["layer-reqvire-context", "Semantic Context", "semantic"],
  ["layer-external-source", "External Sources", "external"],
] as const;

export const ONTOLOGY_DEFAULT_FILTERS = [
  "semantic-query",
  "layer-authored",
  "layer-concepts",
  "ontology-term",
  "shacl-shape",
  "resource",
  "external-reference",
  "class-membership",
  "class-disjointness",
  "class-expressions",
  "domain-range",
  "subclass",
  "membership",
  "disjoint",
  "equivalence",
  "inverse",
  "property-chain",
  "property-characteristic",
  "class-expression",
  "shape-overlay",
  "authored",
  "registry",
  "construct",
] as const;

export interface OntologyControlsPaneProps {
  selectedNodeId: string | null;
  nodes: OntologyGraphNode[];
  onOpenNode: (id: string) => void;
  onClear: () => void;
  filters: Set<string>;
  onToggleFilter: (filter: string) => void;
  onResetLayout: () => void;
  summaryItems: PaneSummaryItem[];
  ttlHref?: string | null;
}
export function OntologyControlsPane(props: OntologyControlsPaneProps) {
  return <><OntologyGraphSearch /><OntologyControls {...props} /></>;
}
function OntologyControls({ selectedNodeId, nodes, onOpenNode, onClear, filters,
  onToggleFilter, onResetLayout, summaryItems, ttlHref }: OntologyControlsPaneProps) {
  const layerCounts = new Map<string, number>();
  for (const node of nodes) layerCounts.set(node.layer, (layerCounts.get(node.layer) ?? 0) + 1);
    return (
      <>
        <PaneControlSection aria-label="Ontology controls">
          <OntologySelectedNodeLink
            selectedNodeId={selectedNodeId}
            nodes={nodes}
            onOpenOntologyNode={onOpenNode}
            onClear={onClear}
          />
          <PaneFilterGroup label="Graph">
            <PaneActionRow>
              {ttlHref ? (
                <PaneGhostLink
                  href={ttlHref}
                  title="Download the exported ontology as Turtle (ontologies.ttl)"
                >
                  <Icon name="download" />
                  Download .ttl
                </PaneGhostLink>
              ) : null}
              <Button
                tone="ghost"
                size="sm"
                iconLeft={<Icon name="rotate-ccw" />}
                onClick={onResetLayout}
              >
                Reset layout
              </Button>
            </PaneActionRow>
          </PaneFilterGroup>
          <PaneFilterGroup label="Overlays">
            {ONTOLOGY_LAYER_FILTERS.map(([value, label]) => {
              const layer = value.replace("layer-", "");
              const count = formatCompactCount(layerCounts.get(layer) ?? 0);
              return (
                <ToggleRow
                  key={value}
                  label={label}
                  colorToken={ontologyLayerColorToken(value)}
                  on={filters.has(value)}
                  variant="filter"
                  meta={count}
                  title={ontologyLayerDescription(value)}
                  onToggle={() => onToggleFilter(value)}
                />
              );
            })}
          </PaneFilterGroup>
          <PaneFilterGrid columns="two">
            <div>
              <PaneFilterGroup label="Types">
                <ToggleRow label="Semantic query" colorToken="--color-role-ontology" variant="filter"
                  on={filters.has("semantic-query")}
                  onToggle={() => onToggleFilter("semantic-query")} />
                <PaneLegend
                  rows={[
                    { id: "class", label: "Class", colorToken: ontologyColorToken("class") },
                    { id: "skos-concept", label: "Concept", colorToken: ontologyColorToken("skos-concept") },
                    { id: "skos-concept-scheme", label: "Concept Scheme", colorToken: ontologyColorToken("skos-concept-scheme") },
                    { id: "named-individual", label: "Individual", colorToken: ontologyColorToken("named-individual") },
                    { id: "datatype", label: "Datatype", colorToken: ontologyColorToken("datatype") },
                    { id: "class-expression", label: "Class expr.", colorToken: ontologyColorToken("class-expression") },
                    { id: "node-shape", label: "Node shape", colorToken: ontologyColorToken("node-shape") },
                    { id: "property-shape", label: "Property shape", colorToken: ontologyColorToken("property-shape") },
                    { id: "resource", label: "Resource", colorToken: ontologyColorToken("resource") },
                  ]}
                />
                <PaneLegend rows={[{ id: "relation", label: "Relation", colorToken: "--text-muted", line: true }]} />
              </PaneFilterGroup>
            </div>
            <div>
              <PaneFilterGroup label="Notation">
                <PaneNotationLegend
                  rows={[
                    { symbol: "D/R", label: "Domain/range" },
                    { symbol: "⊆", label: "Subclass" },
                    { symbol: "∈", label: "Membership" },
                    { symbol: "⟂", label: "Disjoint" },
                    { symbol: "⇔", label: "Equivalence" },
                    { symbol: "⟲", label: "Inverse" },
                    { symbol: "∘", label: "Property chain" },
                    { symbol: "∩", label: "Class expr." },
                    { symbol: "SH", label: "SHACL overlay" },
                  ]}
                />
              </PaneFilterGroup>
            </div>
          </PaneFilterGrid>
        </PaneControlSection>
        <PaneSummary items={summaryItems} placement="footer" />
      </>
    );

}

function KnowledgeGraphSelectedElementLink({
  selectedNodeId,
  nodes,
  elementById,
  onOpenElement,
  onClear,
}: {
  selectedNodeId: string | null;
  nodes: KnowledgeGraphNode[];
  elementById: (id: string) => GraphElement | undefined;
  onOpenElement: (id: string) => void;
  onClear: () => void;
}) {
  const node = selectedNodeId
    ? nodes.find((candidate) => candidate.id === selectedNodeId)
    : undefined;
  const element = node?.identifier ? elementById(node.identifier) : undefined;
  const kind = element?.element_type || node?.element_type || node?.node_type || node?.type || "element";

  return (
    <PaneSelection
      ariaLabel="Selected graph element"
      emptyMessage="Select a graph node to inspect its details."
      selection={
        element
          ? {
              icon: (
                <ElementIcon
                  type={element.element_type}
                  family={element.type_family}
                  title={element.element_type}
                  size="sm"
                />
              ),
              name: element.name,
              kind,
            }
          : undefined
      }
      openTitle="Open element details"
      onOpen={element ? () => onOpenElement(element.id) : undefined}
      onClear={onClear}
    />
  );
}


function OntologySelectedNodeLink({
  selectedNodeId,
  nodes,
  onOpenOntologyNode,
  onClear,
}: {
  selectedNodeId: string | null;
  nodes: OntologyGraphNode[];
  onOpenOntologyNode: (id: string) => void;
  onClear: () => void;
}) {
  const node = selectedNodeId
    ? nodes.find((candidate) => candidate.id === selectedNodeId)
    : undefined;
  const kind = node ? node.semantic_type || node.node_type || node.type || "resource" : "";

  return (
    <PaneSelection
      ariaLabel="Selected ontology node"
      emptyMessage="Select a graph node to inspect its details."
      selection={
        node
          ? {
              icon: <TokenSwatch colorToken={ontologyColorToken(kind)} title={kind} />,
              name: node.label || node.id,
              kind,
            }
          : undefined
      }
      openTitle="Open node details"
      onOpen={node ? () => onOpenOntologyNode(node.id) : undefined}
      onClear={onClear}
    />
  );
}


function formatCompactCount(value: number): string {
  if (value >= 1_000_000) return `${trimCompactNumber(value / 1_000_000)}M`;
  if (value >= 1_000) return `${trimCompactNumber(value / 1_000)}K`;
  return value.toLocaleString();
}


function trimCompactNumber(value: number): string {
  return value >= 10 ? Math.round(value).toString() : value.toFixed(1).replace(/\.0$/, "");
}


function humanize(value: string) {
  return value.replace(/-/g, " ").replace(/\b\w/g, (match) => match.toUpperCase());
}



function ontologyColorToken(value: string): DesignSystemColorToken {
  const colors: Record<string, DesignSystemColorToken> = {
    class: "--color-role-rdf-class",
    "semantic-query": "--color-role-ontology",
    "object-property": "--color-role-rdf-objprop",
    "datatype-property": "--color-role-rdf-dtprop",
    "rdf-property": "--color-role-rdf-rdfprop",
    property: "--color-role-rdf-objprop",
    "named-individual": "--color-role-rdf-individual",
    "skos-concept": "--color-role-rdf-concept",
    "skos-concept-scheme": "--color-role-rdf-concept-scheme",
    datatype: "--color-role-rdf-datatype",
    restriction: "--color-role-rdf-restriction",
    "class-expression": "--color-role-rdf-classexpr",
    "node-shape": "--color-role-rdf-nodeshape",
    "property-shape": "--color-role-rdf-propshape",
    resource: "--color-role-rdf-resource",
    relation: "--color-role-edge-default",
  };
  return colors[value] ?? colors.resource;
}


function ontologyLayerColorToken(value: string): DesignSystemColorToken {
  const colors: Record<string, DesignSystemColorToken> = {
    "layer-authored": "--color-role-ontology",
    "layer-concepts": "--color-role-rdf-concept",
    "layer-reqvire-context": "--info",
    "layer-external-source": "--color-role-other",
  };
  return colors[value] ?? "--text-muted";
}


function ontologyLayerDescription(value: string): string {
  const descriptions: Record<string, string> = {
    "layer-authored": "Authored OWL/RDFS/SHACL structural ontology nodes and projection edges.",
    "layer-concepts": "Curated SKOS concept nodes, concept taxonomy edges, and mapsToConcept bridge edges.",
    "layer-reqvire-context": "Semantic context: model elements that declare or reference ontology terms.",
    "layer-external-source": "Used external ontology subset triples derived from declared external sources.",
  };
  return descriptions[value] ?? "";
}

function OntologyGraphSearch() {
  const [query, setQuery] = useState("");

  useEffect(() => {
    window.filterOntologyGraph?.(query);
  }, [query]);

  function submitSearch(event: TargetedSubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    window.filterOntologyGraph?.(query);
  }

  return (
    <PaneSearchForm
      searchInputId="ontology-graph-search"
      inputLabel="Search Explorer"
      placeholder="Search ontology graph..."
      value={query}
      resultsId="ontology-graph-results"
      onQueryChange={setQuery}
      onSubmit={submitSearch}
    />
  );
}
