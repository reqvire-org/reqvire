import { cx } from "@linaria/atomic";
import {
  relationKindBaseUX,
  relationKindSkinX,
  relationListUX,
  relationRowBaseUX,
} from "./detailStyles";
import { DetailSection } from "./DetailSection";
import { RelationEndpoint } from "./RelationEndpoint";
import type { DetailRelationItem, OpenElementHandler, OpenResourceHandler } from "./types";

export interface RelationListProps {
  title?: string;
  relations: DetailRelationItem[];
  onOpenElement: OpenElementHandler;
  onOpenResource?: OpenResourceHandler;
}

export function RelationList({
  title = "Relations",
  relations,
  onOpenElement,
  onOpenResource,
}: RelationListProps) {
  if (relations.length === 0) return null;

  return (
    <DetailSection title={title}>
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
    </DetailSection>
  );
}
