import type { AnchorHTMLAttributes, ButtonHTMLAttributes, ReactNode } from "react";

export type RelationPillProps = {
  kind?: ReactNode;
  label: ReactNode;
  className?: string;
  pipColor?: string;
} & (
  | ({ href: string } & AnchorHTMLAttributes<HTMLAnchorElement>)
  | ({ href?: undefined } & ButtonHTMLAttributes<HTMLButtonElement>)
);

export function RelationPill({
  kind,
  label,
  className = "",
  pipColor,
  ...props
}: RelationPillProps) {
  const content = (
    <>
      {pipColor ? <span className="rq-relation__pip" style={{ background: pipColor }} /> : null}
      <span className="rq-relation__txt">{label}</span>
    </>
  );
  if ("href" in props && props.href) {
    return (
      <span className={["rq-relation", className].filter(Boolean).join(" ")}>
        {kind ? <span className="rq-relation__kind">{kind}</span> : null}
        <a className="rq-relation__target" {...props}>
          {content}
        </a>
      </span>
    );
  }
  const buttonProps = props as ButtonHTMLAttributes<HTMLButtonElement>;
  return (
    <span className={["rq-relation", className].filter(Boolean).join(" ")}>
      {kind ? <span className="rq-relation__kind">{kind}</span> : null}
      <button className="rq-relation__target" {...buttonProps} type={buttonProps.type ?? "button"}>
        {content}
      </button>
    </span>
  );
}
