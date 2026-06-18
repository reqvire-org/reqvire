import { cx } from "@linaria/atomic";
import {
  relationKindBaseUX,
  relationKindSkinX,
  relationListUX,
  relationRowBaseUX,
} from "./detailStyles";
import { DetailSection } from "./DetailSection";
import type {
  DetailReusedContractContextItem,
  OpenElementHandler,
  OpenResourceHandler,
} from "./types";
import { RelationEndpoint } from "./RelationEndpoint";

export interface ReusedContractContextListProps {
  title?: string;
  reused_contract_context: DetailReusedContractContextItem[];
  onOpenElement: OpenElementHandler;
  onOpenResource?: OpenResourceHandler;
}

export function ReusedContractContextList({
  title = "Reused Contract Context",
  reused_contract_context,
  onOpenElement,
  onOpenResource,
}: ReusedContractContextListProps) {
  if (reused_contract_context.length === 0) return null;

  return (
    <DetailSection title={title}>
      <div className={cx(relationListUX)}>
        {reused_contract_context.map((reused_contract_context) => (
          <ReusedContractContextTarget
            key={reused_contract_context.id}
            reused_contract_context={reused_contract_context}
            onOpenElement={onOpenElement}
            onOpenResource={onOpenResource}
          />
        ))}
      </div>
    </DetailSection>
  );
}

function ReusedContractContextTarget({
  reused_contract_context,
  onOpenElement,
  onOpenResource,
}: {
  reused_contract_context: DetailReusedContractContextItem;
  onOpenElement: OpenElementHandler;
  onOpenResource?: OpenResourceHandler;
}) {
  return (
    <div className={cx(relationRowBaseUX)}>
      <span className={cx(relationKindBaseUX, relationKindSkinX)}>{reused_contract_context.kind}</span>
      <RelationEndpoint
        endpoint={{
          id: reused_contract_context.targetId,
          label: reused_contract_context.label,
          kind: reused_contract_context.resourceKind ?? reused_contract_context.kind,
          elementType: reused_contract_context.elementType,
          typeFamily: reused_contract_context.typeFamily,
          href: reused_contract_context.href,
          external: reused_contract_context.external,
        }}
        onOpenElement={onOpenElement}
        onOpenResource={onOpenResource}
      />
    </div>
  );
}
