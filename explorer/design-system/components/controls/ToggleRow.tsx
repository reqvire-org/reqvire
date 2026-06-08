import type { ButtonHTMLAttributes, CSSProperties, ReactNode } from "react";

export type ToggleRowProps = Omit<ButtonHTMLAttributes<HTMLButtonElement>, "onToggle"> & {
  label: ReactNode;
  color?: string;
  icon?: ReactNode;
  meta?: ReactNode;
  on?: boolean;
  line?: boolean;
  onToggle?: () => void;
};

export function ToggleRow({
  label,
  color,
  icon,
  meta,
  on = true,
  line = false,
  className = "",
  onToggle,
  ...props
}: ToggleRowProps) {
  const swatchStyle = color
    ? line
      ? ({ borderColor: color } as CSSProperties)
      : ({ background: color, borderColor: color } as CSSProperties)
    : undefined;
  return (
    <button
      type="button"
      className={[
        "rq-togglerow",
        line ? "rq-togglerow--line" : "",
        on ? "" : "is-off",
        className,
      ].filter(Boolean).join(" ")}
      aria-pressed={on}
      onClick={onToggle}
      {...props}
    >
      {icon ? <span className="rq-togglerow__icon">{icon}</span> : <span className="rq-togglerow__swatch" style={swatchStyle} />}
      <span className="rq-togglerow__label">{label}</span>
      {meta != null ? <span className="rq-togglerow__meta">{meta}</span> : null}
    </button>
  );
}
