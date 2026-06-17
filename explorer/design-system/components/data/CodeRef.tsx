import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";

export interface CodeRefProps extends Omit<HTMLAttributes<HTMLElement>, "style"> {
  children?: ReactNode;
}

const baseUX = css`
  display: inline-block;
  min-width: var(--ds-coderef-min-w, auto);
  max-width: 100%;
  padding: var(--ds-coderef-py, var(--space-1)) var(--ds-coderef-px, var(--space-3));
  border-radius: var(--radius-sm);
  background: var(--ds-coderef-bg, var(--bg-sunken));
  color: var(--ds-coderef-color, var(--text-code));
  font-family: var(--font-mono);
  font-size: inherit;
  line-height: 1.35;
  overflow-wrap: var(--ds-coderef-ow, normal);
  text-align: var(--ds-coderef-text-align, inherit);
  white-space: var(--ds-coderef-ws, nowrap);
`;

export function CodeRef({ children, className = "", ...props }: CodeRefProps) {
  return (
    <code className={cx("ds-coderef", baseUX, className)} {...props}>
      {children}
    </code>
  );
}
