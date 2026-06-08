import type { ButtonHTMLAttributes, ReactNode } from "react";

export interface ChipProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  active?: boolean;
  icon?: ReactNode;
  count?: ReactNode;
  children: ReactNode;
}

export function Chip({
  active = false,
  icon,
  count,
  children,
  className = "",
  ...props
}: ChipProps) {
  return (
    <button
      type="button"
      aria-pressed={active}
      className={["rq-chip", active ? "is-active" : "", className].filter(Boolean).join(" ")}
      {...props}
    >
      {icon}
      <span>{children}</span>
      {count != null ? <span className="rq-chip__count">{count}</span> : null}
    </button>
  );
}
