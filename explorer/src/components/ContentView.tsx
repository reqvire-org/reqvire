import { useEffect, useMemo } from "react";
import {
  ConceptElementDetailContent,
  DocumentPanel,
  ElementDetailContent,
  RendererNotice,
  SourcePageElement,
  SourcePageElements,
  type DetailConceptReferenceItem,
  type DetailContractBindingItem,
} from "@ds";
import { useStore } from "../store/StoreContext";
import type { ProjectStoreElement } from "../store/types";
import { MarkdownContent, stripRenderedDetailSections } from "./MarkdownContent";
import { routeForView } from "../router/routes";
import { SourceCodePreview } from "./SourceCodePreview";
import {
  buildConceptElementDetail,
  buildMetaBadges,
  contractBindingsDisplayTarget,
  isDetailRelationItem,
  isPromotedConceptRelation,
  relationFlowFromSelectedElement,
  sourceAnchorRoute,
} from "./ElementDetailModal";

interface ContentViewProps {
  path: string;
}

export function ContentView({ path }: ContentViewProps) {
  const { store, elementById } = useStore();
  const [filePath, fragmentId] = splitContentPath(path);
  const file = store.files.find((f) => f.path === filePath);
  const sourceResource = store.resources.find(
    (resource) => resource.file_path === filePath || resource.target === filePath,
  );
  const sourcePath = sourceResource?.file_path ?? sourceResource?.target ?? filePath;

  useEffect(() => {
    if (!fragmentId) return;
    const frame = window.requestAnimationFrame(() => {
      document.getElementById(fragmentId)?.scrollIntoView({ block: "start" });
    });
    return () => {
      window.cancelAnimationFrame(frame);
    };
  }, [fragmentId, filePath]);

  if (!file) {
    if (sourceResource?.source_text) {
      return (
        <DocumentPanel toolbar={contentToolbar(sourcePath, "Source file")}>
          <SourceCodePreview
            path={sourcePath}
            content={sourceResource.source_text}
            kind={sourceResource.kind}
            relationTypes={sourceResource.relation_types}
            showPath={false}
          />
        </DocumentPanel>
      );
    }

    return (
      <DocumentPanel toolbar={contentToolbar(filePath)}>
        <RendererNotice tone="empty">
          File not found: {filePath}
        </RendererNotice>
      </DocumentPanel>
    );
  }

  if (file.element_ids.length === 0 || sourceResource?.source_text) {
    return (
      <DocumentPanel toolbar={contentToolbar(file.path, "Source file")}>
        <SourceCodePreview
          path={file.path}
          content={sourceResource?.source_text ?? file.markdown_content}
          kind={sourceResource?.kind ?? "source file"}
          relationTypes={sourceResource?.relation_types ?? []}
          showPath={false}
        />
      </DocumentPanel>
    );
  }

  const elements = file.element_ids.map((id) => elementById(id)).filter(isProjectStoreElement);

  return (
    <DocumentPanel toolbar={contentToolbar(file.path)}>
      <SourcePageElements>
        {elements.map((element) => (
          <SourceElementView key={element.id} element={element} />
        ))}
      </SourcePageElements>
    </DocumentPanel>
  );
}

function SourceElementView({ element }: { element: ProjectStoreElement }) {
  const { store, elementById } = useStore();
  const resourceById = useMemo(
    () => new Map(store.resources.map((resource) => [resource.id, resource])),
    [store.resources],
  );
  const relations = useMemo(
    () => store.relations.filter((relation) => relation.source_id === element.id || relation.target_id === element.id),
    [element.id, store.relations],
  );
  const contractBindings = useMemo(
    () => store.contract_bindings.filter((contractBinding) => contractBinding.source_id === element.id),
    [element.id, store.contract_bindings],
  );
  const conceptRefs = useMemo(
    () => store.concept_refs.filter((conceptRef) => conceptRef.source_id === element.id),
    [element.id, store.concept_refs],
  );
  const relationItems = useMemo(
    () =>
      relations
        .filter((relation) => !isPromotedConceptRelation(element, relation))
        .map((relation) => relationFlowFromSelectedElement(relation, element.id, elementById, resourceById))
        .filter(isDetailRelationItem),
    [element, elementById, relations, resourceById],
  );
  const contractBindingItems = useMemo(
    () =>
      contractBindings.map((contractBinding): DetailContractBindingItem => {
        const target = contractBindingsDisplayTarget(contractBinding, elementById, resourceById);
        return {
          id: contractBinding.id,
          targetId: contractBinding.target,
          kind: contractBinding.target_kind,
          resourceKind: target.resourceKind,
          label: target.label,
          elementType: target.elementType,
          typeFamily: target.typeFamily,
          href: target.href,
          external: target.external,
        };
      }),
    [contractBindings, elementById, resourceById],
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
    if (element.element_type !== "concept" && element.element_type !== "concept-scheme") return null;
    return buildConceptElementDetail({
      element,
      elements: store.elements,
      ontologyNodes: store.ontology.graph_data?.nodes ?? [],
      ontologyEdges: store.ontology.graph_data?.edges ?? [],
      conceptRefs: store.concept_refs,
      elementById,
    });
  }, [element, elementById, store.concept_refs, store.elements, store.ontology.graph_data?.edges, store.ontology.graph_data?.nodes]);

  const openElement = (id: string) => {
    const target = elementById(id);
    if (!target) return;
    window.location.hash = sourceAnchorRoute(target.source_anchor, target.file_path);
  };
  const openResource = (href: string) => {
    window.location.hash = href;
  };

  return (
    <SourcePageElement
      id={elementAnchorId(element)}
      title={element.name}
      elementType={element.element_type}
      typeFamily={element.type_family}
    >
      {conceptDetail ? (
        <ConceptElementDetailContent
          metaBadges={buildMetaBadges(element)}
          definition={
            <MarkdownContent
              markdown={conceptDetail.definition || element.content}
              sourceFilePath={element.file_path}
              sourceAnchor={sourceAnchorRoute(element.source_anchor, element.file_path)}
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
          contract_bindings={contractBindingItems}
          conceptReferences={conceptReferenceItems}
          detailListsDefaultExpanded={false}
          onOpenElement={openElement}
          onOpenConceptReference={(reference) => {
            if (reference.elementId) openElement(reference.elementId);
          }}
          onOpenResource={openResource}
        />
      ) : (
        <ElementDetailContent
          metaBadges={buildMetaBadges(element)}
          content={
            <MarkdownContent
              markdown={stripRenderedDetailSections(element.content)}
              sourceFilePath={element.file_path}
              sourceAnchor={sourceAnchorRoute(element.source_anchor, element.file_path)}
              conceptReferences={conceptReferenceItems}
              onOpenConceptReference={(reference) => {
                if (reference.elementId) openElement(reference.elementId);
              }}
            />
          }
          relations={relationItems}
          contract_bindings={contractBindingItems}
          detailListsDefaultExpanded={false}
          onOpenElement={openElement}
          onOpenResource={openResource}
        />
      )}
    </SourcePageElement>
  );
}

function isString(value: string | undefined): value is string {
  return typeof value === "string" && value.trim().length > 0;
}

function isProjectStoreElement(value: ProjectStoreElement | undefined): value is ProjectStoreElement {
  return Boolean(value);
}

function elementAnchorId(element: ProjectStoreElement) {
  const hashIndex = element.source_anchor.indexOf("#", "#/content/".length);
  if (hashIndex !== -1) {
    return element.source_anchor.slice(hashIndex + 1);
  }
  return element.id;
}

function contentToolbar(filePath: string, label = "Source page") {
  return {
    label,
    title: filePath,
    actionHref: routeForView("model"),
    actionLabel: "Back to model",
  };
}

function splitContentPath(path: string) {
  const fragmentIndex = path.indexOf("#");
  if (fragmentIndex === -1) return [path, null] as const;
  return [path.slice(0, fragmentIndex), path.slice(fragmentIndex + 1) || null] as const;
}
