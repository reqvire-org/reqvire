import { cx } from "@linaria/atomic";
import { CodeRef } from "../../components/data/CodeRef";
import { detailContentFlowUX, detailMutedUX } from "./detailStyles";
import { ReusedContractContextList } from "./ReusedContractContextList";
import { ConceptReferenceList } from "./ConceptReferenceList";
import { DetailSection } from "./DetailSection";
import { MetadataStrip } from "./MetadataStrip";
import { RelationList } from "./RelationList";
import type {
  DetailReusedContractContextItem,
  DetailConceptReferenceItem,
  DetailMetaBadge,
  DetailRelationItem,
  ElementDetailContentSlot,
  OpenConceptReferenceHandler,
  OpenElementHandler,
  OpenResourceHandler,
} from "./types";

export interface ElementDetailContentProps {
  metaBadges?: DetailMetaBadge[];
  content: ElementDetailContentSlot;
  relations?: DetailRelationItem[];
  reused_contract_context?: DetailReusedContractContextItem[];
  conceptReferences?: DetailConceptReferenceItem[];
  onOpenElement: OpenElementHandler;
  onOpenConceptReference?: OpenConceptReferenceHandler;
  onOpenResource?: OpenResourceHandler;
}

export function ElementDetailContent({
  metaBadges = [],
  content,
  relations = [],
  reused_contract_context = [],
  conceptReferences = [],
  onOpenElement,
  onOpenConceptReference,
  onOpenResource,
}: ElementDetailContentProps) {
  return (
    <div className={cx(detailContentFlowUX)}>
      <MetadataStrip badges={metaBadges} />

      <div className={cx(detailContentFlowUX)}>
        <DetailSection title="Content">{content}</DetailSection>
        <RelationList relations={relations} onOpenElement={onOpenElement} onOpenResource={onOpenResource} />
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

export interface ElementDetailMissingStateProps {
  identifier: string;
}

export function ElementDetailMissingState({ identifier }: ElementDetailMissingStateProps) {
  return (
    <p className={cx(detailMutedUX)}>
      No Project Store element matches <CodeRef>{identifier}</CodeRef>.
    </p>
  );
}
