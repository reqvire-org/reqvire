import type { ComponentChildren } from "preact";
import { TextLink } from "@ds";
import { ShowcaseRegion } from "@ds";
import { useState } from "preact/hooks";
import { ToastDemo } from "../../ToastDemo";
import {
  Alert,
  Badge,
  Button,
  Card,
  ExpandableViewport,
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

export function CoreExamples() {
  const [modalOpen, setModalOpen] = useState(false);
  const [viewportExpanded, setViewportExpanded] = useState(false);

  return (
    <ShowcaseRegion regions={["page"]}>

      <Section title="Toast" desc="Real notifications: compact stacking, hover expansion, swipe dismissal, and promise states.">
        <ToastDemo />
      </Section>

      <Section title="Button" desc="tone × size. Default tone is secondary, default size is md.">
        {(["primary", "accent", "secondary", "ghost", "danger", "link"] as const).map((tone) => (
          <ShowcaseRegion key={tone} regions={["col"]}>
            <ShowcaseRegion regions={["label"]}>{tone}</ShowcaseRegion>
            <ShowcaseRegion regions={["row", "row--center"]}>
              {(["sm", "md", "lg"] as const).map((size) => (
                <Button key={size} tone={tone} size={size}>{size}</Button>
              ))}
              <Button tone={tone} disabled>disabled</Button>
              <Button tone={tone} iconLeft={<Icon name="plus" />}>with icon</Button>
            </ShowcaseRegion>
          </ShowcaseRegion>
        ))}
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>block</ShowcaseRegion>
          <Button tone="secondary" block iconLeft={<Icon name="download" />}>Full-width button</Button>
        </ShowcaseRegion>
      </Section>

      <Section title="IconButton" desc="Icon-only variant. Ghost by default.">
        <ShowcaseRegion regions={["row", "row--center"]}>
          {(["secondary", "ghost"] as const).map((tone) => (
            <ShowcaseRegion key={tone} regions={["col"]}>
              <ShowcaseRegion regions={["label"]}>{tone}</ShowcaseRegion>
              <ShowcaseRegion regions={["row", "row--center"]}>
                <IconButton tone={tone} size="sm" aria-label="settings sm"><Icon name="settings" /></IconButton>
                <IconButton tone={tone} size="md" aria-label="settings md"><Icon name="settings" /></IconButton>
              </ShowcaseRegion>
            </ShowcaseRegion>
          ))}
          <ShowcaseRegion regions={["col"]}>
            <ShowcaseRegion regions={["label"]}>active</ShowcaseRegion>
            <IconButton tone="ghost" aria-label="active" aria-pressed="true"><Icon name="filter" /></IconButton>
          </ShowcaseRegion>
        </ShowcaseRegion>
      </Section>

      <Section title="Badge" desc="Count pills and status indicators.">
        <ShowcaseRegion regions={["row", "row--center"]}>
          <ShowcaseRegion regions={["col"]}>
            <ShowcaseRegion regions={["label"]}>variants</ShowcaseRegion>
            <ShowcaseRegion regions={["row", "row--center"]}>
              <Badge>12</Badge>
              <Badge variant="accent">5</Badge>
              <Badge variant="solid">64</Badge>
              <Badge variant="dot" />
            </ShowcaseRegion>
          </ShowcaseRegion>
          <ShowcaseRegion regions={["col"]}>
            <ShowcaseRegion regions={["label"]}>in context</ShowcaseRegion>
            <ShowcaseRegion regions={["row", "row--center"]}>
              <Button tone="secondary" iconRight={<Badge>3</Badge>}>Pending</Button>
              <Button tone="ghost">Relations <Badge variant="solid">14</Badge></Button>
            </ShowcaseRegion>
          </ShowcaseRegion>
        </ShowcaseRegion>
      </Section>

      <Section title="Alert" desc="Inline status message.">
        <ShowcaseRegion regions={["col"]}>
          <Alert>Default: model validation completed.</Alert>
          <Alert variant="success">Success: all 640 elements parsed without errors.</Alert>
          <Alert variant="warning">Warning: 3 relations reference unknown target identifiers.</Alert>
          <Alert variant="danger">Error: schema version mismatch — expected 4, found 3.</Alert>
        </ShowcaseRegion>
      </Section>

      <Section title="Card" desc="Surface container with optional accent, interactive, and selected states.">
        <ShowcaseRegion regions={["row"]}>
          <ShowcaseRegion regions={["card-demo"]} asChild><Card >
            <ShowcaseRegion regions={["card-demo__title"]}>Default card</ShowcaseRegion>
            <ShowcaseRegion regions={["card-demo__desc"]}>Static surface</ShowcaseRegion>
          </Card></ShowcaseRegion>
          <ShowcaseRegion regions={["card-demo"]} asChild><Card interactive >
            <ShowcaseRegion regions={["card-demo__title"]}>Interactive</ShowcaseRegion>
            <ShowcaseRegion regions={["card-demo__desc"]}>Hover to see lift</ShowcaseRegion>
          </Card></ShowcaseRegion>
          <ShowcaseRegion regions={["card-demo"]} asChild><Card interactive selected >
            <ShowcaseRegion regions={["card-demo__title"]}>Selected</ShowcaseRegion>
            <ShowcaseRegion regions={["card-demo__desc"]}>Accent ring</ShowcaseRegion>
          </Card></ShowcaseRegion>
          <ShowcaseRegion regions={["card-demo"]} asChild><Card accentColorToken="--color-role-requirement" >
            <ShowcaseRegion regions={["card-demo__title"]}>Accent bar</ShowcaseRegion>
            <ShowcaseRegion regions={["card-demo__desc"]}>Left color rail</ShowcaseRegion>
          </Card></ShowcaseRegion>
        </ShowcaseRegion>
      </Section>

      <Section title="ExpandableViewport" desc="A persistent canvas frame that expands to the page. Close or Escape returns to the embedded view.">
        <ShowcaseRegion regions={["canvas"]}>
          <ExpandableViewport expanded={viewportExpanded} onExpandedChange={setViewportExpanded} label="Expanded preview">
            <Card>
              <ShowcaseRegion regions={["row", "row--center"]}>
                <IconButton aria-label={viewportExpanded ? "Close expanded preview" : "Expand preview"}
                  aria-expanded={viewportExpanded} data-expanded-close={viewportExpanded || undefined}
                  onClick={() => setViewportExpanded(value => !value)}>
                  <Icon name={viewportExpanded ? "x" : "maximize"} />
                </IconButton>
                <span>Preview content stays mounted while the viewport expands.</span>
              </ShowcaseRegion>
            </Card>
          </ExpandableViewport>
        </ShowcaseRegion>
      </Section>

      <Section title="Modal" desc="Portal-rendered dialog. Escape key and scrim click both close.">
        <ShowcaseRegion regions={["row"]}>
          <Button tone="secondary" onClick={() => setModalOpen(true)}>Open element detail modal</Button>
        </ShowcaseRegion>
        <Modal open={modalOpen} onOpenChange={setModalOpen}>
          <ModalContent>
            <ModalHeader>
              <ModalTitle>System Modeling Capability</ModalTitle>
            </ModalHeader>
            <ModalBody>
              <ShowcaseRegion regions={["modal-copy"]} as="p">
                As a <strong>System Engineer</strong>, I want a well-defined Reqvire model structure,
                so that I can manage system requirements and traceability across all project phases.
              </ShowcaseRegion>
            </ModalBody>
            <ModalFooter>
              <TextLink href="#">
                <Icon name="external-link" size={14} /> Open source page
              </TextLink>
              <ShowcaseRegion regions={["modal-footer-spacer"]} />
              <ModalClose asChild>
                <Button tone="primary" size="sm">Close</Button>
              </ModalClose>
            </ModalFooter>
          </ModalContent>
        </Modal>
      </Section>

    </ShowcaseRegion>
  );
}
