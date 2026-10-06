import { cx } from "@linaria/atomic";
import { useEffect, useMemo, useRef, useState } from "react";
import { Icon } from "../../components/core/Icon";
import { ElementIcon } from "../../components/data/ElementIcon";
import { TreeItem } from "../../components/navigation/TreeItem";
import { PaneTree, PaneTreeNode } from "../side-pane";

import { paneSectionLabelClass } from "../side-pane/classes";

export interface CoverageNavigationScope {
  identifier: string;
  name: string;
  depth: number;
}

export interface CoverageNavigationProps {
  scopes: readonly CoverageNavigationScope[];
  selectedId: string | null;
  onSelect: (identifier: string | null) => void;
}

/** Navigation consumes a published, ordered hierarchy; it does not evaluate coverage. */
export function CoverageNavigation({ scopes, selectedId, onSelect }: CoverageNavigationProps) {
  const [closed, setClosed] = useState<ReadonlySet<string>>(() => new Set());
  const rows = useRef(new Map<string, HTMLDivElement>());
  const hierarchy = useMemo(() => {
    const ancestors: string[] = [];
    const capabilities = scopes.map((scope, index) => {
      ancestors.length = scope.depth;
      const parents = [...ancestors];
      ancestors.push(scope.identifier);
      return { ...scope, depth: scope.depth + 1, parents: ["", ...parents], expandable: (scopes[index + 1]?.depth ?? 0) > scope.depth };
    });
    // Empty identity is a UI scope root, never a modeled capability or published identifier.
    return [{ identifier: "", name: "Whole Model", depth: 0, parents: [] as string[], expandable: scopes.length > 0 }, ...capabilities];
  }, [scopes]);

  // Copied links and history traversal reveal the selected capability in its tree.
  useEffect(() => {
    const parents = hierarchy.find(scope => scope.identifier === selectedId)?.parents ?? [];
    setClosed(previous => {
      if (!parents.some(id => previous.has(id))) return previous;
      return new Set([...previous].filter(id => !parents.includes(id)));
    });
  }, [hierarchy, selectedId]);

  const visible = hierarchy.filter(scope => !scope.parents.some(id => closed.has(id)));
  const toggle = (id: string) => setClosed(previous => {
    const next = new Set(previous);
    if (next.has(id)) next.delete(id);
    else next.add(id);
    return next;
  });
  const focus = (id?: string) => { if (id !== undefined) rows.current.get(id)?.focus(); };

  return <>
    <div className={cx(paneSectionLabelClass)}>Capabilities</div>
      <PaneTree aria-label="Coverage capabilities">
        {visible.map((scope, index) => <PaneTreeNode key={scope.identifier}>
          <TreeItem role="treeitem" aria-label={scope.name} aria-level={scope.depth + 1}
            aria-selected={(selectedId ?? "") === scope.identifier}
            aria-expanded={scope.expandable ? !closed.has(scope.identifier) : undefined}
            tabIndex={0} data-capability-id={scope.identifier || undefined}
            data-coverage-whole-model={scope.identifier === "" || undefined}
            ref={node => { if (node) rows.current.set(scope.identifier, node); else rows.current.delete(scope.identifier); }}
            label={scope.name} icon={scope.identifier ? <ElementIcon type="capability" size="sm" /> : <Icon name="network" />}
            kind="element" depth={scope.depth} expandable={scope.expandable}
            open={!closed.has(scope.identifier)} selected={(selectedId ?? "") === scope.identifier}
            onSelect={() => onSelect(scope.identifier || null)} onToggle={() => toggle(scope.identifier)}
            onKeyDown={event => {
              if (event.key === "Enter" || event.key === " ") onSelect(scope.identifier || null);
              else if (event.key === "ArrowDown") focus(visible[index + 1]?.identifier);
              else if (event.key === "ArrowUp") focus(visible[index - 1]?.identifier);
              else if (event.key === "Home") focus(visible[0]?.identifier);
              else if (event.key === "End") focus(visible.at(-1)?.identifier);
              else if (event.key === "ArrowRight" && scope.expandable) {
                if (closed.has(scope.identifier)) toggle(scope.identifier);
                else focus(visible[index + 1]?.identifier);
              } else if (event.key === "ArrowLeft") {
                if (scope.expandable && !closed.has(scope.identifier)) toggle(scope.identifier);
                else focus(scope.parents.at(-1));
              } else return;
              event.preventDefault();
            }} />
        </PaneTreeNode>)}
      </PaneTree>
  </>;
}
