import { ShowcaseRegion, WorkspaceToolbar, Button } from "@ds";
import { useState } from "preact/hooks";

import { fixture } from "./WorkspaceToolbar.fixture";
export const scenario = {"title": "WorkspaceToolbar", "covers": ["WorkspaceToolbar"]};
export default function Scenario() {
const [count,setCount] = useState(0);
return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<WorkspaceToolbar><Button onClick={() => setCount(count+1)}>Refresh fixture ({count})</Button></WorkspaceToolbar>
</ShowcaseRegion>;
}
