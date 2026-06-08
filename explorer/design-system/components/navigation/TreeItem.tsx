import type { CSSProperties, HTMLAttributes, ReactNode } from "react";

export type TreeItemProps = HTMLAttributes<HTMLDivElement> & {
  label: ReactNode;
  icon?: ReactNode;
  count?: ReactNode;
  depth?: number;
  open?: boolean;
  selected?: boolean;
  expandable?: boolean;
  kind?: "folder" | "file" | "element";
  onToggle?: () => void;
  onSelect?: () => void;
};

export function TreeItem({
  label,
  icon,
  count,
  depth = 0,
  open = false,
  selected = false,
  expandable = false,
  kind = "folder",
  onToggle,
  onSelect,
  className = "",
  ...props
}: TreeItemProps) {
  const indent = Math.max(0, depth) * 24;

  return (
    <div
      className={[
        "rq-treeitem",
        `rq-treeitem--${kind}`,
        open ? "is-open" : "",
        selected ? "is-selected" : "",
        className,
      ].filter(Boolean).join(" ")}
      style={
        {
          "--tree-depth": depth,
          "--tree-indent": `${indent}px`,
          paddingLeft: `calc(var(--space-5) + ${indent}px)`,
        } as CSSProperties
      }
      onClick={onSelect}
      {...props}
    >
      <span
        className="rq-treeitem__twist"
        onClick={(event) => {
          event.stopPropagation();
          onToggle?.();
        }}
      >
        {expandable ? (
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
            <path d="m9 18 6-6-6-6" />
          </svg>
        ) : null}
      </span>
      {icon ? <span className="rq-treeitem__icon">{icon}</span> : null}
      <span className="rq-treeitem__label">{label}</span>
      {count != null ? (
        <span className="rq-treeitem__count">
          <span className="rq-badge">{count}</span>
        </span>
      ) : null}
    </div>
  );
}
