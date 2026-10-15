import { ShowcaseRegion } from "@ds";
import { COVERAGE_PATTERN_ITEMS } from "../fixtures/coverage";
import { useEffect, useState } from "preact/hooks";
import { type ComponentChildren } from "preact";
import {
  AppShell,
  Button,
  CodeBody,
  CodePreviewFrame,
  CodeToolbar,
  CoverageDashboard,
  CoverageDrilldown,
  CoveragePanel,
  CodeFallback,
  ResourceDetailCard, ResourceEmptyState, ResourcePageHeader, ResourceList, InlineConceptReference,
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
  PaneTree,
  PaneTreeNode,
  RelationList,
  RendererNotice,
  ReqvireRailMark,
  SidePaneFrame,
  StoreNotice,
  TreeItem,
  TypeBadge,
  WorktreeSelector,
  WorktreeLoadDialog,
  type DetailContractBindingItem,
  type DetailMetaBadge,
  type DetailRelationItem,
  type ShellActionItem,
  type ShellNavigationItem,
} from "@ds";
import {
  CODE_SAMPLE,
  DETAIL_ATTACHMENTS,
  DETAIL_META_BADGES,
  DETAIL_RELATIONS,
  FILE_BROWSER_BREADCRUMBS,
  FILE_BROWSER_ITEMS,
  MODEL_TREE_ROWS,
  ONTOLOGY_COMPACT_NODE,
  ONTOLOGY_NODES,
  ONTOLOGY_REQUIREMENT_NODE,
  SHOWCASE_WORKTREES,
  SHOWCASE_WORKTREE_DIALOGS,
} from "../fixtures/productPatterns";

const PRIMARY_DETAIL_ELEMENT_ID = "REQ-DET-042";

const SHELL_PATTERN_NAVIGATION = [
  {
    value: "workspace",
    icon: "circle",
    label: (
      <>
        <ShowcaseRegion regions={["shell-visually-hidden"]} as="span">Workspace slot</ShowcaseRegion>
        <ShowcaseRegion regions={["shell-nav-placeholder", "shell-nav-placeholder--wide"]} as="span" aria-hidden="true" />
      </>
    ),
  },
  {
    value: "secondary",
    icon: "circle",
    label: (
      <>
        <ShowcaseRegion regions={["shell-visually-hidden"]} as="span">Secondary slot</ShowcaseRegion>
        <ShowcaseRegion regions={["shell-nav-placeholder"]} as="span" aria-hidden="true" />
      </>
    ),
  },
  {
    value: "tertiary",
    icon: "circle",
    label: (
      <>
        <ShowcaseRegion regions={["shell-visually-hidden"]} as="span">Tertiary slot</ShowcaseRegion>
        <ShowcaseRegion regions={["shell-nav-placeholder", "shell-nav-placeholder--medium"]} as="span" aria-hidden="true" />
      </>
    ),
    badge: <ShowcaseRegion regions={["shell-badge-placeholder"]} as="span" aria-hidden="true" />,
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
  content: ComponentChildren;
  relations: DetailRelationItem[];
  contract_bindings: DetailContractBindingItem[];
}

const SHOWCASE_DETAIL_ELEMENTS: Record<string, ShowcaseDetailElement> = {
  "SPEC-DET-001": {
    id: "SPEC-DET-001", title: "Coverage Report Specification", elementType: "specification",
    typeFamily: "contract", sourceHref: "#/content/system-model/Specifications.md",
    metaBadges: [], content: <p>Coverage reports distinguish verification and implementation evidence.</p>,
    relations: [], contract_bindings: [],
  },
  "REQ-DET-042": {
    id: "REQ-DET-042",
    title: "Traceability Coverage Requirement",
    elementType: "requirement",
    typeFamily: "requirement",
    sourceHref: "#/content/system-model/SystemRequirements.md",
    metaBadges: DETAIL_META_BADGES,
    content: (
      <MarkdownFrame>
        <p>
          The coverage report lists every approved requirement and its verification evidence.
        </p>
      </MarkdownFrame>
    ),
    relations: DETAIL_RELATIONS,
    contract_bindings: DETAIL_ATTACHMENTS,
  },
  "VER-DET-010": {
    id: "VER-DET-010",
    title: "Coverage Export Verification",
    elementType: "test-verification",
    typeFamily: "verification",
    sourceHref: "#/content/system-model/Verifications.md",
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
    contract_bindings: DETAIL_ATTACHMENTS,
  },
  "CAP-DET-001": {
    id: "CAP-DET-001",
    title: "Project traceability capability",
    elementType: "capability",
    typeFamily: "capability",
    sourceHref: "#/content/system-model/Capabilities.md",
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
    contract_bindings: [],
  },
};

function Section({ title, desc, children }: { title: string; desc?: string; children: ComponentChildren }) {
  return (
    <ShowcaseRegion regions={["section"]} as="section">
      <ShowcaseRegion regions={["section__heading"]}>
        <ShowcaseRegion regions={["section__title"]}>{title}</ShowcaseRegion>
        {desc && <ShowcaseRegion regions={["section__desc"]}>{desc}</ShowcaseRegion>}
      </ShowcaseRegion>
      {children}
    </ShowcaseRegion>
  );
}

export function PatternsPage() {
  const [worktreeLoad, setWorktreeLoad] = useState<keyof typeof SHOWCASE_WORKTREE_DIALOGS | null>(null);
  useEffect(() => {
    if (worktreeLoad !== "pending") return;
    const timer = setTimeout(() => setWorktreeLoad(null), 5000);
    return () => clearTimeout(timer);
  }, [worktreeLoad]);
  const [worktreeId, setWorktreeId] = useState(SHOWCASE_WORKTREES[0].id);
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
    <ShowcaseRegion regions={["page", "page--wide"]}>
      <Section
        title="Explorer Shell"
        desc="Shared shell with pane-aligned branding and view tabs, and a branch control at the far right after header actions. The real Explorer mock supports pane resizing and collapse with this same header."
      >
        <ShowcaseRegion regions={["shell-demo"]}>
          <ShowcaseRegion regions={["shell-demo__app"]} asChild><AppShell
            brandLabel="REQVIRE"
            headerContext={<WorktreeSelector density="compact" menuAlign="end" value={worktreeId} choices={SHOWCASE_WORKTREES}
              branch={SHOWCASE_WORKTREES.find(choice => choice.id === worktreeId)?.branch}
              onChange={setWorktreeId} onOpen={() => {}} />}
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
          /></ShowcaseRegion>
        </ShowcaseRegion>
      </Section>

      <Section title="Worktree Selector" desc="Choose an admitted branch and worktree. Unavailable contexts remain visible and disabled. In Mocks, this control switches the displayed fixture model through the Explorer shell.">
        <WorktreeSelector
          value={worktreeId}
          choices={SHOWCASE_WORKTREES}
          branch={SHOWCASE_WORKTREES.find(choice => choice.id === worktreeId)?.branch}
          onChange={setWorktreeId}
          onOpen={() => {}}
        />
      </Section>

      <Section title="Worktree Loading and Errors"
        desc="The shared Explorer dialog identifies the branch, explains recovery, and keeps full diagnostics readable. The loading preview completes after five seconds; errors support Close, Escape and outside dismissal.">
        <ShowcaseRegion regions={["row"]}>
          <Button onClick={() => setWorktreeLoad("pending")}>Preview worktree loading</Button>
          <Button onClick={() => setWorktreeLoad("connection")}>Preview connection error</Button>
          <Button onClick={() => setWorktreeLoad("validation")}>Preview validation error</Button>
          <Button onClick={() => setWorktreeLoad("refresh")}>Preview refresh error</Button>
        </ShowcaseRegion>
        {worktreeLoad && <WorktreeLoadDialog {...SHOWCASE_WORKTREE_DIALOGS[worktreeLoad]}
          onDismiss={() => setWorktreeLoad(null)} />}
      </Section>

      <Section
        title="Coverage Drill-down"
        desc="The same coverage component used by Explorer, with shared Verification and Implementation column headings. Expand capabilities and parent requirements to inspect their children. Select requirement names in Mocks to inspect evidence through the full application."
      >
        <CoverageDashboard>
          <CoveragePanel>
            <CoverageDrilldown items={COVERAGE_PATTERN_ITEMS}
              onInspect={target => pushElementDialog(target.id)} />
          </CoveragePanel>
        </CoverageDashboard>
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
        <ShowcaseRegion regions={["pattern-grid", "pattern-grid--two"]}>
          <ShowcaseRegion regions={["pattern-card"]}>
            <PaneChromeHeader title="Model" />
            <ShowcaseRegion regions={["pattern-card__body"]}>
              <ReqvireRailMark placement="inline" />
              <ShowcaseRegion regions={["pattern-muted"]} as="span">Rail mark and pane header keep Explorer chrome consistent.</ShowcaseRegion>
            </ShowcaseRegion>
          </ShowcaseRegion>
          <ShowcaseRegion regions={["pattern-card"]}>
            <PaneTree aria-label="Example file tree">
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
          </ShowcaseRegion>
        </ShowcaseRegion>
      </Section>

      <Section
        title="Content Renderer Frames"
        desc="Renderer shell patterns use already-rendered mock content instead of app renderers."
      >
        <ShowcaseRegion regions={["pattern-grid", "pattern-grid--two"]}>
          <ShowcaseRegion regions={["pattern-card", "pattern-card--tall"]}>
            <DocumentPanel
              layout="embedded"
              toolbar={{
                label: "Source page",
                title: "system-model/SystemRequirements.md",
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
                  <rect x="16" y="42" width="160" height="76" rx="8" fill="var(--color-role-requirement-tint)" stroke="var(--color-role-requirement)" />
                  <text x="96" y="76" text-anchor="middle" fill="var(--text-strong)">Requirement</text>
                  <rect x="232" y="42" width="160" height="76" rx="8" fill="var(--color-role-verification-tint)" stroke="var(--color-role-verification)" />
                  <text x="312" y="76" text-anchor="middle" fill="var(--text-strong)">Verification</text>
                  <rect x="448" y="42" width="156" height="76" rx="8" fill="var(--color-role-resource-tint)" stroke="var(--color-role-resource)" />
                  <text x="526" y="76" text-anchor="middle" fill="var(--text-strong)">Evidence</text>
                  <path d="M176 80 H232" stroke="var(--color-role-edge-trace)" stroke-width="3" />
                  <path d="M392 80 H448" stroke="var(--color-role-edge-bind)" stroke-width="3" />
                </svg>
              </DiagramBlockFrame>
              <RendererNotice title="Renderer note">
                This preview is driven by static showcase content; markdown parsing remains in the app container.
              </RendererNotice>
            </DocumentPanel>
          </ShowcaseRegion>

          <ShowcaseRegion regions={["pattern-card"]}>
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
                <CodeFallback>
                  {CODE_SAMPLE}
                </CodeFallback>
              </CodeBody>
            </CodePreviewFrame>
            <RendererNotice inline tone="empty">
              Empty-state renderer notices can also appear inline.
            </RendererNotice>
          </ShowcaseRegion>
        </ShowcaseRegion>
      </Section>

      <Section
        title="Detail Patterns"
        desc="Element and ontology detail bodies accept DTO-shaped props with inert showcase callbacks."
      >
        <ShowcaseRegion regions={["pattern-grid", "pattern-grid--two"]}>
          <ShowcaseRegion regions={["pattern-card"]}>
            <ShowcaseRegion regions={["pattern-toolbar"]}>
              <TypeBadge type="requirement" family="requirement" tinted>
                requirement
              </TypeBadge>
              <Button tone="secondary" size="sm" onClick={() => openElementDialog()}>
                Open dialog
              </Button>
            </ShowcaseRegion>
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
              contract_references={[{ id: "reference-coverage-contract", targetId: "SPEC-DET-001",
                kind: "element", label: "Coverage Report Specification", elementType: "specification",
                typeFamily: "contract", href: "#/elements/SPEC-DET-001", external: false }]}
              onOpenElement={openElementDialog}
              onOpenResource={() => {}}
            />
          </ShowcaseRegion>

          <ShowcaseRegion regions={["pattern-card"]}>
            <ShowcaseRegion regions={["pattern-toolbar"]}>
              <TypeBadge type="requirement" family="requirement" tinted>
                relations
              </TypeBadge>
            </ShowcaseRegion>
            <RelationList
              relations={DETAIL_RELATIONS}
              onOpenElement={openElementDialog}
              onOpenResource={() => {}}
            />
          </ShowcaseRegion>

          <ShowcaseRegion regions={["pattern-card"]}>
            <ShowcaseRegion regions={["pattern-toolbar"]}>
              <TypeBadge type="ontology" family="ontology" tinted>
                ontology
              </TypeBadge>
              <Button tone="secondary" size="sm" onClick={() => setOntologyDialogNodeId(ONTOLOGY_REQUIREMENT_NODE.id)}>
                Open dialog
              </Button>
            </ShowcaseRegion>
            <OntologyNodeDetailContent
              node={ONTOLOGY_COMPACT_NODE}
              nodes={[ONTOLOGY_COMPACT_NODE]}
              copiedUri={copiedUri}
              onCopyUri={() => setCopiedUri(true)}
              onOpenSource={() => {}}
            />
          </ShowcaseRegion>
        </ShowcaseRegion>
        <ShowcaseRegion regions={["canvas"]}>
          <OntologyMetadata
            node={ONTOLOGY_REQUIREMENT_NODE}
            copiedUri={copiedUri}
            onCopyUri={() => setCopiedUri(true)}
          />
        </ShowcaseRegion>
      </Section>

      <Section title="Resources" desc="Resource list, source identity and empty state from the same public API used by Explorer.">
        <ResourcePageHeader count={2} />
        <ResourceList resources={[
          {id:"fixture-source",kind:"file",display:"Coverage implementation",target:"src/lib/coverage.ts",relationTypes:["satisfiedBy"]},
          {id:"fixture-reference",kind:"url",display:"Reqvire documentation",target:"https://reqvire.com",externalUrl:"https://reqvire.com"},
        ]} />
        <ResourceDetailCard kind="file" target="src/lib/coverage.ts" />
        <ResourceEmptyState>No resources in this scope.</ResourceEmptyState>
      </Section>
      <Section title="Inline concept reference" desc="A public concept reference opens the existing ontology detail dialog.">
        <InlineConceptReference reference={{id:"fixture-concept-reference",iri:ONTOLOGY_REQUIREMENT_NODE.full_uri??ONTOLOGY_REQUIREMENT_NODE.id,
          label:ONTOLOGY_REQUIREMENT_NODE.label??"Requirement",ontologyNodeId:ONTOLOGY_REQUIREMENT_NODE.id}}
          onOpenConceptReference={()=>setOntologyDialogNodeId(ONTOLOGY_REQUIREMENT_NODE.id)} />
      </Section>

      <Section
        title="Feedback Patterns"
        desc="Help and store-state surfaces rendered from product-pattern exports."
      >
        <ShowcaseRegion regions={["pattern-grid", "pattern-grid--two"]}>
          <ShowcaseRegion regions={["pattern-card"]}>
            <ShowcaseRegion regions={["pattern-toolbar"]}>
              <ShowcaseRegion regions={["label"]} as="span">Help content</ShowcaseRegion>
              <Button tone="secondary" size="sm" onClick={() => setHelpDialogOpen(true)}>
                Open help
              </Button>
            </ShowcaseRegion>
            <HelpContent />
          </ShowcaseRegion>
          <ShowcaseRegion regions={["col"]}>
            <ElementDetailMissingState identifier="REQ-MISSING-404" />
            <ShowcaseRegion regions={["pattern-card", "pattern-card--notice"]}>
              <StoreNotice layout="embedded"
                reason="No Project Store seed was found for this showcase scenario."
                detail="assets/project-store.js"
              />
            </ShowcaseRegion>
          </ShowcaseRegion>
        </ShowcaseRegion>
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
          contract_bindings={currentElement.contract_bindings}
          onOpenElement={pushElementDialog}
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
    </ShowcaseRegion>
  );
}

function ShellSidePaneMock() {
  return (
    <ShowcaseRegion regions={["shell-pane-mock"]} aria-label="Placeholder side pane content">
      <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--search"]} />
      <ShowcaseRegion regions={["shell-placeholder-group"]}>
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--heading"]} />
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--row"]} />
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--row", "shell-placeholder--short"]} />
      </ShowcaseRegion>
      <ShowcaseRegion regions={["shell-placeholder-group"]}>
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--heading"]} />
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--tile"]} />
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--tile"]} />
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--tile", "shell-placeholder--short"]} />
      </ShowcaseRegion>
    </ShowcaseRegion>
  );
}

function ShellMainMock() {
  return (
    <ShowcaseRegion regions={["shell-main-mock"]} aria-label="Placeholder workspace content">
      <ShowcaseRegion regions={["shell-workspace-header"]}>
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--icon"]} />
        <ShowcaseRegion regions={["shell-placeholder-stack"]}>
          <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--title"]} />
          <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--subtitle"]} />
        </ShowcaseRegion>
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--action"]} />
      </ShowcaseRegion>
      <ShowcaseRegion regions={["shell-stat-grid"]} aria-hidden="true">
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--stat"]} />
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--stat"]} />
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--stat"]} />
      </ShowcaseRegion>
      <ShowcaseRegion regions={["shell-board"]} aria-hidden="true">
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--panel"]} />
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--panel", "shell-placeholder--panel-accent"]} />
      </ShowcaseRegion>
    </ShowcaseRegion>
  );
}

function ShellDetailRail() {
  return (
    <ShowcaseRegion regions={["shell-detail-rail"]} as="aside" aria-label="Placeholder detail rail">
      <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--title"]} />
      <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--row"]} />
      <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--row", "shell-placeholder--short"]} />
      <ShowcaseRegion regions={["shell-placeholder-group"]}>
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--tile"]} />
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--tile"]} />
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--tile", "shell-placeholder--short"]} />
      </ShowcaseRegion>
    </ShowcaseRegion>
  );
}
