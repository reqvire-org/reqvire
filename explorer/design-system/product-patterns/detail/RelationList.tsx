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
  relationKindBaseUX,
  relationKindSkinX,
  relationListUX,
  relationRowBaseUX,
} from "./detailStyles";
import { RelationEndpoint } from "./RelationEndpoint";
import type { DetailRelationItem, OpenElementHandler, OpenResourceHandler } from "./types";

export interface RelationListProps {
  title?: string;
  relations: DetailRelationItem[];
  defaultExpanded?: boolean;
  onOpenElement: OpenElementHandler;
  onOpenResource?: OpenResourceHandler;
}

export function RelationList({
  title = "Relations",
  relations,
  defaultExpanded = true,
  onOpenElement,
  onOpenResource,
}: RelationListProps) {
  const [expanded, setExpanded] = useState(defaultExpanded);

  if (relations.length === 0) return null;

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
        <span className={cx(relationDisclosureCountUX, relationDisclosureCountSkinX)}>{relations.length}</span>
      </button>
      {expanded ? (
        <div className={cx(relationListUX)}>
          {relations.map((relation, index) => (
            <div
              key={relation.id ?? `${relation.label}-${relation.target.id}-${index}`}
              className={cx(relationRowBaseUX)}
            >
              <span className={cx(relationKindBaseUX, relationKindSkinX)}>{relation.label}</span>
              <RelationEndpoint
                endpoint={relation.target}
                onOpenElement={onOpenElement}
                onOpenResource={onOpenResource}
              />
            </div>
          ))}
        </div>
      ) : null}
    </section>
  );
}
