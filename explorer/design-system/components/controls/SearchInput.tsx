import { css, cx } from "@linaria/atomic";
import type { InputHTMLAttributes, ReactNode } from "react";

const baseUX = css`
  position: relative;
  display: flex;
  align-items: center;
  width: 100%;

  .rq-search__input {
    width: 100%;
    height: var(--rq-search-input-h, var(--control-md));
    padding: var(--rq-search-input-p, 0 var(--space-8) 0 var(--space-16));
    font-family: var(--font-sans);
    font-size: var(--rq-search-input-fs, var(--text-sm));
    outline: none;
    box-shadow: var(--rq-search-input-shadow);
    transition:
      border-color var(--dur-fast),
      box-shadow var(--dur-fast),
      background var(--dur-fast);
  }

  .rq-search__input:focus {
    box-shadow: var(--rq-search-input-focus-shadow, var(--ring-focus));
  }

  .rq-search__icon {
    position: absolute;
    left: var(--rq-search-icon-left, var(--space-7));
    display: inline-flex;
    pointer-events: none;
  }

  .rq-search__icon svg {
    width: var(--rq-search-icon-sz, var(--icon-sm));
    height: var(--rq-search-icon-sz, var(--icon-sm));
  }

  .rq-search__kbd {
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
  .rq-search__input {
    color: var(--rq-search-input-color, var(--text-body));
    background: var(--rq-search-input-bg, var(--bg-surface));
    border: var(--rq-search-input-border, var(--border-w) solid var(--border-default));
    border-radius: var(--rq-search-input-radius, var(--radius-md));
  }

  .rq-search__input::placeholder {
    color: var(--rq-search-input-placeholder-color, var(--text-faint));
  }

  .rq-search__input:hover {
    border-color: var(--rq-search-input-hover-border-color, var(--border-strong));
  }

  .rq-search__input:focus {
    border-color: var(--rq-search-input-focus-border-color, var(--border-focus));
    background: var(--rq-search-input-focus-bg, var(--rq-search-input-bg, var(--bg-surface)));
  }

  .rq-search__icon {
    color: var(--rq-search-icon-color, var(--text-faint));
  }

  .rq-search__kbd {
    color: var(--rq-search-kbd-color, var(--text-faint));
    border: var(--rq-search-kbd-border, var(--border-w) solid var(--border-default));
    border-radius: var(--radius-xs);
  }
`;

const sizeLgUX = css`
  .rq-search__input {
    height: var(--control-lg);
    padding-left: calc(var(--space-16) + var(--space-3));
    font-size: var(--text-base);
  }

  .rq-search__icon {
    left: var(--space-8);
  }

  .rq-search__icon svg {
    width: var(--icon-md);
    height: var(--icon-md);
  }
`;

export interface SearchInputProps extends Omit<InputHTMLAttributes<HTMLInputElement>, "size"> {
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
    <div className={cx("rq-search", baseUX, skinX, size === "lg" && "rq-search--lg", size === "lg" && sizeLgUX, className)}>
      <span className="rq-search__icon" aria-hidden="true">
        {icon ?? (
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <circle cx="11" cy="11" r="8" />
            <path d="m21 21-4.3-4.3" />
          </svg>
        )}
      </span>
      <input className="rq-search__input" type={type} {...props} />
      {kbd ? <span className="rq-search__kbd">{kbd}</span> : null}
    </div>
  );
}
