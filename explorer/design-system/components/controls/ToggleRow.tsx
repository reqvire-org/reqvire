import { css, cx } from "@linaria/atomic";
import type { ButtonHTMLAttributes, MouseEvent, ReactNode } from "react";
import { Badge } from "../core/Badge";
import type { DesignSystemColorToken } from "../../palette";

const baseUX = css`
  display: flex;
  align-items: center;
  justify-content: var(--ds-togglerow-jc, flex-start);
  gap: var(--ds-togglerow-gap, var(--space-6));
  width: 100%;
  height: var(--ds-togglerow-h, var(--control-md));
  min-height: var(--ds-togglerow-min-h, var(--ds-togglerow-h, var(--control-md)));
  padding: var(--ds-togglerow-p, 0 var(--space-7));
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  text-align: left;
  cursor: pointer;
  transition:
    background var(--dur-fast),
    border-color var(--dur-fast),
    opacity var(--dur-fast);

  .ds-togglerow__swatch {
    flex: none;
    width: var(--ds-togglerow-swatch-w, var(--icon-xs));
    height: var(--ds-togglerow-swatch-h, var(--icon-xs));
    border-radius: var(--radius-xs);
    background: var(--ds-togglerow-swatch-color, transparent);
    border-color: var(--ds-togglerow-swatch-color, transparent);
    box-shadow: inset 0 0 0 var(--border-w) var(--control-swatch-ring);
  }

  .ds-togglerow__icon {
    display: inline-flex;
    flex: none;
    align-items: center;
    justify-content: center;
    width: var(--icon-md);
    height: var(--icon-md);
  }

  .ds-togglerow__label {
    flex: 1 1 auto;
    min-width: var(--ds-togglerow-label-min-w, 0);
    overflow: var(--ds-togglerow-label-of, hidden);
    text-overflow: var(--ds-togglerow-label-toe, ellipsis);
    white-space: var(--ds-togglerow-label-ws, nowrap);
  }

  .ds-togglerow__meta {
    display: var(--ds-togglerow-meta-display, inline-flex);
    align-items: var(--ds-togglerow-meta-ai, center);
    justify-content: var(--ds-togglerow-meta-jc, center);
    min-width: var(--ds-togglerow-meta-min-w, var(--control-xs));
    height: var(--ds-togglerow-meta-h, var(--control-xs));
    padding: var(--ds-togglerow-meta-p, 0 var(--space-3));
    border-radius: var(--ds-togglerow-meta-radius, var(--radius-pill));
    font-size: var(--text-micro);
    font-weight: var(--ds-togglerow-meta-fw, var(--weight-semibold));
    font-variant-numeric: tabular-nums;
    line-height: var(--ds-togglerow-meta-lh, 1);
  }

  svg {
    display: block;
    flex: 0 0 auto;
  }
`;

const skinX = css`
  color: var(--text-body);
  background: var(--ds-togglerow-bg, transparent);
  border: var(--ds-togglerow-border, 0);
  border-radius: var(--ds-togglerow-radius, 0);
  box-shadow: var(--ds-togglerow-shadow, none);

  &:hover {
    background: var(--ds-togglerow-hover-bg, var(--bg-hover));
    border-color: var(--ds-togglerow-hover-border, var(--border-strong));
  }

  .ds-togglerow__meta {
    color: var(--ds-togglerow-meta-color, var(--text-secondary));
    background: var(--ds-togglerow-meta-bg, var(--bg-sunken));
  }

  &.is-off {
    color: var(--ds-togglerow-off-color);
    background: var(--ds-togglerow-off-bg, var(--ds-togglerow-bg, transparent));
    opacity: var(--ds-togglerow-off-opacity, 0.45);
  }

  &.is-off:hover {
    color: var(--ds-togglerow-off-hover-color);
    opacity: var(--ds-togglerow-off-hover-opacity, 0.45);
  }

  &.is-off .ds-togglerow__label {
    text-decoration: var(--ds-togglerow-off-label-td, none);
    text-decoration-color: var(--ds-togglerow-off-label-td-color);
    text-decoration-thickness: var(--ds-togglerow-off-label-td-w);
  }

  &.is-off .ds-togglerow__swatch {
    background: var(--ds-togglerow-off-swatch-bg, var(--slate-300)) !important;
    border-color: var(--ds-togglerow-off-swatch-border) !important;
  }

  &.is-off .ds-togglerow__icon {
    opacity: var(--ds-togglerow-off-icon-opacity, 0.7);
  }

  &.is-off .ds-togglerow__icon .ds-elemicon {
    color: transparent !important;
    background: transparent !important;
    box-shadow: inset 0 0 0 var(--border-w) var(--ds-togglerow-off-swatch-border, var(--border-strong)) !important;
  }

  &.is-off .ds-togglerow__icon .ds-elemicon__pip,
  &.is-off .ds-togglerow__icon .ds-elemicon__glyph {
    color: transparent !important;
    background: transparent !important;
  }
`;

const tokenSkinX = css`
  &[data-color-token="--accent"] { --ds-togglerow-swatch-color: var(--accent); }
  &[data-color-token="--success"] { --ds-togglerow-swatch-color: var(--success); }
  &[data-color-token="--text-muted"] { --ds-togglerow-swatch-color: var(--text-muted); }
  &[data-color-token="--border-default"] { --ds-togglerow-swatch-color: var(--border-default); }
  &[data-color-token="--capability"] { --ds-togglerow-swatch-color: var(--capability); }
  &[data-color-token="--requirement"] { --ds-togglerow-swatch-color: var(--requirement); }
  &[data-color-token="--contract"] { --ds-togglerow-swatch-color: var(--contract); }
  &[data-color-token="--semantic-contract"] { --ds-togglerow-swatch-color: var(--semantic-contract); }
  &[data-color-token="--verification"] { --ds-togglerow-swatch-color: var(--verification); }
  &[data-color-token="--ontology"] { --ds-togglerow-swatch-color: var(--ontology); }
  &[data-color-token="--resource"] { --ds-togglerow-swatch-color: var(--resource); }
  &[data-color-token="--other"] { --ds-togglerow-swatch-color: var(--other); }
  &[data-color-token="--edge-derive"] { --ds-togglerow-swatch-color: var(--edge-derive); }
  &[data-color-token="--edge-satisfy"] { --ds-togglerow-swatch-color: var(--edge-satisfy); }
  &[data-color-token="--edge-attach"] { --ds-togglerow-swatch-color: var(--edge-attach); }
  &[data-color-token="--edge-trace"] { --ds-togglerow-swatch-color: var(--edge-trace); }
`;

const skinLineX = css`
  --ds-togglerow-h: var(--ds-togglerow-line-h, var(--control-sm));
  --ds-togglerow-min-h: var(--ds-togglerow-line-min-h, var(--control-sm));
  --ds-togglerow-p: var(--ds-togglerow-line-p, 0 var(--space-2));
  --ds-togglerow-swatch-w: var(--ds-togglerow-line-swatch-w, calc(var(--space-8) + var(--space-1)));
  --ds-togglerow-swatch-h: 0;

  border: 0;
  color: var(--ds-togglerow-line-color, inherit);
  background: transparent;

  &:hover {
    background: transparent;
  }

  .ds-togglerow__swatch {
    height: 0;
    border-top: var(--border-w-thick) solid;
    border-color: var(--ds-togglerow-line-swatch-border, var(--ds-togglerow-swatch-color, currentColor));
    border-radius: 0;
    background: var(--ds-togglerow-line-swatch-bg, transparent) !important;
    box-shadow: none;
  }
`;

const skinStaticX = css`
  cursor: var(--ds-togglerow-static-cursor, default);

  &:hover {
    background: var(--ds-togglerow-static-hover-bg, transparent);
    border-color: var(--ds-togglerow-static-hover-border, var(--border-default));
  }
`;

export type ToggleRowProps = Omit<ButtonHTMLAttributes<HTMLButtonElement>, "onToggle" | "style"> & {
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
  const hasStaticClass = className.split(/\s+/).includes("ds-togglerow--static");
  const toggle = (event: MouseEvent<HTMLButtonElement>) => {
    onClick?.(event);
    if (!event.defaultPrevented) onToggle?.();
  };
  return (
    <button
      type="button"
      className={cx(
        "ds-togglerow",
        baseUX,
        skinX,
        tokenSkinX,
        line && "ds-togglerow--line",
        line && skinLineX,
        (isStatic || hasStaticClass) && "ds-togglerow--static",
        (isStatic || hasStaticClass) && skinStaticX,
        !on && "is-off",
        className,
      )}
      aria-pressed={on}
      data-color-token={colorToken}
      {...props}
      onClick={toggle}
    >
      {icon ? <span className="ds-togglerow__icon">{icon}</span> : <span className="ds-togglerow__swatch" />}
      <span className="ds-togglerow__label">{label}</span>
      {meta != null ? <Badge className="ds-togglerow__meta">{meta}</Badge> : null}
    </button>
  );
}
