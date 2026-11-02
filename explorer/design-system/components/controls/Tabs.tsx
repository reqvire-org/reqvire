import { css, cx } from "@linaria/atomic";
import { type HTMLAttributes, type ComponentChildren } from "preact";
import { useEffect, useRef } from "preact/hooks";

const baseUX = css`
  display: flex;
  align-items: stretch;
  gap: var(--space-1);
  position: relative;
  min-width: 0;
  height: var(--ds-tabs-h);
  overflow-x: auto;
  overflow-y: hidden;
  scrollbar-width: none;

  svg {
    display: block;
    flex: 0 0 auto;
  }
`;

const tabBaseUX = css`
  display: inline-flex;
  flex: 0 0 auto;
  position: relative;
  align-items: center;
  gap: var(--space-5);
  height: var(--ds-tab-h, var(--app-header-height));
  padding: 0 var(--control-padding-sm);
  border: 0;
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  white-space: nowrap;
  cursor: pointer;
  transition: color var(--dur-fast);

  /* Inside the list, which scrolls and so clips anything outside it. */
  &:focus-visible {
    outline-offset: calc(var(--focus-w) * -1);
  }

  svg {
    width: var(--icon-sm);
    height: var(--icon-sm);
    opacity: 0.85;
  }

  .ds-tab__icon {
    display: inline-flex;
  }

  .ds-tab__badge {
    color: var(--text-muted);
    font-size: var(--text-micro);
    font-variant-numeric: tabular-nums;
  }
`;

const skinUnderlineX = css`
  border-bottom: var(--ds-tabs-border-bottom, var(--border-w) solid var(--border-subtle));
`;

const tabSkinUnderlineX = css`
  color: var(--text-secondary);
  background: transparent;

  &:hover {
    color: var(--text-strong);
  }

  &::after {
    content: "";
    position: absolute;
    right: var(--control-padding-sm);
    bottom: 0;
    left: var(--control-padding-sm);
    height: var(--border-w-thick);
    border-radius: var(--border-w-thick) var(--border-w-thick) 0 0;
    background: transparent;
    transition: background var(--dur-fast);
  }

  &.is-active {
    color: var(--text-strong);
  }

  &.is-active svg {
    color: var(--accent);
    opacity: 1;
  }

  &.is-active::after {
    background: var(--accent);
  }
`;

const skinPillX = css`
  gap: var(--space-2);
  padding: calc(var(--control-inset) + var(--border-w));
  border-top: 0;
  border-right: 0;
  border-bottom: 0;
  border-left: 0;
  border-radius: var(--radius-md);
  background: var(--bg-sunken);
`;

const tabSkinPillX = css`
  height: var(--control-height-sm);
  padding: 0 var(--control-padding-sm);
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  background: transparent;

  &:hover {
    color: var(--text-strong);
  }

  &.is-active {
    color: var(--text-inverse);
    background: var(--text-strong);
    box-shadow: var(--shadow-xs);
  }

  &.is-active:hover {
    color: var(--text-inverse);
  }

  &.is-active span {
    color: var(--text-inverse);
  }

  &.is-active svg {
    color: var(--text-inverse);
    opacity: 1;
  }
`;

export interface TabItem<T extends string = string> {
  value: T;
  label?: ComponentChildren;
  icon?: ComponentChildren;
  badge?: ComponentChildren;
}

export interface TabsProps<T extends string = string> extends Omit<(HTMLAttributes<HTMLDivElement> & { className?: string }), "onChange" | "style"> {
  items?: TabItem<T>[];
  value?: T;
  onChange?: (value: T) => void;
  variant?: "underline" | "pill";
}

export function Tabs<T extends string = string>({
  items = [],
  value,
  onChange,
  variant = "underline",
  className = "",
  ...props
}: TabsProps<T>) {
  const listRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const list = listRef.current;
    if (!list) return;
    const reveal = () => {
      const tab = list.querySelector('[aria-selected="true"]');
      if (!tab || list.scrollWidth <= list.clientWidth) return;
      const box = list.getBoundingClientRect(), target = tab.getBoundingClientRect();
      // scrollLeft and not scrollIntoView, which would also scroll the page around the list.
      if (target.left < box.left) list.scrollLeft += target.left - box.left;
      else if (target.right > box.right) list.scrollLeft += target.right - box.right;
    };
    reveal();
    if (typeof ResizeObserver === "undefined") return;
    const observer = new ResizeObserver(reveal);
    observer.observe(list);
    return () => observer.disconnect();
  }, [value, items.length]);
  const cls = cx(
    "ds-tabs",
    baseUX,
    `ds-tabs--${variant}`,
    variant === "underline" && skinUnderlineX,
    variant === "pill" && skinPillX,
    className,
  );
  return (
    <div ref={listRef} className={cls} role="tablist" {...props}>
      {items.map((it) => {
        const active = it.value === value;
        const tabSkinX = variant === "pill" ? tabSkinPillX : tabSkinUnderlineX;
        return (
          <button
            key={it.value}
            type="button"
            role="tab"
            aria-selected={active}
            className={cx("ds-tab", tabBaseUX, tabSkinX, active && "is-active")}
            onClick={() => onChange?.(it.value)}
          >
            {it.icon ? <span className="ds-tab__icon">{it.icon}</span> : null}
            <span>{it.label ?? it.value}</span>
            {it.badge != null ? <span className="ds-tab__badge">{it.badge}</span> : null}
          </button>
        );
      })}
    </div>
  );
}
