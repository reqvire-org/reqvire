import { useState, type ReactNode } from "react";
import {
  Icon,
  Modal,
  ModalBody,
  ModalClose,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalTitle,
  TypeBadge,
} from "@ds";
import { useStore } from "../store/StoreContext";
import type {
  OntologyGraphConstructDetail,
  OntologyGraphNode,
  OntologyGraphSlotFacet,
  OntologyGraphSource,
} from "../store/types";

type MetaRow = { key: string; value: ReactNode };

export function OntologyNodeDetailModal({
  nodeId,
  onClose,
}: {
  nodeId: string | null;
  onClose: () => void;
}) {
  const { store } = useStore();
  const node = nodeId
    ? (store.ontology.graph_data?.nodes ?? []).find((candidate) => candidate.id === nodeId)
    : undefined;
  const graphNodes = store.ontology.graph_data?.nodes ?? [];
  const [copiedUri, setCopiedUri] = useState(false);

  return (
    <Modal open={nodeId !== null} onOpenChange={(open) => !open && onClose()}>
      <ModalContent className="element-detail-dialog ontology-node-detail-dialog" showCloseButton={false}>
        {!node ? (
          <>
            <ModalHeader className="element-detail-header">
              <ModalTitle>Ontology node not found</ModalTitle>
            </ModalHeader>
            <ModalBody className="element-detail-body">
              <p className="element-detail-muted">
                No exported ontology graph node matches{" "}
                <code className="rq-coderef">{nodeId ?? ""}</code>.
              </p>
            </ModalBody>
          </>
        ) : (
          <>
            <OntologyNodeModalHeader node={node} />
            <ModalBody className="element-detail-body">
              <div className="ontology-modal-layout">
                <main className="ontology-modal-main">
                  <OntologyPropertyUsages node={node} nodes={graphNodes} />
                  {isPropertyNode(node) && (node.domain?.length || node.range?.length) ? (
                    <Section title="Domain / range">
                      <div className="ontology-term-grid">
                        {node.domain?.length ? <TermRefs kind="domain" terms={node.domain} /> : null}
                        {node.range?.length ? <TermRefs kind="range" terms={node.range} /> : null}
                      </div>
                    </Section>
                  ) : null}
                  {node.slot_facets?.length ? (
                    <Section title={isPropertyNode(node) ? "Used as slot / facets" : "Slots / facets"}>
                      <div className="ontology-modal-card-list">
                        {node.slot_facets.map((slot, index) => (
                          <SlotFacetCard
                            key={`${slot.slot_iri}-${slot.source_shape_iri}-${index}`}
                            slot={slot}
                            propertyContext={isPropertyNode(node)}
                          />
                        ))}
                      </div>
                    </Section>
                  ) : null}
                  {node.constructs?.length ? (
                    <Section title="Projection constructs">
                      <div className="ontology-modal-card-list">
                        {visibleConstructs(node).map((construct) => (
                          <ConstructRow key={construct.id} construct={construct} />
                        ))}
                      </div>
                    </Section>
                  ) : null}
                  {node.literal_values?.length ? (
                    <Section title="Literal values">
                      <div className="ontology-modal-card-list">
                        {node.literal_values.map((literal, index) => (
                          <div className="ontology-modal-card ontology-modal-card--compact" key={`${literal.predicate}-${index}`}>
                            <span className="rq-relation__kind">{literal.predicate || "value"}</span>
                            <strong>{literal.value}</strong>
                          </div>
                        ))}
                      </div>
                    </Section>
                  ) : null}
                  {node.constraints?.length ? (
                    <Section title="Raw SHACL evidence">
                      <div className="ontology-modal-card-list">
                        {node.constraints.map((constraint) => (
                          <div className="ontology-modal-card ontology-modal-card--compact" key={`${constraint.property}-${constraint.value}`}>
                            <span className="rq-relation__kind">{constraint.property}</span>
                            <code className="rq-coderef">{constraint.value}</code>
                          </div>
                        ))}
                      </div>
                    </Section>
                  ) : null}
                  {node.sources?.length ? (
                    <Section title="Sources">
                      <div className="ontology-modal-source-list">
                        {dedupeSources(node.sources).map((source, index) => (
                          <SourceRow key={`${source.link}-${index}`} source={source} />
                        ))}
                      </div>
                    </Section>
                  ) : null}
                </main>
                <aside className="ontology-modal-rail">
                  <OntologyNodeMetadata
                    node={node}
                    copiedUri={copiedUri}
                    onCopyUri={() => {
                      const uri = node.full_uri || node.id;
                      void navigator.clipboard?.writeText(uri);
                      setCopiedUri(true);
                      window.setTimeout(() => setCopiedUri(false), 1400);
                    }}
                  />
                  {node.comment ? (
                    <Section title="Description">
                      <p>{node.comment}</p>
                    </Section>
                  ) : null}
                  {node.badges?.length ? (
                    <Section title="Notation">
                      <div className="ontology-modal-badge-row">
                        {node.badges.map((badge) => (
                          <span className="ontology-modal-symbol" key={`${badge.kind}-${badge.symbol}`}>
                            <span aria-hidden="true">{badge.symbol}</span>
                            {badge.label}
                          </span>
                        ))}
                      </div>
                    </Section>
                  ) : null}
                </aside>
              </div>
            </ModalBody>
            <ModalFooter className="element-detail-footer">
              <div className="element-detail-footer-row">
                {node.sources?.[0]?.link ? (
                  <a
                    href={node.sources[0].link}
                    className="element-detail-source-link"
                    onClick={(event) => {
                      event.preventDefault();
                      window.location.hash = node.sources[0].link;
                    }}
                  >
                    <Icon name="external-link" className="ex-icon-sm" /> Open ontology source
                  </a>
                ) : (
                  <span />
                )}
                <ModalClose asChild>
                  <button type="button" className="rq-btn rq-btn--primary rq-btn--sm">
                    Close
                  </button>
                </ModalClose>
              </div>
            </ModalFooter>
          </>
        )}
      </ModalContent>
    </Modal>
  );
}

function OntologyNodeModalHeader({ node }: { node: OntologyGraphNode }) {
  const kind = ontologyNodeKind(node);
  return (
    <ModalHeader className="element-detail-header">
      <div className="element-detail-title-row">
        <TypeBadge type="ontology" family="ontology" tinted className="element-detail-family-badge">
          ontology
        </TypeBadge>
        {kind !== "ontology" ? (
          <TypeBadge type={kind} family="ontology" tinted>
            {kind}
          </TypeBadge>
        ) : null}
        <ModalTitle>{node.label || node.id}</ModalTitle>
        <ModalClose asChild>
          <button type="button" className="rq-iconbtn rq-iconbtn--ghost element-detail-close" aria-label="Close">
            <Icon name="x" />
          </button>
        </ModalClose>
      </div>
    </ModalHeader>
  );
}

function OntologyNodeMetadata({
  node,
  copiedUri,
  onCopyUri,
}: {
  node: OntologyGraphNode;
  copiedUri: boolean;
  onCopyUri: () => void;
}) {
  const rdfTypes = Array.from(new Set(
    [
      ontologyNodeKind(node),
      ...(node.rdf_types?.length ? node.rdf_types : [node.type || ontologyNodeKind(node)]),
    ]
      .filter(Boolean)
      .map((type) => String(type).toLowerCase()),
  ));
  return (
    <div className="ex-meta">
      <div className="ex-meta__row">
        <span className="ex-meta__k">RDF type</span>
        <span className="ex-meta__v ontology-badge-row">
          {rdfTypes.map((type) => (
            <span key={type} className="ontology-type-pill">
              {type}
            </span>
          ))}
        </span>
      </div>
      <div className="ex-meta__row">
        <span className="ex-meta__k">Full URI</span>
        <span className="ex-meta__v">
          <button type="button" className="ontology-uri-copy" onClick={onCopyUri} title="Copy URI">
            <code className="rq-coderef">{node.full_uri || node.id}</code>
            <Icon name={copiedUri ? "check" : "copy"} />
          </button>
        </span>
      </div>
    </div>
  );
}

function Section({ title, children }: { title: string; children: ReactNode }) {
  return (
    <section className="element-detail-section">
      <h3>{title}</h3>
      {children}
    </section>
  );
}

function TermRefs({
  kind,
  terms,
}: {
  kind: string;
  terms: { label: string; iri: string; kind: string }[];
}) {
  return (
    <>
      {terms.map((term) => (
        <div className="ontology-modal-card ontology-modal-card--compact" key={`${kind}-${term.iri}`}>
          <span className="rq-relation__kind">{kind}</span>
          <OntologyTerm term={term} />
        </div>
      ))}
    </>
  );
}

function OntologyPropertyUsages({
  node,
  nodes,
}: {
  node: OntologyGraphNode;
  nodes: OntologyGraphNode[];
}) {
  const usages = propertyUsagesForNode(node, nodes);
  if (!usages.length) return null;
  return (
    <Section title="Properties">
      <div className="ontology-modal-card-list">
        {usages.map((usage) => (
          <div className="ontology-modal-card" key={`${usage.property.id}-${usage.role}`}>
            <div className="ontology-modal-card-head">
              <strong className="rq-mono" title={usage.property.full_uri || usage.property.id}>
                {usage.property.label || shortLabel(usage.property.id)}
              </strong>
              <TypeBadge type={usage.property.semantic_type} family="ontology" dot={false} className={rdfBadgeClass(usage.property.semantic_type)}>
                {humanizeSemanticType(usage.property.semantic_type)}
              </TypeBadge>
            </div>
            <MetaRows
              rows={[
                { key: "role", value: usage.role },
                { key: "domain", value: <TermList terms={usage.domains} emptyLabel="Any" /> },
                { key: "range", value: <TermList terms={usage.ranges} emptyLabel="Any" /> },
              ]}
            />
            {usage.facets.length ? (
              <div className="ontology-modal-facet-row">
                {usage.facets.flatMap((facet) => facet.facets ?? []).map((facet, index) => (
                  <span className="ontology-modal-facet" key={`${facet.name}-${facet.value}-${index}`}>
                    <span>{facet.name}</span>
                    {facet.value}
                  </span>
                ))}
              </div>
            ) : null}
          </div>
        ))}
      </div>
    </Section>
  );
}

function SlotFacetCard({
  slot,
  propertyContext,
}: {
  slot: OntologyGraphSlotFacet;
  propertyContext: boolean;
}) {
  const title = propertyContext
    ? slot.target_class_label || shortLabel(slot.target_class_iri)
    : slot.slot_label || shortLabel(slot.slot_iri) || "slot";
  const rows = ([
    propertyContext
      ? { key: "path", value: slot.slot_label || shortLabel(slot.slot_iri) }
      : slot.target_class_label
        ? { key: "applies to", value: <OntologyTerm term={{ label: slot.target_class_label, iri: slot.target_class_iri, kind: "class" }} /> }
        : null,
    slot.source_shape_label
      ? { key: "shape", value: <OntologyTerm term={{ label: slot.source_shape_label, iri: slot.source_shape_iri, kind: "node shape" }} /> }
      : null,
  ] as Array<MetaRow | null>).filter(isMetaRow);
  return (
    <div className="ontology-modal-card">
      <div className="ontology-modal-card-head">
        <strong className="rq-mono" title={propertyContext ? slot.target_class_iri : slot.slot_iri}>
          {title}
        </strong>
      </div>
      {rows.length ? <MetaRows rows={rows} /> : null}
      {slot.facets?.length ? (
        <div className="ontology-modal-facet-row">
          {slot.facets.map((facet) => (
            <span className="ontology-modal-facet" key={`${facet.name}-${facet.value}`}>
              <span>{facet.name}</span>
              {facet.value}
            </span>
          ))}
        </div>
      ) : (
        <span className="ex-panel-muted">No explicit facets.</span>
      )}
    </div>
  );
}

function ConstructRow({ construct }: { construct: OntologyGraphConstructDetail }) {
  const label = construct.label || construct.kind || construct.family || "construct";
  const glyph = construct.badge?.symbol || constructGlyph(construct);
  const glyphLabel = construct.badge?.label || label;
  const fields = ([
    construct.subject ? { key: "subject", value: construct.subject } : null,
    construct.predicate ? { key: "predicate", value: construct.predicate } : null,
    construct.property ? { key: "property", value: construct.property } : null,
    construct.object ? { key: "object", value: construct.object } : null,
    construct.members?.length ? { key: "members", value: construct.members.join(" -> ") } : null,
    construct.source
      ? {
        key: "source",
        value: `${construct.source.source_name || construct.source.source || construct.source.file_path}${construct.source.line_number ? `:${construct.source.line_number}` : ""}`,
      }
      : null,
  ] as Array<MetaRow | null>).filter(isMetaRow);
  return (
    <div className="ontology-modal-card">
      <div className="ontology-modal-card-head">
        <span className="ontology-modal-construct-title">
          {glyph ? (
            <span
              className="ontology-modal-construct-glyph"
              title={glyphLabel}
              aria-label={glyphLabel}
            >
              {glyph}
            </span>
          ) : null}
          <span>{label}</span>
        </span>
        <TypeBadge type={construct.kind || construct.family} family="ontology" dot={false} className={rdfBadgeClass(construct.kind || construct.family)}>
          {construct.kind || construct.family}
        </TypeBadge>
      </div>
      {fields.length ? <MetaRows rows={fields} mono /> : null}
    </div>
  );
}

function MetaRows({
  rows,
  mono = false,
}: {
  rows: MetaRow[];
  mono?: boolean;
}) {
  return (
    <dl className={["ontology-meta-rows", mono ? "ontology-meta-rows--mono" : ""].filter(Boolean).join(" ")}>
      {rows.map((row) => (
        <div className="ontology-meta-row" key={row.key}>
          <dt>{row.key}</dt>
          <dd>{row.value}</dd>
        </div>
      ))}
    </dl>
  );
}

function isMetaRow(row: MetaRow | null): row is MetaRow {
  return row !== null;
}

function dedupeSources(sources: OntologyGraphSource[]): OntologyGraphSource[] {
  const byLocation = new Map<string, OntologyGraphSource>();
  for (const source of sources) {
    const key = `${source.file_path}:${source.line_number ?? ""}`;
    const existing = byLocation.get(key);
    if (!existing || (!existing.kind && source.kind)) {
      byLocation.set(key, source);
    }
  }
  return [...byLocation.values()];
}

function SourceRow({ source }: { source: OntologyGraphSource }) {
  return (
    <a
      className="ontology-modal-source"
      href={source.link}
      onClick={(event) => {
        event.preventDefault();
        window.location.hash = source.link;
      }}
    >
      <span className="ontology-modal-source-main">
        <span className="ontology-modal-source-name">{source.source_name || source.source || source.file_path}</span>
        {source.kind ? (
          <TypeBadge type="ontology" family="ontology" tinted dot={false}>
            {source.kind}
          </TypeBadge>
        ) : null}
      </span>
      <span className="ontology-modal-source-loc rq-mono">
        {source.file_path}
        {source.line_number ? `:${source.line_number}` : ""}
      </span>
    </a>
  );
}

function ontologyNodeKind(node: OntologyGraphNode) {
  return node.semantic_type || node.node_type || node.type || "resource";
}

function isPropertyNode(node: OntologyGraphNode) {
  return String(node.semantic_type || "").endsWith("property")
    || Boolean(node.domain?.length)
    || Boolean(node.range?.length);
}

function propertyUsagesForNode(node: OntologyGraphNode, nodes: OntologyGraphNode[]) {
  return nodes
    .filter(isPropertyNode)
    .flatMap((property) => {
      const domains = property.domain ?? [];
      const ranges = property.range ?? [];
      const isDomain = domains.some((term) => termMatchesNode(term, node));
      const isRange = ranges.some((term) => termMatchesNode(term, node));
      if (!isDomain && !isRange) return [];
      const role = isDomain && isRange ? "domain/range" : isDomain ? "domain" : "range";
      return [{
        property,
        role,
        domains,
        ranges,
        facets: isDomain
          ? (node.slot_facets ?? []).filter((facet) => facet.slot_iri === property.id || facet.slot_iri === property.full_uri)
          : [],
      }];
    })
    .sort((a, b) => {
      const propertyCompare = (a.property.label || a.property.id).localeCompare(b.property.label || b.property.id);
      return propertyCompare || a.role.localeCompare(b.role);
    });
}

function termMatchesNode(
  term: { label: string; iri: string; kind: string },
  node: OntologyGraphNode,
) {
  const values = new Set([node.id, node.full_uri, node.label].filter(Boolean));
  return values.has(term.iri) || values.has(term.label);
}

function TermList({
  terms,
  emptyLabel,
}: {
  terms: { label: string; iri: string; kind: string }[];
  emptyLabel: string;
}) {
  if (!terms.length) return <span className="ex-panel-muted">{emptyLabel}</span>;
  return (
    <span className="ontology-modal-term-list">
      {terms.map((term) => (
        <OntologyTerm key={`${term.iri}-${term.kind}`} term={term} />
      ))}
    </span>
  );
}

function OntologyTerm({ term }: { term: { label: string; iri: string; kind: string } }) {
  return (
    <span className="ontology-modal-term" title={term.iri || term.label}>
      {term.label || shortLabel(term.iri)}
      <span>{term.kind || "term"}</span>
    </span>
  );
}

function rdfBadgeClass(type: string | undefined) {
  const normalized = type ?? "";
  if (normalized.includes("object")) return "ontology-rdf-badge ontology-rdf-badge--objprop";
  if (normalized.includes("datatype")) return "ontology-rdf-badge ontology-rdf-badge--dtprop";
  if (normalized.includes("shape")) return "ontology-rdf-badge ontology-rdf-badge--shape";
  return "ontology-rdf-badge";
}

function visibleConstructs(node: OntologyGraphNode) {
  return (node.constructs ?? []).filter((construct) => (
    node.semantic_type === "class-expression"
      ? !["class-expression", "property-domain", "property-range"].includes(construct.kind)
      : true
  ));
}

function humanizeSemanticType(value: string | undefined) {
  const labels: Record<string, string> = {
    "object-property": "object property",
    "datatype-property": "datatype property",
    "rdf-property": "RDF property",
    class: "class",
    "named-individual": "individual",
    "node-shape": "node shape",
    "property-shape": "property shape",
    restriction: "restriction",
    "class-expression": "class expression",
    datatype: "datatype",
    literal: "literal",
    resource: "resource",
  };
  return labels[value ?? ""] ?? value ?? "resource";
}

function constructGlyph(construct: OntologyGraphConstructDetail) {
  const kind = construct.kind || "";
  const family = construct.family || "";
  if (kind === "property-domain" || kind === "property-range" || family === "property-domain-range") return "D/R";
  if (kind === "subclass-inclusion") return "⊆";
  if (kind === "membership") return "∈";
  if (kind === "disjointness") return "⟂";
  if (kind === "equivalence-group") return "⇔";
  if (kind === "inverse-property") return "⟲";
  if (kind === "property-chain" || family === "property-chain") return "∘";
  if (kind === "property-characteristic" || family === "property-characteristic") return "→";
  if (kind === "restriction" || family === "restriction") return "∀";
  if (kind === "class-expression" || family === "class-expression") return "∩";
  if (kind === "shape-overlay" || family === "shape-overlay") return "SH";
  return "";
}

function shortLabel(value: string | undefined) {
  const text = value ?? "";
  if (text.includes("#")) return text.slice(text.lastIndexOf("#") + 1);
  if (text.includes("/")) return text.slice(text.lastIndexOf("/") + 1);
  return text;
}
