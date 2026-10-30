import { type ButtonHTMLAttributes, type ComponentChildren } from "preact";
import { css, cx } from "@linaria/atomic";
import type { ButtonSize, ButtonTone } from "./button_contract";
import { BUTTON_SIZE_CLASSES, BUTTON_TONE_CLASSES } from "./button_contract";

export interface ButtonProps extends Omit<(ButtonHTMLAttributes<HTMLButtonElement> & { className?: string }), "style"> {
  children?: ComponentChildren;
  tone?: ButtonTone;
  size?: ButtonSize;
  iconLeft?: ComponentChildren;
  iconRight?: ComponentChildren;
  block?: boolean;
}

const baseUX = css`
  display: inline-flex;
  &[data-block] {
    display: flex;
    width: 100%;
  }
  height: var(--control-height-md);
  align-items: center;
  justify-content: center;
  gap: var(--space-4);
  padding: 0 var(--control-padding-md);
  border-width: var(--border-w);
  border-style: solid;
  border-color: transparent;
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
    pointer-events: none;
  }
`;

const primarySkinX = css`
  background: var(--slate-12);
  border-color: var(--slate-12);
  color: var(--slate-1);

  &:hover {
    background: var(--slate-11);
    border-color: var(--slate-11);
  }

  &:active {
    background: var(--slate-10);
  }

  &:disabled {
    background: var(--bg-disabled);
    border-color: var(--border-disabled);
    color: var(--text-disabled);
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

  &:disabled {
    background: var(--bg-disabled);
    border-color: var(--border-disabled);
    color: var(--text-disabled);
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

  &:disabled {
    background: var(--bg-disabled);
    border-color: var(--border-disabled);
    color: var(--text-disabled);
    box-shadow: none;
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

  &:disabled {
    color: var(--text-disabled);
  }
`;

const dangerSkinX = css`
  background: var(--danger);
  border-color: var(--danger);
  color: var(--color-paper);

  &:hover {
    filter: var(--filter-danger-hover);
  }

  &:disabled {
    background: var(--bg-disabled);
    border-color: var(--border-disabled);
    color: var(--text-disabled);
    filter: none;
  }
`;

const smUX = css`
  height: var(--control-height-sm);
  padding: 0 var(--control-padding-sm);
  border-radius: var(--radius-sm);
  font-size: var(--text-caption);
`;

const lgUX = css`
  height: var(--control-height-lg);
  padding: 0 var(--control-padding-lg);
  font-size: var(--text-base);
`;

const iconUX = css`
  display: inline-flex;
`;

const linkSkinX = css`
  height: auto;
  padding: var(--space-2) 0;
  border-width: 0;
  border-style: none;
  border-color: currentColor;
  background: transparent;
  color: var(--text-secondary);

  &:hover {
    color: var(--text-strong);
  }

  &:disabled {
    color: var(--text-disabled);
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
      data-block={block || undefined}
      className={cx(
        "ds-btn",
        baseUX,
        toneSkinX[tone],
        BUTTON_TONE_CLASSES[tone],
        sizeUX[size],
        BUTTON_SIZE_CLASSES[size],
        block ? "ds-btn--block" : "",
        className,
      )}
      {...props}
    >
      {iconLeft ? <span className={cx("ds-btn__icon", iconUX)}>{iconLeft}</span> : null}
      {children ? <span className="ds-btn__label">{children}</span> : null}
      {iconRight ? <span className={cx("ds-btn__icon", iconUX)}>{iconRight}</span> : null}
    </button>
  );
}
