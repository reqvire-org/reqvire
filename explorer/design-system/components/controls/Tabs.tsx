import type { HTMLAttributes, ReactNode } from "react";

export interface TabItem<T extends string = string> {
  value: T;
  label?: ReactNode;
  icon?: ReactNode;
  badge?: ReactNode;
}

export interface TabsProps<T extends string = string> extends Omit<HTMLAttributes<HTMLDivElement>, "onChange"> {
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
  const cls = ["rq-tabs", `rq-tabs--${variant}`, className].filter(Boolean).join(" ");
  return (
    <div className={cls} role="tablist" {...props}>
      {items.map((it) => {
        const active = it.value === value;
        return (
          <button
            key={it.value}
            type="button"
            role="tab"
            aria-selected={active}
            className={["rq-tab", active ? "is-active" : ""].filter(Boolean).join(" ")}
            onClick={() => onChange?.(it.value)}
          >
            {it.icon ? <span className="rq-tab__icon">{it.icon}</span> : null}
            <span>{it.label ?? it.value}</span>
            {it.badge != null ? <span className="rq-tab__badge">{it.badge}</span> : null}
          </button>
        );
      })}
    </div>
  );
}
