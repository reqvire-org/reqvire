import { ShowcaseRegion, PaneControlSection, PaneSearchForm, PaneFilterGroup, PaneFilterGrid, ToggleRow, PaneFilterNavList, PaneFilterNavRow } from "@ds";
import { useState } from "preact/hooks";

import { fixture } from "./PaneControls.fixture";
export const scenario = {"title": "PaneControls", "covers": ["PaneControlSection", "PaneSearchForm", "PaneFilterGroup", "PaneFilterGrid", "PaneFilterNavList", "PaneFilterNavRow"]};
export default function Scenario() {
const [query,setQuery] = useState(""); const [enabled,setEnabled] = useState(true);
return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<PaneControlSection title="Model filters"><PaneSearchForm searchInputId="fixture-filter" inputLabel="Filter model" value={query} onQueryChange={setQuery} />
<PaneFilterGroup label="Result types"><PaneFilterGrid><ToggleRow label="Requirements" on={enabled} onClick={() => setEnabled(!enabled)} /></PaneFilterGrid></PaneFilterGroup>
<PaneFilterNavList><PaneFilterNavRow icon="folder" label="Requirements" count={12} onClick={() => setEnabled(!enabled)} /></PaneFilterNavList></PaneControlSection>
</ShowcaseRegion>;
}
