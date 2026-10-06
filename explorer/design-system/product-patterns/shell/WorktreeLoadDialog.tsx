import { useEffect, useId, useRef } from "react";
import { css, cx } from "@linaria/atomic";
import { Modal, ModalBody, ModalContent, ModalTitle } from "../../components/core/Modal";
import { Spinner } from "../../components/core/Spinner";
import { Button } from "../../components/core/Button";

const baseUX = css`
  text-align: center;
  .ux-worktree-load-body {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-10);
  }
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
  const dialog = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const previous = document.activeElement instanceof HTMLElement ? document.activeElement : null;
    dialog.current?.focus();
    return () => { if (previous?.isConnected) previous.focus(); };
  }, []);
  return <Modal open onOpenChange={open => { if (!open && error) onDismiss(); }}>
    <ModalContent ref={dialog} centered size="compact" className={cx("ux-worktree-load-dialog", baseUX)}
      aria-labelledby={titleId} aria-busy={!error} onKeyDown={event => {
        if (event.key === "Tab") {
          event.preventDefault();
          if (error) dialog.current?.querySelector<HTMLButtonElement>("button")?.focus();
        }
      }}>
      <ModalBody className="ux-worktree-load-body">
        <ModalTitle id={titleId}>{error
          ? operation === "refresh" ? "Couldn’t refresh model" : "Couldn’t load worktree"
          : "Loading worktree"}</ModalTitle>
        <p>{branch}</p>
        {error ? <><p role="alert">{error}</p>
          {operation === "refresh" && <p>{retryAutomatically
            ? "Showing the last valid model. Refresh will retry automatically."
            : "Showing the last valid model. Select a branch to retry."}</p>}
          <Button onClick={onDismiss}>Close</Button></>
          : <Spinner size="md" label={`Loading ${branch}`} />}
      </ModalBody>
    </ModalContent>
  </Modal>;
}
