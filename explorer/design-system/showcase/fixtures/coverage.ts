import type { CoverageDrilldownItem, CoverageDrilldownTarget } from "@ds";

const requirement: CoverageDrilldownTarget = {
  id: "REQ-DET-042", label: "Traceability Coverage Requirement", kind: "element",
  elementType: "requirement", href: "#/elements/REQ-DET-042", external: false,
};

// Fixture data for the public coverage patterns. Application integration tests own full Project Stores.
export const COVERAGE_PATTERN_ITEMS: CoverageDrilldownItem[] = [{
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
