import { ShowcaseRegion } from "@ds";
import { type ComponentChildren } from "preact";
import { FileBrowserFrame, FileBrowserList, FileBrowserToolbar } from "@ds";
import { fixture, FILE_BROWSER_BREADCRUMBS, FILE_BROWSER_ITEMS } from "./FileBrowser.fixture";
export const scenario = {"title":"File Browser","covers":["ShowcaseRegion","FileBrowserFrame","FileBrowserToolbar","FileBrowserList"]};
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
        <FileBrowserFrame>
          <FileBrowserToolbar breadcrumbs={[...FILE_BROWSER_BREADCRUMBS]} layout="list" resultCount={FILE_BROWSER_ITEMS.length} onOpenFolder={() => { }} onLayoutChange={() => { }}/>
          <FileBrowserList items={[...FILE_BROWSER_ITEMS]} sortKey="name" sortDirection="asc" onSort={() => { }} onOpenFolder={() => { }} onOpenFile={() => { }}/>
        </FileBrowserFrame>
      </Section>
</ShowcaseRegion>;
}
