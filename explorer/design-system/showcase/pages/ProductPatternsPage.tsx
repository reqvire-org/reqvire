import { useState, type ReactNode } from "react";
import {
  AppShell,
  Button,
  CodeBody,
  CodePreviewFrame,
  CodeToolbar,
  CODE_PREVIEW_FALLBACK_CLASS,
  DetailDialog,
  DiagramBlockFrame,
  DocumentPanel,
  ElementDetailContent,
  ElementDetailMissingState,
  FileBrowserFrame,
  FileBrowserList,
  FileBrowserToolbar,
  HelpContent,
  HelpDialog,
  Icon,
  MarkdownFrame,
  OntologyMetadata,
  OntologyNodeDetailContent,
  OntologyNodeDetailDialog,
  PaneChromeHeader,
  PaneFilterSection,
  PaneTree,
  PaneTreeNode,
  RelationList,
  RendererNotice,
  ReqvireRailMark,
  SidePaneFrame,
  StoreNotice,
  TreeItem,
  TypeBadge,
  type DetailReusedContractContextItem,
  type DetailConceptReferenceItem,
  type DetailMetaBadge,
  type DetailRelationItem,
  type ShellActionItem,
  type ShellNavigationItem,
} from "@ds";
import {
  CODE_SAMPLE,
  DETAIL_ATTACHMENTS,
  DETAIL_CONCEPT_REFERENCES,
  DETAIL_META_BADGES,
  DETAIL_RELATIONS,
  FILE_BROWSER_BREADCRUMBS,
  FILE_BROWSER_ITEMS,
  MODEL_TREE_ROWS,
  ONTOLOGY_COMPACT_NODE,
  ONTOLOGY_NODES,
  ONTOLOGY_REQUIREMENT_NODE,
} from "../fixtures/productPatterns";

const PRIMARY_DETAIL_ELEMENT_ID = "REQ-DET-042";

const SHELL_PATTERN_NAVIGATION = [
  {
    value: "workspace",
    icon: "circle",
    label: (
      <>
        <span className="showcase-shell-visually-hidden">Workspace slot</span>
        <span className="showcase-shell-nav-placeholder showcase-shell-nav-placeholder--wide" aria-hidden="true" />
      </>
    ),
  },
  {
    value: "secondary",
    icon: "circle",
    label: (
      <>
        <span className="showcase-shell-visually-hidden">Secondary slot</span>
        <span className="showcase-shell-nav-placeholder" aria-hidden="true" />
      </>
    ),
  },
  {
    value: "tertiary",
    icon: "circle",
    label: (
      <>
        <span className="showcase-shell-visually-hidden">Tertiary slot</span>
        <span className="showcase-shell-nav-placeholder showcase-shell-nav-placeholder--medium" aria-hidden="true" />
      </>
    ),
    badge: <span className="showcase-shell-badge-placeholder" aria-hidden="true" />,
  },
] as const satisfies readonly ShellNavigationItem[];

const SHELL_PATTERN_ACTIONS = [
  { id: "shell-action-a", label: "Shell action A", icon: "circle" },
  { id: "shell-action-b", label: "Shell action B", icon: "circle" },
  { id: "shell-action-c", label: "Shell action C", icon: "circle" },
] as const satisfies readonly Omit<ShellActionItem, "onClick">[];

interface ShowcaseDetailElement {
  id: string;
  title: string;
  elementType: string;
  typeFamily: string;
  sourceHref: string;
  metaBadges: DetailMetaBadge[];
  content: ReactNode;
  relations: DetailRelationItem[];
  reused_contract_context: DetailReusedContractContextItem[];
  conceptReferences: DetailConceptReferenceItem[];
}

const SHOWCASE_DETAIL_ELEMENTS: Record<string, ShowcaseDetailElement> = {
  "REQ-DET-042": {
    id: "REQ-DET-042",
    title: "Traceability Coverage Requirement",
    elementType: "requirement",
    typeFamily: "requirement",
    sourceHref: "#/content/requirements/SystemRequirements.md",
    metaBadges: DETAIL_META_BADGES,
    content: (
      <MarkdownFrame>
        <p>
          The coverage report lists every approved requirement and its verification evidence.
        </p>
      </MarkdownFrame>
    ),
    relations: DETAIL_RELATIONS,
    reused_contract_context: DETAIL_ATTACHMENTS,
    conceptReferences: DETAIL_CONCEPT_REFERENCES,
  },
  "VER-DET-010": {
    id: "VER-DET-010",
    title: "Coverage Export Verification",
    elementType: "test-verification",
    typeFamily: "verification",
    sourceHref: "#/content/requirements/Verifications.md",
    metaBadges: DETAIL_META_BADGES,
    content: (
      <MarkdownFrame>
        <p>
          The verification checks that exported coverage reports include requirements,
          verification links, and uncovered obligations.
        </p>
      </MarkdownFrame>
    ),
    relations: [
      {
        id: "rel-verifies-requirement",
        label: "verify",
        target: {
          id: "REQ-DET-042",
          label: "Traceability Coverage Requirement",
          kind: "element",
          elementType: "requirement",
          typeFamily: "requirement",
          href: "#/model/REQ-DET-042",
          external: false,
        },
      },
      {
        id: "rel-verification-capability",
        label: "derivedFrom",
        target: {
          id: "CAP-DET-001",
          label: "Project traceability capability",
          kind: "element",
          elementType: "capability",
          typeFamily: "capability",
          href: "#/model/CAP-DET-001",
          external: false,
        },
      },
    ],
    reused_contract_context: DETAIL_ATTACHMENTS,
    conceptReferences: DETAIL_CONCEPT_REFERENCES,
  },
  "CAP-DET-001": {
    id: "CAP-DET-001",
    title: "Project traceability capability",
    elementType: "capability",
    typeFamily: "capability",
    sourceHref: "#/content/requirements/Capabilities.md",
    metaBadges: DETAIL_META_BADGES,
    content: (
      <MarkdownFrame>
        <p>
          This capability organizes model traceability from requirements through verification
          evidence and coverage reporting.
        </p>
      </MarkdownFrame>
    ),
    relations: [
      {
        id: "rel-capability-requirement",
        label: "specifiedBy",
        target: {
          id: "REQ-DET-042",
          label: "Traceability Coverage Requirement",
          kind: "element",
          elementType: "requirement",
          typeFamily: "requirement",
          href: "#/model/REQ-DET-042",
          external: false,
        },
      },
    ],
    reused_contract_context: [],
    conceptReferences: DETAIL_CONCEPT_REFERENCES,
  },
};

function Section({ title, desc, children }: { title: string; desc?: string; children: React.ReactNode }) {
  return (
    <section className="showcase-section">
      <div className="showcase-section__heading">
        <div className="showcase-section__title">{title}</div>
        {desc && <div className="showcase-section__desc">{desc}</div>}
      </div>
      {children}
    </section>
  );
}

export function ProductPatternsPage() {
  const [leftPaneOpen, setLeftPaneOpen] = useState(true);
  const [codeExpanded, setCodeExpanded] = useState(true);
  const [codeWrapped, setCodeWrapped] = useState(false);
  const [codeCopied, setCodeCopied] = useState(false);
  const [elementDialogOpen, setElementDialogOpen] = useState(false);
  const [elementDialogStack, setElementDialogStack] = useState<string[]>([]);
  const [ontologyDialogNodeId, setOntologyDialogNodeId] = useState<string | null>(null);
  const [helpDialogOpen, setHelpDialogOpen] = useState(false);
  const [copiedUri, setCopiedUri] = useState(false);

  const currentElementId = elementDialogStack.at(-1) ?? PRIMARY_DETAIL_ELEMENT_ID;
  const currentElement = SHOWCASE_DETAIL_ELEMENTS[currentElementId] ?? SHOWCASE_DETAIL_ELEMENTS[PRIMARY_DETAIL_ELEMENT_ID];
  const previousElementId = elementDialogStack.length > 1
    ? elementDialogStack[elementDialogStack.length - 2]
    : null;
  const previousElement = previousElementId ? SHOWCASE_DETAIL_ELEMENTS[previousElementId] : null;
  const ontologyDialogNode = ONTOLOGY_NODES.find((node) => node.id === ontologyDialogNodeId);

  const inertShellActions = SHELL_PATTERN_ACTIONS.map((action) => ({
    ...action,
    onClick: () => {},
  }));

  const openElementDialog = (id = PRIMARY_DETAIL_ELEMENT_ID) => {
    const nextId = SHOWCASE_DETAIL_ELEMENTS[id] ? id : PRIMARY_DETAIL_ELEMENT_ID;
    setElementDialogStack([nextId]);
    setElementDialogOpen(true);
  };

  const pushElementDialog = (id: string) => {
    if (!SHOWCASE_DETAIL_ELEMENTS[id]) return;
    setElementDialogStack((stack) => {
      const currentId = stack.at(-1);
      return currentId === id ? stack : [...stack, id];
    });
    setElementDialogOpen(true);
  };

  const popElementDialog = () => {
    setElementDialogStack((stack) => (stack.length > 1 ? stack.slice(0, -1) : stack));
  };

  const handleElementDialogOpenChange = (open: boolean) => {
    setElementDialogOpen(open);
    if (!open) setElementDialogStack([]);
  };

  return (
    <div className="showcase-page showcase-page--wide">
      <Section
        title="Explorer Shell"
        desc="App shell, chrome, side pane, workspace area, and detail rail composed with neutral placeholder regions."
      >
        <div className="showcase-shell-demo">
          <AppShell
            brandLabel="REQVIRE"
            navigationItems={[...SHELL_PATTERN_NAVIGATION]}
            activeNavigationValue="workspace"
            headerActions={inertShellActions}
            leftPaneOpen={leftPaneOpen}
            leftPaneWidth={318}
            leftPaneMinWidth={260}
            leftPaneMaxWidth={420}
            onToggleLeftPane={() => setLeftPaneOpen((open) => !open)}
            onLeftPaneResizePointerDown={() => {}}
            onNavigate={() => {}}
            className="showcase-shell-demo__app"
            sidePane={
              <SidePaneFrame
                open={leftPaneOpen}
                chrome="app"
                onToggle={() => setLeftPaneOpen((open) => !open)}
              >
                <ShellSidePaneMock />
              </SidePaneFrame>
            }
            main={<ShellMainMock />}
            detailPane={<ShellDetailRail />}
          />
        </div>
      </Section>

      <Section
        title="File Browser"
        desc="Explorer file/model table product pattern. This is the real table surface used by the model browser."
      >
        <FileBrowserFrame>
          <FileBrowserToolbar
            breadcrumbs={[...FILE_BROWSER_BREADCRUMBS]}
            layout="list"
            resultCount={FILE_BROWSER_ITEMS.length}
            onOpenFolder={() => {}}
            onLayoutChange={() => {}}
          />
          <FileBrowserList
            items={[...FILE_BROWSER_ITEMS]}
            sortKey="name"
            sortDirection="asc"
            onSort={() => {}}
            onOpenFolder={() => {}}
            onOpenFile={() => {}}
          />
        </FileBrowserFrame>
      </Section>

      <Section
        title="Pane Chrome And Trees"
        desc="Standalone chrome pieces and tree rows shown outside the app shell."
      >
        <div className="showcase-pattern-grid showcase-pattern-grid--two">
          <div className="showcase-pattern-card">
            <PaneChromeHeader title="Model" />
            <div className="showcase-pattern-card__body">
              <ReqvireRailMark placement="inline" />
              <span className="showcase-pattern-muted">Rail mark and pane header keep Explorer chrome consistent.</span>
            </div>
          </div>
          <div className="showcase-pattern-card">
            <PaneFilterSection title="File tree">
              <PaneTree>
                {MODEL_TREE_ROWS.map((row) => (
                  <PaneTreeNode key={row.id}>
                    <TreeItem
                      label={row.label}
                      icon={<Icon name={row.icon} size={14} />}
                      count={row.count}
                      depth={row.depth}
                      kind={row.kind}
                      expandable={row.expandable}
                      open={row.open}
                      selected={row.selected}
                    />
                  </PaneTreeNode>
                ))}
              </PaneTree>
            </PaneFilterSection>
          </div>
        </div>
      </Section>

      <Section
        title="Content Renderer Frames"
        desc="Renderer shell patterns use already-rendered mock content instead of app renderers."
      >
        <div className="showcase-pattern-grid showcase-pattern-grid--two">
          <div className="showcase-pattern-card showcase-pattern-card--tall">
            <DocumentPanel
              layout="embedded"
              toolbar={{
                label: "Source page",
                title: "requirements/SystemRequirements.md",
                actionHref: "#open-source",
                actionLabel: "Open source",
              }}
            >
              <MarkdownFrame>
                <h2>Traceability Coverage Requirement</h2>
                <p>
                  Every approved requirement exposes at least one verification relation before
                  the release coverage report is accepted.
                </p>
                <blockquote>Coverage gaps remain visible until an explicit verification target is linked.</blockquote>
              </MarkdownFrame>
              <DiagramBlockFrame>
                <svg viewBox="0 0 620 160" role="img" aria-label="Mock traceability diagram">
                  <rect x="16" y="42" width="160" height="76" rx="8" fill="var(--requirement-tint)" stroke="var(--requirement)" />
                  <text x="96" y="76" textAnchor="middle" fill="var(--text-strong)">Requirement</text>
                  <rect x="232" y="42" width="160" height="76" rx="8" fill="var(--verification-tint)" stroke="var(--verification)" />
                  <text x="312" y="76" textAnchor="middle" fill="var(--text-strong)">Verification</text>
                  <rect x="448" y="42" width="156" height="76" rx="8" fill="var(--resource-tint)" stroke="var(--resource)" />
                  <text x="526" y="76" textAnchor="middle" fill="var(--text-strong)">Evidence</text>
                  <path d="M176 80 H232" stroke="var(--edge-trace)" strokeWidth="3" />
                  <path d="M392 80 H448" stroke="var(--edge-reuse)" strokeWidth="3" />
                </svg>
              </DiagramBlockFrame>
              <RendererNotice title="Renderer note">
                This preview is driven by static showcase content; markdown parsing remains in the app container.
              </RendererNotice>
            </DocumentPanel>
          </div>

          <div className="showcase-pattern-card">
            <CodePreviewFrame wrapped={codeWrapped}>
              <CodeToolbar
                expanded={codeExpanded}
                onToggleExpanded={() => setCodeExpanded((expanded) => !expanded)}
                kind="source"
                path="src/lib/coverage.ts"
                languageLabel="TypeScript"
                lineCount={4}
                relationTypes={["specifiedIn", "verifiedBy"]}
                wrapped={codeWrapped}
                copied={codeCopied}
                onToggleWrap={() => setCodeWrapped((wrapped) => !wrapped)}
                onCopy={() => setCodeCopied(true)}
              />
              <CodeBody collapsed={!codeExpanded} lineCount={4}>
                <pre className={CODE_PREVIEW_FALLBACK_CLASS}>
                  <code>{CODE_SAMPLE}</code>
                </pre>
              </CodeBody>
            </CodePreviewFrame>
            <RendererNotice inline tone="empty">
              Empty-state renderer notices can also appear inline.
            </RendererNotice>
          </div>
        </div>
      </Section>

      <Section
        title="Detail Patterns"
        desc="Element and ontology detail bodies accept DTO-shaped props with inert showcase callbacks."
      >
        <div className="showcase-pattern-grid showcase-pattern-grid--two">
          <div className="showcase-pattern-card">
            <div className="showcase-pattern-toolbar">
              <TypeBadge type="requirement" family="requirement" tinted>
                requirement
              </TypeBadge>
              <Button tone="secondary" size="sm" onClick={() => openElementDialog()}>
                Open dialog
              </Button>
            </div>
            <ElementDetailContent
              metaBadges={DETAIL_META_BADGES}
              content={
                <MarkdownFrame>
                  <p>
                    As a systems engineer, I need release coverage reports to expose every
                    requirement without verification evidence.
                  </p>
                </MarkdownFrame>
              }
              relations={DETAIL_RELATIONS}
              reused_contract_context={DETAIL_ATTACHMENTS}
              conceptReferences={DETAIL_CONCEPT_REFERENCES}
              onOpenElement={openElementDialog}
              onOpenConceptReference={(reference) => {
                if (reference.ontologyNodeId) setOntologyDialogNodeId(reference.ontologyNodeId);
              }}
              onOpenResource={() => {}}
            />
          </div>

          <div className="showcase-pattern-card">
            <div className="showcase-pattern-toolbar">
              <TypeBadge type="requirement" family="requirement" tinted>
                relations
              </TypeBadge>
            </div>
            <RelationList
              relations={DETAIL_RELATIONS}
              onOpenElement={openElementDialog}
              onOpenResource={() => {}}
            />
          </div>

          <div className="showcase-pattern-card">
            <div className="showcase-pattern-toolbar">
              <TypeBadge type="ontology" family="ontology" tinted>
                ontology
              </TypeBadge>
              <Button tone="secondary" size="sm" onClick={() => setOntologyDialogNodeId(ONTOLOGY_REQUIREMENT_NODE.id)}>
                Open dialog
              </Button>
            </div>
            <OntologyNodeDetailContent
              node={ONTOLOGY_COMPACT_NODE}
              nodes={[ONTOLOGY_COMPACT_NODE]}
              copiedUri={copiedUri}
              onCopyUri={() => setCopiedUri(true)}
              onOpenSource={() => {}}
            />
          </div>
        </div>
        <div className="showcase-canvas">
          <OntologyMetadata
            node={ONTOLOGY_REQUIREMENT_NODE}
            copiedUri={copiedUri}
            onCopyUri={() => setCopiedUri(true)}
          />
        </div>
      </Section>

      <Section
        title="Feedback Patterns"
        desc="Help and store-state surfaces rendered from product-pattern exports."
      >
        <div className="showcase-pattern-grid showcase-pattern-grid--two">
          <div className="showcase-pattern-card">
            <div className="showcase-pattern-toolbar">
              <span className="showcase-label">Help content</span>
              <Button tone="secondary" size="sm" onClick={() => setHelpDialogOpen(true)}>
                Open help
              </Button>
            </div>
            <HelpContent />
          </div>
          <div className="showcase-pattern-card showcase-pattern-card--notice">
            <StoreNotice
              reason="No Project Store seed was found for this showcase scenario."
              detail="assets/project-store.js"
            />
          </div>
          <div className="showcase-pattern-card showcase-pattern-card--notice">
            <ElementDetailMissingState identifier="REQ-MISSING-404" />
          </div>
        </div>
      </Section>

      <DetailDialog
        open={elementDialogOpen}
        onOpenChange={handleElementDialogOpenChange}
        title={currentElement.title}
        typeFamily={currentElement.typeFamily}
        elementType={currentElement.elementType}
        sourceHref={currentElement.sourceHref}
        backLabel={previousElement ? `Back to ${previousElement.title}` : undefined}
        onNavigateBack={previousElement ? popElementDialog : undefined}
        onOpenSource={() => {}}
      >
        <ElementDetailContent
          metaBadges={currentElement.metaBadges}
          content={currentElement.content}
          relations={currentElement.relations}
          reused_contract_context={currentElement.reused_contract_context}
          conceptReferences={currentElement.conceptReferences}
          onOpenElement={pushElementDialog}
          onOpenConceptReference={(reference) => {
            if (reference.ontologyNodeId) setOntologyDialogNodeId(reference.ontologyNodeId);
          }}
          onOpenResource={() => {}}
        />
      </DetailDialog>

      <OntologyNodeDetailDialog
        open={ontologyDialogNodeId !== null}
        onOpenChange={(open) => {
          if (!open) setOntologyDialogNodeId(null);
        }}
        node={ontologyDialogNode}
        nodes={ONTOLOGY_NODES}
        missingNodeId={ontologyDialogNodeId}
        copiedUri={copiedUri}
        onCopyUri={() => setCopiedUri(true)}
        onOpenSource={() => {}}
      />

      <HelpDialog open={helpDialogOpen} onOpenChange={setHelpDialogOpen} />
    </div>
  );
}

function ShellSidePaneMock() {
  return (
    <div className="showcase-shell-pane-mock" aria-label="Placeholder side pane content">
      <div className="showcase-shell-placeholder showcase-shell-placeholder--search" />
      <div className="showcase-shell-placeholder-group">
        <div className="showcase-shell-placeholder showcase-shell-placeholder--heading" />
        <div className="showcase-shell-placeholder showcase-shell-placeholder--row" />
        <div className="showcase-shell-placeholder showcase-shell-placeholder--row showcase-shell-placeholder--short" />
      </div>
      <div className="showcase-shell-placeholder-group">
        <div className="showcase-shell-placeholder showcase-shell-placeholder--heading" />
        <div className="showcase-shell-placeholder showcase-shell-placeholder--tile" />
        <div className="showcase-shell-placeholder showcase-shell-placeholder--tile" />
        <div className="showcase-shell-placeholder showcase-shell-placeholder--tile showcase-shell-placeholder--short" />
      </div>
    </div>
  );
}

function ShellMainMock() {
  return (
    <div className="showcase-shell-main-mock" aria-label="Placeholder workspace content">
      <div className="showcase-shell-workspace-header">
        <div className="showcase-shell-placeholder showcase-shell-placeholder--icon" />
        <div className="showcase-shell-placeholder-stack">
          <div className="showcase-shell-placeholder showcase-shell-placeholder--title" />
          <div className="showcase-shell-placeholder showcase-shell-placeholder--subtitle" />
        </div>
        <div className="showcase-shell-placeholder showcase-shell-placeholder--action" />
      </div>
      <div className="showcase-shell-stat-grid" aria-hidden="true">
        <div className="showcase-shell-placeholder showcase-shell-placeholder--stat" />
        <div className="showcase-shell-placeholder showcase-shell-placeholder--stat" />
        <div className="showcase-shell-placeholder showcase-shell-placeholder--stat" />
      </div>
      <div className="showcase-shell-board" aria-hidden="true">
        <div className="showcase-shell-placeholder showcase-shell-placeholder--panel" />
        <div className="showcase-shell-placeholder showcase-shell-placeholder--panel showcase-shell-placeholder--panel-accent" />
      </div>
    </div>
  );
}

function ShellDetailRail() {
  return (
    <aside className="showcase-shell-detail-rail" aria-label="Placeholder detail rail">
      <div className="showcase-shell-placeholder showcase-shell-placeholder--title" />
      <div className="showcase-shell-placeholder showcase-shell-placeholder--row" />
      <div className="showcase-shell-placeholder showcase-shell-placeholder--row showcase-shell-placeholder--short" />
      <div className="showcase-shell-placeholder-group">
        <div className="showcase-shell-placeholder showcase-shell-placeholder--tile" />
        <div className="showcase-shell-placeholder showcase-shell-placeholder--tile" />
        <div className="showcase-shell-placeholder showcase-shell-placeholder--tile showcase-shell-placeholder--short" />
      </div>
    </aside>
  );
}
