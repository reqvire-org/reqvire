import type { MouseEvent, HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import { Badge } from "../core/Badge";

const baseUX = css`
  box-sizing: border-box;
  display: flex;
  width: var(--rq-treeitem-w, 100%);
  min-width: 0;
  height: var(--rq-treeitem-h, var(--row-h));
  align-items: center;
  gap: var(--rq-treeitem-gap, var(--space-4));
  padding-block: 0;
  padding-inline: var(--rq-treeitem-pr, var(--space-6));
  padding-inline-start: calc(var(--rq-treeitem-pl-base, var(--space-5)) + var(--rq-treeitem-depth-indent, var(--space-0)));
  border-left: var(--rq-treeitem-border-l, var(--border-w-thick) solid transparent);
  border-radius: var(--rq-treeitem-radius, var(--radius-sm));
  font-size: var(--text-sm);
  line-height: var(--rq-treeitem-lh, var(--leading-tight));
  cursor: pointer;
  user-select: none;
  transition:
    background var(--dur-fast) var(--ease-standard),
    border-color var(--dur-fast) var(--ease-standard),
    color var(--dur-fast) var(--ease-standard),
    box-shadow var(--dur-fast) var(--ease-standard);

  &:hover {
    background: var(--rq-treeitem-hover-bg, var(--bg-hover));
  }

  &:focus-visible {
    outline: none;
    box-shadow: var(--ring-focus);
  }

  &.is-selected {
    border-left-color: var(--rq-treeitem-sel-border, var(--accent));
    background: var(--rq-treeitem-sel-bg, var(--accent-subtle));
    color: var(--rq-treeitem-sel-color, var(--text-strong));
    font-weight: var(--rq-treeitem-sel-fw, var(--weight-medium));
  }

  .rq-treeitem__twist {
    display: inline-flex;
    width: var(--rq-treeitem-twist-w, var(--space-7));
    height: var(--icon-sm);
    flex: 0 0 var(--rq-treeitem-twist-w, var(--space-7));
    align-items: center;
    justify-content: center;
    color: var(--rq-treeitem-twist-color, var(--text-faint));
    transition: transform var(--dur-fast) var(--ease-standard);
  }

  .rq-treeitem__twist svg {
    width: var(--rq-treeitem-twist-icon-size, var(--space-6));
    height: var(--rq-treeitem-twist-icon-size, var(--space-6));
  }

  &.is-open > .rq-treeitem__twist {
    transform: rotate(90deg);
  }

  .rq-treeitem__icon {
    display: inline-flex;
    flex: 0 0 auto;
    align-items: center;
    justify-content: center;
    color: var(--rq-treeitem-icon-color, var(--text-muted));
  }

  .rq-treeitem__icon svg {
    width: var(--icon-sm);
    height: var(--icon-sm);
  }

  &.is-selected .rq-treeitem__icon {
    color: var(--rq-treeitem-sel-icon-color, var(--accent));
  }

  .rq-treeitem__label {
    min-width: 0;
    flex: var(--rq-treeitem-label-flex, 1 1 auto);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .rq-treeitem__count {
    display: inline-flex;
    flex: 0 0 auto;
    margin-left: var(--rq-treeitem-count-ml, 0);
  }

  .rq-badge {
    flex: 0 0 auto;
  }

  &.rq-treeitem--depth-1 {
    --rq-treeitem-depth-indent: var(--rq-treeitem-indent-step, calc(var(--space-8) + var(--space-4)));
  }

  &.rq-treeitem--depth-2 {
    --rq-treeitem-depth-indent: calc(var(--rq-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--rq-treeitem-indent-step, calc(var(--space-8) + var(--space-4))));
  }

  &.rq-treeitem--depth-3 {
    --rq-treeitem-depth-indent: calc(var(--rq-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--rq-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--rq-treeitem-indent-step, calc(var(--space-8) + var(--space-4))));
  }

  &.rq-treeitem--depth-4 {
    --rq-treeitem-depth-indent: calc(var(--rq-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--rq-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--rq-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--rq-treeitem-indent-step, calc(var(--space-8) + var(--space-4))));
  }

  &.rq-treeitem--depth-5 {
    --rq-treeitem-depth-indent: calc(var(--rq-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--rq-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--rq-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--rq-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--rq-treeitem-indent-step, calc(var(--space-8) + var(--space-4))));
  }

  &.rq-treeitem--depth-6 {
    --rq-treeitem-depth-indent: calc(var(--rq-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--rq-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--rq-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--rq-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--rq-treeitem-indent-step, calc(var(--space-8) + var(--space-4))) + var(--rq-treeitem-indent-step, calc(var(--space-8) + var(--space-4))));
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
  --rq-treeitem-icon-color: var(--resource-ink);

  .rq-treeitem__icon {
    width: var(--icon-md);
    height: var(--icon-md);
    border-radius: var(--radius-xs);
    background: color-mix(in srgb, var(--resource) 12%, transparent);
  }

  .rq-treeitem__label {
    color: var(--text-body);
  }

  &.is-selected .rq-treeitem__icon {
    color: var(--resource-ink);
    background: var(--resource-tint);
  }
`;

const elementSkinX = css`
  --rq-treeitem-twist-w: var(--space-5);
  --rq-treeitem-hover-bg: color-mix(in srgb, var(--accent) 4%, transparent);
  --rq-treeitem-sel-bg: color-mix(in srgb, var(--accent) 9%, transparent);

  color: var(--text-secondary);

  .rq-treeitem__icon {
    margin-left: var(--space-1);
  }

  .rq-treeitem__label {
    color: var(--text-secondary);
  }

  &.is-selected .rq-treeitem__label {
    color: var(--text-strong);
  }
`;

export type TreeItemProps = HTMLAttributes<HTMLDivElement> & {
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
  const depthClass = `rq-treeitem--depth-${Math.min(Math.max(0, depth), 6)}`;
  const select = (event: MouseEvent<HTMLDivElement>) => {
    onClick?.(event);
    if (!event.defaultPrevented) onSelect?.();
  };

  return (
    <div
      className={cx(
        "rq-treeitem",
        baseUX,
        skinX,
        kind === "file" ? fileSkinX : undefined,
        kind === "element" ? elementSkinX : undefined,
        `rq-treeitem--${kind}`,
        depthClass,
        open ? "is-open" : undefined,
        selected ? "is-selected" : undefined,
        className,
      )}
      {...props}
      onClick={select}
    >
      <span
        className="rq-treeitem__twist"
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
      {icon ? <span className="rq-treeitem__icon">{icon}</span> : null}
      <span className="rq-treeitem__label">{label}</span>
      {count != null ? (
        <span className="rq-treeitem__count">
          <Badge>{count}</Badge>
        </span>
      ) : null}
    </div>
  );
}
