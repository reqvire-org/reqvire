import { ShowcaseRegion, PaneSelection } from "@ds";

import { fixture } from "./PaneSelection.fixture";
export const scenario = {"title": "PaneSelection", "covers": ["PaneSelection"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<PaneSelection ariaLabel="Selected element" emptyMessage="Select a requirement in the model." />
</ShowcaseRegion>;
}
