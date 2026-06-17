import { cx } from "@linaria/atomic";
import { CodeRef } from "../../components/data/CodeRef";
import { detailContentFlowUX, detailMutedUX } from "./detailStyles";
import { AttachmentList } from "./AttachmentList";
import { ConceptReferenceList } from "./ConceptReferenceList";
import { DetailSection } from "./DetailSection";
import { MetadataStrip } from "./MetadataStrip";
import { RelationList } from "./RelationList";
import type {
  DetailAttachmentItem,
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
  attachments?: DetailAttachmentItem[];
  conceptReferences?: DetailConceptReferenceItem[];
  onOpenElement: OpenElementHandler;
  onOpenConceptReference?: OpenConceptReferenceHandler;
  onOpenResource?: OpenResourceHandler;
}

export function ElementDetailContent({
  metaBadges = [],
  content,
  relations = [],
  attachments = [],
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
        <AttachmentList
          attachments={attachments}
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
