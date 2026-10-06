import type { CoverageDrilldownItem, CoverageDrilldownTarget } from "@ds";
import { routeForElement } from "../router/routes";
import type { CapabilityCoverageDetails, RequirementCoverageDetails, CoverageProjection, ExplorerProjectStore } from "../store/types";

export type CoverageRow = CapabilityCoverageDetails & { depth: number };

type CapabilityIdentity = { identifier: string; name: string };
const compareCapabilityNames = (a: CapabilityIdentity, b: CapabilityIdentity) =>
  a.name.localeCompare(b.name) || a.identifier.localeCompare(b.identifier);

/** List every published scope using the same display parents as the coverage rows. */
export function coverageScopeOptions(store: ExplorerProjectStore): (CapabilityIdentity & { depth: number })[] {
  const scopes = Object.values(store.coverage.scope_index).map(entry => ({
    identifier: entry.scope.capability_identifier, name: entry.scope.capability_name,
  }));
  return capabilityHierarchy(store, scopes, compareCapabilityNames);
}

/** Select already-classified records; membership and totals come from the shared report. */
export function projectCoverage(coverage: CoverageProjection, scopeId: string | null): CoverageProjection {
  const entry = scopeId ? coverage.scope_index[scopeId] : undefined;
  if (!entry) return coverage;
  const requirements = new Set(entry.scope.requirement_ids);
  const verifications = new Set(entry.scope.verification_ids);
  const capabilities = new Set(entry.scope.capability_ids);
  return {
    ...coverage,
    scope: entry.scope,
    summary: entry.summary,
    verified_leaf_requirements: selectRecords(coverage.verified_leaf_requirements, requirements),
    unverified_leaf_requirements: selectRecords(coverage.unverified_leaf_requirements, requirements),
    covered_requirements: selectRecords<RequirementCoverageDetails>(coverage.covered_requirements, requirements),
    uncovered_requirements: selectRecords<RequirementCoverageDetails>(coverage.uncovered_requirements, requirements),
    satisfied_test_verifications: selectRecords(coverage.satisfied_test_verifications, verifications),
    unsatisfied_test_verifications: selectRecords(coverage.unsatisfied_test_verifications, verifications),
    orphaned_verifications: { files: {} },
    capability_coverage: { capabilities: coverage.capability_coverage.capabilities.filter(row => capabilities.has(row.identifier)) },
  };
}

function selectRecords<T extends { identifier: string }>(section: unknown, ids: Set<string>) {
  const files = (section as { files?: Record<string, T[]> } | undefined)?.files ?? {};
  return { files: Object.fromEntries(Object.entries(files).map(([file, entries]) =>
    [file, entries.filter(entry => ids.has(entry.identifier))] as const).filter(([, entries]) => entries.length)) };
}

/** Parent choices affect row order only, never report subjects or coverage evidence. */
export function coverageRows(store: ExplorerProjectStore, coverage: CoverageProjection): CoverageRow[] {
  const compareCoverage = (a: CapabilityCoverageDetails, b: CapabilityCoverageDetails) =>
    (a.verification_coverage_percentage ?? 0) - (b.verification_coverage_percentage ?? 0)
    || (a.implementation_coverage_percentage ?? 0) - (b.implementation_coverage_percentage ?? 0) || compareCapabilityNames(a, b);
  return capabilityHierarchy(store, coverage.capability_coverage.capabilities, compareCoverage);
}

function capabilityHierarchy<T extends CapabilityIdentity>(
  store: ExplorerProjectStore, rows: T[], compare: (a: T, b: T) => number,
): (T & { depth: number })[] {
  const ids = new Set(rows.map(row => row.identifier));
  const parents = new Map<string, Set<string>>();
  for (const relation of store.relations) {
    let parent: string, child: string;
    if (relation.relation_type === "derive") {
      parent = relation.source_id; child = relation.target_id;
    } else if (relation.relation_type === "derivedFrom") {
      parent = relation.target_id; child = relation.source_id;
    } else continue;
    if (!ids.has(parent) || !ids.has(child)) continue;
    const choices = parents.get(child) ?? new Set<string>();
    choices.add(parent);
    parents.set(child, choices);
  }
  const roots = rows.filter(row => !parents.has(row.identifier)).sort(compare);
  const children = new Map<string, T[]>();
  for (const row of rows) {
    const parent = [...(parents.get(row.identifier) ?? [])].sort()[0];
    if (parent) children.set(parent, [...(children.get(parent) ?? []), row]);
  }
  const result: (T & { depth: number })[] = [];
  const visited = new Set<string>();
  const visit = (row: T, depth: number) => {
    if (visited.has(row.identifier)) return;
    visited.add(row.identifier);
    result.push({ ...row, depth });
    for (const child of (children.get(row.identifier) ?? []).sort(compare)) visit(child, depth + 1);
  };
  for (const root of roots) visit(root, 0);
  return result;
}

export const COVERAGE_SOURCE_LABELS: Record<string, string> = {
  direct_satisfied: "Direct evidence",
  requirement_rollup: "Requirement rollup",
  contract_consumer_rollup: "Contract consumer rollup",
  combined_rollup: "Combined rollup",
  uncovered: "Uncovered",
};

function records<T>(section: unknown): T[] {
  return Object.values((section as { files?: Record<string, T[]> } | undefined)?.files ?? {}).flat();
}

/** Arrange published assessments for presentation; never evaluate coverage or extend scope. */
export function coverageDrilldownItems(store: ExplorerProjectStore, coverage: CoverageProjection): CoverageDrilldownItem[] {
  const elements = new Map(store.elements.map(element => [element.id, element]));
  const scope = coverage.scope ? new Set(coverage.scope.requirement_ids) : null;
  const endpoint = (id: string): CoverageDrilldownTarget => {
    const element = elements.get(id);
    if (element) return { id, label: element.name, kind: "element", elementType: element.element_type,
      typeFamily: element.type_family, href: routeForElement(id), external: false,
      outsideScope: Boolean(scope && element.element_type === "requirement" && !scope.has(id)) };
    return { id, label: id, kind: "element", href: routeForElement(id), external: false };
  };
  const covered = records<RequirementCoverageDetails>(coverage.covered_requirements);
  const uncovered = records<RequirementCoverageDetails>(coverage.uncovered_requirements);
  const coveredIds = new Set(covered.map(record => record.identifier));
  const fullCovered = new Set(records<RequirementCoverageDetails>(store.coverage.covered_requirements).map(record => record.identifier));
  const fullUncovered = new Set(records<RequirementCoverageDetails>(store.coverage.uncovered_requirements).map(record => record.identifier));
  const hierarchyChildren = new Map<string, Set<string>>();
  for (const relation of store.relations) {
    if (relation.relation_type !== "derive" && relation.relation_type !== "derivedFrom") continue;
    const parent = relation.relation_type === "derive" ? relation.source_id : relation.target_id;
    const child = relation.relation_type === "derive" ? relation.target_id : relation.source_id;
    const children = hierarchyChildren.get(parent) ?? new Set<string>();
    children.add(child); hierarchyChildren.set(parent, children);
  }
  const requirements = new Map<string, CoverageDrilldownItem>();
  for (const record of [...covered, ...uncovered]) {
    const complete = coveredIds.has(record.identifier);
    const blockers = (record.blocking_requirements).filter(id => id !== record.identifier);
    requirements.set(record.identifier, {
      target: { ...endpoint(record.identifier), label: record.name, kind: "element", elementType: "requirement", href: routeForElement(record.identifier) },
      verification: { covered: record.aggregate_verified_leaf_requirements, total: record.aggregate_leaf_requirements },
      implementation: { covered: record.aggregate_covered_terminal_requirements,
        total: record.aggregate_terminal_requirements, complete },
      children: [],
      assessment: {
        blockers: blockers.map(endpoint),
        contributions: record.contributing_requirements.map(id => ({ ...endpoint(id),
          dependencyKind: hierarchyChildren.get(record.identifier)?.has(id) ? "child" : "contract",
          implementationCovered: fullCovered.has(id) ? true : fullUncovered.has(id) ? false : undefined,
        })),
      },
    });
  }

  const roots: CoverageDrilldownItem[] = [];
  const capabilities = new Map<string, CoverageDrilldownItem>();
  const ancestors: CoverageDrilldownItem[] = [];
  for (const row of coverageRows(store, coverage)) {
    const item: CoverageDrilldownItem = {
      target: { ...endpoint(row.identifier), label: row.name, kind: "element", elementType: "capability", href: routeForElement(row.identifier) },
      verification: { covered: row.aggregate_verified_leaf_requirements, total: row.aggregate_leaf_requirements },
      implementation: {
        covered: row.aggregate_covered_terminal_requirements,
        total: row.aggregate_terminal_requirements,
        complete: row.implementation_covered,
      }, children: [],
    };
    capabilities.set(row.identifier, item);
    ancestors.length = row.depth;
    const parent = ancestors.at(-1);
    (parent?.children ?? roots).push(item);
    ancestors.push(item);
  }

  const parents = new Map<string, Set<string>>();
  const children = new Map<string, Set<string>>();
  const attachments = new Map<string, Set<string>>();
  const add = (map: Map<string, Set<string>>, key: string, value: string) => {
    const choices = map.get(key) ?? new Set<string>();
    choices.add(value); map.set(key, choices);
  };
  for (const relation of store.relations) {
    const { source_id: source, target_id: target, relation_type: type } = relation;
    const parent = type === "derivedFrom" ? target : source;
    const child = type === "derivedFrom" ? source : target;
    if ((type === "derive" || type === "derivedFrom") && requirements.has(parent) && requirements.has(child)) {
      add(parents, child, parent);
      add(children, parent, child);
    }
    const capability = type === "specify" ? target : source;
    const requirement = type === "specify" ? source : target;
    if ((type === "specify" || type === "specifiedBy") && capabilities.has(capability) && requirements.has(requirement)) add(attachments, capability, requirement);
  }
  const ordered = [...requirements.values()].sort((a, b) => a.target.label.localeCompare(b.target.label) || a.target.id.localeCompare(b.target.id));
  // Each capability discloses its own attachments. A requirement's parent may
  // specify another capability; that must not make this attachment inaccessible.
  const requirementForest = (seeds: Iterable<string>): CoverageDrilldownItem[] => {
    const members = new Map<string, CoverageDrilldownItem>();
    const pending = [...seeds];
    while (pending.length) {
      const id = pending.pop()!;
      if (members.has(id)) continue;
      members.set(id, { ...requirements.get(id)!, children: [] });
      pending.push(...(children.get(id) ?? []));
    }
    const forest: CoverageDrilldownItem[] = [];
    for (const record of ordered) {
      const item = members.get(record.target.id);
      if (!item) continue;
      const parent = [...(parents.get(item.target.id) ?? [])].filter(id => members.has(id)).sort()[0];
      const container = parent ? members.get(parent) : undefined;
      (container?.children ?? forest).push(item);
    }
    return forest;
  };
  for (const [id, capability] of capabilities) {
    capability.children!.push(...requirementForest(attachments.get(id) ?? []));
  }
  return roots;
}
