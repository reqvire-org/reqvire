import { type HTMLAttributes } from "preact";
import { css, cx } from "@linaria/atomic";

export interface SpinnerProps extends Omit<(HTMLAttributes<HTMLSpanElement> & { className?: string }), "style"> {
  size?: "sm" | "md";
  label?: string;
}

const baseUX = css`
  --ds-spinner-size: var(--icon-sm);
  display: inline-block;
  width: var(--ds-spinner-size);
  height: var(--ds-spinner-size);
  flex: 0 0 auto;
  border-radius: var(--radius-full);
  animation: ds-spinner-spin calc(var(--dur-base) * 4) linear infinite;

  &[data-size="md"] {
    --ds-spinner-size: var(--icon-md);
  }

  @keyframes ds-spinner-spin {
    to {
      transform: rotate(1turn);
    }
  }
  @media (prefers-reduced-motion: reduce) { animation: none; }
`;

const skinX = css`
  border-width: var(--border-w);
  border-style: solid;
  border-color: var(--accent) var(--border-default) var(--border-default);
  color: var(--accent);
`;

export function Spinner({
  size = "sm",
  label = "Loading",
  className = "",
  ...props
}: SpinnerProps) {
  return (
    <span
      data-size={size}
      role="status"
      aria-label={label}
      className={cx("ds-spinner", baseUX, skinX, className)}
      {...props}
    />
  );
}
