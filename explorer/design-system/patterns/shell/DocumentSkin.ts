import { css, cx } from "@linaria/atomic";

// Keep document presentation in the base cascade layer so component-owned
// declarations retain the same precedence as before foundation migration.
const documentSkin = css`
  @layer base {
    h1, h2, h3, h4, h5, h6 {
      font-weight: var(--weight-semibold);
      color: var(--text-strong);
      line-height: var(--leading-tight);
    }
    a { color: var(--text-link); }
    a:hover { text-decoration: underline; }
    code, kbd, pre, samp {
      font-family: var(--font-mono);
      font-size: var(--text-inline-code);
    }
    &, * { scrollbar-width: thin; scrollbar-color: var(--scroll-thumb) var(--scroll-track); }
    &::-webkit-scrollbar, *::-webkit-scrollbar { width: var(--space-5); height: var(--space-5); }
    &::-webkit-scrollbar-track, *::-webkit-scrollbar-track { background: var(--scroll-track); }
    &::-webkit-scrollbar-thumb, *::-webkit-scrollbar-thumb {
      background-color: var(--scroll-thumb);
      border-radius: var(--radius-full);
      border: var(--border-w-heavy) solid transparent;
      background-clip: content-box;
    }
    &::-webkit-scrollbar-thumb:hover, *::-webkit-scrollbar-thumb:hover {
      background-color: var(--text-faint);
      background-clip: content-box;
    }
  }
`;

/** Apply to the document body in product and isolated preview mounts, including portals. */
export function documentClasses() {
  return cx("ux-document-skin", documentSkin);
}
