import { useState } from "react";
import { Breadcrumb, Icon, IconButton, SidebarSection, TreeItem } from "@ds";

function Section({ title, desc, children }: { title: string; desc?: string; children: React.ReactNode }) {
  return (
    <section className="sc-section">
      <div className="sc-section__heading">
        <div className="sc-section__title">{title}</div>
        {desc && <div className="sc-section__desc">{desc}</div>}
      </div>
      {children}
    </section>
  );
}

export function NavigationPage() {
  const [open, setOpen] = useState<Record<string, boolean>>({ requirements: true, verifications: false });
  const [selected, setSelected] = useState<string>("REQ-001");

  const toggle = (id: string) => setOpen((s) => ({ ...s, [id]: !s[id] }));

  return (
    <div className="sc-page">

      <Section title="TreeItem" desc="Indented tree row with expand, icon, label, count. Used in model/file sidebar.">
        <div style={{ width: 320, background: "var(--bg-surface)", border: "var(--border-w) solid var(--border-subtle)", borderRadius: "var(--radius-lg)", padding: "var(--space-8)", display: "flex", flexDirection: "column", gap: "var(--space-1)" }}>
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
        <div className="sc-col">
          <div className="sc-label">File path</div>
          <Breadcrumb
            items={[
              { label: "requirements", onClick: () => {} },
              { label: "SystemRequirements.md", onClick: () => {} },
              { label: "Model Structure Specification" },
            ]}
          />
        </div>
        <div className="sc-col">
          <div className="sc-label">Short path</div>
          <Breadcrumb
            items={[
              { label: "requirements", onClick: () => {} },
              { label: "Capabilities.md" },
            ]}
          />
        </div>
      </Section>

      <Section title="SidebarSection" desc="Collapsible section with eyebrow title and optional action slot. Used throughout the left panel.">
        <div style={{ width: 320, display: "flex", flexDirection: "column", gap: "var(--space-2)" }}>
          <SidebarSection
            title="Summary"
            action={<IconButton tone="ghost" size="sm" aria-label="refresh"><Icon name="rotate-ccw" /></IconButton>}
          >
            <div style={{ fontSize: "var(--text-caption)", color: "var(--text-muted)", display: "flex", flexDirection: "column", gap: "var(--space-3)" }}>
              <div style={{ display: "flex", justifyContent: "space-between" }}>
                <span>Elements</span><strong style={{ color: "var(--text-strong)" }}>640</strong>
              </div>
              <div style={{ display: "flex", justifyContent: "space-between" }}>
                <span>Relations</span><strong style={{ color: "var(--text-strong)" }}>1,090</strong>
              </div>
              <div style={{ display: "flex", justifyContent: "space-between" }}>
                <span>Files</span><strong style={{ color: "var(--text-strong)" }}>38</strong>
              </div>
            </div>
          </SidebarSection>
          <SidebarSection title="Show">
            <div style={{ fontSize: "var(--text-sm)", color: "var(--text-muted)", fontStyle: "italic" }}>
              Type filter toggles would appear here.
            </div>
          </SidebarSection>
          <SidebarSection>
            <div style={{ fontSize: "var(--text-sm)", color: "var(--text-muted)", fontStyle: "italic" }}>
              Section without a title.
            </div>
          </SidebarSection>
        </div>
      </Section>

    </div>
  );
}
