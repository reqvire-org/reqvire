import type { ComponentChildren } from "preact";
import { TextLink } from "@ds";
import { ShowcaseRegion } from "@ds/review";
import { useState } from "preact/hooks";
import { Button, Icon, Modal, ModalBody, ModalDescription, ModalClose, ModalContent, ModalFooter, ModalHeader, ModalTitle } from "@ds";
import { fixture } from "./Modal.fixture";
export const scenario = {"title":"Modal",covers: ["Button","Modal","ModalContent","ModalHeader","ModalTitle","ModalBody","ModalDescription","ModalFooter","TextLink","Icon","ModalClose"]};
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
const [modalOpen, setModalOpen] = useState(false);
return <ShowcaseRegion regions={["page"]}>
<Section title={fixture.title} desc={fixture.description}>
        <ShowcaseRegion regions={["row"]}>
          <Button tone="secondary" onClick={() => setModalOpen(true)}>Open element detail modal</Button>
        </ShowcaseRegion>
        <Modal open={modalOpen} onOpenChange={setModalOpen}>
          <ModalContent>
            <ModalHeader>
              <ModalTitle>System Modeling Capability</ModalTitle>
            </ModalHeader>
            <ModalDescription>A review of the modal component.</ModalDescription><ModalBody>
              <ShowcaseRegion regions={["modal-copy"]} as="p">
                As a <strong>System Engineer</strong>, I want a well-defined Reqvire model structure,
                so that I can manage system requirements and traceability across all project phases.
              </ShowcaseRegion>
            </ModalBody>
            <ModalFooter>
              <TextLink href="#">
                <Icon name="external-link" size={14}/> Open source page
              </TextLink>
              <ShowcaseRegion regions={["modal-footer-spacer"]}/>
              <ModalClose asChild>
                <Button tone="primary" size="sm">Close</Button>
              </ModalClose>
            </ModalFooter>
          </ModalContent>
        </Modal>
      </Section>
</ShowcaseRegion>;
}
