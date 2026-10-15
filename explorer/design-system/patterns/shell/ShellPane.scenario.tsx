import { ShowcaseRegion, ShellPane } from "@ds";

import { fixture } from "./ShellPane.fixture";
export const scenario = {"title": "ShellPane", "covers": ["ShellPane"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<ShellPane><p>Navigation pane content.</p></ShellPane>
</ShowcaseRegion>;
}
