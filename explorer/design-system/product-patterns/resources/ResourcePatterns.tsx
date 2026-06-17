import type { ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import { Badge } from "../../components/core/Badge";
import { Card } from "../../components/core/Card";
import { CodeRef } from "../../components/data/CodeRef";
import { TypeBadge } from "../../components/data/TypeBadge";

export interface ResourceListItem {
  id: string;
  kind: string;
  display: ReactNode;
  target: ReactNode;
  href?: string | null;
  externalUrl?: string | null;
  relationTypes?: readonly string[];
}

export interface ResourcePageHeaderProps {
  title?: ReactNode;
  count: number;
}

export interface ResourceListProps {
  resources: readonly ResourceListItem[];
  emptyMessage?: ReactNode;
}

export interface ResourceDetailCardProps {
  kind: string;
  target: ReactNode;
  externalUrl?: string | null;
}

export interface ResourceEmptyStateProps {
  children?: ReactNode;
}

const resourceLinkBaseUX = css`
  font-size: var(--text-sm);
  text-decoration: none;
`;

const resourceLinkSkinX = css`
  color: var(--text-secondary);

  &:hover {
    color: var(--text-strong);
    text-decoration: underline;
  }
`;

const detailCardBaseUX = css`
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: var(--space-6);
  font-size: var(--text-caption);
`;

const detailCardSkinX = css`
  color: var(--text-muted);
  box-shadow: none;
`;

const resourcesHeaderBaseUX = css`
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: var(--space-6);
  margin-bottom: var(--space-10);
`;

const panelTitleBaseUX = css`
  margin: 0;
  font-size: var(--text-lg);
  font-weight: var(--weight-semibold);
  line-height: 1.25;
`;

const panelTitleSkinX = css`
  color: var(--text-body);
`;

const resourcesListBaseUX = css`
  display: flex;
  flex-direction: column;
  gap: var(--gap-list-stack);

  .ux-resource-list-card-row {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: var(--space-6);
  }

  .ux-resource-list-card-meta {
    font-size: var(--text-caption);
  }
`;

const resourcesListSkinX = css`
  .ux-resource-list-card {
    box-shadow: none;
  }

  .ux-resource-list-card-meta {
    color: var(--text-muted);
  }
`;

const emptyBaseUX = css`
  font-size: var(--text-sm);
  font-style: italic;
  line-height: 1.45;
`;

const emptySkinX = css`
  color: var(--text-muted);
`;

export function ResourcePageHeader({ title = "Resources", count }: ResourcePageHeaderProps) {
  return (
    <div className={cx("ux-resources-header", resourcesHeaderBaseUX)}>
      <h1 className={cx("ux-resources-title", panelTitleBaseUX, panelTitleSkinX)}>{title}</h1>
      <Badge>{count} resources</Badge>
    </div>
  );
}

export function ResourceList({ resources, emptyMessage = "No resources in store." }: ResourceListProps) {
  return (
    <div className={cx("ux-resource-list", resourcesListBaseUX, resourcesListSkinX)}>
      {resources.map((resource) => (
        <Card key={resource.id} className="ux-resource-list-card">
          <div className="ux-resource-list-card-row">
            <TypeBadge type={resource.kind} family={resource.kind}>
              {resource.kind}
            </TypeBadge>
            <ResourceTargetLink resource={resource} />
            {(resource.relationTypes?.length ?? 0) > 0 ? (
              <span className="ux-resource-list-card-meta">
                via {resource.relationTypes?.join(", ")}
              </span>
            ) : null}
          </div>
        </Card>
      ))}
      {resources.length === 0 ? <ResourceEmptyState>{emptyMessage}</ResourceEmptyState> : null}
    </div>
  );
}

export function ResourceDetailCard({ kind, target, externalUrl }: ResourceDetailCardProps) {
  return (
    <Card className={cx("ux-resource-detail-card", detailCardBaseUX, detailCardSkinX)}>
      <TypeBadge type={kind} family={kind}>
        {kind}
      </TypeBadge>
      {externalUrl ? (
        <a
          href={externalUrl}
          target="_blank"
          rel="noreferrer"
          className={cx("ux-resource-link", resourceLinkBaseUX, resourceLinkSkinX)}
        >
          {externalUrl}
        </a>
      ) : (
        <CodeRef>{target}</CodeRef>
      )}
    </Card>
  );
}

export function ResourceEmptyState({ children }: ResourceEmptyStateProps) {
  return <span className={cx("ux-resource-empty", emptyBaseUX, emptySkinX)}>{children}</span>;
}

function ResourceTargetLink({ resource }: { resource: ResourceListItem }) {
  if (resource.externalUrl) {
    return (
      <a
        href={resource.externalUrl}
        target="_blank"
        rel="noreferrer"
        className={cx("ux-resource-link", resourceLinkBaseUX, resourceLinkSkinX)}
      >
        {resource.display}
      </a>
    );
  }

  if (resource.href) {
    return (
      <a href={resource.href} className={cx("ux-resource-link", resourceLinkBaseUX, resourceLinkSkinX)}>
        {resource.display}
      </a>
    );
  }

  return <CodeRef>{resource.target}</CodeRef>;
}
