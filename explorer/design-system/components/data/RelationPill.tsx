import type { AnchorHTMLAttributes, ButtonHTMLAttributes, CSSProperties, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import type { DesignSystemColorToken } from "../../palette";

const baseUX = css`
  display: inline-flex;
  align-items: center;
  max-width: 100%;
  gap: var(--space-4);

  .rq-relation__kind {
    padding: var(--space-1) var(--space-4);
    border-radius: var(--radius-sm);
    font-family: var(--font-mono);
    font-size: var(--text-micro);
    font-weight: var(--weight-medium);
    letter-spacing: var(--tracking-mono);
    line-height: 1.4;
    white-space: nowrap;
  }

  .rq-relation__target {
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

  .rq-relation__target:disabled {
    cursor: default;
    opacity: 0.72;
    pointer-events: none;
  }

  .rq-relation__target .rq-relation__txt {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .rq-relation__pip {
    flex: none;
    width: var(--rq-relation-pip-size, calc(var(--space-3) + var(--space-1) / 2));
    height: var(--rq-relation-pip-size, calc(var(--space-3) + var(--space-1) / 2));
    border-radius: var(--rq-relation-pip-radius, calc(var(--radius-xs) / 2));
    background: var(--rq-relation-pip-color);
  }

  svg {
    display: block;
    flex: 0 0 auto;
  }
`;

const skinX = css`
  .rq-relation__kind {
    color: var(--text-muted);
    background: var(--bg-sunken);
  }

  .rq-relation__target {
    color: var(--text-body);
    background: transparent;
    border: none;
  }

  .rq-relation__target:hover {
    background: var(--bg-hover);
  }

  .rq-relation__target:disabled:hover {
    background: transparent;
  }
`;

export type RelationPillProps = {
  kind?: ReactNode;
  label: ReactNode;
  className?: string;
  pipColorToken?: DesignSystemColorToken;
} & (
  | ({ href: string } & AnchorHTMLAttributes<HTMLAnchorElement>)
  | ({ href?: undefined } & ButtonHTMLAttributes<HTMLButtonElement>)
);

export function RelationPill({
  kind,
  label,
  className = "",
  pipColorToken,
  ...props
}: RelationPillProps) {
  const style = pipColorToken
    ? ({ "--rq-relation-pip-color": `var(${pipColorToken})` } as CSSProperties)
    : undefined;
  const content = (
    <>
      {pipColorToken ? <span className="rq-relation__pip" /> : null}
      <span className="rq-relation__txt">{label}</span>
    </>
  );
  if ("href" in props && props.href) {
    return (
      <span className={cx("rq-relation", baseUX, skinX, className)} style={style}>
        {kind ? <span className="rq-relation__kind">{kind}</span> : null}
        <a className="rq-relation__target" {...props}>
          {content}
        </a>
      </span>
    );
  }
  const buttonProps = props as ButtonHTMLAttributes<HTMLButtonElement>;
  return (
    <span className={cx("rq-relation", baseUX, skinX, className)} style={style}>
      {kind ? <span className="rq-relation__kind">{kind}</span> : null}
      <button className="rq-relation__target" {...buttonProps} type={buttonProps.type ?? "button"}>
        {content}
      </button>
    </span>
  );
}
