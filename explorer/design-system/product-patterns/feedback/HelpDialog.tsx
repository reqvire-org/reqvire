import { css, cx } from "@linaria/atomic";
import { Button } from "../../components/core/Button";
import { Icon } from "../../components/core/Icon";
import { IconButton } from "../../components/core/IconButton";
import {
  Modal,
  ModalBody,
  ModalClose,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalTitle,
} from "../../components/core/Modal";
import { HelpContent } from "./HelpContent";

export interface HelpDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

const helpDialogBaseUX = css`
  --ux-help-dialog-w: 1120px;
  width: min(var(--ux-help-dialog-w), calc(100vw - var(--space-24)));
  max-width: min(var(--ux-help-dialog-w), calc(100vw - var(--space-24)));
`;

const helpHeaderUX = css`
  align-items: center;

  h2 {
    flex: 1 1 auto;
  }
`;

const footerSpacerUX = css`
  flex: 1 1 auto;
`;

export function HelpDialog({ open, onOpenChange }: HelpDialogProps) {
  return (
    <Modal open={open} onOpenChange={onOpenChange}>
      <ModalContent className={cx(helpDialogBaseUX)} aria-label="Help" data-product-pattern="help-dialog">
        <ModalHeader className={cx(helpHeaderUX)}>
          <ModalTitle>Help</ModalTitle>
          <ModalClose asChild>
            <IconButton tone="ghost" aria-label="Close help">
              <Icon name="x" />
            </IconButton>
          </ModalClose>
        </ModalHeader>
        <ModalBody>
          <HelpContent />
        </ModalBody>
        <ModalFooter>
          <span className={cx(footerSpacerUX)} />
          <ModalClose asChild>
            <Button tone="primary">Close</Button>
          </ModalClose>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
}
