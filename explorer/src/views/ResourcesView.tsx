import { useStore } from "../store/StoreContext";
import { ViewFrame } from "./ViewFrame";
import type { ExplorerViewProps } from "../components/ExplorerViewProps";
import { SourceCodePreview } from "../components/SourceCodePreview";
import { Card } from "@ds";

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
        <div className="ex-route ex-route-single">
          <div className="ex-document-panel">
            <div className="content-page-toolbar">
              <div className="content-page-title">
                <span>Resource</span>
                <strong>{resource?.display ?? resourceId}</strong>
              </div>
              <a className="ex-command" href="#/model">
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
              <Card className="resource-detail-card">
                <span className="rq-typebadge">{resource.kind}</span>
                {resource.external_url ? (
                  <a href={resource.external_url} target="_blank" rel="noreferrer" className="rq-btn rq-btn--link">
                    {resource.external_url}
                  </a>
                ) : (
                  <code className="rq-coderef">{resource.target}</code>
                )}
              </Card>
            ) : (
              <div className="ex-empty">Resource not found: {resourceId}</div>
            )}
          </div>
        </div>
      </ViewFrame>
    );
  }

  return (
    <ViewFrame testId="resources">
      <div className="ex-route ex-route-single">
        <div className="ex-document-panel">
        <div className="resources-header">
          <h1 className="ex-panel-title">
            Resources
          </h1>
          <span className="rq-badge">{store.resources.length} resources</span>
        </div>
        <div className="resources-list">
          {store.resources.map((r) => (
            <Card key={r.id} className="resource-list-card">
              <div className="resource-list-card-row">
                <span className="rq-typebadge">{r.kind}</span>
                {r.external_url ? (
                  <a href={r.external_url} target="_blank" rel="noreferrer" className="rq-btn rq-btn--link">
                    {r.display}
                  </a>
                ) : r.file_path ? (
                  <a href={`#/content/${r.file_path}`} className="rq-btn rq-btn--link">
                    {r.display}
                  </a>
                ) : (
                  <code className="rq-coderef">{r.target}</code>
                )}
                {r.relation_types.length > 0 && (
                  <span className="resource-list-card-meta">
                    via {r.relation_types.join(", ")}
                  </span>
                )}
              </div>
            </Card>
          ))}
          {store.resources.length === 0 && <span className="ex-empty">No resources in store.</span>}
        </div>
        </div>
      </div>
    </ViewFrame>
  );
}
