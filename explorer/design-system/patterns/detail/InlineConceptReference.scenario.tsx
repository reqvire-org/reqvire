import { ShowcaseRegion } from "@ds";
import { useState } from "preact/hooks";
import { type ComponentChildren } from "preact";
import { InlineConceptReference, OntologyNodeDetailDialog } from "@ds";
import { fixture, ONTOLOGY_NODES, ONTOLOGY_REQUIREMENT_NODE } from "./InlineConceptReference.fixture";
export const scenario = {"title":"Inline concept reference","covers":["ShowcaseRegion","InlineConceptReference","OntologyNodeDetailDialog"]};
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
const [ontologyDialogNodeId, setOntologyDialogNodeId] = useState<string | null>(null);
const [copiedUri, setCopiedUri] = useState(false);
const ontologyDialogNode = ONTOLOGY_NODES.find((node) => node.id === ontologyDialogNodeId);
return <ShowcaseRegion regions={["page"]}>
<Section title={fixture.title} desc={fixture.description}>
        <InlineConceptReference reference={{ id: "fixture-concept-reference", iri: ONTOLOGY_REQUIREMENT_NODE.full_uri ?? ONTOLOGY_REQUIREMENT_NODE.id,
        label: ONTOLOGY_REQUIREMENT_NODE.label ?? "Requirement", ontologyNodeId: ONTOLOGY_REQUIREMENT_NODE.id }} onOpenConceptReference={() => setOntologyDialogNodeId(ONTOLOGY_REQUIREMENT_NODE.id)}/>
      </Section>
<OntologyNodeDetailDialog open={ontologyDialogNodeId !== null} onOpenChange={(open) => {
        if (!open)
            setOntologyDialogNodeId(null);
    }} node={ontologyDialogNode} nodes={ONTOLOGY_NODES} missingNodeId={ontologyDialogNodeId} copiedUri={copiedUri} onCopyUri={() => setCopiedUri(true)} onOpenSource={() => { }}/>
</ShowcaseRegion>;
}
