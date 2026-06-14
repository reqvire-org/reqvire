import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import { Icon } from "../core/Icon";

const baseUX = css`
  display: inline-flex;
  align-items: center;
  flex-wrap: wrap;
  gap: var(--space-3);
  font-size: var(--text-base);
  line-height: var(--leading-tight);

  .rq-crumbs__segment {
    display: inline-flex;
    min-width: 0;
    align-items: center;
    gap: var(--space-3);
  }

  .rq-crumbs__item {
    display: inline-flex;
    min-width: 0;
    max-width: 28ch;
    align-items: center;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    cursor: pointer;
    transition: color var(--dur-fast) var(--ease-standard);
  }

  .rq-crumbs__item.is-current {
    cursor: default;
  }

  .rq-crumbs__item:focus-visible {
    outline: none;
    border-radius: var(--radius-xs);
    box-shadow: var(--ring-focus);
  }

  .rq-crumbs__sep {
    display: inline-flex;
    flex: 0 0 auto;
    align-items: center;
    justify-content: center;
  }

  svg {
    display: block;
    flex: 0 0 auto;
    width: var(--icon-xs);
    height: var(--icon-xs);
  }
`;

const skinX = css`
  .rq-crumbs__item {
    color: var(--text-muted);
  }

  .rq-crumbs__item:hover {
    color: var(--text-body);
  }

  .rq-crumbs__item.is-current {
    color: var(--text-strong);
    font-weight: var(--weight-medium);
  }

  .rq-crumbs__item.is-current:hover {
    color: var(--text-strong);
  }

  .rq-crumbs__sep {
    color: var(--text-faint);
  }
`;

export interface BreadcrumbItem {
  label: ReactNode;
  onClick?: () => void;
}

export interface BreadcrumbProps extends HTMLAttributes<HTMLElement> {
  items?: BreadcrumbItem[];
}

export function Breadcrumb({ items = [], className = "", ...props }: BreadcrumbProps) {
  return (
    <nav className={cx("rq-crumbs", baseUX, skinX, className)} aria-label="Breadcrumb" {...props}>
      {items.map((it, i) => {
        const last = i === items.length - 1;
        return (
          <span className="rq-crumbs__segment" key={i}>
            <span
              className={["rq-crumbs__item", last ? "is-current" : ""].filter(Boolean).join(" ")}
              onClick={!last ? it.onClick : undefined}
            >
              {it.label}
            </span>
            {!last ? (
              <span className="rq-crumbs__sep" aria-hidden="true">
                <Icon name="chevron-right" size={14} />
              </span>
            ) : null}
          </span>
        );
      })}
    </nav>
  );
}
