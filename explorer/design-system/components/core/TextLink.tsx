import { type AnchorHTMLAttributes, type ComponentChildren } from "preact";
import { css, cx } from "@linaria/atomic";

export interface TextLinkProps extends Omit<(AnchorHTMLAttributes<HTMLAnchorElement> & { className?: string }), "style" | "role" | "href"> {
  href: string;
  icon?: ComponentChildren;
}

const linkSkin = css`
  display: inline-flex;
  align-items: center;
  gap: var(--space-3);
  color: var(--text-link);
  font-size: var(--text-sm);

  &:focus-visible {
    outline: var(--focus-w) solid var(--focus-ring);
    outline-offset: var(--focus-offset);
  }
`;

export function TextLink({ icon, children, className, ...props }: TextLinkProps) {
  return <a className={cx("ds-text-link", linkSkin, className)} {...props}>{icon}{children}</a>;
}
