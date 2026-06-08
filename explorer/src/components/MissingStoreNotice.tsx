import { Card, Icon } from "@ds";

/*
 * Visible fail-closed diagnostic shown when the Project Store seed is missing,
 * malformed, or incompatible. The Explorer must not render partial views
 * without a valid seed.
 */
export function MissingStoreNotice({
  reason,
  detail,
}: {
  reason: string;
  detail?: string;
}) {
  return (
    <div className="missing-store-notice">
      <Card className="missing-store-card">
        <h1>Reqvire Explorer</h1>
        <div className="missing-store-alert" role="alert">
          <Icon name="alert-triangle" aria-hidden="true" />
          <span>{reason}</span>
        </div>
        {detail && (
          <p>
            <code className="rq-coderef">{detail}</code>
          </p>
        )}
        <p>
          The Explorer shell loads a browser-local Project Store seed produced by
          <code className="rq-coderef">reqvire serve</code>. Open this page from
          the served Explorer, or run{" "}
          <code className="rq-coderef">npm run dev</code> in{" "}
          <code className="rq-coderef">explorer/</code> to browse the shell
          with fixture data.
        </p>
      </Card>
    </div>
  );
}
