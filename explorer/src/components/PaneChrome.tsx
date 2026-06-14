import { css, cx } from "@linaria/atomic";
import { BrandMark } from "@ds";

export const railMarkClass = css`
  box-sizing: border-box;
`;

const railMarkBaseUX = css`
  position: absolute;
  top: var(--space-5);
  left: 50%;
  display: block;
  width: var(--space-10);
  height: var(--space-10);
  transform: translateX(-50%);
`;

const headerMarkClass = css`
  position: static;
  top: auto;
  left: auto;
  display: block;
  flex: 0 0 auto;
  width: var(--icon-lg);
  height: var(--icon-lg);
  transform: none;
`;

const headerBaseUX = css`
  display: flex;
  flex: 0 0 auto;
  align-items: center;
  gap: var(--space-5);
  min-height: var(--space-24);
  padding: 0 var(--space-8);

  span {
    display: inline-flex;
    align-items: center;
    font-size: var(--text-caption);
    font-weight: var(--weight-bold);
    letter-spacing: 0.08em;
    line-height: 1;
    text-transform: uppercase;
  }

  .ex-app & {
    display: none;
  }
`;

const headerSkinX = css`
  border-bottom: var(--border-w) solid var(--border-subtle);
  background: var(--bg-surface);
  color: var(--text-strong);
`;

export function ReqvireRailMark({ className = "" }: { className?: string }) {
  return (
    <BrandMark
      className={cx(railMarkClass, railMarkBaseUX, className)}
      decorative
    />
  );
}

export function PaneChromeHeader({ title }: { title: string }) {
  return (
    <div className={cx("ex-pane-chrome-header", headerBaseUX, headerSkinX)}>
      <ReqvireRailMark className={headerMarkClass} />
      <span>{title}</span>
    </div>
  );
}
