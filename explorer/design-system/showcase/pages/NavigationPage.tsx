import { useState } from "react";
import { Breadcrumb, Icon, IconButton, SidebarSection, Tabs, TreeItem } from "@ds";

function Section({ title, desc, children }: { title: string; desc?: string; children: React.ReactNode }) {
  return (
    <section className="showcase-section">
      <div className="showcase-section__heading">
        <div className="showcase-section__title">{title}</div>
        {desc && <div className="showcase-section__desc">{desc}</div>}
      </div>
      {children}
    </section>
  );
}

export function NavigationPage() {
  const [open, setOpen] = useState<Record<string, boolean>>({ requirements: true, verifications: false });
  const [selected, setSelected] = useState<string>("REQ-001");
  const [tabUnderline, setTabUnderline] = useState("model");
  const [tabPill, setTabPill] = useState("grid");

  const toggle = (id: string) => setOpen((s) => ({ ...s, [id]: !s[id] }));

  return (
    <div className="showcase-page">

      <Section title="Tabs" desc="Navigation tab primitive. Underline is used for top navigation; pill is used for local navigation.">
        <div className="showcase-col">
          <div className="showcase-label">Underline</div>
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
        </div>
        <div className="showcase-col">
          <div className="showcase-label">Pill</div>
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
        </div>
      </Section>

      <Section title="TreeItem" desc="Indented tree row with expand, icon, label, count. Used in model/file sidebar.">
        <div className="showcase-tree-demo">
          <TreeItem
            label="requirements/"
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
        </div>
      </Section>

      <Section title="Breadcrumb" desc="Slash-delimited path bar. Last item is current (non-link).">
        <div className="showcase-col">
          <div className="showcase-label">File path</div>
          <Breadcrumb
            items={[
              { label: "requirements", onClick: () => {} },
              { label: "SystemRequirements.md", onClick: () => {} },
              { label: "Model Structure Specification" },
            ]}
          />
        </div>
        <div className="showcase-col">
          <div className="showcase-label">Short path</div>
          <Breadcrumb
            items={[
              { label: "requirements", onClick: () => {} },
              { label: "Capabilities.md" },
            ]}
          />
        </div>
      </Section>

      <Section title="SidebarSection" desc="Collapsible section with eyebrow title and optional action slot. Used throughout the left panel.">
        <div className="showcase-sidebar-demo">
          <SidebarSection
            title="Summary"
            action={<IconButton tone="ghost" size="sm" aria-label="refresh"><Icon name="rotate-ccw" /></IconButton>}
          >
            <div className="showcase-sidebar-stats">
              <div className="showcase-stat-line">
                <span>Elements</span><strong>640</strong>
              </div>
              <div className="showcase-stat-line">
                <span>Relations</span><strong>1,090</strong>
              </div>
              <div className="showcase-stat-line">
                <span>Files</span><strong>38</strong>
              </div>
            </div>
          </SidebarSection>
          <SidebarSection title="Show">
            <div className="showcase-muted-note">
              Type filter toggles would appear here.
            </div>
          </SidebarSection>
          <SidebarSection>
            <div className="showcase-muted-note">
              Section without a title.
            </div>
          </SidebarSection>
        </div>
      </Section>

    </div>
  );
}
