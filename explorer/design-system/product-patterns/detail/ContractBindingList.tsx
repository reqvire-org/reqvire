import { useState } from "react";
import { cx } from "@linaria/atomic";
import { Icon } from "../../components/core/Icon";
import {
  detailSectionUX,
  relationDisclosureBaseUX,
  relationDisclosureCountSkinX,
  relationDisclosureCountUX,
  relationDisclosureSkinX,
  relationDisclosureTitleUX,
  relationListUX,
  relationRowBaseUX,
} from "./detailStyles";
import type {
  DetailContractBindingItem,
  OpenElementHandler,
  OpenResourceHandler,
} from "./types";
import { RelationEndpoint } from "./RelationEndpoint";

export interface ContractBindingListProps {
  title?: string;
  contract_bindings: DetailContractBindingItem[];
  defaultExpanded?: boolean;
  onOpenElement: OpenElementHandler;
  onOpenResource?: OpenResourceHandler;
}

export function ContractBindingList({
  title = "Contract Bindings",
  contract_bindings,
  defaultExpanded = true,
  onOpenElement,
  onOpenResource,
}: ContractBindingListProps) {
  const [expanded, setExpanded] = useState(defaultExpanded);

  if (contract_bindings.length === 0) return null;

  return (
    <section className={cx(detailSectionUX)}>
      <button
        type="button"
        className={cx(relationDisclosureBaseUX, relationDisclosureSkinX)}
        aria-expanded={expanded}
        onClick={() => setExpanded((value) => !value)}
      >
        <Icon name={expanded ? "chevron-down" : "chevron-right"} size={14} />
        <span className={cx(relationDisclosureTitleUX)}>{title}</span>
        <span className={cx(relationDisclosureCountUX, relationDisclosureCountSkinX)}>{contract_bindings.length}</span>
      </button>
      {expanded ? (
        <div className={cx(relationListUX)}>
          {contract_bindings.map((contract_bindings) => (
            <ContractBindingTarget
              key={contract_bindings.id}
              contract_bindings={contract_bindings}
              onOpenElement={onOpenElement}
              onOpenResource={onOpenResource}
            />
          ))}
        </div>
      ) : null}
    </section>
  );
}

function ContractBindingTarget({
  contract_bindings,
  onOpenElement,
  onOpenResource,
}: {
  contract_bindings: DetailContractBindingItem;
  onOpenElement: OpenElementHandler;
  onOpenResource?: OpenResourceHandler;
}) {
  return (
    <div className={cx(relationRowBaseUX)}>
      <RelationEndpoint
        endpoint={{
          id: contract_bindings.targetId,
          label: contract_bindings.label,
          kind: contract_bindings.resourceKind ?? contract_bindings.kind,
          elementType: contract_bindings.elementType,
          typeFamily: contract_bindings.typeFamily,
          href: contract_bindings.href,
          external: contract_bindings.external,
        }}
        onOpenElement={onOpenElement}
        onOpenResource={onOpenResource}
      />
    </div>
  );
}
