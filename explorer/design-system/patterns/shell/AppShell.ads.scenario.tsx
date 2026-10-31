import { ShowcaseRegion } from "@ds/review";
import { useState } from "preact/hooks";
import { type ComponentChildren } from "preact";
import { AppShell, SidePaneFrame, WorktreeSelector } from "@ds";
import { fixture, SHOWCASE_WORKTREES, SHELL_PATTERN_NAVIGATION, SHELL_PATTERN_ACTIONS } from "./AppShell.ads.fixture";
export const scenario = {"title":"Explorer Shell",covers: ["AppShell","SidePaneFrame"]};
function Section({ title, desc, children }: {
    title: string;
    desc?: string;
    children: ComponentChildren;
}) {
    return (<ShowcaseRegion regions={["section"]} as="section">
      <ShowcaseRegion regions={["section__heading"]}>
        <ShowcaseRegion regions={["section__title"]}>{title}</ShowcaseRegion>
        {desc && <ShowcaseRegion regions={["section__desc"]}>{desc}</ShowcaseRegion>}
      </ShowcaseRegion>
      {children}
    </ShowcaseRegion>);
}
function ShellSidePaneMock() {
    return (<ShowcaseRegion regions={["shell-pane-mock"]} role="group" aria-label="Placeholder side pane content">
      <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--search"]}/>
      <ShowcaseRegion regions={["shell-placeholder-group"]}>
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--heading"]}/>
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--row"]}/>
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--row", "shell-placeholder--short"]}/>
      </ShowcaseRegion>
      <ShowcaseRegion regions={["shell-placeholder-group"]}>
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--heading"]}/>
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--tile"]}/>
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--tile"]}/>
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--tile", "shell-placeholder--short"]}/>
      </ShowcaseRegion>
    </ShowcaseRegion>);
}
function ShellMainMock() {
    return (<ShowcaseRegion regions={["shell-main-mock"]} role="group" aria-label="Placeholder workspace content">
      <ShowcaseRegion regions={["shell-workspace-header"]}>
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--icon"]}/>
        <ShowcaseRegion regions={["shell-placeholder-stack"]}>
          <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--title"]}/>
          <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--subtitle"]}/>
        </ShowcaseRegion>
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--action"]}/>
      </ShowcaseRegion>
      <ShowcaseRegion regions={["shell-stat-grid"]} aria-hidden="true">
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--stat"]}/>
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--stat"]}/>
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--stat"]}/>
      </ShowcaseRegion>
      <ShowcaseRegion regions={["shell-board"]} aria-hidden="true">
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--panel"]}/>
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--panel", "shell-placeholder--panel-accent"]}/>
      </ShowcaseRegion>
    </ShowcaseRegion>);
}
function ShellDetailRail() {
    return (<ShowcaseRegion regions={["shell-detail-rail"]} as="aside" aria-label="Placeholder detail rail">
      <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--title"]}/>
      <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--row"]}/>
      <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--row", "shell-placeholder--short"]}/>
      <ShowcaseRegion regions={["shell-placeholder-group"]}>
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--tile"]}/>
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--tile"]}/>
        <ShowcaseRegion regions={["shell-placeholder", "shell-placeholder--tile", "shell-placeholder--short"]}/>
      </ShowcaseRegion>
    </ShowcaseRegion>);
}
export default function Scenario() {
const [worktreeId, setWorktreeId] = useState(SHOWCASE_WORKTREES[0].id);
const [leftPaneOpen, setLeftPaneOpen] = useState(true);
const inertShellActions = SHELL_PATTERN_ACTIONS.map((action) => ({
    ...action,
    onClick: () => { },
}));
return <ShowcaseRegion regions={["page"]}>
<Section title={fixture.title} desc={fixture.description}>
        <ShowcaseRegion regions={["shell-demo"]}>
          <ShowcaseRegion regions={["shell-demo__app"]} asChild><AppShell brandLabel="REQVIRE" headerContext={<WorktreeSelector density="compact" menuAlign="end" value={worktreeId} choices={SHOWCASE_WORKTREES} branch={SHOWCASE_WORKTREES.find(choice => choice.id === worktreeId)?.branch} onChange={setWorktreeId} onOpen={() => { }}/>} navigationItems={[...SHELL_PATTERN_NAVIGATION]} activeNavigationValue="workspace" headerActions={inertShellActions} leftPaneOpen={leftPaneOpen} leftPaneWidth={318} leftPaneMinWidth={260} leftPaneMaxWidth={420} onToggleLeftPane={() => setLeftPaneOpen((open) => !open)} onLeftPaneResizePointerDown={() => { }} onNavigate={() => { }} sidePane={<SidePaneFrame open={leftPaneOpen} chrome="app" onToggle={() => setLeftPaneOpen((open) => !open)}>
                <ShellSidePaneMock />
              </SidePaneFrame>} main={<ShellMainMock />} detailPane={<ShellDetailRail />}/></ShowcaseRegion>
        </ShowcaseRegion>
      </Section>
</ShowcaseRegion>;
}
