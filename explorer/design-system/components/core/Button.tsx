import type { ButtonHTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import type { ButtonSize, ButtonTone } from "./button_contract";
import { BUTTON_SIZE_CLASSES, BUTTON_TONE_CLASSES } from "./button_contract";

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  children?: ReactNode;
  tone?: ButtonTone;
  size?: ButtonSize;
  iconLeft?: ReactNode;
  iconRight?: ReactNode;
  block?: boolean;
}

const baseUX = css`
  display: inline-flex;
  height: var(--control-md);
  align-items: center;
  justify-content: center;
  gap: var(--space-4);
  padding: 0 var(--space-8);
  border: var(--border-w) solid transparent;
  border-radius: var(--radius-md);
  cursor: pointer;
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  line-height: 1;
  text-decoration: none;
  user-select: none;
  white-space: nowrap;
  transition:
    background var(--dur-fast) var(--ease-standard),
    border-color var(--dur-fast) var(--ease-standard),
    box-shadow var(--dur-fast) var(--ease-standard),
    color var(--dur-fast) var(--ease-standard);

  svg {
    display: block;
    width: var(--icon-sm);
    height: var(--icon-sm);
    flex: 0 0 auto;
  }

  &:focus-visible {
    outline: none;
    box-shadow: var(--ring-focus);
  }

  &:disabled {
    opacity: 0.45;
    pointer-events: none;
  }
`;

const primarySkinX = css`
  background: var(--slate-900);
  border-color: var(--slate-900);
  color: var(--slate-0);

  &:hover {
    background: var(--slate-800);
    border-color: var(--slate-800);
  }

  &:active {
    background: var(--slate-950);
  }

  [data-theme="dark"] & {
    background: var(--slate-0);
    border-color: var(--slate-0);
    color: var(--slate-900);
  }

  [data-theme="dark"] &:hover {
    background: var(--slate-150);
    border-color: var(--slate-150);
  }
`;

const accentSkinX = css`
  background: var(--accent);
  border-color: var(--accent);
  color: var(--accent-fg);

  &:hover {
    background: var(--accent-hover);
    border-color: var(--accent-hover);
  }

  &:active {
    background: var(--accent-active);
  }
`;

const secondarySkinX = css`
  background: var(--bg-surface);
  border-color: var(--border-default);
  color: var(--text-body);
  box-shadow: var(--shadow-xs);

  &:hover {
    background: var(--bg-hover);
    border-color: var(--border-strong);
  }

  &:active {
    background: var(--bg-active);
  }
`;

const ghostSkinX = css`
  background: transparent;
  color: var(--text-secondary);

  &:hover {
    background: var(--bg-hover);
    color: var(--text-strong);
  }

  &:active {
    background: var(--bg-active);
  }
`;

const dangerSkinX = css`
  background: var(--danger);
  border-color: var(--danger);
  color: var(--slate-0);

  &:hover {
    filter: brightness(0.94);
  }
`;

const smUX = css`
  height: var(--control-sm);
  padding: 0 var(--space-6);
  border-radius: var(--radius-sm);
  font-size: var(--text-caption);
`;

const lgUX = css`
  height: var(--control-lg);
  padding: 0 var(--space-12);
  font-size: var(--text-base);
`;

const blockUX = css`
  display: flex;
  width: 100%;
`;

const iconUX = css`
  display: inline-flex;
`;

const linkSkinX = css`
  height: auto;
  padding: var(--space-2) 0;
  border: 0;
  background: transparent;
  color: var(--text-secondary);

  &:hover {
    color: var(--text-strong);
  }
`;

const toneSkinX: Record<ButtonTone, string> = {
  primary: primarySkinX,
  accent: accentSkinX,
  secondary: secondarySkinX,
  ghost: ghostSkinX,
  danger: dangerSkinX,
  link: linkSkinX,
};

const sizeUX: Record<ButtonSize, string> = {
  sm: smUX,
  md: "",
  lg: lgUX,
};

export function Button({
  children,
  tone = "secondary",
  size = "md",
  iconLeft,
  iconRight,
  block = false,
  className = "",
  type = "button",
  ...props
}: ButtonProps) {
  return (
    <button
      type={type}
      className={cx(
        "rq-btn",
        baseUX,
        toneSkinX[tone],
        BUTTON_TONE_CLASSES[tone],
        sizeUX[size],
        BUTTON_SIZE_CLASSES[size],
        block ? blockUX : "",
        block ? "rq-btn--block" : "",
        className,
      )}
      {...props}
    >
      {iconLeft ? <span className={cx("rq-btn__icon", iconUX)}>{iconLeft}</span> : null}
      {children ? <span className="rq-btn__label">{children}</span> : null}
      {iconRight ? <span className={cx("rq-btn__icon", iconUX)}>{iconRight}</span> : null}
    </button>
  );
}
