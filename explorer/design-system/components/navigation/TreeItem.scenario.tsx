import type { ComponentChildren } from "preact";
import { ShowcaseRegion } from "@ds/review";
import { useState } from "preact/hooks";
import { Icon, TreeItem } from "@ds";
import { fixture } from "./TreeItem.fixture";
export const scenario = {"title":"TreeItem",covers: ["TreeItem","Icon"]};
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
const [open, setOpen] = useState<Record<string, boolean>>({ requirements: true, verifications: false });
const [selected, setSelected] = useState<string>("REQ-001");
const toggle = (id: string) => setOpen((s) => ({ ...s, [id]: !s[id] }));
return <ShowcaseRegion regions={["page"]}>
<Section title={fixture.title} desc={fixture.description}>
        <ShowcaseRegion regions={["tree-demo"]}>
          <TreeItem label="system-model/" icon={<Icon name="folder-open" size={14}/>} count={3} depth={0} expandable open={open.requirements} kind="folder" onToggle={() => toggle("requirements")}/>
          {open.requirements && (<>
              <TreeItem label="Capabilities.md" icon={<Icon name="file-text" size={14}/>} count={4} depth={1} expandable kind="file" selected={selected === "capabilities"} onSelect={() => setSelected("capabilities")}/>
              <TreeItem label="SystemRequirements.md" icon={<Icon name="file-text" size={14}/>} count={12} depth={1} expandable open kind="file" selected={selected === "sr"} onSelect={() => setSelected("sr")}/>
              <TreeItem label="Model Structure Specification" icon={<Icon name="box" size={13}/>} depth={2} kind="element" selected={selected === "REQ-001"} onSelect={() => setSelected("REQ-001")}/>
              <TreeItem label="Traceability Coverage Requirement" icon={<Icon name="box" size={13}/>} depth={2} kind="element" selected={selected === "REQ-002"} onSelect={() => setSelected("REQ-002")}/>
              <TreeItem label="Specifications.md" icon={<Icon name="file-text" size={14}/>} count={8} depth={1} kind="file" selected={selected === "specs"} onSelect={() => setSelected("specs")}/>
            </>)}
          <TreeItem label="verifications/" icon={open.verifications ? <Icon name="folder-open" size={14}/> : <Icon name="folder" size={14}/>} count={8} depth={0} expandable open={open.verifications} kind="folder" onToggle={() => toggle("verifications")}/>
          {open.verifications && (<TreeItem label="ModelStructure.md" icon={<Icon name="file-text" size={14}/>} count={3} depth={1} kind="file"/>)}
          <TreeItem label="ontologies/" icon={<Icon name="folder" size={14}/>} count={1} depth={0} expandable kind="folder"/>
        </ShowcaseRegion>
      </Section>
</ShowcaseRegion>;
}
