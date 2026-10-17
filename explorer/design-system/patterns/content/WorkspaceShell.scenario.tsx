import { WorkspaceShell } from "@ds";
import { ShowcaseRegion } from "@ds/review";

import { fixture } from "./WorkspaceShell.fixture";
export const scenario = {"title": "WorkspaceShell", covers: ["WorkspaceShell"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<WorkspaceShell rootLabel="Model" currentLabel="Requirements"><p>The workspace content slot.</p></WorkspaceShell>
</ShowcaseRegion>;
}
