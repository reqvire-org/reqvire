import { css, cx } from "@linaria/atomic";
import type { ButtonHTMLAttributes, CSSProperties, MouseEvent, ReactNode } from "react";
import { Badge } from "../core/Badge";
import type { DesignSystemColorToken } from "../../palette";

const baseUX = css`
  display: flex;
  align-items: center;
  justify-content: var(--rq-togglerow-jc, flex-start);
  gap: var(--rq-togglerow-gap, var(--space-6));
  width: 100%;
  height: var(--rq-togglerow-h, var(--control-md));
  min-height: var(--rq-togglerow-min-h, var(--rq-togglerow-h, var(--control-md)));
  padding: var(--rq-togglerow-p, 0 var(--space-7));
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  text-align: left;
  cursor: pointer;
  transition:
    background var(--dur-fast),
    border-color var(--dur-fast),
    opacity var(--dur-fast);

  .rq-togglerow__swatch {
    flex: none;
    width: var(--rq-togglerow-swatch-w, var(--icon-xs));
    height: var(--rq-togglerow-swatch-h, var(--icon-xs));
    border-radius: var(--radius-xs);
    background: var(--rq-togglerow-swatch-color, transparent);
    border-color: var(--rq-togglerow-swatch-color, transparent);
    box-shadow: inset 0 0 0 var(--border-w) color-mix(in srgb, var(--slate-950) 8%, transparent);
  }

  .rq-togglerow__icon {
    display: inline-flex;
    flex: none;
    align-items: center;
    justify-content: center;
    width: var(--icon-md);
    height: var(--icon-md);
  }

  .rq-togglerow__label {
    flex: 1 1 auto;
    min-width: var(--rq-togglerow-label-min-w);
    overflow: var(--rq-togglerow-label-of);
    text-overflow: var(--rq-togglerow-label-toe);
    white-space: var(--rq-togglerow-label-ws);
  }

  .rq-togglerow__meta {
    display: var(--rq-togglerow-meta-display, inline);
    align-items: var(--rq-togglerow-meta-ai);
    justify-content: var(--rq-togglerow-meta-jc);
    min-width: var(--rq-togglerow-meta-min-w);
    height: var(--rq-togglerow-meta-h);
    padding: var(--rq-togglerow-meta-p, 0);
    border-radius: var(--rq-togglerow-meta-radius);
    font-size: var(--text-micro);
    font-weight: var(--rq-togglerow-meta-fw, inherit);
    font-variant-numeric: tabular-nums;
    line-height: var(--rq-togglerow-meta-lh, inherit);
  }

  svg {
    display: block;
    flex: 0 0 auto;
  }
`;

const skinX = css`
  color: var(--text-body);
  background: var(--rq-togglerow-bg, var(--bg-surface));
  border: var(--rq-togglerow-border, var(--border-w) solid var(--border-default));
  border-radius: var(--rq-togglerow-radius, var(--radius-md));
  box-shadow: var(--rq-togglerow-shadow, none);

  &:hover {
    background: var(--rq-togglerow-hover-bg, var(--bg-hover));
    border-color: var(--rq-togglerow-hover-border, var(--border-strong));
  }

  .rq-togglerow__meta {
    color: var(--rq-togglerow-meta-color, var(--text-muted));
    background: var(--rq-togglerow-meta-bg);
  }

  &.is-off {
    color: var(--rq-togglerow-off-color);
    background: var(--rq-togglerow-off-bg, var(--rq-togglerow-bg, var(--bg-surface)));
    opacity: var(--rq-togglerow-off-opacity, 0.45);
  }

  &.is-off:hover {
    color: var(--rq-togglerow-off-hover-color);
    opacity: var(--rq-togglerow-off-hover-opacity, 0.45);
  }

  &.is-off .rq-togglerow__label {
    text-decoration: var(--rq-togglerow-off-label-td, none);
    text-decoration-color: var(--rq-togglerow-off-label-td-color);
    text-decoration-thickness: var(--rq-togglerow-off-label-td-w);
  }

  &.is-off .rq-togglerow__swatch {
    background: var(--rq-togglerow-off-swatch-bg, var(--slate-300)) !important;
    border-color: var(--rq-togglerow-off-swatch-border) !important;
  }

  &.is-off .rq-togglerow__icon {
    opacity: var(--rq-togglerow-off-icon-opacity, 0.7);
  }

  &.is-off .rq-togglerow__icon .rq-elemicon {
    color: transparent !important;
    background: transparent !important;
    box-shadow: inset 0 0 0 var(--border-w) var(--rq-togglerow-off-swatch-border, var(--border-strong)) !important;
  }

  &.is-off .rq-togglerow__icon .rq-elemicon__pip,
  &.is-off .rq-togglerow__icon .rq-elemicon__glyph {
    color: transparent !important;
    background: transparent !important;
  }
`;

const skinLineX = css`
  --rq-togglerow-h: var(--rq-togglerow-line-h, var(--control-sm));
  --rq-togglerow-min-h: var(--rq-togglerow-line-min-h, var(--control-sm));
  --rq-togglerow-p: var(--rq-togglerow-line-p, 0 var(--space-2));
  --rq-togglerow-swatch-w: var(--rq-togglerow-line-swatch-w, calc(var(--space-8) + var(--space-1)));
  --rq-togglerow-swatch-h: 0;

  border: 0;
  color: var(--rq-togglerow-line-color, inherit);
  background: transparent;

  &:hover {
    background: transparent;
  }

  .rq-togglerow__swatch {
    height: 0;
    border-top: var(--border-w-thick) solid;
    border-color: var(--rq-togglerow-line-swatch-border, var(--rq-togglerow-swatch-color, currentColor));
    border-radius: 0;
    background: var(--rq-togglerow-line-swatch-bg, transparent) !important;
    box-shadow: none;
  }
`;

const skinStaticX = css`
  cursor: var(--rq-togglerow-static-cursor, default);

  &:hover {
    background: var(--rq-togglerow-static-hover-bg, var(--bg-surface));
    border-color: var(--rq-togglerow-static-hover-border, var(--border-default));
  }
`;

export type ToggleRowProps = Omit<ButtonHTMLAttributes<HTMLButtonElement>, "onToggle"> & {
  label: ReactNode;
  colorToken?: DesignSystemColorToken;
  icon?: ReactNode;
  meta?: ReactNode;
  on?: boolean;
  line?: boolean;
  static?: boolean;
  onToggle?: () => void;
};

export function ToggleRow({
  label,
  colorToken,
  icon,
  meta,
  on = true,
  line = false,
  static: isStatic = false,
  className = "",
  onToggle,
  onClick,
  ...props
}: ToggleRowProps) {
  const hasStaticClass = className.split(/\s+/).includes("rq-togglerow--static");
  const swatchStyle = colorToken
    ? ({ "--rq-togglerow-swatch-color": `var(${colorToken})` } as CSSProperties)
    : undefined;
  const toggle = (event: MouseEvent<HTMLButtonElement>) => {
    onClick?.(event);
    if (!event.defaultPrevented) onToggle?.();
  };
  return (
    <button
      type="button"
      className={cx(
        "rq-togglerow",
        baseUX,
        skinX,
        line && "rq-togglerow--line",
        line && skinLineX,
        (isStatic || hasStaticClass) && "rq-togglerow--static",
        (isStatic || hasStaticClass) && skinStaticX,
        !on && "is-off",
        className,
      )}
      aria-pressed={on}
      {...props}
      onClick={toggle}
    >
      {icon ? <span className="rq-togglerow__icon">{icon}</span> : <span className="rq-togglerow__swatch" style={swatchStyle} />}
      <span className="rq-togglerow__label">{label}</span>
      {meta != null ? <Badge className="rq-togglerow__meta">{meta}</Badge> : null}
    </button>
  );
}
