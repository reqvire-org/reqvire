import { useState } from "react";
import {
  Alert,
  Badge,
  Button,
  Card,
  Icon,
  IconButton,
  Modal,
  ModalBody,
  ModalClose,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalTitle,
} from "@ds";

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

export function CorePage() {
  const [modalOpen, setModalOpen] = useState(false);

  return (
    <div className="sc-page">

      <Section title="Button" desc="tone × size. Default tone is secondary, default size is md.">
        {(["primary", "accent", "secondary", "ghost", "danger", "link"] as const).map((tone) => (
          <div key={tone} className="sc-col">
            <div className="sc-label">{tone}</div>
            <div className="sc-row sc-row--center">
              {(["sm", "md", "lg"] as const).map((size) => (
                <Button key={size} tone={tone} size={size}>{size}</Button>
              ))}
              <Button tone={tone} disabled>disabled</Button>
              <Button tone={tone} iconLeft={<Icon name="plus" />}>with icon</Button>
            </div>
          </div>
        ))}
        <div className="sc-col">
          <div className="sc-label">block</div>
          <Button tone="secondary" block iconLeft={<Icon name="download" />}>Full-width button</Button>
        </div>
      </Section>

      <Section title="IconButton" desc="Icon-only variant. Ghost by default.">
        <div className="sc-row sc-row--center">
          {(["secondary", "ghost"] as const).map((tone) => (
            <div key={tone} className="sc-col">
              <div className="sc-label">{tone}</div>
              <div className="sc-row sc-row--center">
                <IconButton tone={tone} size="sm" aria-label="settings sm"><Icon name="settings" /></IconButton>
                <IconButton tone={tone} size="md" aria-label="settings md"><Icon name="settings" /></IconButton>
              </div>
            </div>
          ))}
          <div className="sc-col">
            <div className="sc-label">active</div>
            <IconButton tone="ghost" aria-label="active" aria-pressed="true"><Icon name="filter" /></IconButton>
          </div>
        </div>
      </Section>

      <Section title="Badge" desc="Count pills and status indicators.">
        <div className="sc-row sc-row--center">
          <div className="sc-col">
            <div className="sc-label">variants</div>
            <div className="sc-row sc-row--center">
              <Badge>12</Badge>
              <Badge variant="accent">5</Badge>
              <Badge variant="solid">64</Badge>
              <Badge variant="dot" />
            </div>
          </div>
          <div className="sc-col">
            <div className="sc-label">in context</div>
            <div className="sc-row sc-row--center">
              <Button tone="secondary" iconRight={<Badge>3</Badge>}>Pending</Button>
              <Button tone="ghost">Relations <Badge variant="solid">14</Badge></Button>
            </div>
          </div>
        </div>
      </Section>

      <Section title="Alert" desc="Inline status message.">
        <div className="sc-col">
          <Alert>Default: model validation completed.</Alert>
          <Alert variant="success">Success: all 640 elements parsed without errors.</Alert>
          <Alert variant="warning">Warning: 3 relations reference unknown target identifiers.</Alert>
          <Alert variant="danger">Error: schema version mismatch — expected 4, found 3.</Alert>
        </div>
      </Section>

      <Section title="Card" desc="Surface container with optional accent, interactive, and selected states.">
        <div className="sc-row">
          <Card style={{ width: 200 }}>
            <div style={{ fontSize: "var(--text-sm)", color: "var(--text-body)" }}>Default card</div>
            <div style={{ fontSize: "var(--text-caption)", color: "var(--text-muted)" }}>Static surface</div>
          </Card>
          <Card interactive style={{ width: 200 }}>
            <div style={{ fontSize: "var(--text-sm)", color: "var(--text-body)" }}>Interactive</div>
            <div style={{ fontSize: "var(--text-caption)", color: "var(--text-muted)" }}>Hover to see lift</div>
          </Card>
          <Card interactive selected style={{ width: 200 }}>
            <div style={{ fontSize: "var(--text-sm)", color: "var(--text-body)" }}>Selected</div>
            <div style={{ fontSize: "var(--text-caption)", color: "var(--text-muted)" }}>Accent ring</div>
          </Card>
          <Card accentColorToken="--requirement" style={{ width: 200 }}>
            <div style={{ fontSize: "var(--text-sm)", color: "var(--text-body)" }}>Accent bar</div>
            <div style={{ fontSize: "var(--text-caption)", color: "var(--text-muted)" }}>Left color rail</div>
          </Card>
        </div>
      </Section>

      <Section title="Modal" desc="Portal-rendered dialog. Escape key and scrim click both close.">
        <div className="sc-row">
          <Button tone="secondary" onClick={() => setModalOpen(true)}>Open element detail modal</Button>
        </div>
        <Modal open={modalOpen} onOpenChange={setModalOpen}>
          <ModalContent>
            <ModalHeader>
              <ModalTitle>System Modeling Capability</ModalTitle>
            </ModalHeader>
            <ModalBody>
              <p style={{ color: "var(--text-body)", fontSize: "var(--text-sm)", lineHeight: "var(--leading-normal)" }}>
                As a <strong>System Engineer</strong>, I want a well-defined Reqvire model structure,
                so that I can manage system requirements and traceability across all project phases.
              </p>
            </ModalBody>
            <ModalFooter>
              <a href="#" style={{ fontSize: "var(--text-sm)", color: "var(--text-link)" }}>
                <Icon name="external-link" size={14} /> Open source page
              </a>
              <div style={{ flex: 1 }} />
              <ModalClose asChild>
                <Button tone="primary" size="sm">Close</Button>
              </ModalClose>
            </ModalFooter>
          </ModalContent>
        </Modal>
      </Section>

    </div>
  );
}
