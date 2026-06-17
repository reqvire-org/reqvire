import type { MouseEvent, HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import { Badge } from "../core/Badge";

const baseUX = css`
  box-sizing: border-box;
  display: flex;
  width: var(--ds-treeitem-w, 100%);
  min-width: 0;
  height: var(--ds-treeitem-h, var(--row-h));
  align-items: center;
  gap: var(--ds-treeitem-gap, var(--space-4));
  padding-block: 0;
  padding-inline: var(--ds-treeitem-pr, var(--space-6));
  padding-inline-start: calc(var(--ds-treeitem-pl-base, var(--space-5)) + var(--ds-treeitem-depth-indent, var(--space-0)));
  border-left: var(--ds-treeitem-border-l, var(--border-w-thick) solid transparent);
  border-radius: var(--ds-treeitem-radius, 0);
  font-size: var(--text-sm);
  line-height: var(--ds-treeitem-lh, var(--leading-tight));
  cursor: pointer;
  user-select: none;
  transition:
    background var(--dur-fast) var(--ease-standard),
    border-color var(--dur-fast) var(--ease-standard),
    color var(--dur-fast) var(--ease-standard),
    box-shadow var(--dur-fast) var(--ease-standard);

  &:hover {
    background: var(--ds-treeitem-hover-bg, var(--bg-hover));
  }

  &:focus-visible {
    outline: none;
    box-shadow: var(--ring-focus);
  }

  &.is-selected {
    border-left-color: var(--ds-treeitem-sel-border, transparent);
    background: var(--ds-treeitem-sel-bg, var(--bg-selected));
    color: var(--ds-treeitem-sel-color, var(--text-body));
    font-weight: var(--ds-treeitem-sel-fw, var(--weight-semibold));
  }

  .ds-treeitem__twist {
    display: inline-flex;
    width: var(--ds-treeitem-twist-w, var(--space-7));
    height: var(--icon-sm);
    flex: 0 0 var(--ds-treeitem-twist-w, var(--space-7));
    align-items: center;
    justify-content: center;
    color: var(--ds-treeitem-twist-color, var(--text-faint));
    transition: transform var(--dur-fast) var(--ease-standard);
  }

  .ds-treeitem__twist svg {
    width: var(--ds-treeitem-twist-icon-size, var(--space-6));
    height: var(--ds-treeitem-twist-icon-size, var(--space-6));
  }

  &.is-open > .ds-treeitem__twist {
    transform: rotate(90deg);
  }

  .ds-treeitem__icon {
    display: inline-flex;
    flex: 0 0 auto;
    align-items: center;
    justify-content: center;
    color: var(--ds-treeitem-icon-color, var(--text-muted));
  }

  .ds-treeitem__icon svg {
    width: var(--icon-sm);
    height: var(--icon-sm);
  }

  &.is-selected .ds-treeitem__icon {
    color: var(--ds-treeitem-sel-icon-color, var(--accent));
  }

  .ds-treeitem__label {
    min-width: 0;
    flex: var(--ds-treeitem-label-flex, 1 1 auto);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .ds-treeitem__count {
    display: inline-flex;
    flex: 0 0 auto;
    margin-left: var(--ds-treeitem-count-ml, 0);
  }

  .ds-badge {
    flex: 0 0 auto;
  }

  &.ds-treeitem--depth-1 {
    --ds-treeitem-depth-indent: var(--ds-treeitem-indent-step, calc(var(--space-8) + var(--space-4)));
  }

  &.ds-treeitem--depth-2 {
    --ds-treeitem-depth-indent: calc(var(--ds-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--ds-treeitem-indent-step, calc(var(--space-8) + var(--space-4))));
  }

  &.ds-treeitem--depth-3 {
    --ds-treeitem-depth-indent: calc(var(--ds-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--ds-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--ds-treeitem-indent-step, calc(var(--space-8) + var(--space-4))));
  }

  &.ds-treeitem--depth-4 {
    --ds-treeitem-depth-indent: calc(var(--ds-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--ds-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--ds-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--ds-treeitem-indent-step, calc(var(--space-8) + var(--space-4))));
  }

  &.ds-treeitem--depth-5 {
    --ds-treeitem-depth-indent: calc(var(--ds-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--ds-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--ds-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--ds-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--ds-treeitem-indent-step, calc(var(--space-8) + var(--space-4))));
  }

  &.ds-treeitem--depth-6 {
    --ds-treeitem-depth-indent: calc(var(--ds-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--ds-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--ds-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--ds-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--ds-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--ds-treeitem-indent-step, calc(var(--space-8) + var(--space-4))));
  }

  svg {
    display: block;
    flex: 0 0 auto;
  }
`;

const skinX = css`
  color: var(--text-body);
  background: transparent;
`;

const fileSkinX = css`
  --ds-treeitem-icon-color: var(--resource-ink);

  .ds-treeitem__icon {
    width: var(--icon-md);
    height: var(--icon-md);
    border-radius: var(--radius-xs);
    background: var(--resource-tint);
  }

  .ds-treeitem__label {
    color: var(--text-body);
  }

  &.is-selected .ds-treeitem__icon {
    color: var(--resource-ink);
    background: var(--resource-tint);
  }
`;

const elementSkinX = css`
  --ds-treeitem-twist-w: var(--space-5);
  --ds-treeitem-hover-bg: var(--bg-hover);
  --ds-treeitem-sel-bg: var(--bg-selected);

  color: var(--text-secondary);

  .ds-treeitem__icon {
    margin-left: var(--space-1);
  }

  .ds-treeitem__label {
    color: var(--text-secondary);
  }

  &.is-selected .ds-treeitem__label {
    color: var(--text-strong);
  }
`;

export type TreeItemProps = Omit<HTMLAttributes<HTMLDivElement>, "style"> & {
  label: ReactNode;
  icon?: ReactNode;
  count?: ReactNode;
  depth?: number;
  open?: boolean;
  selected?: boolean;
  expandable?: boolean;
  kind?: "folder" | "file" | "element";
  onToggle?: () => void;
  onSelect?: () => void;
};

export function TreeItem({
  label,
  icon,
  count,
  depth = 0,
  open = false,
  selected = false,
  expandable = false,
  kind = "folder",
  onToggle,
  onSelect,
  className = "",
  onClick,
  ...props
}: TreeItemProps) {
  const depthClass = `ds-treeitem--depth-${Math.min(Math.max(0, depth), 6)}`;
  const select = (event: MouseEvent<HTMLDivElement>) => {
    onClick?.(event);
    if (!event.defaultPrevented) onSelect?.();
  };

  return (
    <div
      className={cx(
        "ds-treeitem",
        baseUX,
        skinX,
        kind === "file" ? fileSkinX : undefined,
        kind === "element" ? elementSkinX : undefined,
        `ds-treeitem--${kind}`,
        depthClass,
        open ? "is-open" : undefined,
        selected ? "is-selected" : undefined,
        className,
      )}
      {...props}
      onClick={select}
    >
      <span
        className="ds-treeitem__twist"
        onClick={(event) => {
          event.stopPropagation();
          onToggle?.();
        }}
      >
        {expandable ? (
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
            <path d="m9 18 6-6-6-6" />
          </svg>
        ) : null}
      </span>
      {icon ? <span className="ds-treeitem__icon">{icon}</span> : null}
      <span className="ds-treeitem__label">{label}</span>
      {count != null ? (
        <span className="ds-treeitem__count">
          <Badge>{count}</Badge>
        </span>
      ) : null}
    </div>
  );
}
