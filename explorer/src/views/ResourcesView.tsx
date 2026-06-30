import { useStore } from "../store/StoreContext";
import { ViewFrame } from "./ViewFrame";
import type { ExplorerViewProps } from "./types/ExplorerViewProps";
import { SourceCodePreview } from "../rendering/SourceCodePreview";
import { routeForResource } from "../router/routes";
import {
  DocumentPanelToolbar,
  ResourceDetailCard,
  ResourceEmptyState,
  ResourceList,
  ResourcePageHeader,
  RouteLayout,
  RoutePanel,
  type ResourceListItem,
} from "@ds";

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
        <RouteLayout>
          <RoutePanel>
            <DocumentPanelToolbar
              label="Resource"
              title={resource?.display ?? resourceId}
              actionHref="#/model"
              actionLabel="Back to model"
            />
            {resource?.source_text ? (
              <SourceCodePreview
                path={resource.file_path ?? resource.target}
                content={resource.source_text}
                kind={resource.kind}
                relationTypes={resource.relation_types}
              />
            ) : resource ? (
              <ResourceDetailCard
                kind={resource.kind}
                target={resource.target}
                externalUrl={resource.external_url}
              />
            ) : (
              <ResourceEmptyState>Resource not found: {resourceId}</ResourceEmptyState>
            )}
          </RoutePanel>
        </RouteLayout>
      </ViewFrame>
    );
  }

  const resources: ResourceListItem[] = store.resources.map((r) => ({
    id: r.id,
    kind: r.kind,
    display: r.display,
    target: r.target,
    href: routeForResource(r.id),
    externalUrl: r.external_url,
    relationTypes: r.relation_types,
  }));

  return (
    <ViewFrame testId="resources">
      <RouteLayout>
        <RoutePanel>
          <ResourcePageHeader count={resources.length} />
          <ResourceList resources={resources} />
        </RoutePanel>
      </RouteLayout>
    </ViewFrame>
  );
}
