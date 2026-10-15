import type { ComponentChildren } from "preact";
import { ShowcaseRegion } from "@ds";
import { useState } from "preact/hooks";
import { Breadcrumb, Icon, IconButton, SidebarSection, Tabs, TreeItem } from "@ds";

function Section({ title, desc, children }: { title: string; desc?: string; children: ComponentChildren }) {
  return (
    <ShowcaseRegion regions={["section"]} as="section">
      <ShowcaseRegion regions={["section__heading"]}>
        <ShowcaseRegion regions={["section__title"]}>{title}</ShowcaseRegion>
        {desc && <ShowcaseRegion regions={["section__desc"]}>{desc}</ShowcaseRegion>}
      </ShowcaseRegion>
      {children}
    </ShowcaseRegion>
  );
}

export function NavigationExamples() {
  const [open, setOpen] = useState<Record<string, boolean>>({ requirements: true, verifications: false });
  const [selected, setSelected] = useState<string>("REQ-001");
  const [tabUnderline, setTabUnderline] = useState("model");
  const [tabPill, setTabPill] = useState("grid");

  const toggle = (id: string) => setOpen((s) => ({ ...s, [id]: !s[id] }));

  return (
    <ShowcaseRegion regions={["page"]}>

      <Section title="Tabs" desc="Navigation tab primitive. Underline is used for top navigation; pill is used for local navigation.">
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>Underline</ShowcaseRegion>
          <Tabs
            items={[
              { value: "model", label: "Model", icon: <Icon name="folder" size={14} /> },
              { value: "graph", label: "Knowledge Graph", icon: <Icon name="network" size={14} /> },
              { value: "ontologies", label: "Ontologies", icon: <Icon name="globe" size={14} /> },
              { value: "search", label: "Search", icon: <Icon name="search" size={14} /> },
              { value: "traces", label: "Traces", icon: <Icon name="activity" size={14} />, badge: 145 },
            ]}
            value={tabUnderline}
            onChange={setTabUnderline}
          />
        </ShowcaseRegion>
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>Pill</ShowcaseRegion>
          <Tabs
            variant="pill"
            items={[
              { value: "grid", label: "Grid" },
              { value: "list", label: "List" },
              { value: "sunburst", label: "Sunburst" },
            ]}
            value={tabPill}
            onChange={setTabPill}
          />
        </ShowcaseRegion>
      </Section>

      <Section title="TreeItem" desc="Indented tree row with expand, icon, label, count. Used in model/file sidebar.">
        <ShowcaseRegion regions={["tree-demo"]}>
          <TreeItem
            label="system-model/"
            icon={<Icon name="folder-open" size={14} />}
            count={3}
            depth={0}
            expandable
            open={open.requirements}
            kind="folder"
            onToggle={() => toggle("requirements")}
          />
          {open.requirements && (
            <>
              <TreeItem
                label="Capabilities.md"
                icon={<Icon name="file-text" size={14} />}
                count={4}
                depth={1}
                expandable
                kind="file"
                selected={selected === "capabilities"}
                onSelect={() => setSelected("capabilities")}
              />
              <TreeItem
                label="SystemRequirements.md"
                icon={<Icon name="file-text" size={14} />}
                count={12}
                depth={1}
                expandable
                open
                kind="file"
                selected={selected === "sr"}
                onSelect={() => setSelected("sr")}
              />
              <TreeItem
                label="Model Structure Specification"
                icon={<Icon name="box" size={13} />}
                depth={2}
                kind="element"
                selected={selected === "REQ-001"}
                onSelect={() => setSelected("REQ-001")}
              />
              <TreeItem
                label="Traceability Coverage Requirement"
                icon={<Icon name="box" size={13} />}
                depth={2}
                kind="element"
                selected={selected === "REQ-002"}
                onSelect={() => setSelected("REQ-002")}
              />
              <TreeItem
                label="Specifications.md"
                icon={<Icon name="file-text" size={14} />}
                count={8}
                depth={1}
                kind="file"
                selected={selected === "specs"}
                onSelect={() => setSelected("specs")}
              />
            </>
          )}
          <TreeItem
            label="verifications/"
            icon={open.verifications ? <Icon name="folder-open" size={14} /> : <Icon name="folder" size={14} />}
            count={8}
            depth={0}
            expandable
            open={open.verifications}
            kind="folder"
            onToggle={() => toggle("verifications")}
          />
          {open.verifications && (
            <TreeItem
              label="ModelStructure.md"
              icon={<Icon name="file-text" size={14} />}
              count={3}
              depth={1}
              kind="file"
            />
          )}
          <TreeItem
            label="ontologies/"
            icon={<Icon name="folder" size={14} />}
            count={1}
            depth={0}
            expandable
            kind="folder"
          />
        </ShowcaseRegion>
      </Section>

      <Section title="Breadcrumb" desc="Slash-delimited path bar. Last item is current (non-link).">
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>File path</ShowcaseRegion>
          <Breadcrumb
            items={[
              { label: "requirements", onClick: () => {} },
              { label: "SystemRequirements.md", onClick: () => {} },
              { label: "Model Structure Specification" },
            ]}
          />
        </ShowcaseRegion>
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>Short path</ShowcaseRegion>
          <Breadcrumb
            items={[
              { label: "requirements", onClick: () => {} },
              { label: "Capabilities.md" },
            ]}
          />
        </ShowcaseRegion>
      </Section>

      <Section title="SidebarSection" desc="Collapsible section with eyebrow title and optional action slot. Used throughout the left panel.">
        <ShowcaseRegion regions={["sidebar-demo"]}>
          <SidebarSection
            title="Summary"
            action={<IconButton tone="ghost" size="sm" aria-label="refresh"><Icon name="rotate-ccw" /></IconButton>}
          >
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

    </ShowcaseRegion>
  );
}
