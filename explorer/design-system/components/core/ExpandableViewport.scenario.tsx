import type { ComponentChildren } from "preact";
import { ShowcaseRegion } from "@ds";
import { useState } from "preact/hooks";
import { Card, ExpandableViewport, Icon, IconButton } from "@ds";
import { fixture } from "./ExpandableViewport.fixture";
export const scenario = {"title":"ExpandableViewport","covers":["ExpandableViewport","Card","IconButton","Icon"]};
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
const [viewportExpanded, setViewportExpanded] = useState(false);
return <ShowcaseRegion regions={["page"]}>
<Section title={fixture.title} desc={fixture.description}>
        <ShowcaseRegion regions={["canvas"]}>
          <ExpandableViewport expanded={viewportExpanded} onExpandedChange={setViewportExpanded} label="Expanded preview">
            <Card>
              <ShowcaseRegion regions={["row", "row--center"]}>
                <IconButton aria-label={viewportExpanded ? "Close expanded preview" : "Expand preview"} aria-expanded={viewportExpanded} data-expanded-close={viewportExpanded || undefined} onClick={() => setViewportExpanded(value => !value)}>
                  <Icon name={viewportExpanded ? "x" : "maximize"}/>
                </IconButton>
                <span>Preview content stays mounted while the viewport expands.</span>
              </ShowcaseRegion>
            </Card>
          </ExpandableViewport>
        </ShowcaseRegion>
      </Section>
</ShowcaseRegion>;
}
