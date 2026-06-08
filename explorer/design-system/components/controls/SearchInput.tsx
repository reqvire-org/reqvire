import type { InputHTMLAttributes, ReactNode } from "react";

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
    <div className={["rq-search", size === "lg" ? "rq-search--lg" : "", className].filter(Boolean).join(" ")}>
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
