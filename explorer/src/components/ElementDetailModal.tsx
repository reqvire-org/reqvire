import { useMemo } from "react";
import {
  DetailDialog,
  ConceptElementDetailContent,
  ElementDetailContent,
  ElementDetailMissingState,
  type DetailReusedContractContextItem,
  type DetailConceptReferenceItem,
  type DetailMetaBadge,
  type DetailRelationEndpointData,
  type DetailRelationItem,
} from "@ds";
import { useStore } from "../store/StoreContext";
import type {
  OntologyGraphEdge,
  OntologyGraphNode,
  ProjectStoreElement,
  ProjectStoreRelation,
  ProjectStoreResource,
} from "../store/types";
import { routeForContent, routeForElement } from "../router/routes";
import { MarkdownContent, stripConceptReferencesSection } from "./MarkdownContent";

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
      .filter((relation) => !isPromotedConceptRelation(element, relation))
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
        const nativeConcept = store.thesaurus.concepts.find(
          (concept) => concept.element_id === conceptRef.target_element_id,
        );
        return {
          id: conceptRef.id,
          label: conceptRef.label,
          iri: conceptRef.iri,
          elementId: conceptRef.target_element_id,
          matchLabels: nativeConcept
            ? [nativeConcept.label, ...nativeConcept.alt_labels, conceptRef.label]
            : [conceptRef.label].filter(isString),
          ontologyLabel: nativeConcept?.label,
        };
      }),
    [conceptRefs, store.thesaurus.concepts],
  );
  const conceptDetail = useMemo(() => {
    if (!element || (element.element_type !== "concept" && element.element_type !== "concept-scheme")) return null;
    return buildConceptElementDetail({
      element,
      elements: store.elements,
      ontologyNodes: store.ontology.graph_data?.nodes ?? [],
      ontologyEdges: store.ontology.graph_data?.edges ?? [],
      conceptRefs: store.concept_refs,
      elementById,
    });
  }, [
    element,
    elementById,
    store.concept_refs,
    store.elements,
    store.ontology.graph_data?.edges,
    store.ontology.graph_data?.nodes,
  ]);

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
        conceptDetail ? (
          <ConceptElementDetailContent
            metaBadges={buildMetaBadges(element)}
            definition={
              <MarkdownContent
                markdown={conceptDetail.definition || element.content}
                sourceFilePath={element.file_path}
                sourceAnchor={element.source_anchor}
              />
            }
            scheme={conceptDetail.scheme}
            altLabels={conceptDetail.altLabels}
            scopeNote={conceptDetail.scopeNote}
            examples={conceptDetail.examples}
            topConcepts={conceptDetail.topConcepts}
            broader={conceptDetail.broader}
            narrower={conceptDetail.narrower}
            related={conceptDetail.related}
            exactMatches={conceptDetail.exactMatches}
            closeMatches={conceptDetail.closeMatches}
            mappedOntologyTerms={conceptDetail.mappedOntologyTerms}
            usedByModel={conceptDetail.usedByModel}
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
          <ElementDetailContent
            metaBadges={buildMetaBadges(element)}
            content={
              <MarkdownContent
                markdown={stripConceptReferencesSection(element.content)}
                sourceFilePath={element.file_path}
                sourceAnchor={element.source_anchor}
                conceptReferences={conceptReferenceItems}
                onOpenConceptReference={(reference) => {
                  if (reference.elementId) {
                    onOpenElement(reference.elementId);
                  }
                }}
              />
            }
            relations={relationItems}
            reused_contract_context={reusedContractContextItems}
            conceptReferences={[]}
            onOpenElement={onOpenElement}
            onOpenConceptReference={(reference) => {
              if (reference.elementId) {
                onOpenElement(reference.elementId);
              }
            }}
            onOpenResource={openHashRoute}
          />
        )
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

function isString(value: string | undefined): value is string {
  return typeof value === "string" && value.trim().length > 0;
}

interface ConceptElementDetailDto {
  definition: string;
  scheme: DetailRelationEndpointData | null;
  altLabels: string[];
  scopeNote: string;
  examples: string[];
  topConcepts: DetailRelationEndpointData[];
  broader: DetailRelationEndpointData[];
  narrower: DetailRelationEndpointData[];
  related: DetailRelationEndpointData[];
  exactMatches: DetailRelationEndpointData[];
  closeMatches: DetailRelationEndpointData[];
  mappedOntologyTerms: DetailConceptReferenceItem[];
  usedByModel: DetailRelationEndpointData[];
}

function buildConceptElementDetail({
  element,
  elements,
  ontologyNodes,
  ontologyEdges,
  conceptRefs,
  elementById,
}: {
  element: ProjectStoreElement;
  elements: readonly ProjectStoreElement[];
  ontologyNodes: readonly OntologyGraphNode[];
  ontologyEdges: readonly OntologyGraphEdge[];
  conceptRefs: readonly { id: string; source_id: string; label: string; iri: string }[];
  elementById: (id: string) => ProjectStoreElement | undefined;
}): ConceptElementDetailDto {
  const conceptNodes = ontologyNodes.filter((node) => node.semantic_type === "skos-concept");
  const schemeNodes = ontologyNodes.filter((node) => node.semantic_type === "skos-concept-scheme");
  const nodeById = new Map(ontologyNodes.map((node) => [node.id, node]));

  if (element.element_type === "concept-scheme") {
    const schemeNode =
      schemeNodes.find((node) => conceptNodeMatchesElement(node, element)) ??
      schemeNodes.find((node) => conceptLabel(node) === element.name);
    if (!schemeNode) {
      return {
        definition: element.content,
        scheme: null,
        altLabels: [],
        scopeNote: "",
        examples: [],
        topConcepts: [],
        broader: [],
        narrower: [],
        related: [],
        exactMatches: [],
        closeMatches: [],
        mappedOntologyTerms: [],
        usedByModel: [],
      };
    }

    const topConcepts = schemeTopConcepts(schemeNode, conceptNodes, ontologyEdges, nodeById);
    return {
      definition: firstLiteralValue(schemeNode, "definition") || element.content || schemeNode.comment,
      scheme: null,
      altLabels: literalValues(schemeNode, "altLabel"),
      scopeNote: firstLiteralValue(schemeNode, "scopeNote"),
      examples: literalValues(schemeNode, "example"),
      topConcepts: uniqueEndpoints(topConcepts.map((node) => conceptEndpoint(node, elements)).filter(isDetailRelationEndpointData)),
      broader: [],
      narrower: [],
      related: [],
      exactMatches: [],
      closeMatches: [],
      mappedOntologyTerms: [],
      usedByModel: [],
    };
  }

  const conceptNode =
    conceptNodes.find((node) => conceptNodeMatchesElement(node, element)) ??
    conceptNodes.find((node) => conceptLabel(node) === element.name);
  const conceptIds = new Set(conceptNodes.map((node) => node.id));

  if (!conceptNode) {
    return {
      definition: element.content,
      scheme: null,
      altLabels: [],
      scopeNote: "",
      examples: [],
      topConcepts: [],
      broader: [],
      narrower: [],
      related: [],
      exactMatches: [],
      closeMatches: [],
      mappedOntologyTerms: [],
      usedByModel: [],
    };
  }

  const schemeNode = conceptNode.scheme_iri ? nodeById.get(conceptNode.scheme_iri) : undefined;
  const broader: DetailRelationEndpointData[] = [];
  const narrower: DetailRelationEndpointData[] = [];
  const related: DetailRelationEndpointData[] = [];
  const exactMatches: DetailRelationEndpointData[] = [];
  const closeMatches: DetailRelationEndpointData[] = [];
  const mappedOntologyTerms: DetailConceptReferenceItem[] = [];

  for (const edge of ontologyEdges) {
    if (edge.label === "broader") {
      if (edge.source === conceptNode.id && edge.target !== conceptNode.id) {
        pushEndpoint(broader, conceptEndpoint(nodeById.get(edge.target), elements));
      }
      if (edge.target === conceptNode.id && edge.source !== conceptNode.id) {
        pushEndpoint(narrower, conceptEndpoint(nodeById.get(edge.source), elements));
      }
      continue;
    }
    if (edge.label === "narrower") {
      if (edge.source === conceptNode.id && edge.target !== conceptNode.id) {
        pushEndpoint(narrower, conceptEndpoint(nodeById.get(edge.target), elements));
      }
      if (edge.target === conceptNode.id && edge.source !== conceptNode.id) {
        pushEndpoint(broader, conceptEndpoint(nodeById.get(edge.source), elements));
      }
      continue;
    }
    if (edge.label === "related") {
      if (edge.source === conceptNode.id && edge.target !== conceptNode.id) {
        pushEndpoint(related, conceptEndpoint(nodeById.get(edge.target), elements));
      }
      if (edge.target === conceptNode.id && edge.source !== conceptNode.id) {
        pushEndpoint(related, conceptEndpoint(nodeById.get(edge.source), elements));
      }
      continue;
    }
    if (edge.label === "exactMatch") {
      if (edge.source === conceptNode.id && edge.target !== conceptNode.id) {
        pushEndpoint(exactMatches, conceptEndpoint(nodeById.get(edge.target), elements));
      }
      if (edge.target === conceptNode.id && edge.source !== conceptNode.id) {
        pushEndpoint(exactMatches, conceptEndpoint(nodeById.get(edge.source), elements));
      }
      continue;
    }
    if (edge.label === "closeMatch") {
      if (edge.source === conceptNode.id && edge.target !== conceptNode.id) {
        pushEndpoint(closeMatches, conceptEndpoint(nodeById.get(edge.target), elements));
      }
      if (edge.target === conceptNode.id && edge.source !== conceptNode.id) {
        pushEndpoint(closeMatches, conceptEndpoint(nodeById.get(edge.source), elements));
      }
      continue;
    }
    if (edge.label === "mapsToConcept" || edge.label === "mappedFrom") {
      const sourceIsConcept = edge.source === conceptNode.id;
      const targetIsConcept = edge.target === conceptNode.id;
      if (!sourceIsConcept && !targetIsConcept) continue;
      const mappedNode = nodeById.get(sourceIsConcept ? edge.target : edge.source);
      if (mappedNode && mappedNode.id !== conceptNode.id && !conceptIds.has(mappedNode.id)) {
        mappedOntologyTerms.push({
          id: `mapped-${conceptNode.id}-${mappedNode.id}`,
          label: mappedNode.label,
          iri: mappedNode.full_uri || mappedNode.id,
          ontologyNodeId: mappedNode.id,
          ontologyLabel: mappedNode.label,
        });
      }
    }
  }

  return {
    definition: firstLiteralValue(conceptNode, "definition") || element.content || conceptNode.comment,
    scheme: schemeNode ? conceptEndpoint(schemeNode, elements) : null,
    altLabels: literalValues(conceptNode, "altLabel"),
    scopeNote: firstLiteralValue(conceptNode, "scopeNote"),
    examples: literalValues(conceptNode, "example"),
    topConcepts: [],
    broader: uniqueEndpoints(broader),
    narrower: uniqueEndpoints(narrower),
    related: uniqueEndpoints(related),
    exactMatches: uniqueEndpoints(exactMatches),
    closeMatches: uniqueEndpoints(closeMatches),
    mappedOntologyTerms: uniqueConceptReferences(mappedOntologyTerms),
    usedByModel: conceptRefs
      .filter((reference) => conceptReferenceMatchesNode(reference.iri, conceptNode))
      .map((reference) => elementById(reference.source_id))
      .filter(isProjectStoreElement)
      .map((source) => elementEndpoint(source)),
  };

  function pushEndpoint(target: DetailRelationEndpointData[], endpoint: DetailRelationEndpointData | null) {
    if (endpoint) target.push(endpoint);
  }

  function conceptEndpoint(node: OntologyGraphNode | undefined, allElements: readonly ProjectStoreElement[]) {
    if (!node) return null;
    const matchedElement = resolveElementForConceptNode(node, allElements);
    if (matchedElement) return elementEndpoint(matchedElement);
    return {
      id: node.id,
      label: node.label,
      kind: node.semantic_type || "concept",
      elementType: node.semantic_type,
      typeFamily: "concept",
      href: null,
      external: false,
    };
  }
}

function schemeTopConcepts(
  schemeNode: OntologyGraphNode,
  conceptNodes: readonly OntologyGraphNode[],
  ontologyEdges: readonly OntologyGraphEdge[],
  nodeById: ReadonlyMap<string, OntologyGraphNode>,
) {
  const topConceptIds = new Set<string>();
  const broaderSources = new Set<string>();
  for (const edge of ontologyEdges) {
    if (edge.label === "broader") broaderSources.add(edge.source);
    if (edge.label === "hasTopConcept" && edge.source === schemeNode.id) topConceptIds.add(edge.target);
    if (edge.label === "topConceptOf" && edge.target === schemeNode.id) topConceptIds.add(edge.source);
  }
  if (topConceptIds.size > 0) {
    return [...topConceptIds].map((id) => nodeById.get(id)).filter(isOntologyGraphNode);
  }
  return conceptNodes.filter((node) => node.scheme_iri === schemeNode.id && !broaderSources.has(node.id));
}

function conceptNodeMatchesElement(node: OntologyGraphNode, element: ProjectStoreElement) {
  if (node.sources.some((source) => source.source === element.id)) return true;
  return conceptLabel(node) === element.name && node.sources.some((source) => source.file_path === element.file_path);
}

function resolveElementForConceptNode(
  node: OntologyGraphNode,
  elements: readonly ProjectStoreElement[],
) {
  const label = conceptLabel(node);
  const expectedType = node.semantic_type === "skos-concept-scheme" ? "concept-scheme" : "concept";
  const sourceMatch = node.sources
    .map((source) => source.source)
    .find((id) => elements.some((element) => element.id === id && element.element_type === expectedType));
  if (sourceMatch) return elements.find((element) => element.id === sourceMatch);
  const candidates = elements.filter((element) => element.element_type === expectedType && element.name === label);
  const sourceFile = node.sources[0]?.file_path;
  return candidates.find((element) => element.file_path === sourceFile) ?? (candidates.length === 1 ? candidates[0] : undefined);
}

function elementEndpoint(element: ProjectStoreElement): DetailRelationEndpointData {
  return {
    id: element.id,
    label: element.name,
    kind: "element",
    elementType: element.element_type,
    typeFamily: element.type_family,
    href: routeForElement(element.id),
    external: false,
  };
}

function conceptLabel(node: OntologyGraphNode) {
  return firstLiteralValue(node, "prefLabel") || node.label;
}

function literalValues(node: OntologyGraphNode, predicateSuffix: string) {
  return (node.literal_values ?? [])
    .filter((value) => value.predicate.endsWith(predicateSuffix))
    .map((value) => value.value)
    .filter(Boolean);
}

function firstLiteralValue(node: OntologyGraphNode, predicateSuffix: string) {
  return literalValues(node, predicateSuffix)[0] ?? "";
}

function conceptReferenceMatchesNode(iri: string, node: OntologyGraphNode) {
  return iri === node.full_uri || iri === node.id || ontologyNodeCurie(iri) === ontologyNodeCurie(node.full_uri);
}

function ontologyNodeCurie(value: string) {
  return value.startsWith("concept:") ? value.slice("concept:".length) : value;
}

function uniqueEndpoints(endpoints: DetailRelationEndpointData[]) {
  return Array.from(new Map(endpoints.map((endpoint) => [endpoint.id, endpoint])).values());
}

function uniqueConceptReferences(references: DetailConceptReferenceItem[]) {
  return Array.from(new Map(references.map((reference) => [reference.ontologyNodeId ?? reference.iri, reference])).values());
}

function isProjectStoreElement(value: ProjectStoreElement | undefined): value is ProjectStoreElement {
  return Boolean(value);
}

function isOntologyGraphNode(value: OntologyGraphNode | undefined): value is OntologyGraphNode {
  return Boolean(value);
}

function isDetailRelationEndpointData(
  value: DetailRelationEndpointData | null,
): value is DetailRelationEndpointData {
  return Boolean(value);
}

function isDetailRelationItem(value: DetailRelationItem | null): value is DetailRelationItem {
  return value !== null;
}

function isPromotedConceptRelation(element: ProjectStoreElement, relation: ProjectStoreRelation) {
  if (element.element_type !== "concept" && element.element_type !== "concept-scheme") return false;
  if (relation.source_id !== element.id && relation.target_id !== element.id) return false;
  const relationType = relation.canonical_relation_type || relation.relation_type;
  return ["broader", "narrower", "related", "exactMatch", "closeMatch"].includes(relationType);
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
    inverseRelationLabel(relation.relation_type || relation.canonical_relation_type)
  );
}

function inverseRelationLabel(relationType: string): string {
  const inverse: Record<string, string> = {
    constrain: "constrainedBy",
    constrainedBy: "constrain",
    define: "definedBy",
    definedBy: "define",
    derive: "derivedFrom",
    derivedFrom: "derive",
    satisfy: "satisfiedBy",
    satisfiedBy: "satisfy",
    specify: "specifiedBy",
    specifiedBy: "specify",
    use: "usedBy",
    usedBy: "use",
    verify: "verifiedBy",
    verifiedBy: "verify",
  };
  return inverse[relationType] ?? relationType;
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
