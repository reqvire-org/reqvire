import type { HTMLAttributes, ReactNode } from "react";

export type SidebarSectionProps = HTMLAttributes<HTMLElement> & {
  title?: ReactNode;
  action?: ReactNode;
  children: ReactNode;
};

export function SidebarSection({
  title,
  action,
  children,
  className = "",
  ...props
}: SidebarSectionProps) {
  return (
    <section className={["rq-section", className].filter(Boolean).join(" ")} {...props}>
      {title || action ? (
        <div className="rq-section__head">
          {title ? <span className="rq-section__title">{title}</span> : <span />}
          {action ? <span className="rq-section__action">{action}</span> : null}
        </div>
      ) : null}
      <div className="rq-section__body">{children}</div>
    </section>
  );
}
