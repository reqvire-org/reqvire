import type { ReactNode } from "react";

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
    <div className={["rq-segmented", className].filter(Boolean).join(" ")} role="group" aria-label={ariaLabel}>
      {items.map((item) => {
        const active = item.value === value;
        return (
          <button
            key={item.value}
            type="button"
            aria-pressed={active}
            className={["rq-segmented__item", active ? "is-active" : ""].filter(Boolean).join(" ")}
            onClick={() => onChange(item.value)}
          >
            {item.icon ? <span className="rq-segmented__icon">{item.icon}</span> : null}
            {item.label ? <span>{item.label}</span> : null}
          </button>
        );
      })}
    </div>
  );
}
