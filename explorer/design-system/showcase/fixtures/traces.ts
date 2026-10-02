import type { TraceFlowData, TraceFlowElement, TraceFlowRequirement } from "@ds";

export interface TraceExample {
  label: string;
  trace: TraceFlowData;
  descriptions: Record<string, string>;
}

const architecture = "system-model/Architecture/OntologyKernelRequirements.md";
const checks = "system-model/Verifications/Architecture/OntologyKernelVerifications.md";
export const TRACE_CAPABILITY = {
  id: "ontology-exploration", name: "Ontology Exploration", file: "system-model/Capabilities/Ontology.md",
  description: "Explore ontology services, projections, and their verification paths.",
};
const requirement = (id: string, name: string, parentIds: string[], directlyVerified = false,
  file = architecture): TraceFlowRequirement => ({ id, name, parentIds, directlyVerified, file });
const verification = (id: string, name: string): TraceFlowData["verification"] => ({
  id, name, file: checks, type: "test-verification",
});

export const SPLIT_MERGE_TRACE: TraceExample = {
  label: "Repeated splits and merges",
  trace: {
    verification: { ...verification("split-merge-test", "Split and Merge Trace Verification"),
      file: "system-model/Verifications/TraceTopologyVerifications.md" },
    requirements: [
      requirement("input-a", "Source Parsing", ["normalize", "trace-root"], true),
      requirement("input-b", "Reference Resolution", ["normalize", "source-context"], true),
      requirement("normalize", "Normalized Model", ["report-export", "interactive-view"]),
      requirement("source-context", "Source Context", ["interactive-view"]),
      requirement("report-export", "Report Export", ["presentation"]),
      requirement("interactive-view", "Interactive View", ["presentation"]),
      requirement("presentation", "Model Presentation", ["trace-root"]),
      requirement("trace-root", "Model Exploration", []),
    ],
  },
  descriptions: {
    "split-merge-test": "Exercises paths that split and merge repeatedly, including a direct shortcut to an ancestor.",
    "input-a": "The system shall parse model source content.",
    "input-b": "The system shall resolve references between model elements.",
    normalize: "The system shall provide a normalized model for downstream consumers.",
    "source-context": "The system shall preserve source context for interactive navigation.",
    "report-export": "The system shall export model reports.",
    "interactive-view": "The system shall present an interactive model view.",
    presentation: "The system shall present model information through shared interfaces.",
    "trace-root": "The system shall support model exploration.",
  },
};

export const TRACE_EXAMPLES: readonly TraceExample[] = [
  SPLIT_MERGE_TRACE,
  {
    label: "Simple chain",
    trace: {
      verification: verification("classification-test", "O-Kernel Ontology Classification Unit Test Verification"),
      requirements: [
        requirement("classification", "Ontology Construct Classification", ["algorithms"], true),
        requirement("algorithms", "SHACL and Ontology Algorithm Services", []),
      ],
    },
    descriptions: {
      "classification-test": "Checks classification of ontology classes, properties, individuals, and SHACL shapes.",
      classification: "The system shall classify ontology constructs for use by Reqvire projections and reports.",
      algorithms: "The system shall provide shared SHACL and ontology algorithm services.",
    },
  },
  {
    label: "Shared ancestor",
    trace: {
      verification: verification("projection-test", "Ontology Projection Verification"),
      requirements: [
        requirement("class-projection", "Class Projection", ["projection"], true),
        requirement("property-projection", "Property Projection", ["projection", "property-resolution"], true),
        requirement("projection", "Ontology Projection", ["services"]),
        requirement("property-resolution", "Property Resolution", []),
        requirement("services", "Ontology Services", []),
      ],
    },
    descriptions: {
      "projection-test": "Checks the class and property branches of the ontology projection.",
      "class-projection": "The system shall project ontology classes with their source identifiers.",
      "property-projection": "The system shall project properties with domain and range relations.",
      projection: "The system shall provide a common ontology projection for downstream views.",
      "property-resolution": "The system shall resolve property definitions from the model vocabulary.",
      services: "The system shall expose reusable ontology services.",
    },
  },
  {
    label: "Long names",
    trace: {
      verification: { ...verification("vocabulary-test", "Built-In External Ontology Vocabulary Resolution and Source Attribution Verification"),
        file: "system-model/Verifications/Semantics/ExternalVocabularyVerifications.md" },
      requirements: [
        requirement("vocabulary-resolution", "Built-In External Ontology Source Resolution for Referenced Vocabulary Terms", ["vocabulary-exposure"], true,
          "system-model/Processing/Semantics/ExternalVocabularyResolutionRequirements.md"),
        requirement("vocabulary-exposure", "External Vocabulary Description Construction and Exposure Policy", [], false,
          "system-model/Processing/Semantics/ExternalVocabularyExposureRequirements.md"),
      ],
    },
    descriptions: {
      "vocabulary-test": "Checks vocabulary resolution and attribution to the original ontology source.",
      "vocabulary-resolution": "The system shall resolve referenced terms from built-in external ontology sources.",
      "vocabulary-exposure": "The system shall expose descriptions of the external vocabulary used by the model.",
    },
  },
  {
    label: "Unlinked verification",
    trace: {
      verification: { ...verification("unlinked-test", "Pending Vocabulary Verification"),
        file: "system-model/Verifications/Semantics/ExternalVocabularyVerifications.md" },
      requirements: [],
    },
    descriptions: { "unlinked-test": "Verification awaiting requirement trace links." },
  },
];

export function traceExampleElements(example: TraceExample): TraceFlowElement[] {
  return [example.trace.verification, ...example.trace.requirements];
}

export function traceExampleSource(example: TraceExample, file: string): string {
  return "# Elements\n\n" + traceExampleElements(example).filter(element => element.file === file).map(element => {
    const isVerification = element.id === example.trace.verification.id;
    const relations = isVerification
      ? example.trace.requirements.filter(item => item.directlyVerified).map(item => `  * verify: [${item.name}](${item.file}#${item.id})`)
      : example.trace.requirements.find(item => item.id === element.id)!.parentIds.map(id => {
        const parent = example.trace.requirements.find(item => item.id === id)!;
        return `  * derivedFrom: [${parent.name}](${parent.file}#${parent.id})`;
      });
    if (!isVerification && relations.length === 0) relations.push(`  * specify: [${TRACE_CAPABILITY.name}](${TRACE_CAPABILITY.file}#${TRACE_CAPABILITY.id})`);
    return `### ${element.name}\n\n${example.descriptions[element.id]}\n\n#### Metadata\n  * type: ${isVerification ? example.trace.verification.type : "requirement"}\n`
      + (relations.length ? `\n#### Relations\n${relations.join("\n")}\n` : "");
  }).join("\n---\n\n");
}
