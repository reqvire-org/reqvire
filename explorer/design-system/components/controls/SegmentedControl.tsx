import { css, cx } from "@linaria/atomic";
import type { ReactNode } from "react";

const baseUX = css`
  display: inline-flex;
  align-items: center;
  gap: var(--space-1);
  padding: calc(var(--space-1) + var(--border-w));
  border: var(--border-w) solid var(--border-subtle);
  border-radius: var(--radius-md);

  svg {
    display: block;
    flex: 0 0 auto;
  }
`;

const itemBaseUX = css`
  display: inline-flex;
  align-items: center;
  gap: var(--space-4);
  height: var(--ds-segmented-item-h, calc(var(--control-xs) + var(--space-2)));
  padding: 0 var(--space-7);
  border: 0;
  border-radius: var(--radius-sm);
  font-size: var(--text-caption);
  font-weight: var(--weight-medium);
  white-space: nowrap;
  cursor: pointer;
  transition:
    background var(--dur-fast),
    color var(--dur-fast),
    box-shadow var(--dur-fast);

  .ds-segmented__icon {
    display: inline-flex;
  }

  svg {
    width: var(--icon-sm);
    height: var(--icon-sm);
  }
`;

const skinX = css`
  background: var(--bg-sunken);

  .ds-segmented__item {
    color: var(--text-secondary);
    background: transparent;
  }

  .ds-segmented__item:hover {
    color: var(--text-strong);
  }

  .ds-segmented__item.is-active {
    color: var(--text-inverse);
    background: var(--text-strong);
    box-shadow: var(--shadow-xs);
  }
`;

export interface SegmentedControlItem<T extends string> {
  value: T;
  label?: ReactNode;
  icon?: ReactNode;
}

export interface SegmentedControlProps<T extends string> {
  items: SegmentedControlItem<T>[];
  value: T;
  onChange: (value: T) => void;
  className?: string;
  ariaLabel?: string;
}

export function SegmentedControl<T extends string>({
  items,
  value,
  onChange,
  className = "",
  ariaLabel,
}: SegmentedControlProps<T>) {
  return (
    <div className={cx("ds-segmented", baseUX, skinX, className)} role="group" aria-label={ariaLabel}>
      {items.map((item) => {
        const active = item.value === value;
        return (
          <button
            key={item.value}
            type="button"
            aria-pressed={active}
            className={cx("ds-segmented__item", itemBaseUX, active && "is-active")}
            onClick={() => onChange(item.value)}
          >
            {item.icon ? <span className="ds-segmented__icon">{item.icon}</span> : null}
            {item.label ? <span>{item.label}</span> : null}
          </button>
        );
      })}
    </div>
  );
}
