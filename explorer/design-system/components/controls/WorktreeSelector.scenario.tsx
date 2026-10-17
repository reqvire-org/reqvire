import { ShowcaseRegion } from "@ds/review";
import { useState } from "preact/hooks";
import { type ComponentChildren } from "preact";
import { WorktreeSelector } from "@ds";
import { fixture, SHOWCASE_WORKTREES } from "./WorktreeSelector.fixture";
export const scenario = {"title":"Worktree Selector",covers: ["WorktreeSelector"]};
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
const [worktreeId, setWorktreeId] = useState(SHOWCASE_WORKTREES[0].id);
return <ShowcaseRegion regions={["page"]}>
<Section title={fixture.title} desc={fixture.description}>
        <WorktreeSelector value={worktreeId} choices={SHOWCASE_WORKTREES} branch={SHOWCASE_WORKTREES.find(choice => choice.id === worktreeId)?.branch} onChange={setWorktreeId} onOpen={() => { }}/>
      </Section>
</ShowcaseRegion>;
}
