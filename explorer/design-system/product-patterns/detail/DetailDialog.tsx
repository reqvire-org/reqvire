import type { ReactNode } from "react";
import { cx } from "@linaria/atomic";
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
import { TypeBadge } from "../../components/data/TypeBadge";
import {
  detailBodyBaseUX,
  detailBodySkinX,
  detailBackUX,
  detailCloseUX,
  detailDialogBaseUX,
  detailDialogSkinX,
  detailFooterBaseUX,
  detailFooterRowUX,
  detailFooterSkinX,
  detailHeaderBaseUX,
  detailHeaderSkinX,
  detailTitleRowUX,
  iconSmUX,
  sourceLinkBaseUX,
  sourceLinkSkinX,
} from "./detailStyles";
import type { OpenSourceHandler } from "./types";

export interface DetailDialogProps {
  open: boolean;
  onOpenChange?: (open: boolean) => void;
  title: ReactNode;
  typeFamily?: string | null;
  elementType?: string | null;
  showHeaderClose?: boolean;
  sourceHref?: string | null;
  sourceLabel?: ReactNode;
  closeLabel?: ReactNode;
  backLabel?: string;
  onNavigateBack?: () => void;
  onOpenSource?: OpenSourceHandler;
  children: ReactNode;
}

export function DetailDialog({
  open,
  onOpenChange,
  title,
  typeFamily,
  elementType,
  showHeaderClose = true,
  sourceHref,
  sourceLabel = "Open source page",
  closeLabel = "Close",
  backLabel = "Back",
  onNavigateBack,
  onOpenSource,
  children,
}: DetailDialogProps) {
  return (
    <Modal open={open} onOpenChange={onOpenChange}>
      <ModalContent className={cx(detailDialogBaseUX, detailDialogSkinX)} showCloseButton={false}>
        <ModalHeader className={cx(detailHeaderBaseUX, detailHeaderSkinX)}>
          <div className={cx(detailTitleRowUX)}>
            {onNavigateBack ? (
              <IconButton
                tone="secondary"
                size="sm"
                className={cx(detailBackUX)}
                aria-label={backLabel}
                title={backLabel}
                onClick={onNavigateBack}
              >
                <Icon name="chevron-left" />
              </IconButton>
            ) : null}
            {elementType ? (
              <TypeBadge type={elementType} family={typeFamily} tinted dot={false}>
                {elementType}
              </TypeBadge>
            ) : null}
            <ModalTitle>{title}</ModalTitle>
            {showHeaderClose ? (
              <ModalClose asChild>
                <IconButton tone="ghost" className={cx(detailCloseUX)} aria-label="Close">
                  <Icon name="x" />
                </IconButton>
              </ModalClose>
            ) : null}
          </div>
        </ModalHeader>

        <ModalBody className={cx(detailBodyBaseUX, detailBodySkinX)}>{children}</ModalBody>

        {sourceHref ? (
          <ModalFooter className={cx(detailFooterBaseUX, detailFooterSkinX)}>
            <div className={cx(detailFooterRowUX)}>
              <a
                href={sourceHref}
                className={cx(sourceLinkBaseUX, sourceLinkSkinX)}
                onClick={(event) => {
                  if (!onOpenSource) return;
                  event.preventDefault();
                  onOpenSource(sourceHref);
                }}
              >
                <Icon name="external-link" className={cx(iconSmUX)} /> {sourceLabel}
              </a>
              <ModalClose asChild>
                <Button tone="primary" size="sm">
                  {closeLabel}
                </Button>
              </ModalClose>
            </div>
          </ModalFooter>
        ) : null}
      </ModalContent>
    </Modal>
  );
}
