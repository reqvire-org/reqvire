import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";

export interface WorkspaceToolbarProps extends Omit<HTMLAttributes<HTMLDivElement>, "title" | "style"> {
  leading?: ReactNode;
  title?: ReactNode;
  description?: ReactNode;
  meta?: ReactNode;
  actions?: ReactNode;
  children?: ReactNode;
}

const baseUX = css`
  box-sizing: border-box;
`;

const skinX = css`
  color: inherit;
`;

export function WorkspaceToolbar({
  className = "",
  leading,
  title,
  description,
  meta,
  actions,
  children,
  ...props
}: WorkspaceToolbarProps) {
  return (
    <div
      {...props}
      className={cx(
        "ux-workspace-toolbar",
        "ux-graph-control-panel",
        baseUX,
        skinX,
        className,
      )}
      data-product-pattern="workspace-toolbar"
    >
      {leading != null ? <div data-product-pattern-slot="leading">{leading}</div> : null}
      {title != null || description != null || meta != null ? (
        <div data-product-pattern-slot="summary">
          {title != null ? <div data-product-pattern-slot="title">{title}</div> : null}
          {description != null ? <div data-product-pattern-slot="description">{description}</div> : null}
          {meta != null ? <div data-product-pattern-slot="meta">{meta}</div> : null}
        </div>
      ) : null}
      {children}
      {actions != null ? <div data-product-pattern-slot="actions">{actions}</div> : null}
    </div>
  );
}
