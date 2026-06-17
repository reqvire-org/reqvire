import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";

const baseUX = css`
  display: flex;
  min-width: 0;
  flex-direction: column;
  gap: var(--ds-section-gap, var(--space-5));

  .ds-section__head {
    display: flex;
    min-width: 0;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-6);
  }

  .ds-section__title {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .ds-section__action {
    display: inline-flex;
    flex: 0 0 auto;
    align-items: center;
  }

  .ds-section__body {
    display: flex;
    min-width: 0;
    flex-direction: column;
    gap: var(--ds-section-body-gap, var(--gap-list-stack));
  }

  svg {
    display: block;
    flex: 0 0 auto;
  }
`;

const skinX = css`
  .ds-section__head {
    padding: var(--ds-section-head-p, 0 var(--space-2));
  }

  .ds-section__title {
    color: var(--text-muted);
    font-size: var(--text-micro);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-label);
    line-height: var(--leading-tight);
    text-transform: uppercase;
  }

  .ds-section__action {
    color: var(--text-link);
    font-size: var(--text-caption);
    line-height: var(--leading-tight);
  }
`;

export type SidebarSectionProps = Omit<HTMLAttributes<HTMLElement>, "style"> & {
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
    <section className={cx("ds-section", baseUX, skinX, className)} {...props}>
      {title || action ? (
        <div className="ds-section__head">
          {title ? <span className="ds-section__title">{title}</span> : <span />}
          {action ? <span className="ds-section__action">{action}</span> : null}
        </div>
      ) : null}
      <div className="ds-section__body">{children}</div>
    </section>
  );
}
