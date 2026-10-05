import type { CoverageDrilldownItem, CoverageDrilldownTarget } from "@ds";

const requirement: CoverageDrilldownTarget = {
  id: "REQ-DET-042", label: "Traceability Coverage Requirement", kind: "element",
  elementType: "requirement", href: "#/elements/REQ-DET-042", external: false,
};

// Presentation data only. The Mocks tab mounts the real App with its generated Project Store.
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
