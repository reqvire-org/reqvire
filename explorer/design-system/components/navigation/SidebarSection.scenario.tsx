import type { ComponentChildren } from "preact";
import { ShowcaseRegion } from "@ds";
import { Icon, IconButton, SidebarSection } from "@ds";
import { fixture } from "./SidebarSection.fixture";
export const scenario = {"title":"SidebarSection","covers":["SidebarSection","IconButton","Icon"]};
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
        <ShowcaseRegion regions={["sidebar-demo"]}>
          <SidebarSection title="Summary" action={<IconButton tone="ghost" size="sm" aria-label="refresh"><Icon name="rotate-ccw"/></IconButton>}>
            <ShowcaseRegion regions={["sidebar-stats"]}>
              <ShowcaseRegion regions={["stat-line"]}>
                <span>Elements</span><strong>640</strong>
              </ShowcaseRegion>
              <ShowcaseRegion regions={["stat-line"]}>
                <span>Relations</span><strong>1,090</strong>
              </ShowcaseRegion>
              <ShowcaseRegion regions={["stat-line"]}>
                <span>Files</span><strong>38</strong>
              </ShowcaseRegion>
            </ShowcaseRegion>
          </SidebarSection>
          <SidebarSection title="Show">
            <ShowcaseRegion regions={["muted-note"]}>
              Type filter toggles would appear here.
            </ShowcaseRegion>
          </SidebarSection>
          <SidebarSection>
            <ShowcaseRegion regions={["muted-note"]}>
              Section without a title.
            </ShowcaseRegion>
          </SidebarSection>
        </ShowcaseRegion>
      </Section>
</ShowcaseRegion>;
}
