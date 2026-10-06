import { useId, useState, type ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import { Badge } from "../../components/core/Badge";
import { Icon } from "../../components/core/Icon";
import { IconButton } from "../../components/core/IconButton";
import { BarMeterFill } from "../../components/data/TokenVisual";
import { RelationEndpoint } from "../detail/RelationEndpoint";
import { relationEndpointBaseUX } from "../detail/detailStyles";
import type { DetailRelationEndpointData } from "../detail/types";

export interface CoverageDrilldownTarget extends DetailRelationEndpointData {
  outsideScope?: boolean;
  dependencyKind?: "child" | "contract";
  implementationCovered?: boolean;
}

export interface CoverageDrilldownAssessment {
  blockers: CoverageDrilldownTarget[];
  contributions: CoverageDrilldownTarget[];
}

/** Presentation data supplied by a coverage consumer; this pattern does not evaluate coverage. */
export interface CoverageDrilldownItem {
  target: CoverageDrilldownTarget;
  verification: { covered: number; total: number };
  implementation: { covered: number; total: number; complete: boolean };
  children?: CoverageDrilldownItem[];
  assessment?: CoverageDrilldownAssessment;
}

const baseUX = css`
  min-width: 0;
  container-type: inline-size;
  container-name: coverage-drilldown;

  .ux-coverage-drilldown__branch {
    min-width: 0;
  }

  .ux-coverage-drilldown__row,
  .ux-coverage-drilldown__dependency,
  .ux-coverage-drilldown__dependency-heading {
    display: grid;
    grid-template-columns: minmax(0, 1.7fr) repeat(2, minmax(0, 1fr));
    align-items: center;
    gap: var(--space-12);
  }

  .ux-coverage-drilldown__row {
    padding: var(--space-8) var(--space-6);
    border-radius: var(--radius-md);
  }

  .ux-coverage-drilldown__identity {
    display: flex;
    align-items: center;
    min-width: 0;
    gap: var(--space-4);
  }

  .ux-coverage-drilldown__indent {
    display: flex;
    flex: 0 0 auto;
    max-width: 40%;
    overflow: hidden;
  }

  .ux-coverage-drilldown__indent-step {
    flex: 0 0 auto;
    width: calc(var(--space-12) + var(--space-6));
  }

  .ux-coverage-drilldown__disclosure-space {
    display: flex;
    flex: 0 0 var(--control-sm);
    height: var(--control-sm);
  }

  .ux-coverage-drilldown__marker-space {
    flex: 0 0 var(--type-icon-sm);
  }

  .ux-coverage-drilldown__metric {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    min-width: 0;
    font-size: var(--text-caption);
  }

  .ux-coverage-drilldown__metric-head {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: var(--space-2) var(--space-4);
    font-variant-numeric: tabular-nums;
  }

  .ux-coverage-drilldown__metric strong {
    font-weight: var(--weight-medium);
  }

  .ux-coverage-drilldown__bar {
    display: block;
    height: var(--space-3);
    border-radius: var(--radius-pill);
    overflow: hidden;
  }

  .ux-coverage-drilldown__assessment {
    margin-block: var(--space-4) var(--space-8);
    padding-inline: var(--space-6);
  }

  .ux-coverage-drilldown__dependencies {
    margin-block: var(--space-6);
  }

  .ux-coverage-drilldown__dependencies h4 {
    margin: 0 0 var(--space-3);
    font-size: var(--text-caption);
    font-weight: var(--weight-medium);
  }


  .ux-coverage-drilldown__dependency {
    align-items: start;
  }

  .ux-coverage-drilldown__dependency > .ux-coverage-drilldown__identity {
    align-items: start;
  }


  .ux-coverage-drilldown__list {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .ux-coverage-drilldown__list li {
    padding-block: var(--space-6);
    border-radius: var(--radius-md);
  }

  .ux-coverage-drilldown__target-status {
    grid-column: 3;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: flex-end;
    gap: var(--space-4);
  }


  @container coverage-drilldown (max-width: 760px) {
    .ux-coverage-drilldown__row,
    .ux-coverage-drilldown__dependency,
    .ux-coverage-drilldown__dependency-heading {
      grid-template-columns: repeat(2, minmax(0, 1fr));
      gap: var(--space-6);
    }

    .ux-coverage-drilldown__row > .ux-coverage-drilldown__identity,
    .ux-coverage-drilldown__dependency > .ux-coverage-drilldown__identity,
    .ux-coverage-drilldown__dependency-heading > .ux-coverage-drilldown__identity {
      grid-column: 1 / -1;
    }

    .ux-coverage-drilldown__target-status {
      grid-column: 2;
    }

    .ux-coverage-drilldown__indent-step {
      width: calc(var(--space-4) + var(--space-3));
    }
  }
`;

const skinX = css`
  .ux-coverage-drilldown__row[data-coverage-tone="base"],
  .ux-coverage-drilldown__list li:nth-child(odd) {
    background: var(--bg-surface);
  }

  .ux-coverage-drilldown__row[data-coverage-tone="alternate"],
  .ux-coverage-drilldown__list li:nth-child(even) {
    background: var(--bg-sunken);
  }

  .ux-coverage-drilldown__metric {
    color: var(--text-muted);
  }

  .ux-coverage-drilldown__dependency-heading {
    color: var(--text-muted);
  }

  .ux-coverage-drilldown__metric strong {
    color: var(--text-strong);
  }

  .ux-coverage-drilldown__bar {
    background: var(--bg-sunken);
  }

`;

export function CoverageDrilldown({ items, initiallyExpanded = [], onInspect }: {
  items: CoverageDrilldownItem[];
  initiallyExpanded?: string[];
  onInspect: (target: CoverageDrilldownTarget) => void;
}) {
  const [expandedRows, setExpandedRows] = useState<Record<string, boolean>>({});
  const isExpanded = (key: string, item: CoverageDrilldownItem) =>
    canExpand(item) && (expandedRows[key] ?? initiallyExpanded.includes(item.target.id));
  const tones = new Map<string, "base" | "alternate">();
  let visibleRow = 0;
  const visit = (item: CoverageDrilldownItem, key: string) => {
    tones.set(key, visibleRow++ % 2 ? "alternate" : "base");
    if (isExpanded(key, item)) {
      for (const child of item.children ?? []) {
        if (child.target.elementType !== "capability") visit(child, branchKey(key, child));
      }
    }
    for (const child of item.children ?? []) {
      if (child.target.elementType === "capability") visit(child, branchKey(key, child));
    }
  };
  for (const item of items) visit(item, branchKey("", item));
  const disclosure: CoverageDisclosure = {
    isExpanded, tones,
    toggle: (key, item) => setExpandedRows(previous => {
      const open = !(previous[key] ?? initiallyExpanded.includes(item.target.id));
      const next = { ...previous, [key]: open };
      if (!open) {
        // Requirement contents unmount on collapse; visible capability branches retain their state.
        const hidden = (item.children ?? []).filter(child => child.target.elementType !== "capability")
          .map(child => branchKey(key, child));
        for (const path of Object.keys(next)) {
          if (hidden.some(prefix => path === prefix || path.startsWith(`${prefix}/`))) delete next[path];
        }
      }
      return next;
    }),
  };
  return <div className={cx("ux-coverage-drilldown", baseUX, skinX)}>
    {items.map(item => <CoverageBranch key={item.target.id} item={item}
      path={branchKey("", item)} disclosure={disclosure} onInspect={onInspect} />)}
  </div>;
}

interface CoverageDisclosure {
  isExpanded: (key: string, item: CoverageDrilldownItem) => boolean;
  toggle: (key: string, item: CoverageDrilldownItem) => void;
  tones: Map<string, "base" | "alternate">;
}

function branchKey(parent: string, item: CoverageDrilldownItem) {
  return `${parent}/${encodeURIComponent(item.target.id)}`;
}

function canExpand(item: CoverageDrilldownItem) {
  return Boolean(item.children?.some(child => child.target.elementType !== "capability")
    || item.assessment?.contributions.length);
}

function HierarchyIndent({ depth }: { depth: number }) {
  return depth > 0 ? <span className="ux-coverage-drilldown__indent" aria-hidden="true">
    {Array.from({ length: depth }, (_, level) => <span key={level} className="ux-coverage-drilldown__indent-step" />)}
  </span> : null;
}

/** Every identity reserves the same disclosure slot, including linked dependencies. */
function HierarchyIdentity({ depth, disclosure, children }: {
  depth: number;
  disclosure?: ReactNode;
  children: ReactNode;
}) {
  return <div className="ux-coverage-drilldown__identity">
    <HierarchyIndent depth={depth} />
    <span className="ux-coverage-drilldown__disclosure-space" aria-hidden={disclosure ? undefined : true}>{disclosure}</span>
    {children}
  </div>;
}

function CoverageBranch({ item, path, disclosure, onInspect, depth = 0 }: {
  depth?: number;
  item: CoverageDrilldownItem;
  path: string;
  disclosure: CoverageDisclosure;
  onInspect: (target: CoverageDrilldownTarget) => void;
}) {
  const contentId = useId();
  const capabilityChildren = (item.children ?? []).filter(child => child.target.elementType === "capability");
  const requirementChildren = (item.children ?? []).filter(child => child.target.elementType !== "capability");
  const expandable = canExpand(item);
  const expanded = disclosure.isExpanded(path, item);
  return <article className="ux-coverage-drilldown__branch" aria-label={item.target.label}>
    <div className="ux-coverage-drilldown__row" data-kind={item.target.elementType} data-coverage-depth={depth}
      data-coverage-tone={disclosure.tones.get(path)}>
      <HierarchyIdentity depth={depth} disclosure={expandable ? <IconButton size="sm" aria-label={`${expanded ? "Collapse" : "Expand"} ${item.target.label}`}
          aria-expanded={expanded} aria-controls={contentId}
          onClick={() => disclosure.toggle(path, item)}>
          <Icon name={expanded ? "chevron-down" : "chevron-right"} />
        </IconButton> : undefined}>
        <RelationEndpoint endpoint={item.target} onOpenElement={() => onInspect(item.target)} />
      </HierarchyIdentity>
      <CoverageMetric label="Verification" covered={item.verification.covered}
        total={item.verification.total} complete={item.verification.total > 0 && item.verification.covered === item.verification.total}
        unit="leaves" />
      <CoverageMetric label="Implementation" covered={item.implementation.covered}
        total={item.implementation.total} complete={item.implementation.complete} unit="terminal"
        blockingRequirements={item.assessment?.blockers.length} />
    </div>
    <div id={contentId} hidden={!expanded}>
      {expanded && <>
        {requirementChildren.length ? <div className="ux-coverage-drilldown__children">
          {requirementChildren.map(child => <CoverageBranch key={child.target.id} item={child}
            path={branchKey(path, child)} disclosure={disclosure} onInspect={onInspect} depth={depth + 1} />)}
        </div> : null}
        {item.assessment && <RequirementAssessment name={item.target.label} assessment={item.assessment} depth={depth}
          displayedChildren={requirementChildren.map(child => child.target.id)} onInspect={onInspect} />}
      </>}
    </div>
    {capabilityChildren.length > 0 && <div className="ux-coverage-drilldown__children">
      {capabilityChildren.map(child => <CoverageBranch key={child.target.id} item={child}
        path={branchKey(path, child)} disclosure={disclosure} onInspect={onInspect} depth={depth + 1} />)}
    </div>}
  </article>;
}

function CoverageMetric({ label, covered, total, complete, unit, blockingRequirements = 0 }: {
  label: "Verification" | "Implementation";
  covered: number;
  total: number;
  complete: boolean;
  unit: string;
  blockingRequirements?: number;
}) {
  const status = label === "Verification"
    ? complete ? "Verified" : covered > 0 ? "Partially verified" : "Not verified"
    : complete ? "Covered" : blockingRequirements > 0
      ? `Blocked · ${blockingRequirements} ${blockingRequirements === 1 ? "requirement" : "requirements"}`
      : covered > 0 ? "Partial" : "Uncovered";
  return <div className="ux-coverage-drilldown__metric" role="group" aria-label={`${label}: ${status}; ${covered} / ${total} ${unit}`}>
    <div className="ux-coverage-drilldown__metric-head">
      <span>{label}</span>
      {label === "Implementation" && !complete && blockingRequirements > 0 && <strong>{status}</strong>}
    </div>
    <>
      <span className="ux-coverage-drilldown__bar"><BarMeterFill value={total ? covered / total * 100 : 0}
        colorToken={label === "Verification" ? "--verification" : "--resource"} /></span>
      <span>{total ? Math.round(covered / total * 1000) / 10 : 0}% · {covered} / {total} {unit}</span>
    </>
  </div>;
}

function RequirementAssessment({ name, assessment, displayedChildren, onInspect, depth }: {
  name: string;
  assessment: CoverageDrilldownAssessment;
  displayedChildren: string[];
  depth: number;
  onInspect: (target: CoverageDrilldownTarget) => void;
}) {
  const additional = assessment.contributions.filter(target => !displayedChildren.includes(target.id));
  const groups = [
    { label: "Binding consumers", targets: additional.filter(target => target.dependencyKind === "contract") },
    { label: "Additional child requirements", targets: additional.filter(target => target.dependencyKind === "child") },
  ].filter(group => group.targets.length > 0);
  if (!groups.length) return null;
  return <section className="ux-coverage-drilldown__assessment" aria-label={`Coverage details for ${name}`}>
    {groups.map(group => <section key={group.label} className="ux-coverage-drilldown__dependencies"
      aria-label={`${group.label} for ${name}`}>
      <div className="ux-coverage-drilldown__dependency-heading">
        <HierarchyIdentity depth={depth}>
          <div className={relationEndpointBaseUX}>
            <span className="ux-coverage-drilldown__marker-space" aria-hidden="true" />
            <h4>{group.label}</h4>
          </div>
        </HierarchyIdentity>
      </div>
      <ul className="ux-coverage-drilldown__list" aria-label={group.label} tabIndex={0}>
        {group.targets.map(target => <li key={target.id} className="ux-coverage-drilldown__dependency">
          <HierarchyIdentity depth={depth + 1}>
            <RelationEndpoint endpoint={target} onOpenElement={() => onInspect(target)} />
          </HierarchyIdentity>
          <div className="ux-coverage-drilldown__target-status">
            {target.outsideScope && <Badge>Outside scope</Badge>}
            {target.implementationCovered !== undefined && <Badge>{target.implementationCovered ? "Covered" : "Uncovered"}</Badge>}
          </div>
        </li>)}
      </ul>
    </section>)}

  </section>;
}
