import type { HTMLAttributes, ReactNode } from "react";
import { Icon } from "../core/Icon";

export interface BreadcrumbItem {
  label: ReactNode;
  onClick?: () => void;
}

export interface BreadcrumbProps extends HTMLAttributes<HTMLElement> {
  items?: BreadcrumbItem[];
}

export function Breadcrumb({ items = [], className = "", ...props }: BreadcrumbProps) {
  return (
    <nav className={["rq-crumbs", className].filter(Boolean).join(" ")} aria-label="Breadcrumb" {...props}>
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
