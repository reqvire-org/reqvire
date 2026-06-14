import {
  cloneElement,
  createContext,
  isValidElement,
  useContext,
  useEffect,
  type ButtonHTMLAttributes,
  type ComponentProps,
  type HTMLAttributes,
  type ReactElement,
  type ReactNode,
} from "react";
import { css, cx } from "@linaria/atomic";
import { createPortal } from "react-dom";

interface ModalContextValue {
  open: boolean;
  onOpenChange?: (open: boolean) => void;
}

const ModalContext = createContext<ModalContextValue | null>(null);
const ModalContentContext = createContext<ModalContextValue | null>(null);

const overlayBaseUX = css`
  --rq-modal-overlay-backdrop-filter: blur(1.5px);
  position: fixed;
  inset: 0;
  z-index: var(--z-modal);
  display: flex;
  box-sizing: border-box;
  align-items: flex-start;
  justify-content: center;
  overflow: auto;
  padding: var(--space-24) var(--space-10);
  background: color-mix(in srgb, var(--slate-950) 48%, transparent);
  backdrop-filter: var(--rq-modal-overlay-backdrop-filter);
`;

const baseUX = css`
  --rq-modal-w: var(--content-max);
  display: flex;
  width: min(var(--rq-modal-w), 92vw);
  max-height: calc(100vh - 2 * var(--space-24));
  flex-direction: column;
  overflow: hidden;
  border: var(--border-w) solid var(--border-default);
  border-radius: var(--radius-xl);
  background: var(--bg-overlay);
  color: var(--text-body);
  box-shadow: var(--shadow-xl);

  svg {
    display: block;
    flex: 0 0 auto;
  }
`;

const headerBaseUX = css`
  display: flex;
  flex: 0 0 auto;
  align-items: center;
  gap: var(--space-8);
  padding: var(--space-12) var(--space-14);
  border-bottom: var(--border-w) solid var(--border-subtle);
  background: var(--bg-surface);
`;

const bodyBaseUX = css`
  flex: 1 1 auto;
  min-height: 0;
  overflow: auto;
  padding: var(--space-16) var(--space-20);
`;

const footerBaseUX = css`
  display: flex;
  flex: 0 0 auto;
  align-items: center;
  gap: var(--space-8);
  padding: var(--space-10) var(--space-14);
  border-top: var(--border-w) solid var(--border-subtle);
  background: var(--bg-overlay);
`;

const titleBaseUX = css`
  flex: 1 1 auto;
  min-width: 0;
  font-size: var(--text-xl);
`;

const descriptionBaseUX = css`
  max-width: 76ch;
  color: var(--text-body);
  font-size: var(--text-base);
  line-height: var(--leading-normal);
`;

export function Modal({
  open,
  onOpenChange,
  children,
}: {
  open: boolean;
  onOpenChange?: (open: boolean) => void;
  children: ReactNode;
}) {
  useEffect(() => {
    if (!open) return undefined;
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") onOpenChange?.(false);
    };
    document.addEventListener("keydown", onKeyDown);
    return () => document.removeEventListener("keydown", onKeyDown);
  }, [onOpenChange, open]);

  return (
    <ModalContext.Provider value={{ open, onOpenChange }}>
      {children}
    </ModalContext.Provider>
  );
}

export function ModalContent({
  className = "",
  children,
  showCloseButton: _showCloseButton,
  "aria-describedby": ariaDescribedBy,
  ...props
}: HTMLAttributes<HTMLDivElement> & {
  children: ReactNode;
  showCloseButton?: boolean;
}) {
  const context = useContext(ModalContext);
  if (!context?.open) return null;

  return createPortal(
    <ModalContentContext.Provider value={context}>
      <div
        className={cx("rq-modal__overlay", overlayBaseUX)}
        role="presentation"
        onMouseDown={(event) => {
          if (event.target === event.currentTarget) context.onOpenChange?.(false);
        }}
      >
        <div
          className={cx("rq-modal", baseUX, className)}
          role="dialog"
          aria-modal="true"
          aria-describedby={ariaDescribedBy}
          tabIndex={-1}
          {...props}
        >
          {children}
        </div>
      </div>
    </ModalContentContext.Provider>,
    document.body,
  );
}

export function ModalHeader({
  className = "",
  children,
  ...props
}: HTMLAttributes<HTMLDivElement> & { children: ReactNode }) {
  return (
    <div className={cx("rq-modal__head", headerBaseUX, className)} {...props}>
      {children}
    </div>
  );
}

export function ModalBody({
  className = "",
  children,
  ...props
}: HTMLAttributes<HTMLDivElement> & { children: ReactNode }) {
  return (
    <div className={cx("rq-modal__body", bodyBaseUX, className)} {...props}>
      {children}
    </div>
  );
}

export function ModalFooter({
  className = "",
  children,
  ...props
}: HTMLAttributes<HTMLDivElement> & { children: ReactNode }) {
  return (
    <div className={cx("rq-modal__foot", footerBaseUX, className)} {...props}>
      {children}
    </div>
  );
}

export function ModalTitle({
  className = "",
  children,
  ...props
}: HTMLAttributes<HTMLHeadingElement> & { children: ReactNode }) {
  return (
    <h2 className={cx("rq-modal__title", titleBaseUX, className)} {...props}>
      {children}
    </h2>
  );
}

export function ModalDescription({
  className = "",
  children,
  ...props
}: HTMLAttributes<HTMLParagraphElement> & { children: ReactNode }) {
  return (
    <p className={cx("rq-modal__description", descriptionBaseUX, className)} {...props}>
      {children}
    </p>
  );
}

export function ModalClose({
  asChild = false,
  children,
  onClick,
  ...props
}: ButtonHTMLAttributes<HTMLButtonElement> & {
  asChild?: boolean;
  children?: ReactNode;
}) {
  const context = useContext(ModalContentContext);
  const close = () => context?.onOpenChange?.(false);

  if (asChild && isValidElement(children)) {
    const child = children as ReactElement<ComponentProps<"button">>;
    return cloneElement(child, {
      ...props,
      onClick: (event) => {
        child.props.onClick?.(event);
        onClick?.(event);
        if (!event.defaultPrevented) close();
      },
    });
  }

  return (
    <button
      type="button"
      onClick={(event) => {
        onClick?.(event);
        if (!event.defaultPrevented) close();
      }}
      {...props}
    >
      {children}
    </button>
  );
}
