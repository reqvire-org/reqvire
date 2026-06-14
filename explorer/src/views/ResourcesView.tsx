import { css, cx } from "@linaria/atomic";
import { useStore } from "../store/StoreContext";
import { ViewFrame } from "./ViewFrame";
import type { ExplorerViewProps } from "../components/ExplorerViewProps";
import { SourceCodePreview } from "../components/SourceCodePreview";
import { Badge, Card, CodeRef, TypeBadge } from "@ds";

const routeBaseUX = css`
  box-sizing: border-box;
  position: relative;
  display: grid;
  grid-template-columns: minmax(0, 1fr) !important;
  column-gap: 0;
  height: 100vh;
  min-height: 0;
  padding-left: var(--ex-current-left-width);
  padding-right: 0;

  .ex-app & {
    height: 100%;
    min-height: 0;
    overflow: hidden;
    padding-left: 0;
    padding-right: 0;
  }
`;

const routeSingleUX = css`
  grid-template-columns: minmax(0, 1fr) !important;
  column-gap: 0;
`;

const routeSkinX = css`
  background: var(--bg-canvas);
  color: var(--text-body);
`;

const documentPanelBaseUX = css`
  position: relative;
  box-sizing: border-box;
  min-width: 0;
  min-height: 0;
  overflow: auto;
  padding: var(--space-14) var(--space-16);

  .ex-app & {
    height: 100%;
    min-height: 0;
    overflow: auto;
    padding: var(--space-16);
  }
`;

const documentPanelSkinX = css`
  border-left: var(--border-w) solid color-mix(in srgb, var(--border-subtle) 65%, transparent);
  border-right: var(--border-w) solid color-mix(in srgb, var(--border-subtle) 65%, transparent);
  background: var(--bg-surface);

  .ex-app & {
    border-right: 0;
    border-left: 0;
    background: var(--bg-surface);
  }
`;

const contentToolbarBaseUX = css`
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-6);
  margin-bottom: var(--space-8);
  padding: 0 0 var(--space-6);

  .content-page-title {
    display: grid;
    min-width: 0;
    gap: var(--space-1);
  }

  .content-page-title span {
    font-size: var(--text-micro);
    font-weight: var(--weight-bold);
    letter-spacing: 0.07em;
    text-transform: uppercase;
  }

  .content-page-title strong {
    min-width: 0;
    overflow: hidden;
    font-size: var(--text-base);
    text-overflow: ellipsis;
    white-space: nowrap;
  }
`;

const contentToolbarSkinX = css`
  border-bottom: var(--border-w) solid var(--border-subtle);

  .content-page-title span {
    color: var(--text-muted);
  }

  .content-page-title strong {
    color: var(--text-body);
  }
`;

const commandBaseUX = css`
  padding: var(--space-3) var(--space-4);
  text-align: left;
  font-size: var(--text-sm);
  cursor: pointer;
`;

const commandSkinX = css`
  border: 0;
  border-radius: var(--radius-md);
  background: var(--accent);
  color: var(--accent-fg);
`;

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
  gap: var(--space-6);

  .resource-list-card-row {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: var(--space-6);
  }

  .resource-list-card-meta {
    font-size: var(--text-caption);
  }
`;

const resourcesListSkinX = css`
  .resource-list-card {
    box-shadow: none;
  }

  .resource-list-card-meta {
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

/*
 * Resources view (secondary / report artifact, not primary navigation).
 * Lists modeled resource and evidence-file targets from the Project Store,
 * kept distinct from browsable source `files`.
 */
export function ResourcesView({
  resourceId,
}: Partial<ExplorerViewProps> & { resourceId?: string | null } = {}) {
  const { store } = useStore();
  const resource = resourceId
    ? store.resources.find((candidate) => candidate.id === resourceId)
    : undefined;

  if (resourceId) {
    return (
      <ViewFrame testId="resources">
        <div className={cx(routeBaseUX, routeSingleUX, routeSkinX)}>
          <div className={cx(documentPanelBaseUX, documentPanelSkinX)}>
            <div className={cx(contentToolbarBaseUX, contentToolbarSkinX)}>
              <div className="content-page-title">
                <span>Resource</span>
                <strong>{resource?.display ?? resourceId}</strong>
              </div>
              <a className={cx(commandBaseUX, commandSkinX)} href="#/model">
                Back to model
              </a>
            </div>
            {resource?.source_text ? (
              <SourceCodePreview
                path={resource.file_path ?? resource.target}
                content={resource.source_text}
                kind={resource.kind}
                relationTypes={resource.relation_types}
              />
            ) : resource ? (
              <Card className={cx(detailCardBaseUX, detailCardSkinX)}>
                <TypeBadge type={resource.kind} family={resource.kind}>{resource.kind}</TypeBadge>
                {resource.external_url ? (
                  <a
                    href={resource.external_url}
                    target="_blank"
                    rel="noreferrer"
                    className={cx(resourceLinkBaseUX, resourceLinkSkinX)}
                  >
                    {resource.external_url}
                  </a>
                ) : (
                  <CodeRef>{resource.target}</CodeRef>
                )}
              </Card>
            ) : (
              <div className={cx(emptyBaseUX, emptySkinX)}>Resource not found: {resourceId}</div>
            )}
          </div>
        </div>
      </ViewFrame>
    );
  }

  return (
    <ViewFrame testId="resources">
      <div className={cx(routeBaseUX, routeSingleUX, routeSkinX)}>
        <div className={cx(documentPanelBaseUX, documentPanelSkinX)}>
          <div className={cx(resourcesHeaderBaseUX)}>
            <h1 className={cx(panelTitleBaseUX, panelTitleSkinX)}>
              Resources
            </h1>
            <Badge>{store.resources.length} resources</Badge>
          </div>
          <div className={cx(resourcesListBaseUX, resourcesListSkinX)}>
            {store.resources.map((r) => (
              <Card key={r.id} className="resource-list-card">
                <div className="resource-list-card-row">
                  <TypeBadge type={r.kind} family={r.kind}>{r.kind}</TypeBadge>
                  {r.external_url ? (
                    <a
                      href={r.external_url}
                      target="_blank"
                      rel="noreferrer"
                      className={cx(resourceLinkBaseUX, resourceLinkSkinX)}
                    >
                      {r.display}
                    </a>
                  ) : r.file_path ? (
                    <a href={`#/content/${r.file_path}`} className={cx(resourceLinkBaseUX, resourceLinkSkinX)}>
                      {r.display}
                    </a>
                  ) : (
                    <CodeRef>{r.target}</CodeRef>
                  )}
                  {r.relation_types.length > 0 && (
                    <span className="resource-list-card-meta">
                      via {r.relation_types.join(", ")}
                    </span>
                  )}
                </div>
              </Card>
            ))}
            {store.resources.length === 0 && <span className={cx(emptyBaseUX, emptySkinX)}>No resources in store.</span>}
          </div>
        </div>
      </div>
    </ViewFrame>
  );
}
