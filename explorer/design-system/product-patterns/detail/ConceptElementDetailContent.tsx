import { css, cx } from "@linaria/atomic";
import { DetailSection } from "./DetailSection";
import { MetadataStrip } from "./MetadataStrip";
import { RelationEndpoint } from "./RelationEndpoint";
import { RelationList } from "./RelationList";
import { ReusedContractContextList } from "./ReusedContractContextList";
import { ConceptReferenceList } from "./ConceptReferenceList";
import {
  detailContentFlowUX,
  relationKindBaseUX,
  relationKindSkinX,
  relationListUX,
  relationRowBaseUX,
} from "./detailStyles";
import type {
  DetailConceptReferenceItem,
  DetailMetaBadge,
  DetailRelationEndpointData,
  DetailRelationItem,
  DetailReusedContractContextItem,
  ElementDetailContentSlot,
  OpenConceptReferenceHandler,
  OpenElementHandler,
  OpenResourceHandler,
} from "./types";

export interface ConceptElementDetailContentProps {
  metaBadges?: DetailMetaBadge[];
  definition: ElementDetailContentSlot;
  scheme?: DetailRelationEndpointData | null;
  altLabels?: readonly string[];
  scopeNote?: string;
  examples?: readonly string[];
  topConcepts?: readonly DetailRelationEndpointData[];
  broader?: readonly DetailRelationEndpointData[];
  narrower?: readonly DetailRelationEndpointData[];
  related?: readonly DetailRelationEndpointData[];
  exactMatches?: readonly DetailRelationEndpointData[];
  closeMatches?: readonly DetailRelationEndpointData[];
  mappedOntologyTerms?: readonly DetailConceptReferenceItem[];
  usedByModel?: readonly DetailRelationEndpointData[];
  relations?: DetailRelationItem[];
  reused_contract_context?: DetailReusedContractContextItem[];
  conceptReferences?: readonly DetailConceptReferenceItem[];
  onOpenElement: OpenElementHandler;
  onOpenConceptReference?: OpenConceptReferenceHandler;
  onOpenResource?: OpenResourceHandler;
}

const conceptFactsUX = css`
  display: grid;
  grid-template-columns: max-content minmax(0, 1fr);
  align-items: baseline;
  gap: var(--stack-gap-compact) var(--space-6);
`;

const conceptFactRowUX = css`
  display: contents;
`;

const conceptFactKeyUX = css`
  color: var(--text-muted);
  font-size: var(--text-caption);
  font-weight: var(--weight-semibold);
  text-transform: uppercase;
`;

const conceptFactValueUX = css`
  min-width: 0;
  color: var(--text-body);
  font-size: var(--text-sm);
  line-height: var(--leading-relaxed);
`;

const conceptChipRowUX = css`
  display: flex;
  min-width: 0;
  flex-wrap: wrap;
  gap: var(--space-3);
`;

const conceptChipUX = css`
  display: inline-flex;
  align-items: center;
  min-height: var(--control-xs);
  border-radius: var(--radius-sm);
  padding: 0 var(--space-4);
  background: var(--concept-tint);
  color: var(--text-body);
  font-size: var(--text-sm);
`;

export function ConceptElementDetailContent({
  metaBadges = [],
  definition,
  scheme,
  altLabels = [],
  scopeNote,
  examples = [],
  topConcepts = [],
  broader = [],
  narrower = [],
  related = [],
  exactMatches = [],
  closeMatches = [],
  mappedOntologyTerms = [],
  usedByModel = [],
  relations = [],
  reused_contract_context = [],
  conceptReferences = [],
  onOpenElement,
  onOpenConceptReference,
  onOpenResource,
}: ConceptElementDetailContentProps) {
  return (
    <div className={cx(detailContentFlowUX)}>
      <MetadataStrip badges={metaBadges} />

      <div className={cx(detailContentFlowUX)}>
        <DetailSection title="Definition">{definition}</DetailSection>
        <ConceptFacts
          scheme={scheme}
          altLabels={altLabels}
          scopeNote={scopeNote}
          examples={examples}
          onOpenElement={onOpenElement}
          onOpenResource={onOpenResource}
        />
        <ConceptEndpointList title="Top concepts" relationLabel="top concept" endpoints={topConcepts} onOpenElement={onOpenElement} onOpenResource={onOpenResource} />
        <ConceptEndpointList title="Broader concepts" relationLabel="broader" endpoints={broader} onOpenElement={onOpenElement} onOpenResource={onOpenResource} />
        <ConceptEndpointList title="Narrower concepts" relationLabel="narrower" endpoints={narrower} onOpenElement={onOpenElement} onOpenResource={onOpenResource} />
        <ConceptEndpointList title="Related concepts" relationLabel="related" endpoints={related} onOpenElement={onOpenElement} onOpenResource={onOpenResource} />
        <ConceptEndpointList title="Exact matches" relationLabel="exactMatch" endpoints={exactMatches} onOpenElement={onOpenElement} onOpenResource={onOpenResource} />
        <ConceptEndpointList title="Close matches" relationLabel="closeMatch" endpoints={closeMatches} onOpenElement={onOpenElement} onOpenResource={onOpenResource} />
        <ConceptReferenceList
          title="Mapped ontology terms"
          references={mappedOntologyTerms}
          onOpenConceptReference={onOpenConceptReference}
        />
        <ConceptEndpointList title="Used by model" relationLabel="references" endpoints={usedByModel} onOpenElement={onOpenElement} onOpenResource={onOpenResource} />
        <RelationList title="Authored relations" relations={relations} onOpenElement={onOpenElement} onOpenResource={onOpenResource} />
        <ReusedContractContextList
          reused_contract_context={reused_contract_context}
          onOpenElement={onOpenElement}
          onOpenResource={onOpenResource}
        />
        <ConceptReferenceList references={conceptReferences} onOpenConceptReference={onOpenConceptReference} />
      </div>
    </div>
  );
}

function ConceptFacts({
  scheme,
  altLabels,
  scopeNote,
  examples,
  onOpenElement,
  onOpenResource,
}: {
  scheme?: DetailRelationEndpointData | null;
  altLabels: readonly string[];
  scopeNote?: string;
  examples: readonly string[];
  onOpenElement: OpenElementHandler;
  onOpenResource?: OpenResourceHandler;
}) {
  if (!scheme && altLabels.length === 0 && !scopeNote && examples.length === 0) return null;

  return (
    <DetailSection title="Concept properties">
      <div className={cx(conceptFactsUX)}>
        {scheme ? (
          <ConceptFact label="Scheme">
            <RelationEndpoint endpoint={scheme} onOpenElement={onOpenElement} onOpenResource={onOpenResource} />
          </ConceptFact>
        ) : null}
        {altLabels.length > 0 ? (
          <ConceptFact label="Alt labels">
            <ConceptChips values={altLabels} />
          </ConceptFact>
        ) : null}
        {scopeNote ? <ConceptFact label="Scope note">{scopeNote}</ConceptFact> : null}
        {examples.length > 0 ? (
          <ConceptFact label="Examples">
            <ConceptChips values={examples} />
          </ConceptFact>
        ) : null}
      </div>
    </DetailSection>
  );
}

function ConceptFact({ label, children }: { label: string; children: ElementDetailContentSlot }) {
  return (
    <div className={cx(conceptFactRowUX)}>
      <span className={cx(conceptFactKeyUX)}>{label}</span>
      <span className={cx(conceptFactValueUX)}>{children}</span>
    </div>
  );
}

function ConceptChips({ values }: { values: readonly string[] }) {
  return (
    <span className={cx(conceptChipRowUX)}>
      {values.map((value) => (
        <span key={value} className={cx(conceptChipUX)}>{value}</span>
      ))}
    </span>
  );
}

function ConceptEndpointList({
  title,
  relationLabel,
  endpoints,
  onOpenElement,
  onOpenResource,
}: {
  title: string;
  relationLabel: string;
  endpoints: readonly DetailRelationEndpointData[];
  onOpenElement: OpenElementHandler;
  onOpenResource?: OpenResourceHandler;
}) {
  if (endpoints.length === 0) return null;

  return (
    <DetailSection title={title}>
      <div className={cx(relationListUX)}>
        {endpoints.map((endpoint) => (
          <div key={`${relationLabel}-${endpoint.id}`} className={cx(relationRowBaseUX)}>
            <span className={cx(relationKindBaseUX, relationKindSkinX)}>{relationLabel}</span>
            <RelationEndpoint endpoint={endpoint} onOpenElement={onOpenElement} onOpenResource={onOpenResource} />
          </div>
        ))}
      </div>
    </DetailSection>
  );
}
