import type { HTMLAttributes } from "react";

/*
 * Deliberate HTML boundary for trusted renderer output.
 *
 * Current callers are Shiki and Mermaid, both producing local renderer output
 * from workspace content. Keep all dangerous HTML insertion behind this
 * component so future sanitization or renderer changes have one owner.
 */
export function SafeHtml({
  html,
  className = "",
  ...props
}: HTMLAttributes<HTMLDivElement> & { html: string }) {
  return (
    <div
      className={className}
      dangerouslySetInnerHTML={{ __html: html }}
      {...props}
    />
  );
}
