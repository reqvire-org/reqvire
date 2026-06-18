import { css, cx } from "@linaria/atomic";
import { Icon } from "../../components/core/Icon";
import { CodeRef } from "../../components/data/CodeRef";

export interface StoreNoticeProps {
  reason: string;
  detail?: string;
}

const baseUX = css`
  display: flex;
  min-height: 100vh;
  align-items: flex-start;
  justify-content: flex-start;
  padding: var(--space-24);

  @media (max-width: 700px) {
    padding: var(--space-12);
  }
`;

const skinX = css`
  background: var(--bg-canvas);
`;

const panelBaseUX = css`
  --ux-store-notice-max-w: 760px;
  display: flex;
  width: min(var(--ux-store-notice-max-w), 100%);
  flex-direction: column;
  gap: var(--space-12);
  padding: var(--space-16);
`;

const panelSkinX = css`
  background: var(--bg-raised);
  border: var(--border-w) solid var(--border-subtle);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
`;

const headerBaseUX = css`
  display: flex;
  align-items: center;
  gap: var(--space-6);
`;

const markBaseUX = css`
  display: inline-flex;
  width: var(--control-md);
  height: var(--control-md);
  flex: 0 0 auto;
  align-items: center;
  justify-content: center;
`;

const markSkinX = css`
  background: var(--danger-tint);
  border: var(--border-w) solid var(--danger-border);
  border-radius: var(--radius-md);
  color: var(--danger);
`;

const titleBaseUX = css`
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  min-width: 0;
`;

const titleSkinX = css`
  h1 {
    margin: 0;
    color: var(--text-strong);
    font-size: var(--text-xl);
    font-weight: var(--weight-semibold);
    line-height: var(--leading-tight);
  }

  p {
    margin: 0;
    color: var(--text-muted);
    font-size: var(--text-sm);
    line-height: var(--leading-snug);
  }
`;

const statusBaseUX = css`
  display: flex;
  align-items: flex-start;
  gap: var(--space-5);
  padding: var(--space-8) var(--space-10);
  font-size: var(--text-sm);
  line-height: var(--leading-normal);

  svg {
    width: var(--icon-sm);
    height: var(--icon-sm);
    flex: none;
  }
`;

const statusSkinX = css`
  border: var(--border-w) solid var(--danger-border);
  border-radius: var(--radius-md);
  background: var(--danger-tint);
  color: var(--text-strong);

  svg {
    color: var(--danger);
  }
`;

const bodyBaseUX = css`
  display: flex;
  flex-direction: column;
  gap: var(--space-7);
`;

const bodySkinX = css`
  p {
    margin: 0;
    color: var(--text-secondary);
    font-size: var(--text-sm);
    line-height: var(--leading-normal);
  }
`;

const detailBaseUX = css`
  overflow-wrap: anywhere;
`;

const detailSkinX = css`
  color: var(--text-body);
`;

const actionsBaseUX = css`
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: var(--space-6);

  @media (max-width: 700px) {
    grid-template-columns: 1fr;
  }
`;

const actionBaseUX = css`
  display: flex;
  min-width: 0;
  flex-direction: column;
  gap: var(--space-3);
  padding: var(--space-8);
`;

const actionSkinX = css`
  background: var(--bg-surface);
  border: var(--border-w) solid var(--border-subtle);
  border-radius: var(--radius-md);

  span {
    color: var(--text-muted);
    font-size: var(--text-caption);
    line-height: var(--leading-snug);
  }
`;

export function StoreNotice({ reason, detail }: StoreNoticeProps) {
  return (
    <div className={cx(baseUX, skinX)} data-product-pattern="store-notice">
      <section className={cx(panelBaseUX, panelSkinX)} aria-labelledby="store-notice-title">
        <div className={cx(headerBaseUX)}>
          <span className={cx(markBaseUX, markSkinX)} aria-hidden="true">
            <Icon name="alert-triangle" />
          </span>
          <div className={cx(titleBaseUX, titleSkinX)}>
            <h1 id="store-notice-title">Reqvire Explorer</h1>
            <p>Project Store seed is unavailable.</p>
          </div>
        </div>

        <div className={cx(statusBaseUX, statusSkinX)} role="alert">
          <Icon name="alert-triangle" aria-hidden="true" />
          <span>{reason}</span>
        </div>

        <div className={cx(bodyBaseUX, bodySkinX)}>
          {detail ? <p className={cx(detailBaseUX, detailSkinX)}>{detail}</p> : null}
          <p>
            Explorer expects a browser-local seed at <CodeRef>assets/project-store.js</CodeRef>. Use one of the
            supported entry points below so the seed and app bundle are served from the same output.
          </p>
          <div className={cx(actionsBaseUX)} aria-label="Supported Explorer entry points">
            <div className={cx(actionBaseUX, actionSkinX)}>
              <CodeRef>reqvire serve</CodeRef>
              <span>Browse the live workspace Explorer.</span>
            </div>
            <div className={cx(actionBaseUX, actionSkinX)}>
              <CodeRef>reqvire export</CodeRef>
              <span>Open the generated Explorer output directory.</span>
            </div>
            <div className={cx(actionBaseUX, actionSkinX)}>
              <CodeRef>npm run dev</CodeRef>
              <span>
                Run from <CodeRef>explorer/</CodeRef> for fixture-backed development.
              </span>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
}
