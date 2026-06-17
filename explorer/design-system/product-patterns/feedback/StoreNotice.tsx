import { css, cx } from "@linaria/atomic";
import { Card } from "../../components/core/Card";
import { Icon } from "../../components/core/Icon";
import { CodeRef } from "../../components/data/CodeRef";

export interface StoreNoticeProps {
  reason: string;
  detail?: string;
}

const baseUX = css`
  display: flex;
  min-height: 100%;
  align-items: flex-start;
  justify-content: center;
  padding: var(--space-20) var(--space-12);
`;

const skinX = css`
  background: var(--bg-canvas);
`;

const cardBaseUX = css`
  --ux-notice-card-max-w: 640px;
  display: flex;
  width: min(var(--ux-notice-card-max-w), 100%);
  flex-direction: column;
  gap: var(--space-10);
`;

const cardSkinX = css`
  h1 {
    color: var(--text-strong);
    font-size: var(--text-2xl);
    font-weight: var(--weight-semibold);
  }

  p {
    color: var(--text-secondary);
    font-size: var(--text-sm);
    line-height: var(--leading-normal);
  }
`;

const alertBaseUX = css`
  display: flex;
  align-items: flex-start;
  gap: var(--space-6);
  padding: var(--space-8);
  font-size: var(--text-sm);
  line-height: var(--leading-normal);

  svg {
    width: var(--icon-sm);
    height: var(--icon-sm);
    flex: none;
  }
`;

const alertSkinX = css`
  border: var(--border-w) solid var(--danger-border);
  border-radius: var(--radius-md);
  background: var(--danger-tint);
  color: var(--text-strong);

  svg {
    color: var(--danger);
  }
`;

export function StoreNotice({ reason, detail }: StoreNoticeProps) {
  return (
    <div className={cx(baseUX, skinX)} data-product-pattern="store-notice">
      <Card className={cx(cardBaseUX, cardSkinX)}>
        <h1>Reqvire Explorer</h1>
        <div className={cx(alertBaseUX, alertSkinX)} role="alert">
          <Icon name="alert-triangle" aria-hidden="true" />
          <span>{reason}</span>
        </div>
        {detail ? (
          <p>
            <CodeRef>{detail}</CodeRef>
          </p>
        ) : null}
        <p>
          The Explorer shell loads a browser-local Project Store seed produced by <CodeRef>reqvire serve</CodeRef>.
          Open this page from the served Explorer, or run <CodeRef>npm run dev</CodeRef> in{" "}
          <CodeRef>explorer/</CodeRef> to browse the shell with fixture data.
        </p>
      </Card>
    </div>
  );
}
