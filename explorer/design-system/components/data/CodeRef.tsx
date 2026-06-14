import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";

export interface CodeRefProps extends HTMLAttributes<HTMLElement> {
  children?: ReactNode;
}

const baseUX = css`
  display: inline-block;
  min-width: var(--rq-coderef-min-w, auto);
  max-width: 100%;
  padding: var(--rq-coderef-py, var(--space-1)) var(--rq-coderef-px, var(--space-3));
  border-radius: var(--radius-sm);
  background: var(--rq-coderef-bg, var(--bg-sunken));
  color: var(--rq-coderef-color, var(--text-code));
  font-family: var(--font-mono);
  font-size: inherit;
  line-height: 1.35;
  overflow-wrap: var(--rq-coderef-ow, normal);
  text-align: var(--rq-coderef-text-align, inherit);
  white-space: var(--rq-coderef-ws, nowrap);
`;

export function CodeRef({ children, className = "", ...props }: CodeRefProps) {
  return (
    <code className={cx("rq-coderef", baseUX, className)} {...props}>
      {children}
    </code>
  );
}
