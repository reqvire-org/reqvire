import type { ComponentChildren } from "preact";
interface ShowcaseDetailElement {
    id: string;
    title: string;
    elementType: string;
    typeFamily: string;
    sourceHref: string;
    metaBadges: DetailMetaBadge[];
    content: ComponentChildren;
    relations: DetailRelationItem[];
    contract_bindings: DetailContractBindingItem[];
}
import type { DetailContractBindingItem, DetailMetaBadge, DetailRelationItem, OntologyDetailNode } from "@ds";
import { MarkdownFrame } from "@ds";

export const fixture = {"title":"Detail Patterns","description":"Element and ontology detail bodies accept DTO-shaped props with inert showcase callbacks."};
const DETAIL_META_BADGES: DetailMetaBadge[] = [
    { key: "owner", value: "systems", provenance: "explicit" },
    { key: "priority", value: "high", provenance: "explicit" },
    { key: "lifecycle", value: "approved", provenance: "inherited" },
];

const DETAIL_RELATIONS: DetailRelationItem[] = [
    {
        id: "rel-verifies",
        label: "verifiedBy",
        target: {
            id: "VER-DET-010",
            label: "Coverage Export Verification",
            kind: "element",
            elementType: "test-verification",
            typeFamily: "verification",
            href: "#/model/VER-DET-010",
            external: false,
        },
    },
    {
        id: "rel-derived",
        label: "derivedFrom",
        target: {
            id: "CAP-DET-001",
            label: "Project traceability capability",
            kind: "element",
            elementType: "capability",
            typeFamily: "capability",
            href: "#/model/CAP-DET-001",
            external: false,
        },
    },
    {
        id: "rel-source",
        label: "specifiedIn",
        target: {
            id: "system-model/SystemRequirements.md",
            label: "SystemRequirements.md",
            kind: "resource",
            href: "#/content/system-model/SystemRequirements.md",
            external: false,
        },
    },
];

const DETAIL_ATTACHMENTS: DetailContractBindingItem[] = [
    {
        id: "att-report",
        targetId: "reports/coverage-summary.md",
        kind: "resource",
        label: "Coverage summary report",
        href: "#/content/reports/coverage-summary.md",
        external: false,
    },
    {
        id: "att-graph",
        targetId: "visualization/traceability-graph.json",
        kind: "resource",
        label: "Traceability graph export",
        href: "#/content/visualization/traceability-graph.json",
        external: false,
    },
];

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
const PRIMARY_DETAIL_ELEMENT_ID = "REQ-DET-042";

const SHOWCASE_DETAIL_ELEMENTS: Record<string, ShowcaseDetailElement> = {
    "SPEC-DET-001": {
        id: "SPEC-DET-001", title: "Coverage Report Specification", elementType: "specification",
        typeFamily: "contract", sourceHref: "#/content/system-model/Specifications.md",
        metaBadges: [], content: <p>Coverage reports distinguish verification and implementation evidence.</p>,
        relations: [], contract_bindings: [],
    },
    "REQ-DET-042": {
        id: "REQ-DET-042",
        title: "Traceability Coverage Requirement",
        elementType: "requirement",
        typeFamily: "requirement",
        sourceHref: "#/content/system-model/SystemRequirements.md",
        metaBadges: DETAIL_META_BADGES,
        content: (<MarkdownFrame>
        <p>
          The coverage report lists every approved requirement and its verification evidence.
        </p>
      </MarkdownFrame>),
        relations: DETAIL_RELATIONS,
        contract_bindings: DETAIL_ATTACHMENTS,
    },
    "VER-DET-010": {
        id: "VER-DET-010",
        title: "Coverage Export Verification",
        elementType: "test-verification",
        typeFamily: "verification",
        sourceHref: "#/content/system-model/Verifications.md",
        metaBadges: DETAIL_META_BADGES,
        content: (<MarkdownFrame>
        <p>
          The verification checks that exported coverage reports include requirements,
          verification links, and uncovered obligations.
        </p>
      </MarkdownFrame>),
        relations: [
            {
                id: "rel-verifies-requirement",
                label: "verify",
                target: {
                    id: "REQ-DET-042",
                    label: "Traceability Coverage Requirement",
                    kind: "element",
                    elementType: "requirement",
                    typeFamily: "requirement",
                    href: "#/model/REQ-DET-042",
                    external: false,
                },
            },
            {
                id: "rel-verification-capability",
                label: "derivedFrom",
                target: {
                    id: "CAP-DET-001",
                    label: "Project traceability capability",
                    kind: "element",
                    elementType: "capability",
                    typeFamily: "capability",
                    href: "#/model/CAP-DET-001",
                    external: false,
                },
            },
        ],
        contract_bindings: DETAIL_ATTACHMENTS,
    },
    "CAP-DET-001": {
        id: "CAP-DET-001",
        title: "Project traceability capability",
        elementType: "capability",
        typeFamily: "capability",
        sourceHref: "#/content/system-model/Capabilities.md",
        metaBadges: DETAIL_META_BADGES,
        content: (<MarkdownFrame>
        <p>
          This capability organizes model traceability from requirements through verification
          evidence and coverage reporting.
        </p>
      </MarkdownFrame>),
        relations: [
            {
                id: "rel-capability-requirement",
                label: "specifiedBy",
                target: {
                    id: "REQ-DET-042",
                    label: "Traceability Coverage Requirement",
                    kind: "element",
                    elementType: "requirement",
                    typeFamily: "requirement",
                    href: "#/model/REQ-DET-042",
                    external: false,
                },
            },
        ],
        contract_bindings: [],
    },
};
export { DETAIL_ATTACHMENTS, DETAIL_META_BADGES, DETAIL_RELATIONS, ONTOLOGY_COMPACT_NODE, ONTOLOGY_NODES, ONTOLOGY_REQUIREMENT_NODE, PRIMARY_DETAIL_ELEMENT_ID, SHOWCASE_DETAIL_ELEMENTS };
