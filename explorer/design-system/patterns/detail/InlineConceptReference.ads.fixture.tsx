import type { OntologyDetailNode } from "@ds";

export const fixture = {"title":"Inline concept reference","description":"A public concept reference opens the existing ontology detail dialog."};
const ONTOLOGY_REQUIREMENT_NODE: OntologyDetailNode = {
    id: "https://reqvire.dev/ontology#Requirement",
    label: "Requirement",
    semantic_type: "class",
    full_uri: "https://reqvire.dev/ontology#Requirement",
    comment: "A normative statement that constrains or specifies expected system behavior.",
    rdf_types: ["owl:Class", "rdfs:Class"],
    badges: [
        { kind: "class", symbol: "CLS", label: "Class" },
        { kind: "shape", symbol: "SH", label: "Validated by SHACL" },
    ],
    slot_facets: [
        {
            slot_label: "verifiedBy",
            slot_iri: "https://reqvire.dev/ontology#verifiedBy",
            slot_kind: "object-property",
            target_class_label: "Verification",
            target_class_iri: "https://reqvire.dev/ontology#Verification",
            source_shape_label: "RequirementShape",
            source_shape_iri: "https://reqvire.dev/shapes#RequirementShape",
            facets: [
                { name: "minCount", value: "1" },
                { name: "severity", value: "Violation" },
            ],
        },
    ],
    constructs: [
        {
            id: "requirement-subclass",
            kind: "subclass-inclusion",
            label: "Requirement is a model element",
            subject: "Requirement",
            predicate: "rdfs:subClassOf",
            object: "ModelElement",
            source: {
                source_name: "core ontology",
                file_path: "ontologies/core.ttl",
                line_number: 42,
                kind: "ttl",
            },
        },
    ],
    literal_values: [
        { predicate: "skos:prefLabel", value: "Requirement" },
        { predicate: "reqvire:displayOrder", value: "20" },
    ],
    constraints: [
        { property: "sh:targetClass", value: "reqvire:Requirement" },
    ],
    sources: [
        {
            source_name: "core ontology",
            file_path: "ontologies/core.ttl",
            line_number: 42,
            kind: "ttl",
            link: "#/content/ontologies/core.ttl",
        },
        {
            source_name: "requirement shape",
            file_path: "ontologies/shapes.ttl",
            line_number: 88,
            kind: "shacl",
            link: "#/content/ontologies/shapes.ttl",
        },
    ],
};

const ONTOLOGY_COMPACT_NODE: OntologyDetailNode = {
    id: "https://reqvire.dev/ontology#CoverageReport",
    label: "CoverageReport",
    semantic_type: "class",
    full_uri: "https://reqvire.dev/ontology#CoverageReport",
    comment: "Compact showcase node for the inline ontology detail pattern.",
    rdf_types: ["owl:Class"],
    badges: [
        { kind: "class", symbol: "CLS", label: "Class" },
    ],
    sources: [
        {
            source_name: "report ontology",
            file_path: "ontologies/reports.ttl",
            line_number: 24,
            kind: "ttl",
            link: "#/content/ontologies/reports.ttl",
        },
    ],
};

const ONTOLOGY_NODES: OntologyDetailNode[] = [
    ONTOLOGY_REQUIREMENT_NODE,
    ONTOLOGY_COMPACT_NODE,
    {
        id: "https://reqvire.dev/ontology#Traceability",
        label: "Traceability",
        semantic_type: "class",
        full_uri: "https://reqvire.dev/ontology#Traceability",
        comment: "A semantic concept for navigable relationships between model elements.",
        rdf_types: ["owl:Class"],
        sources: [
            {
                source_name: "traceability ontology",
                file_path: "ontologies/traceability.ttl",
                line_number: 18,
                kind: "ttl",
                link: "#/content/ontologies/traceability.ttl",
            },
        ],
    },
    {
        id: "https://reqvire.dev/ontology#VerificationEvidence",
        label: "VerificationEvidence",
        semantic_type: "class",
        full_uri: "https://reqvire.dev/ontology#VerificationEvidence",
        comment: "A semantic concept for evidence that supports verification claims.",
        rdf_types: ["owl:Class"],
        sources: [
            {
                source_name: "verification ontology",
                file_path: "ontologies/verification.ttl",
                line_number: 31,
                kind: "ttl",
                link: "#/content/ontologies/verification.ttl",
            },
        ],
    },
    {
        id: "https://reqvire.dev/ontology#verifiedBy",
        label: "verifiedBy",
        semantic_type: "object-property",
        full_uri: "https://reqvire.dev/ontology#verifiedBy",
        domain: [
            {
                label: "Requirement",
                iri: "https://reqvire.dev/ontology#Requirement",
                kind: "class",
            },
        ],
        range: [
            {
                label: "Verification",
                iri: "https://reqvire.dev/ontology#Verification",
                kind: "class",
            },
        ],
    },
    {
        id: "https://reqvire.dev/ontology#specifiedIn",
        label: "specifiedIn",
        semantic_type: "object-property",
        full_uri: "https://reqvire.dev/ontology#specifiedIn",
        domain: [
            {
                label: "Requirement",
                iri: "https://reqvire.dev/ontology#Requirement",
                kind: "class",
            },
        ],
        range: [
            {
                label: "Resource",
                iri: "https://reqvire.dev/ontology#Resource",
                kind: "class",
            },
        ],
    },
];

export { ONTOLOGY_NODES, ONTOLOGY_REQUIREMENT_NODE };
