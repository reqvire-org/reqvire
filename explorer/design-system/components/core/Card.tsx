import type { CSSProperties, HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import type { DesignSystemColorToken } from "../../palette";

export interface CardProps extends HTMLAttributes<HTMLDivElement> {
  children: ReactNode;
  interactive?: boolean;
  selected?: boolean;
  padded?: boolean;
  accentColorToken?: DesignSystemColorToken;
}

const baseUX = css`
  position: relative;
  transition:
    border-color var(--dur-fast) var(--ease-standard),
    box-shadow var(--dur-fast) var(--ease-standard),
    transform var(--dur-fast) var(--ease-standard);

  svg {
    display: block;
    flex: 0 0 auto;
  }
`;

const skinX = css`
  background: var(--bg-raised);
  border: var(--border-w) solid var(--border-subtle);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-xs);
`;

const paddedUX = css`
  padding: var(--space-12);
`;

const interactiveSkinX = css`
  cursor: pointer;

  &:hover {
    border-color: var(--border-strong);
    box-shadow: var(--shadow-md);
  }
`;

const selectedSkinX = css`
  background: var(--rq-card-selected-bg, color-mix(in srgb, var(--accent) 6%, var(--bg-raised)));
  border-color: var(--rq-card-selected-border, var(--accent));
  box-shadow: var(--rq-card-selected-shadow, 0 0 0 var(--border-w) var(--accent), var(--shadow-md));

  &:hover {
    border-color: var(--rq-card-selected-hover-border, var(--rq-card-selected-border, var(--accent)));
    box-shadow: var(--rq-card-selected-hover-shadow, 0 0 0 var(--border-w) var(--accent), var(--shadow-lg));
  }
`;

const accentUX = css`
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  width: var(--rq-card-accent-w, var(--border-w-heavy));
  border-radius: var(--radius-lg) 0 0 var(--radius-lg);
  background: var(--rq-card-accent-color);
`;

export function Card({
  children,
  interactive = false,
  selected = false,
  padded = true,
  accentColorToken,
  className = "",
  style,
  ...props
}: CardProps) {
  const cardStyle = accentColorToken
    ? ({ "--rq-card-accent-color": `var(${accentColorToken})`, ...style } as CSSProperties)
    : style;

  return (
    <div
      className={cx(
        "rq-card",
        baseUX,
        skinX,
        padded ? paddedUX : "",
        padded ? "rq-card--pad" : "",
        interactive ? interactiveSkinX : "",
        interactive ? "rq-card--interactive" : "",
        selected ? selectedSkinX : "",
        selected ? "rq-card--selected" : "",
        className,
      )}
      style={cardStyle}
      {...props}
    >
      {accentColorToken ? <span className={cx("rq-card__accent", accentUX)} /> : null}
      {children}
    </div>
  );
}
