import { cx } from "@linaria/atomic";
import { CodeRef } from "../../components/data/CodeRef";
import { detailContentFlowUX, detailMutedUX } from "./detailStyles";
import { ContractBindingList } from "./ContractBindingList";
import { DetailSection } from "./DetailSection";
import { MetadataStrip } from "./MetadataStrip";
import { RelationList } from "./RelationList";
import type {
  DetailContractBindingItem,
  DetailMetaBadge,
  DetailRelationItem,
  ElementDetailContentSlot,
  OpenElementHandler,
  OpenResourceHandler,
} from "./types";

export interface ElementDetailContentProps {
  metaBadges?: DetailMetaBadge[];
  content: ElementDetailContentSlot;
  relations?: DetailRelationItem[];
  contract_bindings?: DetailContractBindingItem[];
  detailListsDefaultExpanded?: boolean;
  onOpenElement: OpenElementHandler;
  onOpenResource?: OpenResourceHandler;
}

export function ElementDetailContent({
  metaBadges = [],
  content,
  relations = [],
  contract_bindings = [],
  detailListsDefaultExpanded = true,
  onOpenElement,
  onOpenResource,
}: ElementDetailContentProps) {
  return (
    <div className={cx(detailContentFlowUX)}>
      <MetadataStrip badges={metaBadges} />

      <div className={cx(detailContentFlowUX)}>
        <DetailSection title="Content">{content}</DetailSection>
        <RelationList
          relations={relations}
          defaultExpanded={detailListsDefaultExpanded}
          onOpenElement={onOpenElement}
          onOpenResource={onOpenResource}
        />
        <ContractBindingList
          contract_bindings={contract_bindings}
          defaultExpanded={detailListsDefaultExpanded}
          onOpenElement={onOpenElement}
          onOpenResource={onOpenResource}
        />
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
