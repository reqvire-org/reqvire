import { ShowcaseRegion } from "@ds/review";
import { useEffect, useState } from "preact/hooks";
import { type ComponentChildren } from "preact";
import { Button, WorktreeLoadDialog } from "@ds";
import { fixture, SHOWCASE_WORKTREE_DIALOGS } from "./WorktreeLoadDialog.fixture";
export const scenario = {"title":"Worktree Loading and Errors",covers: ["WorktreeLoadDialog"]};
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
export default function Scenario() {
const [worktreeLoad, setWorktreeLoad] = useState<keyof typeof SHOWCASE_WORKTREE_DIALOGS | null>(null);
useEffect(() => {
    if (worktreeLoad !== "pending")
        return;
    const timer = setTimeout(() => setWorktreeLoad(null), 5000);
    return () => clearTimeout(timer);
}, [worktreeLoad]);
return <ShowcaseRegion regions={["page"]}>
<Section title={fixture.title} desc={fixture.description}>
        <ShowcaseRegion regions={["row"]}>
          <Button onClick={() => setWorktreeLoad("pending")}>Preview worktree loading</Button>
          <Button onClick={() => setWorktreeLoad("connection")}>Preview connection error</Button>
          <Button onClick={() => setWorktreeLoad("validation")}>Preview validation error</Button>
          <Button onClick={() => setWorktreeLoad("refresh")}>Preview refresh error</Button>
        </ShowcaseRegion>
        {worktreeLoad && <WorktreeLoadDialog {...SHOWCASE_WORKTREE_DIALOGS[worktreeLoad]} onDismiss={() => setWorktreeLoad(null)}/>}
      </Section>
</ShowcaseRegion>;
}
