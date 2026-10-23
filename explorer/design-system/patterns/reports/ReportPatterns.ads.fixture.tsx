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
import type { CoverageDrilldownItem, CoverageDrilldownTarget } from "@ds";
import type { DetailContractBindingItem, DetailMetaBadge, DetailRelationItem } from "@ds";
import { MarkdownFrame } from "@ds";

export const fixture = {"title":"Coverage Drill-down","description":"The same coverage component used by Explorer, with shared Verification and Implementation column headings. Expand capabilities and parent requirements to inspect their children. Select requirement names in Mocks to inspect evidence through the full application."};
const requirement: CoverageDrilldownTarget = {
    id: "REQ-DET-042", label: "Traceability Coverage Requirement", kind: "element",
    elementType: "requirement", href: "#/elements/REQ-DET-042", external: false,
};

// Fixture data for the public coverage patterns. Application integration tests own full Project Stores.
const COVERAGE_PATTERN_ITEMS: CoverageDrilldownItem[] = [{
        target: { id: "example-capability", label: "Traceability Reporting", kind: "element",
            elementType: "capability", href: null, external: false },
        verification: { covered: 1, total: 1 },
        implementation: { covered: 1, total: 1, complete: true },
        children: [{
                target: requirement,
                verification: { covered: 1, total: 1 },
                implementation: { covered: 1, total: 1, complete: true },
                assessment: {
                    blockers: [], contributions: [],
                },
            }],
    }];

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
export { COVERAGE_PATTERN_ITEMS, DETAIL_ATTACHMENTS, DETAIL_META_BADGES, DETAIL_RELATIONS, SHOWCASE_DETAIL_ELEMENTS };
