import type { AnchorHTMLAttributes, ButtonHTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import { ElementIcon } from "./ElementIcon";

const baseUX = css`
  display: inline-flex;
  align-items: center;
  max-width: 100%;
  gap: var(--space-4);

  .ds-relation__kind {
    padding: var(--space-1) var(--space-4);
    border-radius: var(--radius-sm);
    font-family: var(--font-mono);
    font-size: var(--text-micro);
    font-weight: var(--weight-medium);
    letter-spacing: var(--tracking-mono);
    line-height: 1.4;
    white-space: nowrap;
  }

  .ds-relation__target {
    display: inline-flex;
    min-width: 0;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-2) var(--space-6);
    border-radius: var(--radius-sm);
    cursor: pointer;
    font-size: var(--text-sm);
    font-weight: var(--weight-medium);
    text-decoration: none;
    transition:
      background var(--dur-fast),
      border-color var(--dur-fast);
  }

  .ds-relation__target:disabled {
    cursor: default;
    opacity: 0.72;
    pointer-events: none;
  }

  .ds-relation__target .ds-relation__txt {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .ds-relation__marker {
    flex: none;
  }

  svg {
    display: block;
    flex: 0 0 auto;
  }
`;

const skinX = css`
  .ds-relation__kind {
    color: var(--text-muted);
    background: var(--bg-sunken);
  }

  .ds-relation__target {
    color: var(--text-body);
    background: transparent;
    border: none;
  }

  .ds-relation__target:hover {
    background: var(--bg-hover);
  }

  .ds-relation__target:disabled:hover {
    background: transparent;
  }
`;

export type RelationPillProps = {
  kind?: ReactNode;
  label: ReactNode;
  className?: string;
  targetType?: string | null;
  targetFamily?: string | null;
} & (
  | ({ href: string } & AnchorHTMLAttributes<HTMLAnchorElement>)
  | ({ href?: undefined } & ButtonHTMLAttributes<HTMLButtonElement>)
);

export function RelationPill({
  kind,
  label,
  className = "",
  targetType,
  targetFamily,
  ...props
}: RelationPillProps) {
  const markerType = targetType ?? targetFamily ?? "other";
  const content = (
    <>
      <ElementIcon
        type={markerType}
        family={targetFamily}
        title={markerType}
        size="sm"
        className="ds-relation__marker"
      />
      <span className="ds-relation__txt">{label}</span>
    </>
  );
  if ("href" in props && props.href) {
    return (
      <span className={cx("ds-relation", baseUX, skinX, className)}>
        {kind ? <span className="ds-relation__kind">{kind}</span> : null}
        <a className="ds-relation__target" {...props}>
          {content}
        </a>
      </span>
    );
  }
  const buttonProps = props as ButtonHTMLAttributes<HTMLButtonElement>;
  return (
    <span className={cx("ds-relation", baseUX, skinX, className)}>
      {kind ? <span className="ds-relation__kind">{kind}</span> : null}
      <button className="ds-relation__target" {...buttonProps} type={buttonProps.type ?? "button"}>
        {content}
      </button>
    </span>
  );
}
