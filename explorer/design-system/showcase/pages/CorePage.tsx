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
    <section className="showcase-section">
      <div className="showcase-section__heading">
        <div className="showcase-section__title">{title}</div>
        {desc && <div className="showcase-section__desc">{desc}</div>}
      </div>
      {children}
    </section>
  );
}

export function CorePage() {
  const [modalOpen, setModalOpen] = useState(false);

  return (
    <div className="showcase-page">

      <Section title="Button" desc="tone × size. Default tone is secondary, default size is md.">
        {(["primary", "accent", "secondary", "ghost", "danger", "link"] as const).map((tone) => (
          <div key={tone} className="showcase-col">
            <div className="showcase-label">{tone}</div>
            <div className="showcase-row showcase-row--center">
              {(["sm", "md", "lg"] as const).map((size) => (
                <Button key={size} tone={tone} size={size}>{size}</Button>
              ))}
              <Button tone={tone} disabled>disabled</Button>
              <Button tone={tone} iconLeft={<Icon name="plus" />}>with icon</Button>
            </div>
          </div>
        ))}
        <div className="showcase-col">
          <div className="showcase-label">block</div>
          <Button tone="secondary" block iconLeft={<Icon name="download" />}>Full-width button</Button>
        </div>
      </Section>

      <Section title="IconButton" desc="Icon-only variant. Ghost by default.">
        <div className="showcase-row showcase-row--center">
          {(["secondary", "ghost"] as const).map((tone) => (
            <div key={tone} className="showcase-col">
              <div className="showcase-label">{tone}</div>
              <div className="showcase-row showcase-row--center">
                <IconButton tone={tone} size="sm" aria-label="settings sm"><Icon name="settings" /></IconButton>
                <IconButton tone={tone} size="md" aria-label="settings md"><Icon name="settings" /></IconButton>
              </div>
            </div>
          ))}
          <div className="showcase-col">
            <div className="showcase-label">active</div>
            <IconButton tone="ghost" aria-label="active" aria-pressed="true"><Icon name="filter" /></IconButton>
          </div>
        </div>
      </Section>

      <Section title="Badge" desc="Count pills and status indicators.">
        <div className="showcase-row showcase-row--center">
          <div className="showcase-col">
            <div className="showcase-label">variants</div>
            <div className="showcase-row showcase-row--center">
              <Badge>12</Badge>
              <Badge variant="accent">5</Badge>
              <Badge variant="solid">64</Badge>
              <Badge variant="dot" />
            </div>
          </div>
          <div className="showcase-col">
            <div className="showcase-label">in context</div>
            <div className="showcase-row showcase-row--center">
              <Button tone="secondary" iconRight={<Badge>3</Badge>}>Pending</Button>
              <Button tone="ghost">Relations <Badge variant="solid">14</Badge></Button>
            </div>
          </div>
        </div>
      </Section>

      <Section title="Alert" desc="Inline status message.">
        <div className="showcase-col">
          <Alert>Default: model validation completed.</Alert>
          <Alert variant="success">Success: all 640 elements parsed without errors.</Alert>
          <Alert variant="warning">Warning: 3 relations reference unknown target identifiers.</Alert>
          <Alert variant="danger">Error: schema version mismatch — expected 4, found 3.</Alert>
        </div>
      </Section>

      <Section title="Card" desc="Surface container with optional accent, interactive, and selected states.">
        <div className="showcase-row">
          <Card className="showcase-card-demo">
            <div className="showcase-card-demo__title">Default card</div>
            <div className="showcase-card-demo__desc">Static surface</div>
          </Card>
          <Card interactive className="showcase-card-demo">
            <div className="showcase-card-demo__title">Interactive</div>
            <div className="showcase-card-demo__desc">Hover to see lift</div>
          </Card>
          <Card interactive selected className="showcase-card-demo">
            <div className="showcase-card-demo__title">Selected</div>
            <div className="showcase-card-demo__desc">Accent ring</div>
          </Card>
          <Card accentColorToken="--requirement" className="showcase-card-demo">
            <div className="showcase-card-demo__title">Accent bar</div>
            <div className="showcase-card-demo__desc">Left color rail</div>
          </Card>
        </div>
      </Section>

      <Section title="Modal" desc="Portal-rendered dialog. Escape key and scrim click both close.">
        <div className="showcase-row">
          <Button tone="secondary" onClick={() => setModalOpen(true)}>Open element detail modal</Button>
        </div>
        <Modal open={modalOpen} onOpenChange={setModalOpen}>
          <ModalContent>
            <ModalHeader>
              <ModalTitle>System Modeling Capability</ModalTitle>
            </ModalHeader>
            <ModalBody>
              <p className="showcase-modal-copy">
                As a <strong>System Engineer</strong>, I want a well-defined Reqvire model structure,
                so that I can manage system requirements and traceability across all project phases.
              </p>
            </ModalBody>
            <ModalFooter>
              <a href="#" className="showcase-modal-link">
                <Icon name="external-link" size={14} /> Open source page
              </a>
              <div className="showcase-modal-footer-spacer" />
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
