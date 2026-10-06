import { useEffect, useId, useRef } from "react";
import { css, cx } from "@linaria/atomic";
import { Modal, ModalBody, ModalContent, ModalFooter, ModalHeader, ModalTitle } from "../../components/core/Modal";
import { Spinner } from "../../components/core/Spinner";
import { Button } from "../../components/core/Button";
import { Icon } from "../../components/core/Icon";

const baseUX = css`
  text-align: left;
  outline: none;
`;
const headerUX = css`
  align-items: flex-start;
  gap: var(--space-6);
  padding: var(--space-10) var(--space-12);
`;
const headingUX = css`
  min-width: 0;
  flex: 1;
`;
const titleUX = css`
  margin: 0;
`;
const statusUX = css`
  display: flex;
  flex: 0 0 auto;
  align-items: center;
  justify-content: center;
  height: var(--control-sm);
  color: var(--danger);
`;
const branchUX = css`
  display: flex;
  align-items: baseline;
  gap: var(--space-4);
  margin: var(--space-4) 0 0;
  font-size: var(--text-sm);
  line-height: var(--leading-normal);
  dt { color: var(--text-muted); }
  dd {
    margin: 0;
    min-width: 0;
    font-family: var(--font-mono);
    overflow-wrap: anywhere;
  }
`;
const bodyUX = css`
  display: flex;
  flex-direction: column;
  gap: var(--space-8);
  padding: var(--space-10) var(--space-12);
`;
const explanationUX = css`
  margin: 0;
  font-size: var(--text-base);
  line-height: var(--leading-normal);
  color: var(--text-muted);
`;
const diagnosticUX = css`
  margin: 0;
  max-height: min(40dvh, calc(var(--space-32) * 4));
  overflow: auto;
  overscroll-behavior: contain;
  padding: var(--space-6);
  border: var(--border-w) solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--bg-sunken);
  font-family: var(--font-mono);
  font-size: var(--text-caption);
  line-height: var(--leading-normal);
  white-space: pre-wrap;
  overflow-wrap: anywhere;
  &:focus-visible { outline: none; box-shadow: var(--ring-focus); }
`;
const diagnosticHeadingUX = css`
  margin: 0 0 var(--space-4);
  font-size: var(--text-caption);
  font-weight: var(--weight-semibold);
`;
const footerUX = css`
  justify-content: flex-end;
  padding: var(--space-8) var(--space-12);
`;
export interface WorktreeLoadDialogProps {
  branch: string;
  error?: string | null;
  operation?: "selection" | "refresh";
  retryAutomatically?: boolean;
  onDismiss: () => void;
}
/** Blocks pending context adoption; failures return to the retained model on dismissal. */
export function WorktreeLoadDialog({ branch, error, operation = "selection", retryAutomatically = false, onDismiss }: WorktreeLoadDialogProps) {
  const titleId = useId();
  const branchId = useId();
  const descriptionId = useId();
  const dialog = useRef<HTMLDivElement>(null);
  const explanation = !error
    ? "Preparing the selected model. The view will update when loading finishes."
    : operation === "selection"
      ? "Your current model is still displayed. Close this dialog to continue, or select this branch again to retry."
      : retryAutomatically
        ? "Showing the last valid model. Refresh will retry automatically."
        : "Showing the last valid model. Select a branch to retry.";
  useEffect(() => {
    const previous = document.activeElement instanceof HTMLElement ? document.activeElement : null;
    dialog.current?.focus();
    return () => { if (previous?.isConnected) previous.focus(); };
  }, []);
  return <Modal open onOpenChange={open => { if (!open && error) onDismiss(); }}>
    <ModalContent ref={dialog} centered size="compact" className={cx("ux-worktree-load-dialog", baseUX)}
      data-product-pattern="worktree-load-dialog" aria-labelledby={titleId}
      aria-describedby={`${branchId} ${descriptionId}`} aria-busy={!error} onKeyDown={event => {
        if (event.key === "Tab") {
          event.preventDefault();
          const stops = [...(dialog.current?.querySelectorAll<HTMLElement>('button, [tabindex="0"]') ?? [])];
          if (!stops.length) return;
          const index = stops.indexOf(document.activeElement as HTMLElement);
          const next = index < 0 ? event.shiftKey ? stops.length - 1 : 0
            : (index + (event.shiftKey ? -1 : 1) + stops.length) % stops.length;
          stops[next].focus();
        }
      }}>
      <ModalHeader className={cx(headerUX)}>
        <span className={cx(statusUX)}>{error ? <Icon name="alert-triangle" />
          : <Spinner size="md" label={`Loading ${branch}`} />}</span>
        <div className={cx(headingUX)}>
          <ModalTitle id={titleId} className={cx(titleUX)}>{error
            ? operation === "refresh" ? "Couldn’t refresh model" : "Couldn’t load worktree"
            : "Loading worktree"}</ModalTitle>
          <dl id={branchId} className={cx(branchUX)}><dt>Branch</dt><dd>{branch}</dd></dl>
        </div>
      </ModalHeader>
      <ModalBody className={cx("ux-worktree-load-body", bodyUX)}>
        <p id={descriptionId} className={cx(explanationUX)}>{explanation}</p>
        {error && <div>
          <h3 className={cx(diagnosticHeadingUX)}>Error details</h3>
          <pre role="alert" aria-label="Error details" tabIndex={0} className={cx(diagnosticUX)}>{error}</pre>
        </div>}
      </ModalBody>
      {error && <ModalFooter className={cx(footerUX)}><Button tone="primary" onClick={onDismiss}>Close</Button></ModalFooter>}
    </ModalContent>
  </Modal>;
}
