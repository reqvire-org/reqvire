import { cx } from "@linaria/atomic";
import {
  relationKindBaseUX,
  relationKindSkinX,
  relationListUX,
  relationRowBaseUX,
} from "./detailStyles";
import { DetailSection } from "./DetailSection";
import type {
  DetailAttachmentItem,
  OpenElementHandler,
  OpenResourceHandler,
} from "./types";
import { RelationEndpoint } from "./RelationEndpoint";

export interface AttachmentListProps {
  title?: string;
  attachments: DetailAttachmentItem[];
  onOpenElement: OpenElementHandler;
  onOpenResource?: OpenResourceHandler;
}

export function AttachmentList({
  title = "Attachments",
  attachments,
  onOpenElement,
  onOpenResource,
}: AttachmentListProps) {
  if (attachments.length === 0) return null;

  return (
    <DetailSection title={title}>
      <div className={cx(relationListUX)}>
        {attachments.map((attachment) => (
          <AttachmentTarget
            key={attachment.id}
            attachment={attachment}
            onOpenElement={onOpenElement}
            onOpenResource={onOpenResource}
          />
        ))}
      </div>
    </DetailSection>
  );
}

function AttachmentTarget({
  attachment,
  onOpenElement,
  onOpenResource,
}: {
  attachment: DetailAttachmentItem;
  onOpenElement: OpenElementHandler;
  onOpenResource?: OpenResourceHandler;
}) {
  return (
    <div className={cx(relationRowBaseUX)}>
      <span className={cx(relationKindBaseUX, relationKindSkinX)}>{attachment.kind}</span>
      <RelationEndpoint
        endpoint={{
          id: attachment.targetId,
          label: attachment.label,
          kind: attachment.resourceKind ?? attachment.kind,
          elementType: attachment.elementType,
          typeFamily: attachment.typeFamily,
          href: attachment.href,
          external: attachment.external,
        }}
        onOpenElement={onOpenElement}
        onOpenResource={onOpenResource}
      />
    </div>
  );
}
