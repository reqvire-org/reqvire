import { ShowcaseRegion, DocumentPanelToolbar } from "@ds";

import { fixture } from "./DocumentPanelToolbar.fixture";
export const scenario = {"title": "DocumentPanelToolbar", "covers": ["DocumentPanelToolbar"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<DocumentPanelToolbar label="Source page" title="system-model/Requirements.md" actionLabel="Open source" actionHref="#source" />
</ShowcaseRegion>;
}
