import { ShowcaseRegion, CoverageNavigation } from "@ds";
import { useState } from "preact/hooks";

import { fixture } from "./CoverageNavigationParts.fixture";
export const scenario = {"title": "CoverageNavigationParts", "covers": ["CoverageNavigation"]};
export default function Scenario() {
const [selected,setSelected] = useState<string|null>(null);
return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<CoverageNavigation scopes={fixture.scopes} selectedId={selected} onSelect={setSelected} />
</ShowcaseRegion>;
}
