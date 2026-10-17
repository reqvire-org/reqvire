import { ShowcaseRegion } from "@ds/review";
import { useState } from "preact/hooks";
import { type ComponentChildren } from "preact";
import { Button, ElementDetailMissingState, HelpContent, HelpDialog, StoreNotice } from "@ds";
import { fixture } from "./HelpContent.fixture";
export const scenario = {"title":"Feedback Patterns",covers: ["HelpContent","ElementDetailMissingState","StoreNotice","HelpDialog"]};
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
const [helpDialogOpen, setHelpDialogOpen] = useState(false);
return <ShowcaseRegion regions={["page"]}>
<Section title={fixture.title} desc={fixture.description}>
        <ShowcaseRegion regions={["pattern-grid", "pattern-grid--two"]}>
          <ShowcaseRegion regions={["pattern-card"]}>
            <ShowcaseRegion regions={["pattern-toolbar"]}>
              <ShowcaseRegion regions={["label"]} as="span">Help content</ShowcaseRegion>
              <Button tone="secondary" size="sm" onClick={() => setHelpDialogOpen(true)}>
                Open help
              </Button>
            </ShowcaseRegion>
            <HelpContent />
          </ShowcaseRegion>
          <ShowcaseRegion regions={["col"]}>
            <ElementDetailMissingState identifier="REQ-MISSING-404"/>
            <ShowcaseRegion regions={["pattern-card", "pattern-card--notice"]}>
              <StoreNotice layout="embedded" reason="No Project Store seed was found for this showcase scenario." detail="assets/project-store.js"/>
            </ShowcaseRegion>
          </ShowcaseRegion>
        </ShowcaseRegion>
      </Section>
<HelpDialog open={helpDialogOpen} onOpenChange={setHelpDialogOpen}/>
</ShowcaseRegion>;
}
