import { DocumentPanelToolbar } from "@ds";
import { ShowcaseRegion } from "@ds/review";

import { fixture } from "./DocumentPanelToolbar.ads.fixture";
export const scenario = {"title": "DocumentPanelToolbar", covers: ["DocumentPanelToolbar"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<DocumentPanelToolbar label="Source page" title="system-model/Requirements.md" actionLabel="Open source" actionHref="#source" />
</ShowcaseRegion>;
}
