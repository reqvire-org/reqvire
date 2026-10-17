import { ShowcaseRegion } from "@ds/review";
import { useState } from "preact/hooks";
import { type ComponentChildren } from "preact";
import { DetailDialog, Button, ElementDetailContent, MarkdownFrame, OntologyMetadata, OntologyNodeDetailContent, OntologyNodeDetailDialog, RelationList, TypeBadge } from "@ds";
import { fixture, DETAIL_META_BADGES, DETAIL_RELATIONS, ONTOLOGY_COMPACT_NODE, ONTOLOGY_NODES, ONTOLOGY_REQUIREMENT_NODE, PRIMARY_DETAIL_ELEMENT_ID, SHOWCASE_DETAIL_ELEMENTS } from "./ElementDetailContent.fixture";
export const scenario = {"title":"Detail Patterns",covers: ["DetailDialog","ElementDetailContent","MarkdownFrame","RelationList","OntologyNodeDetailContent","OntologyMetadata","OntologyNodeDetailDialog"]};
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
const [elementDialogOpen, setElementDialogOpen] = useState(false);
const [elementDialogStack, setElementDialogStack] = useState<string[]>([]);
const [ontologyDialogNodeId, setOntologyDialogNodeId] = useState<string | null>(null);
const [copiedUri, setCopiedUri] = useState(false);
const ontologyDialogNode = ONTOLOGY_NODES.find((node) => node.id === ontologyDialogNodeId);
const openElementDialog = (id = PRIMARY_DETAIL_ELEMENT_ID) => {
    const nextId = SHOWCASE_DETAIL_ELEMENTS[id] ? id : PRIMARY_DETAIL_ELEMENT_ID;
    setElementDialogStack([nextId]);
    setElementDialogOpen(true);
};
const currentElement = SHOWCASE_DETAIL_ELEMENTS[elementDialogStack.at(-1) ?? "REQ-DET-042"];
const pushElementDialog = (id: string) => { if (SHOWCASE_DETAIL_ELEMENTS[id]) setElementDialogStack(stack => [...stack,id]); };
return <ShowcaseRegion regions={["page"]}>
<Section title={fixture.title} desc={fixture.description}>
        <ShowcaseRegion regions={["pattern-grid", "pattern-grid--two"]}>
          <ShowcaseRegion regions={["pattern-card"]}>
            <ShowcaseRegion regions={["pattern-toolbar"]}>
              <TypeBadge type="requirement" family="requirement" tinted>
                requirement
              </TypeBadge>
              <Button tone="secondary" size="sm" onClick={() => openElementDialog()}>
                Open dialog
              </Button>
            </ShowcaseRegion>
            <ElementDetailContent metaBadges={DETAIL_META_BADGES} content={<MarkdownFrame>
                  <p>
                    As a systems engineer, I need release coverage reports to expose every
                    requirement without verification evidence.
                  </p>
                </MarkdownFrame>} relations={DETAIL_RELATIONS} contract_references={[{ id: "reference-coverage-contract", targetId: "SPEC-DET-001",
            kind: "element", label: "Coverage Report Specification", elementType: "specification",
            typeFamily: "contract", href: "#/elements/SPEC-DET-001", external: false }]} onOpenElement={openElementDialog} onOpenResource={() => { }}/>
          </ShowcaseRegion>

          <ShowcaseRegion regions={["pattern-card"]}>
            <ShowcaseRegion regions={["pattern-toolbar"]}>
              <TypeBadge type="requirement" family="requirement" tinted>
                relations
              </TypeBadge>
            </ShowcaseRegion>
            <RelationList relations={DETAIL_RELATIONS} onOpenElement={openElementDialog} onOpenResource={() => { }}/>
          </ShowcaseRegion>

          <ShowcaseRegion regions={["pattern-card"]}>
            <ShowcaseRegion regions={["pattern-toolbar"]}>
              <TypeBadge type="ontology" family="ontology" tinted>
                ontology
              </TypeBadge>
              <Button tone="secondary" size="sm" onClick={() => setOntologyDialogNodeId(ONTOLOGY_REQUIREMENT_NODE.id)}>
                Open dialog
              </Button>
            </ShowcaseRegion>
            <OntologyNodeDetailContent node={ONTOLOGY_COMPACT_NODE} nodes={[ONTOLOGY_COMPACT_NODE]} copiedUri={copiedUri} onCopyUri={() => setCopiedUri(true)} onOpenSource={() => { }}/>
          </ShowcaseRegion>
        </ShowcaseRegion>
        <ShowcaseRegion regions={["canvas"]}>
          <OntologyMetadata node={ONTOLOGY_REQUIREMENT_NODE} copiedUri={copiedUri} onCopyUri={() => setCopiedUri(true)}/>
        </ShowcaseRegion>
      </Section>
<OntologyNodeDetailDialog open={ontologyDialogNodeId !== null} onOpenChange={(open) => {
        if (!open)
            setOntologyDialogNodeId(null);
    }} node={ontologyDialogNode} nodes={ONTOLOGY_NODES} missingNodeId={ontologyDialogNodeId} copiedUri={copiedUri} onCopyUri={() => setCopiedUri(true)} onOpenSource={() => { }}/>
<DetailDialog open={elementDialogOpen} onOpenChange={setElementDialogOpen} title={currentElement.title} typeFamily={currentElement.typeFamily} elementType={currentElement.elementType} sourceHref={currentElement.sourceHref} onOpenSource={() => {}}>
<ElementDetailContent metaBadges={currentElement.metaBadges} content={currentElement.content} relations={currentElement.relations} contract_bindings={currentElement.contract_bindings} onOpenElement={pushElementDialog} onOpenResource={() => {}} />
</DetailDialog>
</ShowcaseRegion>;
}
