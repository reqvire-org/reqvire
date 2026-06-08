import { useMemo, type MouseEvent } from "react";
import {
  ElementIcon,
  Icon,
  Modal,
  ModalBody,
  ModalClose,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalTitle,
  RelationPill,
  TypeBadge,
} from "@ds";
import { useStore } from "../store/StoreContext";
import type { ProjectStoreElement, ProjectStoreRelation, ProjectStoreResource } from "../store/types";
import { routeForContent, routeForElement } from "../router/routes";
import { MarkdownContent } from "./MarkdownContent";

/*
 * Element-detail modal.
 *
 * Renders as an in-shell scrollable dialog over the active Explorer view,
 * backed by Project Store element records. Shows name, type, source file/anchor,
 * metadata, governance, content, relations, attachments, and concept references.
 * Provides a SECONDARY source-page action (the exported source anchor); that
 * action is not the primary navigation target. Closing returns to the
 * underlying route (handled by the caller via onClose).
 */
type MetaBadge = { key: string; value: string; provenance: "explicit" | "inherited" };

/* One flat badge row replaces the old governance pills. Authored metadata is
   explicit; inherited governance keeps only the value here and carries
   provenance in the badge. The `type` key is skipped — the header type badge
   already states it. */
function buildMetaBadges(element: {
  element_type: string;
  metadata: Record<string, string>;
  governance: Record<string, string>;
}): MetaBadge[] {
  const badges: MetaBadge[] = [];
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

function cleanGovernanceValue(raw: string): Pick<MetaBadge, "value" | "provenance"> {
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

export function ElementDetailModal({
  identifier,
  onClose,
  onOpenElement,
}: {
  identifier: string | null;
  onClose: () => void;
  onOpenElement: (id: string) => void;
}) {
  const { store, elementById } = useStore();
  const element = identifier ? elementById(identifier) : undefined;
  const resourceById = useMemo(
    () => new Map(store.resources.map((resource) => [resource.id, resource])),
    [store.resources],
  );

  const { relations, attachments, conceptRefs } = useMemo(() => {
    if (!identifier) {
      return { relations: [], attachments: [], conceptRefs: [] };
    }
    return {
      relations: store.relations.filter((r) => r.source_id === identifier || r.target_id === identifier),
      attachments: store.attachments.filter((a) => a.source_id === identifier),
      conceptRefs: store.concept_refs.filter((c) => c.source_id === identifier),
    };
  }, [identifier, store]);

  const open = identifier !== null;
  const metaBadges = element ? buildMetaBadges(element) : [];

  return (
    <Modal open={open} onOpenChange={(v) => !v && onClose()}>
      <ModalContent className="element-detail-dialog" showCloseButton={false}>
        {!element ? (
          <>
            <ModalHeader className="element-detail-header">
              <ModalTitle>Element not found</ModalTitle>
            </ModalHeader>
            <ModalBody className="element-detail-body">
            <p className="element-detail-muted">
              No Project Store element matches{" "}
              <code className="rq-coderef">{identifier ?? ""}</code>.
            </p>
            </ModalBody>
          </>
        ) : (
          <>
            <ModalHeader className="element-detail-header">
              <div className="element-detail-title-row">
                <TypeBadge type={element.type_family} family={element.type_family} tinted className="element-detail-family-badge">
                  {element.type_family}
                </TypeBadge>
                {element.element_type !== element.type_family ? (
                  <TypeBadge type={element.element_type} family={element.type_family} tinted>
                    {element.element_type}
                  </TypeBadge>
                ) : null}
                <ModalTitle>{element.name}</ModalTitle>
                <ModalClose asChild>
                  <button type="button" className="rq-iconbtn rq-iconbtn--ghost element-detail-close" aria-label="Close">
                    <Icon name="x" />
                  </button>
                </ModalClose>
              </div>
            </ModalHeader>

            <ModalBody className="element-detail-body">
              <div className="element-detail-content-flow">
                {metaBadges.length > 0 && (
                  <div className="ex-gov">
                    {metaBadges.map(({ key, value, provenance }) => (
                      <span className="ex-gov__item" key={`meta-${key}`}>
                        <span className="ex-gov__k">{key}</span>
                        <span className="ex-gov__v">{value}</span>
                        <span className="ex-explicit">{provenance}</span>
                      </span>
                    ))}
                  </div>
                )}

                <div className="element-detail-content-flow">
                  <Section title="Content">
                    <MarkdownContent
                      markdown={element.content}
                      sourceFilePath={element.file_path}
                      sourceAnchor={element.source_anchor}
                    />
                  </Section>

                  <RelationList
                    title="Relations"
                    relations={relations
                      .map((r) => relationFlowFromSelectedElement(r, element.id, elementById, resourceById))
                      .filter(isRelationFlow)}
                    onOpenElement={onOpenElement}
                  />

                  {attachments.length > 0 && (
                    <Section title="Attachments">
                      <div className="ex-rels">
                        {attachments.map((a) => (
                          <AttachmentTarget
                            key={a.id}
                            attachment={a}
                            target={attachmentDisplayTarget(a, elementById, resourceById)}
                            onOpenElement={onOpenElement}
                          />
                        ))}
                      </div>
                    </Section>
                  )}

                  {conceptRefs.length > 0 && (
                    <Section title="Concept references">
                      <div className="ex-rels">
                        {conceptRefs.map((c) => (
                          <div key={c.id} className="element-detail-relation-row">
                            <span className="element-detail-relation-text">{c.label}</span>
                            <code className="rq-coderef">{c.iri}</code>
                          </div>
                        ))}
                      </div>
                    </Section>
                  )}
                </div>
              </div>
            </ModalBody>

            <ModalFooter className="element-detail-footer">
              <div className="element-detail-footer-row">
                <a
                  href={sourceAnchorRoute(element.source_anchor, element.file_path)}
                  className="element-detail-source-link"
                  onClick={(event) => {
                    event.preventDefault();
                    window.location.hash = sourceAnchorRoute(element.source_anchor, element.file_path);
                  }}
                >
                  <Icon name="external-link" className="ex-icon-sm" /> Open source page
                </a>
                <ModalClose asChild>
                  <button type="button" className="rq-btn rq-btn--primary rq-btn--sm">
                    Close
                  </button>
                </ModalClose>
              </div>
            </ModalFooter>
          </>
        )}
      </ModalContent>
    </Modal>
  );
}

function sourceAnchorRoute(sourceAnchor: string, filePath: string): string {
  if (sourceAnchor.startsWith("#/content/")) return sourceAnchor;
  if (sourceAnchor.startsWith("#")) return `${routeForContent(filePath)}${sourceAnchor}`;

  const hashIndex = sourceAnchor.indexOf("#");
  const path = hashIndex === -1 ? sourceAnchor : sourceAnchor.slice(0, hashIndex);
  const fragment = hashIndex === -1 ? "" : sourceAnchor.slice(hashIndex);
  const markdownPath = path.endsWith(".html")
    ? `${path.slice(0, -".html".length)}.md`
    : path;
  return `${routeForContent(markdownPath || filePath)}${fragment}`;
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <section className="element-detail-section">
      <h3>{title}</h3>
      {children}
    </section>
  );
}

function RelationList({
  title,
  relations,
  onOpenElement,
}: {
  title: string;
  relations: RelationFlow[];
  onOpenElement: (id: string) => void;
}) {
  if (relations.length === 0) return null;
  return (
    <Section title={title}>
      <div className="element-detail-relation-list">
        {relations.map((r, i) => (
          <div key={`${r.label}-${r.target.id}-${i}`} className="element-detail-relation-row">
            <span className="element-detail-relation-kind">
              {r.label}
            </span>
            <RelationEndpoint endpoint={r.target} onOpenElement={onOpenElement} />
          </div>
        ))}
      </div>
    </Section>
  );
}

type RelationEndpoint = {
  id: string;
  label: string;
  kind: string;
  elementType?: string;
  typeFamily?: string;
  href: string | null;
  external: boolean;
};

type RelationFlow = {
  label: string;
  target: RelationEndpoint;
};

function isRelationFlow(value: RelationFlow | null): value is RelationFlow {
  return value !== null;
}

function relationFlowFromSelectedElement(
  relation: ProjectStoreRelation,
  selectedId: string,
  elementById: (id: string) => ProjectStoreElement | undefined,
  resourceById: Map<string, ProjectStoreResource>,
): RelationFlow | null {
  if (relation.source_id === selectedId) {
    return {
      label: relation.canonical_relation_type,
      target: relationTargetEndpoint(relation, elementById, resourceById),
    };
  }
  if (relation.target_id === selectedId) {
    return {
      label: selectedTargetRelationLabel(relation),
      target: relationSourceEndpoint(relation, elementById),
    };
  }
  return null;
}

function selectedTargetRelationLabel(relation: ProjectStoreRelation): string {
  return (
    relation.source_relation_types.find((type) => type !== relation.canonical_relation_type)
    ?? relation.relation_type
    ?? relation.canonical_relation_type
  );
}

function relationSourceEndpoint(
  relation: ProjectStoreRelation,
  elementById: (id: string) => ProjectStoreElement | undefined,
): RelationEndpoint {
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
): RelationEndpoint {
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

function RelationEndpoint({
  endpoint,
  onOpenElement,
}: {
  endpoint: RelationEndpoint;
  onOpenElement: (id: string) => void;
}) {
  const className = "element-detail-relation-endpoint";
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
        <span className="rq-relation__pip" style={{ background: relationPipColor(endpoint.kind) }} />
      )}
      <span className="element-detail-relation-endpoint-label">{endpoint.label}</span>
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

function AttachmentTarget({
  attachment,
  target,
  onOpenElement,
}: {
  attachment: { target: string; target_kind: string; resource_id: string | null };
  target: { label: string; href: string | null; external: boolean };
  onOpenElement: (id: string) => void;
}) {
  if (attachment.target_kind === "element") {
    return (
      <RelationPill
        kind={attachment.target_kind}
        label={target.label}
        pipColor={relationPipColor(attachment.target_kind)}
        href={routeForElement(attachment.target)}
        title={attachment.target}
        onClick={(event: MouseEvent<HTMLAnchorElement>) => {
          event.preventDefault();
          onOpenElement(attachment.target);
        }}
      />
    );
  }

  if (target.href) {
    return (
      <RelationPill
        kind={attachment.target_kind}
        label={target.label}
        pipColor={relationPipColor(attachment.target_kind)}
        href={target.href}
        title={attachment.target}
        {...(target.external ? { target: "_blank", rel: "noreferrer" } : {})}
      />
    );
  }

  return <RelationPill kind={attachment.target_kind} label={target.label} title={attachment.target} pipColor={relationPipColor(attachment.target_kind)} disabled />;
}

function relationPipColor(kind: string) {
  const normalized = kind.toLowerCase();
  if (normalized.includes("verif") || normalized.includes("satisf")) return "var(--verification)";
  if (normalized.includes("attach")) return "var(--resource)";
  if (normalized.includes("derive")) return "var(--requirement)";
  if (normalized.includes("specif") || normalized.includes("refin")) return "var(--refinement)";
  return "var(--edge-default)";
}

function attachmentDisplayTarget(
  attachment: { target: string; target_kind: string; resource_id: string | null },
  elementById: (id: string) => { name: string } | undefined,
  resourceById: Map<string, { display: string; target: string; file_path: string | null; external_url: string | null }>,
): { label: string; href: string | null; external: boolean } {
  if (attachment.target_kind === "element") {
    return {
      label: elementById(attachment.target)?.name ?? attachment.target,
      href: routeForElement(attachment.target),
      external: false,
    };
  }
  if (attachment.resource_id) {
    const resource = resourceById.get(attachment.resource_id);
    if (resource) {
      if (resource.external_url) {
        return { label: resource.display || resource.target, href: resource.external_url, external: true };
      }
      if (resource.file_path) {
        return { label: resource.display || resource.target, href: routeForContent(resource.file_path), external: false };
      }
      return { label: resource.display || resource.target, href: null, external: false };
    }
  }
  return { label: attachment.target, href: null, external: false };
}
