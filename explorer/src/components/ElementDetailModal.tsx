import { useMemo, type CSSProperties, type MouseEvent } from "react";
import { css, cx } from "@linaria/atomic";
import {
  Button,
  CodeRef,
  ElementIcon,
  Icon,
  IconButton,
  Modal,
  ModalBody,
  ModalClose,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalTitle,
  RelationPill,
  TypeBadge,
  type DesignSystemColorToken,
} from "@ds";
import { useStore } from "../store/StoreContext";
import type { ProjectStoreElement, ProjectStoreRelation, ProjectStoreResource } from "../store/types";
import { routeForContent, routeForElement } from "../router/routes";
import { MarkdownContent } from "./MarkdownContent";

const detailDialogBaseUX = css`
  --ex-detail-dialog-w: 1120px;
  --ex-detail-dialog-max-h: 980px;
  --ex-detail-dialog-body-max-h: 780px;
  --ex-detail-dialog-chrome-h: 176px;
  --ex-detail-chip-link-max-w: 520px;
  width: min(var(--ex-detail-dialog-w), calc(100vw - var(--space-24)));
  max-width: min(var(--ex-detail-dialog-w), calc(100vw - var(--space-24)));
  max-height: min(92vh, var(--ex-detail-dialog-max-h));

  @media (max-width: 720px) {
    width: calc(100vw - var(--space-10));
    max-width: calc(100vw - var(--space-10));
    max-height: calc(100vh - var(--space-10));
  }
`;

const detailDialogSkinX = css`
  border: var(--border-w) solid var(--border-default);
  border-radius: var(--radius-xl);
  background: var(--bg-overlay);
  color: var(--text-body);
  box-shadow: var(--shadow-xl);

  .ex-markdown pre,
  .source-code-preview-body {
    border: var(--border-w) solid var(--border-subtle);
    background: var(--bg-sunken);
  }

  .ex-markdown code {
    background: var(--bg-sunken);
  }

  .ex-markdown h1,
  .ex-markdown h2,
  .ex-markdown h3,
  .ex-markdown h4 {
    margin: 0.85em 0 0.4em;
    color: var(--text-body);
    font-size: var(--text-sm);
    font-weight: var(--weight-bold);
    letter-spacing: 0.01em;
    line-height: 1.35;
  }
`;

const detailHeaderBaseUX = css`
  padding: var(--space-10) var(--space-10) var(--space-10) var(--space-14);

  @media (max-width: 720px) {
    padding: var(--space-8) var(--space-24) var(--space-8) var(--space-8);
  }
`;

const detailHeaderSkinX = css`
  border-bottom: var(--border-w) solid var(--border-subtle);
  background: var(--bg-overlay);
`;

const detailBodyBaseUX = css`
  max-height: min(74vh, var(--ex-detail-dialog-body-max-h));
  overflow: auto;
  padding: var(--space-14) var(--space-16);

  @media (max-width: 720px) {
    max-height: calc(100vh - var(--ex-detail-dialog-chrome-h));
    padding: var(--space-8);
  }
`;

const detailBodySkinX = css`
  background: var(--bg-surface);
`;

const detailTitleRowUX = css`
  display: flex;
  width: 100%;
  min-width: 0;
  align-items: center;
  gap: var(--space-8);

  h2 {
    flex: 1 1 auto;
    min-width: 0;
  }
`;

const detailFamilyBadgeUX = css`
  flex: 0 0 auto;
`;

const detailCloseUX = css`
  flex: 0 0 auto;
  margin-left: auto;
  margin-right: calc(-1 * var(--space-3));

  svg {
    width: var(--icon-sm);
    height: var(--icon-sm);
  }
`;

const detailContentFlowUX = css`
  display: flex;
  flex-direction: column;
  gap: var(--space-14);
  max-width: 78ch;
`;

const detailSectionUX = css`
  display: grid;
  gap: var(--space-4);

  h3 {
    margin: 0;
    color: var(--text-strong);
    font-size: var(--text-md);
    font-weight: var(--weight-semibold);
    letter-spacing: 0;
    line-height: var(--leading-tight);
  }
`;

const detailMutedUX = css`
  margin: 0;
  color: var(--text-muted);
  font-size: var(--text-sm);
`;

const governanceListUX = css`
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-5);
`;

const governanceItemBaseUX = css`
  display: inline-flex;
  align-items: baseline;
  gap: var(--space-5);
  padding: var(--space-4) var(--space-8);
  font-size: var(--text-sm);
`;

const governanceItemSkinX = css`
  border-radius: var(--radius-md);
  background: var(--bg-sunken);
`;

const governanceKeySkinX = css`
  color: var(--text-muted);
`;

const governanceValueSkinX = css`
  color: var(--text-strong);
  font-weight: var(--weight-medium);
`;

const explicitBadgeSkinX = css`
  border: var(--border-w) solid var(--border-default);
  border-radius: var(--radius-pill);
  background: var(--bg-surface);
  padding: 0 var(--space-3);
  color: var(--text-muted);
  font-size: var(--text-micro);
`;

const relationStackUX = css`
  display: flex;
  flex-direction: column;
  gap: var(--space-6);
`;

const relationListUX = css`
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
`;

const relationRowBaseUX = css`
  display: grid;
  grid-template-columns: minmax(7.5rem, max-content) minmax(0, 1fr);
  align-items: start;
  column-gap: var(--space-5);
  min-width: 0;
  padding: var(--space-2) 0;
`;

const relationRowSkinX = css`
  border-bottom: var(--border-w) solid color-mix(in srgb, var(--border-subtle) 55%, transparent);

  &:last-child {
    border-bottom: 0;
  }
`;

const relationTextSkinX = css`
  color: var(--text-body);
  font-size: var(--text-sm);
`;

const relationKindBaseUX = css`
  display: inline-flex;
  align-items: center;
  justify-self: start;
  gap: var(--space-3);
  padding: var(--space-1) var(--space-5);
  font-family: var(--font-mono);
  font-size: var(--text-caption);
  line-height: 1.45;
`;

const relationKindSkinX = css`
  border-radius: var(--radius-pill);
  background: var(--bg-sunken);
  color: var(--text-muted);
`;

const relationEndpointBaseUX = css`
  display: inline-flex;
  justify-self: start;
  min-width: 0;
  max-width: min(100%, var(--ex-detail-chip-link-max-w));
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-1) var(--space-5);
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  line-height: 1.45;
  text-decoration: none;
  overflow-wrap: anywhere;
`;

const relationEndpointSkinX = css`
  border: var(--border-w) solid var(--border-subtle);
  border-radius: var(--radius-sm);
  background: var(--bg-surface);
  color: var(--text-strong);

  &:hover {
    border-color: var(--border-strong);
    background: var(--bg-hover);
    text-decoration: none;
  }
`;

const relationEndpointLabelUX = css`
  min-width: 0;
  overflow-wrap: anywhere;
`;

const relationEndpointPipUX = css`
  display: inline-block;
  flex-shrink: 0;
  width: var(--space-3);
  height: var(--space-3);
  border-radius: var(--radius-xs);
  background: var(--ex-relation-endpoint-pip);
`;

const detailFooterBaseUX = css`
  padding: var(--space-7) var(--space-16);

  @media (max-width: 720px) {
    padding: var(--space-6) var(--space-8);
  }
`;

const detailFooterSkinX = css`
  border-top: var(--border-w) solid var(--border-subtle);
  background: var(--bg-overlay);
`;

const detailFooterRowUX = css`
  display: flex;
  width: 100%;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-8);
`;

const sourceLinkBaseUX = css`
  display: inline-flex;
  align-items: center;
  gap: var(--space-3);
  min-width: 0;
  font-size: var(--text-sm);
  font-weight: var(--weight-semibold);
  text-decoration: none;
  overflow-wrap: anywhere;
`;

const sourceLinkSkinX = css`
  color: var(--accent);

  &:hover {
    text-decoration: underline;
    text-underline-offset: var(--space-1);
  }
`;

const iconSmUX = css`
  width: var(--space-8);
  height: var(--space-8);
  flex: none;
`;

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
      <ModalContent className={cx(detailDialogBaseUX, detailDialogSkinX)} showCloseButton={false}>
        {!element ? (
          <>
            <ModalHeader className={cx(detailHeaderBaseUX, detailHeaderSkinX)}>
              <ModalTitle>Element not found</ModalTitle>
            </ModalHeader>
            <ModalBody className={cx(detailBodyBaseUX, detailBodySkinX)}>
            <p className={cx(detailMutedUX)}>
              No Project Store element matches{" "}
              <CodeRef>{identifier ?? ""}</CodeRef>.
            </p>
            </ModalBody>
          </>
        ) : (
          <>
            <ModalHeader className={cx(detailHeaderBaseUX, detailHeaderSkinX)}>
              <div className={cx(detailTitleRowUX)}>
                <TypeBadge type={element.type_family} family={element.type_family} tinted className={cx(detailFamilyBadgeUX)}>
                  {element.type_family}
                </TypeBadge>
                {element.element_type !== element.type_family ? (
                  <TypeBadge type={element.element_type} family={element.type_family} tinted>
                    {element.element_type}
                  </TypeBadge>
                ) : null}
                <ModalTitle>{element.name}</ModalTitle>
                <ModalClose asChild>
                  <IconButton tone="ghost" className={cx(detailCloseUX)} aria-label="Close">
                    <Icon name="x" />
                  </IconButton>
                </ModalClose>
              </div>
            </ModalHeader>

            <ModalBody className={cx(detailBodyBaseUX, detailBodySkinX)}>
              <div className={cx(detailContentFlowUX)}>
                {metaBadges.length > 0 && (
                  <div className={cx(governanceListUX)}>
                    {metaBadges.map(({ key, value, provenance }) => (
                      <span className={cx(governanceItemBaseUX, governanceItemSkinX)} key={`meta-${key}`}>
                        <span className={cx(governanceKeySkinX)}>{key}</span>
                        <span className={cx(governanceValueSkinX)}>{value}</span>
                        <span className={cx(explicitBadgeSkinX)}>{provenance}</span>
                      </span>
                    ))}
                  </div>
                )}

                <div className={cx(detailContentFlowUX)}>
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
                      <div className={cx(relationStackUX)}>
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
                      <div className={cx(relationStackUX)}>
                        {conceptRefs.map((c) => (
                          <div key={c.id} className={cx(relationRowBaseUX, relationRowSkinX)}>
                            <span className={cx(relationTextSkinX)}>{c.label}</span>
                            <CodeRef>{c.iri}</CodeRef>
                          </div>
                        ))}
                      </div>
                    </Section>
                  )}
                </div>
              </div>
            </ModalBody>

            <ModalFooter className={cx(detailFooterBaseUX, detailFooterSkinX)}>
              <div className={cx(detailFooterRowUX)}>
                <a
                  href={sourceAnchorRoute(element.source_anchor, element.file_path)}
                  className={cx(sourceLinkBaseUX, sourceLinkSkinX)}
                  onClick={(event) => {
                    event.preventDefault();
                    window.location.hash = sourceAnchorRoute(element.source_anchor, element.file_path);
                  }}
                >
                  <Icon name="external-link" className={cx(iconSmUX)} /> Open source page
                </a>
                <ModalClose asChild>
                  <Button tone="primary" size="sm">Close</Button>
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
    <section className={cx(detailSectionUX)}>
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
      <div className={cx(relationListUX)}>
        {relations.map((r, i) => (
          <div key={`${r.label}-${r.target.id}-${i}`} className={cx(relationRowBaseUX, relationRowSkinX)}>
            <span className={cx(relationKindBaseUX, relationKindSkinX)}>
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
        <span
          className={cx("ex-relation-endpoint-pip", relationEndpointPipUX)}
          style={{ "--ex-relation-endpoint-pip": `var(${relationPipColorToken(endpoint.kind)})` } as CSSProperties}
        />
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
        pipColorToken={relationPipColorToken(attachment.target_kind)}
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
        pipColorToken={relationPipColorToken(attachment.target_kind)}
        href={target.href}
        title={attachment.target}
        {...(target.external ? { target: "_blank", rel: "noreferrer" } : {})}
      />
    );
  }

  return <RelationPill kind={attachment.target_kind} label={target.label} title={attachment.target} pipColorToken={relationPipColorToken(attachment.target_kind)} disabled />;
}

function relationPipColorToken(kind: string): DesignSystemColorToken {
  const normalized = kind.toLowerCase();
  if (normalized.includes("verif") || normalized.includes("satisf")) return "--verification";
  if (normalized.includes("attach")) return "--resource";
  if (normalized.includes("derive")) return "--requirement";
  if (normalized.includes("specif") || normalized.includes("refin")) return "--refinement";
  return "--edge-default";
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
