import { useState, type ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import {
  Button,
  IconButton,
  CodeRef,
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

const detailDialogBaseUX = css`
  --ex-ontology-detail-dialog-w: 1120px;
  --ex-ontology-detail-dialog-max-h: 980px;
  --ex-ontology-detail-dialog-body-max-h: 780px;
  --ex-ontology-detail-dialog-chrome-h: 176px;
  --ex-ontology-meta-key-col: minmax(92px, 160px);
  --ex-ontology-meta-key-col-narrow: minmax(72px, 0.32fr);
  --ex-ontology-modal-rail-col: minmax(260px, 320px);
  width: min(var(--ex-ontology-detail-dialog-w), calc(100vw - var(--space-24)));
  max-width: min(var(--ex-ontology-detail-dialog-w), calc(100vw - var(--space-24)));
  max-height: min(92vh, var(--ex-ontology-detail-dialog-max-h));

  @media (max-width: 720px) {
    width: calc(100vw - var(--space-10));
    max-width: calc(100vw - var(--space-10));
    max-height: calc(100vh - var(--space-10));
  }
`;

const detailDialogSkinX = css`
  border: var(--border-w) solid var(--border-default);
  border-radius: var(--radius-xl);
  background: var(--bg-overlay);
  color: var(--text-body);
  box-shadow: var(--shadow-xl);
`;

const ontologyDialogBaseUX = css`
  --ex-ontology-dialog-w-md: 1040px;
  --ex-ontology-dialog-w-sm: 760px;
  width: min(var(--ex-ontology-dialog-w-md), calc(100vw - var(--space-24)));

  @media (max-width: 980px) {
    width: min(var(--ex-ontology-dialog-w-sm), calc(100vw - var(--space-14)));
  }
`;

const detailHeaderBaseUX = css`
  padding: var(--space-10) var(--space-10) var(--space-10) var(--space-14);

  @media (max-width: 720px) {
    padding: var(--space-8) var(--space-24) var(--space-8) var(--space-8);
  }
`;

const detailHeaderSkinX = css`
  border-bottom: var(--border-w) solid var(--border-subtle);
  background: var(--bg-overlay);
`;

const detailBodyBaseUX = css`
  max-height: min(74vh, var(--ex-ontology-detail-dialog-body-max-h));
  overflow: auto;
  padding: var(--space-14) var(--space-16);

  @media (max-width: 720px) {
    max-height: calc(100vh - var(--ex-ontology-detail-dialog-chrome-h));
    padding: var(--space-8);
  }
`;

const detailBodySkinX = css`
  background: var(--bg-surface);
`;

const ontologyBodyUX = css`
  padding: var(--space-16);
`;

const detailTitleRowUX = css`
  display: flex;
  width: 100%;
  min-width: 0;
  align-items: center;
  gap: var(--space-8);

  h2 {
    flex: 1 1 auto;
    min-width: 0;
  }
`;

const detailFamilyBadgeUX = css`
  flex: 0 0 auto;
`;

const detailCloseUX = css`
  flex: 0 0 auto;
  margin-left: auto;
  margin-right: calc(-1 * var(--space-3));

  svg {
    width: var(--icon-sm);
    height: var(--icon-sm);
  }
`;

const detailSectionUX = css`
  display: grid;
  gap: var(--space-4);

  h3 {
    margin: 0;
    color: var(--text-strong);
    font-size: var(--text-md);
    font-weight: var(--weight-semibold);
    letter-spacing: 0;
    line-height: var(--leading-tight);
  }
`;

const detailMutedUX = css`
  margin: 0;
  color: var(--text-muted);
  font-size: var(--text-sm);
`;

const panelMutedUX = css`
  color: var(--text-muted);
  font-size: var(--text-caption);
  line-height: 1.4;
`;

const detailFooterBaseUX = css`
  padding: var(--space-7) var(--space-16);

  @media (max-width: 720px) {
    padding: var(--space-6) var(--space-8);
  }
`;

const detailFooterSkinX = css`
  border-top: var(--border-w) solid var(--border-subtle);
  background: var(--bg-overlay);
`;

const detailFooterRowUX = css`
  display: flex;
  width: 100%;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-8);
`;

const sourceLinkBaseUX = css`
  display: inline-flex;
  align-items: center;
  gap: var(--space-3);
  min-width: 0;
  font-size: var(--text-sm);
  font-weight: var(--weight-semibold);
  text-decoration: none;
  overflow-wrap: anywhere;
`;

const sourceLinkSkinX = css`
  color: var(--accent);

  &:hover {
    text-decoration: underline;
    text-underline-offset: var(--space-1);
  }
`;

const iconSmUX = css`
  width: var(--space-8);
  height: var(--space-8);
  flex: none;
`;

const metadataBaseUX = css`
  overflow: hidden;
`;

const metadataSkinX = css`
  border: var(--border-w) solid var(--border-subtle);
  border-radius: var(--radius-lg);
  background: var(--bg-surface);
`;

const metadataRowBaseUX = css`
  display: grid;
  grid-template-columns: var(--ex-ontology-meta-key-col) minmax(0, 1fr);
  gap: var(--space-8);
  padding: var(--space-6) var(--space-10);
  font-size: var(--text-sm);
`;

const metadataRowSkinX = css`
  & + & {
    border-top: var(--border-w) solid var(--border-subtle);
  }
`;

const metadataRailRowUX = css`
  grid-template-columns: 1fr;
  gap: var(--space-1);
`;

const metadataKeySkinX = css`
  color: var(--text-muted);
  font-weight: var(--weight-medium);
`;

const metadataValueBaseUX = css`
  min-width: 0;
  overflow-wrap: anywhere;
`;

const metadataValueSkinX = css`
  color: var(--text-body);
`;

const metadataBadgeRowUX = css`
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-4);
  margin-bottom: 0;
`;

const ontologyTypePillBaseUX = css`
  display: inline-block;
  padding: var(--space-1) var(--space-4);
  margin: var(--space-1) var(--space-2) var(--space-1) 0;
  font-size: var(--text-micro);
  text-decoration: none;
`;

const ontologyTypePillSkinX = css`
  border: var(--border-w) solid var(--border-subtle);
  border-radius: var(--radius-sm);
  background: var(--bg-sunken);
  color: var(--text-body);

  &:hover {
    background: var(--bg-hover);
  }
`;

const ontologyUriCopyBaseUX = css`
  display: inline-flex;
  max-width: 100%;
  align-items: center;
  gap: var(--space-4);
  padding: 0;
  cursor: pointer;
  --rq-coderef-min-w: 0;
  --rq-coderef-ow: anywhere;
  --rq-coderef-text-align: left;
  --rq-coderef-ws: normal;

  svg {
    width: var(--icon-sm);
    height: var(--icon-sm);
    flex: 0 0 auto;
  }
`;

const ontologyUriCopySkinX = css`
  border: 0;
  border-radius: var(--radius-md);
  background: transparent;
  color: var(--text-body);

  svg {
    color: var(--text-muted);
  }

  &:hover svg {
    color: var(--text-strong);
  }
`;

const ontologyLayoutUX = css`
  display: grid;
  grid-template-columns: minmax(0, 1fr) var(--ex-ontology-modal-rail-col);
  gap: var(--space-16);
  align-items: start;

  @media (max-width: 980px) {
    grid-template-columns: 1fr;
  }
`;

const ontologyColumnUX = css`
  display: flex;
  min-width: 0;
  flex-direction: column;
  gap: var(--space-12);
`;

const ontologyRailUX = css`
  position: sticky;
  top: 0;

  @media (max-width: 980px) {
    position: static;
    order: -1;
  }
`;

const ontologyRailParagraphUX = css`
  margin: 0;
  color: var(--text-body);
  font-size: var(--text-sm);
  line-height: 1.55;
`;

const ontologyInlineListUX = css`
  display: flex;
  min-width: 0;
  flex-wrap: wrap;
  align-items: center;
  gap: var(--space-5);
`;

const ontologySymbolBaseUX = css`
  display: inline-flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-1) var(--space-4);
  font-size: var(--text-caption);
  font-weight: var(--weight-medium);
`;

const ontologySymbolSkinX = css`
  border: var(--border-w) solid var(--border-default);
  border-radius: var(--radius-pill);
  background: var(--bg-sunken);
  color: var(--text-secondary);
`;

const ontologyGridListUX = css`
  display: grid;
  gap: var(--space-7);
`;

const ontologyCardBaseUX = css`
  display: grid;
  min-width: 0;
  gap: var(--space-6);
  padding: var(--space-8);
`;

const ontologyCardSkinX = css`
  border: var(--border-w) solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--bg-sunken);
`;

const ontologyCardCompactUX = css`
  display: flex;
  align-items: center;
  gap: var(--space-7);
  flex-wrap: wrap;
  padding: var(--space-7) var(--space-8);
`;

const ontologyCardHeadUX = css`
  display: flex;
  min-width: 0;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-8);

  strong {
    min-width: 0;
    color: var(--text-strong);
    overflow-wrap: anywhere;
  }
`;

const ontologyKindBaseUX = css`
  padding: var(--space-1) var(--space-4);
  font-family: var(--font-mono);
  font-size: var(--text-micro);
  font-weight: var(--weight-medium);
  letter-spacing: var(--tracking-mono);
  line-height: 1.4;
  white-space: nowrap;
`;

const ontologyKindSkinX = css`
  border-radius: var(--radius-sm);
  background: var(--bg-sunken);
  color: var(--text-muted);
`;

const ontologyFacetBaseUX = css`
  display: inline-flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-1) var(--space-4);
  font-family: var(--font-mono);
  font-size: var(--text-caption);

  span {
    color: var(--text-muted);
    font-size: 0.9em;
    font-weight: var(--weight-medium);
  }
`;

const ontologyFacetSkinX = css`
  border: var(--border-w) solid var(--border-subtle);
  border-radius: var(--radius-pill);
  background: var(--bg-surface);
  color: var(--text-strong);
`;

const ontologyConstructTitleUX = css`
  display: inline-flex;
  min-width: 0;
  align-items: center;
  gap: var(--space-6);
  color: var(--text-strong);
  font-weight: var(--weight-semibold);
  overflow-wrap: anywhere;
`;

const ontologyConstructGlyphBaseUX = css`
  display: inline-flex;
  min-width: var(--row-h);
  height: var(--space-12);
  align-items: center;
  justify-content: center;
  font-family: var(--font-mono);
  font-size: var(--text-caption);
  font-weight: var(--weight-semibold);
  line-height: 1;
`;

const ontologyConstructGlyphSkinX = css`
  border: var(--border-w) solid color-mix(in srgb, var(--rdf-class) 36%, var(--border-default));
  border-radius: var(--radius-pill);
  background: color-mix(in srgb, var(--rdf-class) 12%, var(--bg-surface));
  color: var(--text-strong);
`;

const ontologyMetaRowsUX = css`
  display: grid;
  gap: var(--space-3);
  margin: 0;
`;

const ontologyMetaRowsMonoUX = css`
  dd {
    font-family: var(--font-mono);
    font-size: var(--text-caption);
  }
`;

const ontologyMetaRowUX = css`
  display: grid;
  grid-template-columns: var(--ex-ontology-meta-key-col-narrow) minmax(0, 1fr);
  gap: var(--space-7);
  align-items: baseline;
  min-width: 0;
  color: var(--text-body);
  font-size: var(--text-sm);

  dt {
    color: var(--text-muted);
    font-family: var(--font-mono);
    font-size: var(--text-caption);
    font-weight: var(--weight-semibold);
    letter-spacing: 0.02em;
    text-transform: lowercase;
  }

  dd {
    min-width: 0;
    margin: 0;
    overflow-wrap: anywhere;
  }
`;

const ontologyTermBaseUX = css`
  display: inline-flex;
  min-width: 0;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-1) var(--space-4);
  font-size: var(--text-caption);
  font-weight: var(--weight-medium);
  overflow-wrap: anywhere;

  span {
    color: var(--text-muted);
    font-size: 0.9em;
    font-weight: var(--weight-medium);
  }
`;

const ontologyTermSkinX = css`
  border: var(--border-w) solid var(--border-subtle);
  border-radius: var(--radius-pill);
  background: var(--bg-surface);
  color: var(--text-strong);
`;

const ontologyRdfBadgeSkinX = css`
  border: var(--border-w) solid var(--border-default);
  background: var(--bg-surface);
  color: var(--text-secondary);
`;

const ontologyRdfBadgeObjPropSkinX = css`
  border-color: color-mix(in srgb, var(--rdf-objprop) 34%, var(--border-default));
  color: var(--rdf-objprop);
`;

const ontologyRdfBadgeDtPropSkinX = css`
  border-color: color-mix(in srgb, var(--rdf-dtprop) 34%, var(--border-default));
  color: var(--rdf-dtprop);
`;

const ontologyRdfBadgeShapeSkinX = css`
  border-color: color-mix(in srgb, var(--rdf-shacl) 34%, var(--border-default));
  color: var(--rdf-shacl);
`;

const ontologySourceBaseUX = css`
  display: grid;
  gap: var(--space-4);
  padding: var(--space-8);
  text-decoration: none;
`;

const ontologySourceSkinX = css`
  border: var(--border-w) solid var(--border-subtle);
  border-radius: var(--radius-lg);
  background: var(--bg-sunken);
  color: var(--text-body);

  &:hover {
    border-color: var(--border-default);
    background: var(--bg-hover);
  }
`;

const ontologySourceMainUX = css`
  display: flex;
  min-width: 0;
  align-items: center;
  gap: var(--space-5);
  flex-wrap: wrap;
`;

const ontologySourceNameUX = css`
  min-width: 0;
  color: var(--text-strong);
  font-weight: var(--weight-semibold);
  overflow-wrap: anywhere;
`;

const ontologySourceLocUX = css`
  color: var(--text-secondary);
  font-size: var(--text-caption);
  overflow-wrap: anywhere;
`;

const monoTextUX = css`
  font-family: var(--font-mono);
  font-size: 0.92em;
  letter-spacing: var(--tracking-mono);
  font-feature-settings: "zero" 1;
`;

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
      <ModalContent className={cx(detailDialogBaseUX, detailDialogSkinX, ontologyDialogBaseUX)} showCloseButton={false}>
        {!node ? (
          <>
            <ModalHeader className={cx(detailHeaderBaseUX, detailHeaderSkinX)}>
              <ModalTitle>Ontology node not found</ModalTitle>
            </ModalHeader>
            <ModalBody className={cx(detailBodyBaseUX, detailBodySkinX, ontologyBodyUX)}>
              <p className={cx(detailMutedUX)}>
                No exported ontology graph node matches{" "}
                <CodeRef>{nodeId ?? ""}</CodeRef>.
              </p>
            </ModalBody>
          </>
        ) : (
          <>
            <OntologyNodeModalHeader node={node} />
            <ModalBody className={cx(detailBodyBaseUX, detailBodySkinX, ontologyBodyUX)}>
              <div className={cx(ontologyLayoutUX)}>
                <main className={cx(ontologyColumnUX)}>
                  <OntologyPropertyUsages node={node} nodes={graphNodes} />
                  {isPropertyNode(node) && (node.domain?.length || node.range?.length) ? (
                    <Section title="Domain / range">
                      <div className={cx(ontologyGridListUX)}>
                        {node.domain?.length ? <TermRefs kind="domain" terms={node.domain} /> : null}
                        {node.range?.length ? <TermRefs kind="range" terms={node.range} /> : null}
                      </div>
                    </Section>
                  ) : null}
                  {node.slot_facets?.length ? (
                    <Section title={isPropertyNode(node) ? "Used as slot / facets" : "Slots / facets"}>
                      <div className={cx(ontologyGridListUX)}>
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
                      <div className={cx(ontologyGridListUX)}>
                        {visibleConstructs(node).map((construct) => (
                          <ConstructRow key={construct.id} construct={construct} />
                        ))}
                      </div>
                    </Section>
                  ) : null}
                  {node.literal_values?.length ? (
                    <Section title="Literal values">
                      <div className={cx(ontologyGridListUX)}>
                        {node.literal_values.map((literal, index) => (
                          <div className={cx(ontologyCardBaseUX, ontologyCardSkinX, ontologyCardCompactUX)} key={`${literal.predicate}-${index}`}>
                            <span className={cx(ontologyKindBaseUX, ontologyKindSkinX)}>{literal.predicate || "value"}</span>
                            <strong>{literal.value}</strong>
                          </div>
                        ))}
                      </div>
                    </Section>
                  ) : null}
                  {node.constraints?.length ? (
                    <Section title="Raw SHACL evidence">
                      <div className={cx(ontologyGridListUX)}>
                        {node.constraints.map((constraint) => (
                          <div className={cx(ontologyCardBaseUX, ontologyCardSkinX, ontologyCardCompactUX)} key={`${constraint.property}-${constraint.value}`}>
                            <span className={cx(ontologyKindBaseUX, ontologyKindSkinX)}>{constraint.property}</span>
                            <CodeRef>{constraint.value}</CodeRef>
                          </div>
                        ))}
                      </div>
                    </Section>
                  ) : null}
                  {node.sources?.length ? (
                    <Section title="Sources">
                      <div className={cx(ontologyGridListUX)}>
                        {dedupeSources(node.sources).map((source, index) => (
                          <SourceRow key={`${source.link}-${index}`} source={source} />
                        ))}
                      </div>
                    </Section>
                  ) : null}
                </main>
                <aside className={cx(ontologyColumnUX, ontologyRailUX)}>
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
                      <p className={cx(ontologyRailParagraphUX)}>{node.comment}</p>
                    </Section>
                  ) : null}
                  {node.badges?.length ? (
                    <Section title="Notation">
                      <div className={cx(ontologyInlineListUX)}>
                        {node.badges.map((badge) => (
                          <span className={cx(ontologySymbolBaseUX, ontologySymbolSkinX)} key={`${badge.kind}-${badge.symbol}`}>
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
            <ModalFooter className={cx(detailFooterBaseUX, detailFooterSkinX)}>
              <div className={cx(detailFooterRowUX)}>
                {node.sources?.[0]?.link ? (
                  <a
                    href={node.sources[0].link}
                    className={cx(sourceLinkBaseUX, sourceLinkSkinX)}
                    onClick={(event) => {
                      event.preventDefault();
                      window.location.hash = node.sources[0].link;
                    }}
                  >
                    <Icon name="external-link" className={cx(iconSmUX)} /> Open ontology source
                  </a>
                ) : (
                  <span />
                )}
                <ModalClose asChild>
                  <Button tone="primary" size="sm">Close</Button>
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
    <ModalHeader className={cx(detailHeaderBaseUX, detailHeaderSkinX)}>
      <div className={cx(detailTitleRowUX)}>
        <TypeBadge type="ontology" family="ontology" tinted className={cx(detailFamilyBadgeUX)}>
          ontology
        </TypeBadge>
        {kind !== "ontology" ? (
          <TypeBadge type={kind} family="ontology" tinted>
            {kind}
          </TypeBadge>
        ) : null}
        <ModalTitle>{node.label || node.id}</ModalTitle>
        <ModalClose asChild>
          <IconButton tone="ghost" className={cx(detailCloseUX)} aria-label="Close">
            <Icon name="x" />
          </IconButton>
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
    <div className={cx(metadataBaseUX, metadataSkinX)}>
      <div className={cx(metadataRowBaseUX, metadataRowSkinX, metadataRailRowUX)}>
        <span className={cx(metadataKeySkinX)}>RDF type</span>
        <span className={cx(metadataValueBaseUX, metadataValueSkinX, metadataBadgeRowUX)}>
          {rdfTypes.map((type) => (
            <span key={type} className={cx(ontologyTypePillBaseUX, ontologyTypePillSkinX)}>
              {type}
            </span>
          ))}
        </span>
      </div>
      <div className={cx(metadataRowBaseUX, metadataRowSkinX, metadataRailRowUX)}>
        <span className={cx(metadataKeySkinX)}>Full URI</span>
        <span className={cx(metadataValueBaseUX, metadataValueSkinX)}>
          <button type="button" className={cx(ontologyUriCopyBaseUX, ontologyUriCopySkinX)} onClick={onCopyUri} title="Copy URI">
            <CodeRef>{node.full_uri || node.id}</CodeRef>
            <Icon name={copiedUri ? "check" : "copy"} />
          </button>
        </span>
      </div>
    </div>
  );
}

function Section({ title, children }: { title: string; children: ReactNode }) {
  return (
    <section className={cx(detailSectionUX)}>
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
        <div className={cx(ontologyCardBaseUX, ontologyCardSkinX, ontologyCardCompactUX)} key={`${kind}-${term.iri}`}>
          <span className={cx(ontologyKindBaseUX, ontologyKindSkinX)}>{kind}</span>
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
      <div className={cx(ontologyGridListUX)}>
        {usages.map((usage) => (
          <div className={cx(ontologyCardBaseUX, ontologyCardSkinX)} key={`${usage.property.id}-${usage.role}`}>
            <div className={cx(ontologyCardHeadUX)}>
              <strong className={cx(monoTextUX)} title={usage.property.full_uri || usage.property.id}>
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
              <div className={cx(ontologyInlineListUX)}>
                {usage.facets.flatMap((facet) => facet.facets ?? []).map((facet, index) => (
                  <span className={cx(ontologyFacetBaseUX, ontologyFacetSkinX)} key={`${facet.name}-${facet.value}-${index}`}>
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
    <div className={cx(ontologyCardBaseUX, ontologyCardSkinX)}>
      <div className={cx(ontologyCardHeadUX)}>
        <strong className={cx(monoTextUX)} title={propertyContext ? slot.target_class_iri : slot.slot_iri}>
          {title}
        </strong>
      </div>
      {rows.length ? <MetaRows rows={rows} /> : null}
      {slot.facets?.length ? (
        <div className={cx(ontologyInlineListUX)}>
          {slot.facets.map((facet) => (
            <span className={cx(ontologyFacetBaseUX, ontologyFacetSkinX)} key={`${facet.name}-${facet.value}`}>
              <span>{facet.name}</span>
              {facet.value}
            </span>
          ))}
        </div>
      ) : (
        <span className={cx(panelMutedUX)}>No explicit facets.</span>
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
    <div className={cx(ontologyCardBaseUX, ontologyCardSkinX)}>
      <div className={cx(ontologyCardHeadUX)}>
        <span className={cx(ontologyConstructTitleUX)}>
          {glyph ? (
            <span
              className={cx(ontologyConstructGlyphBaseUX, ontologyConstructGlyphSkinX)}
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
    <dl className={cx(ontologyMetaRowsUX, mono ? ontologyMetaRowsMonoUX : undefined)}>
      {rows.map((row) => (
        <div className={cx(ontologyMetaRowUX)} key={row.key}>
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
      className={cx(ontologySourceBaseUX, ontologySourceSkinX)}
      href={source.link}
      onClick={(event) => {
        event.preventDefault();
        window.location.hash = source.link;
      }}
    >
      <span className={cx(ontologySourceMainUX)}>
        <span className={cx(ontologySourceNameUX)}>{source.source_name || source.source || source.file_path}</span>
        {source.kind ? (
          <TypeBadge type="ontology" family="ontology" tinted dot={false}>
            {source.kind}
          </TypeBadge>
        ) : null}
      </span>
      <span className={cx(ontologySourceLocUX, monoTextUX)}>
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
  if (!terms.length) return <span className={cx(panelMutedUX)}>{emptyLabel}</span>;
  return (
    <span className={cx(ontologyInlineListUX)}>
      {terms.map((term) => (
        <OntologyTerm key={`${term.iri}-${term.kind}`} term={term} />
      ))}
    </span>
  );
}

function OntologyTerm({ term }: { term: { label: string; iri: string; kind: string } }) {
  return (
    <span className={cx(ontologyTermBaseUX, ontologyTermSkinX)} title={term.iri || term.label}>
      {term.label || shortLabel(term.iri)}
      <span>{term.kind || "term"}</span>
    </span>
  );
}

function rdfBadgeClass(type: string | undefined) {
  const normalized = type ?? "";
  if (normalized.includes("object")) return cx(ontologyRdfBadgeSkinX, ontologyRdfBadgeObjPropSkinX);
  if (normalized.includes("datatype")) return cx(ontologyRdfBadgeSkinX, ontologyRdfBadgeDtPropSkinX);
  if (normalized.includes("shape")) return cx(ontologyRdfBadgeSkinX, ontologyRdfBadgeShapeSkinX);
  return cx(ontologyRdfBadgeSkinX);
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
