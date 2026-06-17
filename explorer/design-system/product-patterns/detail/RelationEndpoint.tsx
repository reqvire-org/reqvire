import type { MouseEvent } from "react";
import { cx } from "@linaria/atomic";
import { ElementIcon } from "../../components/data/ElementIcon";
import {
  relationEndpointBaseUX,
  relationEndpointLabelUX,
  relationEndpointSkinX,
} from "./detailStyles";
import type {
  DetailRelationEndpointData,
  DetailResourceTarget,
  OpenElementHandler,
  OpenResourceHandler,
} from "./types";

export interface RelationEndpointProps {
  endpoint: DetailRelationEndpointData;
  onOpenElement: OpenElementHandler;
  onOpenResource?: OpenResourceHandler;
}

export function RelationEndpoint({ endpoint, onOpenElement, onOpenResource }: RelationEndpointProps) {
  const className = cx(relationEndpointBaseUX, relationEndpointSkinX);
  const content = (
    <>
      {endpoint.kind === "element" ? (
        <ElementIcon
          type={endpoint.elementType}
          family={endpoint.typeFamily}
          title={endpoint.elementType}
          size="sm"
        />
      ) : (
        <ElementIcon type={endpoint.kind} title={endpoint.kind} size="sm" />
      )}
      <span className={cx(relationEndpointLabelUX)}>{endpoint.label}</span>
    </>
  );

  if (endpoint.kind === "element" && endpoint.href) {
    return (
      <a
        className={className}
        href={endpoint.href}
        title={endpoint.id}
        onClick={(event) => {
          event.preventDefault();
          onOpenElement(endpoint.id);
        }}
      >
        {content}
      </a>
    );
  }

  if (endpoint.href) {
    return (
      <a
        className={className}
        href={endpoint.href}
        title={endpoint.id}
        onClick={(event) => handleResourceClick(event, endpoint, onOpenResource)}
        {...(endpoint.external ? { target: "_blank", rel: "noreferrer" } : {})}
      >
        {content}
      </a>
    );
  }

  return (
    <span className={className} title={endpoint.id}>
      {content}
    </span>
  );
}

function handleResourceClick(
  event: MouseEvent<HTMLAnchorElement>,
  endpoint: DetailRelationEndpointData,
  onOpenResource: OpenResourceHandler | undefined,
) {
  if (endpoint.external || !endpoint.href || !onOpenResource) return;
  event.preventDefault();
  onOpenResource(endpoint.href, resourceTargetFromEndpoint(endpoint, endpoint.href));
}

function resourceTargetFromEndpoint(endpoint: DetailRelationEndpointData, href: string): DetailResourceTarget {
  return {
    id: endpoint.id,
    href,
    kind: endpoint.kind,
    label: endpoint.label,
  };
}
