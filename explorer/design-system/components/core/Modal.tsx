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
import { createPortal } from "react-dom";

interface ModalContextValue {
  open: boolean;
  onOpenChange?: (open: boolean) => void;
}

const ModalContext = createContext<ModalContextValue | null>(null);
const ModalContentContext = createContext<ModalContextValue | null>(null);

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
        className="ex-modal-scrim"
        role="presentation"
        onMouseDown={(event) => {
          if (event.target === event.currentTarget) context.onOpenChange?.(false);
        }}
      >
        <div
          className={["ex-modal", className].filter(Boolean).join(" ")}
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
    <div className={["ex-modal__head", className].filter(Boolean).join(" ")} {...props}>
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
    <div className={["ex-modal__body", className].filter(Boolean).join(" ")} {...props}>
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
    <div className={["ex-modal__foot", className].filter(Boolean).join(" ")} {...props}>
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
    <h2 className={className} {...props}>
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
    <p className={className} {...props}>
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
