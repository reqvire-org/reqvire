import { css, cx } from "@linaria/atomic";
import type { InputHTMLAttributes, ReactNode } from "react";

const baseUX = css`
  position: relative;
  display: flex;
  align-items: center;
  width: 100%;

  .ds-search__input {
    width: 100%;
    height: var(--ds-search-input-h, var(--control-md));
    padding: var(--ds-search-input-p, 0 var(--space-8) 0 var(--space-16));
    font-family: var(--font-sans);
    font-size: var(--ds-search-input-fs, var(--text-sm));
    outline: none;
    box-shadow: var(--ds-search-input-shadow);
    transition:
      border-color var(--dur-fast),
      box-shadow var(--dur-fast),
      background var(--dur-fast);
  }

  .ds-search__input:focus {
    box-shadow: var(--ds-search-input-focus-shadow, var(--ring-focus));
  }

  .ds-search__icon {
    position: absolute;
    left: var(--ds-search-icon-left, var(--space-7));
    display: inline-flex;
    pointer-events: none;
  }

  .ds-search__icon svg {
    width: var(--ds-search-icon-sz, var(--icon-sm));
    height: var(--ds-search-icon-sz, var(--icon-sm));
  }

  .ds-search__kbd {
    position: absolute;
    right: var(--space-5);
    padding: var(--space-1) var(--space-3);
    font-family: var(--font-mono);
    font-size: var(--text-micro);
    line-height: var(--leading-snug);
  }

  svg {
    display: block;
    flex: 0 0 auto;
  }
`;

const skinX = css`
  .ds-search__input {
    color: var(--ds-search-input-color, var(--text-body));
    background: var(--ds-search-input-bg, var(--bg-surface));
    border: var(--ds-search-input-border, var(--border-w) solid var(--border-default));
    border-radius: var(--ds-search-input-radius, var(--radius-md));
  }

  .ds-search__input::placeholder {
    color: var(--ds-search-input-placeholder-color, var(--text-faint));
  }

  .ds-search__input:hover {
    border-color: var(--ds-search-input-hover-border-color, var(--border-strong));
  }

  .ds-search__input:focus {
    border-color: var(--ds-search-input-focus-border-color, var(--border-focus));
    background: var(--ds-search-input-focus-bg, var(--ds-search-input-bg, var(--bg-surface)));
  }

  .ds-search__icon {
    color: var(--ds-search-icon-color, var(--text-faint));
  }

  .ds-search__kbd {
    color: var(--ds-search-kbd-color, var(--text-faint));
    border: var(--ds-search-kbd-border, var(--border-w) solid var(--border-default));
    border-radius: var(--radius-xs);
  }
`;

const sizeLgUX = css`
  .ds-search__input {
    height: var(--control-lg);
    padding-left: calc(var(--space-16) + var(--space-3));
    font-size: var(--text-base);
  }

  .ds-search__icon {
    left: var(--space-8);
  }

  .ds-search__icon svg {
    width: var(--icon-md);
    height: var(--icon-md);
  }
`;

export interface SearchInputProps extends Omit<InputHTMLAttributes<HTMLInputElement>, "size" | "style"> {
  size?: "md" | "lg";
  icon?: ReactNode;
  kbd?: ReactNode;
}

export function SearchInput({
  size = "md",
  icon,
  kbd,
  className = "",
  type = "text",
  ...props
}: SearchInputProps) {
  return (
    <div className={cx("ds-search", baseUX, skinX, size === "lg" && "ds-search--lg", size === "lg" && sizeLgUX, className)}>
      <span className="ds-search__icon" aria-hidden="true">
        {icon ?? (
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <circle cx="11" cy="11" r="8" />
            <path d="m21 21-4.3-4.3" />
          </svg>
        )}
      </span>
      <input className="ds-search__input" type={type} {...props} />
      {kbd ? <span className="ds-search__kbd">{kbd}</span> : null}
    </div>
  );
}
