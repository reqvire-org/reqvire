import { ShowcaseRegion, ShellMain } from "@ds";

import { fixture } from "./ShellMain.fixture";
export const scenario = {"title": "ShellMain", "covers": ["ShellMain"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<ShellMain><p>Main workspace content.</p></ShellMain>
</ShowcaseRegion>;
}
