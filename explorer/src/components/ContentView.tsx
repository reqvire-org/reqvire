import { DocumentPanel, RendererNotice, type DetailConceptReferenceItem } from "@ds";
import { useStore } from "../store/StoreContext";
import { MarkdownContent, stripConceptReferencesSection } from "./MarkdownContent";
import { routeForElement, routeForView } from "../router/routes";
import { SourceCodePreview } from "./SourceCodePreview";

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

  const conceptReferences = conceptReferencesForFile(file.element_ids, store, elementById);

  return (
    <DocumentPanel toolbar={contentToolbar(file.path)}>
      <MarkdownContent
        markdown={stripConceptReferencesSection(file.markdown_content)}
        sourceFilePath={file.path}
        sourceAnchor={fragmentId ? `#/content/${file.path}#${fragmentId}` : `#/content/${file.path}`}
        scrollToAnchor={fragmentId}
        conceptReferences={conceptReferences}
        onOpenConceptReference={(reference) => {
          if (reference.elementId) {
            window.location.hash = routeForElement(reference.elementId);
          }
        }}
      />
    </DocumentPanel>
  );
}

function conceptReferencesForFile(
  elementIds: readonly string[],
  store: ReturnType<typeof useStore>["store"],
  elementById: ReturnType<typeof useStore>["elementById"],
): DetailConceptReferenceItem[] {
  const sourceIds = new Set(elementIds);
  return store.concept_refs
    .filter((conceptRef) => sourceIds.has(conceptRef.source_id))
    .map((conceptRef): DetailConceptReferenceItem => {
      const nativeConcept = store.thesaurus.concepts.find(
        (concept) => concept.element_id === conceptRef.target_element_id,
      );
      const sourceElement = elementById(conceptRef.source_id);
      return {
        id: `${sourceElement?.id ?? conceptRef.source_id}:${conceptRef.id}`,
        label: conceptRef.label,
        iri: conceptRef.iri,
        elementId: conceptRef.target_element_id,
        matchLabels: nativeConcept
          ? [nativeConcept.label, ...nativeConcept.alt_labels, conceptRef.label].filter(isString)
          : [conceptRef.label].filter(isString),
        ontologyLabel: nativeConcept?.label,
      };
    });
}

function isString(value: string | undefined): value is string {
  return typeof value === "string" && value.trim().length > 0;
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
