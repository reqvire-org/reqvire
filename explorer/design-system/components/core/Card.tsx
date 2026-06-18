import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import type { DesignSystemColorToken } from "../../palette";

export interface CardProps extends Omit<HTMLAttributes<HTMLDivElement>, "style"> {
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
  background: var(--ds-card-selected-bg, var(--bg-selected));
  border-color: var(--ds-card-selected-border, var(--accent));
  box-shadow: var(--ds-card-selected-shadow, 0 0 0 var(--border-w) var(--accent), var(--shadow-md));

  &:hover {
    border-color: var(--ds-card-selected-hover-border, var(--ds-card-selected-border, var(--accent)));
    box-shadow: var(--ds-card-selected-hover-shadow, 0 0 0 var(--border-w) var(--accent), var(--shadow-lg));
  }
`;

const accentUX = css`
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  width: var(--ds-card-accent-w, var(--border-w-heavy));
  border-radius: var(--radius-lg) 0 0 var(--radius-lg);
  background: var(--ds-card-accent-color);
`;

const accentTokenSkinX = css`
  &[data-accent-token="--accent"] { --ds-card-accent-color: var(--accent); }
  &[data-accent-token="--capability"] { --ds-card-accent-color: var(--capability); }
  &[data-accent-token="--requirement"] { --ds-card-accent-color: var(--requirement); }
  &[data-accent-token="--contract"] { --ds-card-accent-color: var(--contract); }
  &[data-accent-token="--semantic-contract"] { --ds-card-accent-color: var(--semantic-contract); }
  &[data-accent-token="--verification"] { --ds-card-accent-color: var(--verification); }
  &[data-accent-token="--ontology"] { --ds-card-accent-color: var(--ontology); }
  &[data-accent-token="--resource"] { --ds-card-accent-color: var(--resource); }
  &[data-accent-token="--other"] { --ds-card-accent-color: var(--other); }
`;

export function Card({
  children,
  interactive = false,
  selected = false,
  padded = true,
  accentColorToken,
  className = "",
  ...props
}: CardProps) {
  return (
    <div
      className={cx(
        "ds-card",
        baseUX,
        skinX,
        padded ? paddedUX : "",
        padded ? "ds-card--pad" : "",
        interactive ? interactiveSkinX : "",
        interactive ? "ds-card--interactive" : "",
        selected ? selectedSkinX : "",
        selected ? "ds-card--selected" : "",
        accentColorToken ? accentTokenSkinX : "",
        className,
      )}
      data-accent-token={accentColorToken}
      {...props}
    >
      {accentColorToken ? <span className={cx("ds-card__accent", accentUX)} /> : null}
      {children}
    </div>
  );
}
