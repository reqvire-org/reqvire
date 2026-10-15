import type { ComponentChildren } from "preact";
import { ShowcaseRegion } from "@ds";
import { Alert } from "@ds";
import { fixture } from "./Alert.fixture";
export const scenario = {"title":"Alert","covers":["Alert"]};
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

return <ShowcaseRegion regions={["page"]}>
<Section title={fixture.title} desc={fixture.description}>
        <ShowcaseRegion regions={["col"]}>
          <Alert>Default: model validation completed.</Alert>
          <Alert variant="success">Success: all 640 elements parsed without errors.</Alert>
          <Alert variant="warning">Warning: 3 relations reference unknown target identifiers.</Alert>
          <Alert variant="danger">Error: schema version mismatch — expected 4, found 3.</Alert>
        </ShowcaseRegion>
      </Section>
</ShowcaseRegion>;
}
