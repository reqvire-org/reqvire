import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";

const baseUX = css`
  display: flex;
  min-width: 0;
  flex-direction: column;
  gap: var(--rq-section-gap, var(--space-5));

  .rq-section__head {
    display: flex;
    min-width: 0;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-6);
  }

  .rq-section__title {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .rq-section__action {
    display: inline-flex;
    flex: 0 0 auto;
    align-items: center;
  }

  .rq-section__body {
    display: flex;
    min-width: 0;
    flex-direction: column;
    gap: var(--rq-section-body-gap, var(--space-4));
  }

  svg {
    display: block;
    flex: 0 0 auto;
  }
`;

const skinX = css`
  .rq-section__head {
    padding: var(--rq-section-head-p, 0 var(--space-2));
  }

  .rq-section__title {
    color: var(--text-muted);
    font-size: var(--text-micro);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-label);
    line-height: var(--leading-tight);
    text-transform: uppercase;
  }

  .rq-section__action {
    color: var(--text-link);
    font-size: var(--text-caption);
    line-height: var(--leading-tight);
  }
`;

export type SidebarSectionProps = HTMLAttributes<HTMLElement> & {
  title?: ReactNode;
  action?: ReactNode;
  children: ReactNode;
};

export function SidebarSection({
  title,
  action,
  children,
  className = "",
  ...props
}: SidebarSectionProps) {
  return (
    <section className={cx("rq-section", baseUX, skinX, className)} {...props}>
      {title || action ? (
        <div className="rq-section__head">
          {title ? <span className="rq-section__title">{title}</span> : <span />}
          {action ? <span className="rq-section__action">{action}</span> : null}
        </div>
      ) : null}
      <div className="rq-section__body">{children}</div>
    </section>
  );
}
