import { useMemo } from "react";
import {
  DetailDialog,
  ElementDetailContent,
  ElementDetailMissingState,
  type DetailReusedContractContextItem,
  type DetailConceptReferenceItem,
  type DetailMetaBadge,
  type DetailRelationEndpointData,
  type DetailRelationItem,
} from "@ds";
import { useStore } from "../store/StoreContext";
import type { ProjectStoreElement, ProjectStoreRelation, ProjectStoreResource } from "../store/types";
import { routeForContent, routeForElement } from "../router/routes";
import { MarkdownContent } from "./MarkdownContent";

/*
 * Element-detail modal container.
 *
 * Keeps Project Store reads, route construction, and browser hash mutation in
 * src while delegating visual detail layout to design-system product patterns.
 */
export function ElementDetailModal({
  identifier,
  onClose,
  onOpenElement,
  onOpenOntologyNode,
  onNavigateBack,
  previousElementLabel,
}: {
  identifier: string | null;
  onClose: () => void;
  onOpenElement: (id: string) => void;
  onOpenOntologyNode: (nodeId: string) => void;
  onNavigateBack?: () => void;
  previousElementLabel?: string | null;
}) {
  const { store, elementById } = useStore();
  const element = identifier ? elementById(identifier) : undefined;
  const resourceById = useMemo(
    () => new Map(store.resources.map((resource) => [resource.id, resource])),
    [store.resources],
  );
  const ontologyNodeByIri = useMemo(() => {
    const byIri = new Map<string, { id: string; label: string }>();
    for (const node of store.ontology.graph_data?.nodes ?? []) {
      const target = { id: node.id, label: node.label || node.full_uri || node.id };
      byIri.set(node.full_uri, target);
      byIri.set(node.id, target);
    }
    return byIri;
  }, [store.ontology.graph_data?.nodes]);

  const { relations, reused_contract_context, conceptRefs } = useMemo(() => {
    if (!identifier) {
      return { relations: [], reused_contract_context: [], conceptRefs: [] };
    }
    return {
      relations: store.relations.filter((r) => r.source_id === identifier || r.target_id === identifier),
      reused_contract_context: store.reused_contract_context.filter((a) => a.source_id === identifier),
      conceptRefs: store.concept_refs.filter((c) => c.source_id === identifier),
    };
  }, [identifier, store]);

  const relationItems = useMemo(() => {
    if (!element) return [];
    return relations
      .map((relation) => relationFlowFromSelectedElement(relation, element.id, elementById, resourceById))
      .filter(isDetailRelationItem);
  }, [element, elementById, relations, resourceById]);

  const reusedContractContextItems = useMemo(
    () =>
      reused_contract_context.map((reused_contract_context): DetailReusedContractContextItem => {
        const target = reusedContractContextDisplayTarget(reused_contract_context, elementById, resourceById);
        return {
          id: reused_contract_context.id,
          targetId: reused_contract_context.target,
          kind: reused_contract_context.target_kind,
          resourceKind: target.resourceKind,
          label: target.label,
          elementType: target.elementType,
          typeFamily: target.typeFamily,
          href: target.href,
          external: target.external,
        };
      }),
    [reused_contract_context, elementById, resourceById],
  );

  const conceptReferenceItems = useMemo(
    () =>
      conceptRefs.map((conceptRef): DetailConceptReferenceItem => {
        const ontologyNode = ontologyNodeByIri.get(conceptRef.iri);
        return {
          id: conceptRef.id,
          label: conceptRef.label,
          iri: conceptRef.iri,
          ontologyNodeId: ontologyNode?.id,
          ontologyLabel: ontologyNode?.label,
        };
      }),
    [conceptRefs, ontologyNodeByIri],
  );

  const openHashRoute = (href: string) => {
    window.location.hash = href;
  };

  const open = identifier !== null;
  const previousElement = previousElementLabel ? elementById(previousElementLabel) : undefined;
  const previousTitle = previousElement?.name ?? previousElementLabel ?? undefined;

  return (
    <DetailDialog
      open={open}
      onOpenChange={(value) => !value && onClose()}
      title={element ? element.name : "Element not found"}
      typeFamily={element?.type_family}
      elementType={element?.element_type}
      showHeaderClose={Boolean(element)}
      sourceHref={element ? sourceAnchorRoute(element.source_anchor, element.file_path) : null}
      backLabel={previousTitle ? `Back to ${previousTitle}` : undefined}
      onNavigateBack={onNavigateBack}
      onOpenSource={openHashRoute}
    >
      {element ? (
        <ElementDetailContent
          metaBadges={buildMetaBadges(element)}
          content={
            <MarkdownContent
              markdown={element.content}
              sourceFilePath={element.file_path}
              sourceAnchor={element.source_anchor}
            />
          }
          relations={relationItems}
          reused_contract_context={reusedContractContextItems}
          conceptReferences={conceptReferenceItems}
          onOpenElement={onOpenElement}
          onOpenConceptReference={(reference) => {
            if (reference.ontologyNodeId) onOpenOntologyNode(reference.ontologyNodeId);
          }}
          onOpenResource={openHashRoute}
        />
      ) : (
        <ElementDetailMissingState identifier={identifier ?? ""} />
      )}
    </DetailDialog>
  );
}

/* One quiet metadata row replaces the old governance pills. Authored metadata is
   explicit; inherited governance keeps only the value here and carries
   provenance in a low-emphasis label. The `type` key is skipped because the
   header type badge already states it. */
function buildMetaBadges(element: {
  element_type: string;
  metadata: Record<string, string>;
  governance: Record<string, string>;
}): DetailMetaBadge[] {
  const badges: DetailMetaBadge[] = [];
  for (const [key, value] of Object.entries(element.metadata)) {
    if (key === "type" && value === element.element_type) continue;
    badges.push({ key, value, provenance: "explicit" });
  }
  const seen = new Set(badges.map((badge) => badge.key.toLowerCase()));
  for (const [key, raw] of Object.entries(element.governance)) {
    if (seen.has(key.toLowerCase())) continue;
    const { value, provenance } = cleanGovernanceValue(raw);
    badges.push({
      key,
      value,
      provenance,
    });
  }
  return badges;
}

function cleanGovernanceValue(raw: string): Pick<DetailMetaBadge, "value" | "provenance"> {
  const explicitMatch = raw.match(/^(.*?)\s*\(explicit\)$/);
  if (explicitMatch) {
    return { value: explicitMatch[1].trim(), provenance: "explicit" };
  }
  const inheritedMatch = raw.match(/^(.*?)\s*\(inherited(?:,\s*from\s+[^)]*)?\)$/);
  if (inheritedMatch) {
    return { value: inheritedMatch[1].trim(), provenance: "inherited" };
  }
  return { value: raw, provenance: "inherited" };
}

function sourceAnchorRoute(sourceAnchor: string, filePath: string): string {
  if (sourceAnchor.startsWith("#/content/")) return sourceAnchor;
  if (sourceAnchor.startsWith("#")) return `${routeForContent(filePath)}${sourceAnchor}`;

  const hashIndex = sourceAnchor.indexOf("#");
  const path = hashIndex === -1 ? sourceAnchor : sourceAnchor.slice(0, hashIndex);
  const fragment = hashIndex === -1 ? "" : sourceAnchor.slice(hashIndex);
  const markdownPath = path.endsWith(".html") ? `${path.slice(0, -".html".length)}.md` : path;
  return `${routeForContent(markdownPath || filePath)}${fragment}`;
}

function isDetailRelationItem(value: DetailRelationItem | null): value is DetailRelationItem {
  return value !== null;
}

function relationFlowFromSelectedElement(
  relation: ProjectStoreRelation,
  selectedId: string,
  elementById: (id: string) => ProjectStoreElement | undefined,
  resourceById: Map<string, ProjectStoreResource>,
): DetailRelationItem | null {
  if (relation.source_id === selectedId) {
    return {
      id: relation.id,
      label: relation.canonical_relation_type,
      target: relationTargetEndpoint(relation, elementById, resourceById),
    };
  }
  if (relation.target_id === selectedId) {
    return {
      id: relation.id,
      label: selectedTargetRelationLabel(relation),
      target: relationSourceEndpoint(relation, elementById),
    };
  }
  return null;
}

function selectedTargetRelationLabel(relation: ProjectStoreRelation): string {
  return (
    relation.source_relation_types.find((type) => type !== relation.canonical_relation_type) ??
    relation.relation_type ??
    relation.canonical_relation_type
  );
}

function relationSourceEndpoint(
  relation: ProjectStoreRelation,
  elementById: (id: string) => ProjectStoreElement | undefined,
): DetailRelationEndpointData {
  const element = elementById(relation.source_id);
  return {
    id: relation.source_id,
    label: element?.name ?? relation.source_id,
    kind: "element",
    elementType: element?.element_type,
    typeFamily: element?.type_family,
    href: routeForElement(relation.source_id),
    external: false,
  };
}

function relationTargetEndpoint(
  relation: ProjectStoreRelation,
  elementById: (id: string) => ProjectStoreElement | undefined,
  resourceById: Map<string, ProjectStoreResource>,
): DetailRelationEndpointData {
  if (relation.target_kind === "element") {
    const element = elementById(relation.target_id);
    return {
      id: relation.target_id,
      label: element?.name ?? relation.target_id,
      kind: "element",
      elementType: element?.element_type,
      typeFamily: element?.type_family,
      href: routeForElement(relation.target_id),
      external: false,
    };
  }
  if (relation.resource_id) {
    const resource = resourceById.get(relation.resource_id);
    if (resource?.external_url) {
      return {
        id: relation.resource_id,
        label: resource.display || resource.target,
        kind: resource.kind,
        href: resource.external_url,
        external: true,
      };
    }
    if (resource?.file_path) {
      return {
        id: relation.resource_id,
        label: resource.display || resource.target,
        kind: resource.kind,
        href: routeForContent(resource.file_path),
        external: false,
      };
    }
    if (resource) {
      return {
        id: relation.resource_id,
        label: resource.display || resource.target,
        kind: resource.kind,
        href: null,
        external: false,
      };
    }
  }
  return {
    id: relation.target_id,
    label: relation.target_id,
    kind: relation.target_kind,
    href: null,
    external: false,
  };
}

function reusedContractContextDisplayTarget(
  reused_contract_context: { target: string; target_kind: string; resource_id: string | null },
  elementById: (id: string) => Pick<ProjectStoreElement, "name" | "element_type" | "type_family"> | undefined,
  resourceById: Map<string, { display: string; target: string; kind: string; file_path: string | null; external_url: string | null }>,
): { label: string; resourceKind?: string; elementType?: string; typeFamily?: string; href: string | null; external: boolean } {
  if (reused_contract_context.target_kind === "element") {
    const element = elementById(reused_contract_context.target);
    return {
      label: element?.name ?? reused_contract_context.target,
      elementType: element?.element_type,
      typeFamily: element?.type_family,
      href: routeForElement(reused_contract_context.target),
      external: false,
    };
  }
  if (reused_contract_context.resource_id) {
    const resource = resourceById.get(reused_contract_context.resource_id);
    if (resource) {
      if (resource.external_url) {
        return { label: resource.display || resource.target, resourceKind: resource.kind, href: resource.external_url, external: true };
      }
      if (resource.file_path) {
        return { label: resource.display || resource.target, resourceKind: resource.kind, href: routeForContent(resource.file_path), external: false };
      }
      return { label: resource.display || resource.target, resourceKind: resource.kind, href: null, external: false };
    }
  }
  return { label: reused_contract_context.target, href: null, external: false };
}
