import { type TargetedMouseEvent, type ComponentChildren } from "preact";

export type ToastId = string | number;
export type ToastVariant = "default" | "success" | "danger" | "warning" | "info" | "loading";
export type ToastPosition = "top-left" | "top-center" | "top-right" | "bottom-left" | "bottom-center" | "bottom-right";

export interface ToastAction {
  label: ComponentChildren;
  /** Prevent the event's default action to keep the toast open. */
  onClick: (event: TargetedMouseEvent<HTMLButtonElement>) => void;
}

export interface ToastOptions {
  id?: ToastId;
  toasterId?: string;
  description?: ComponentChildren;
  variant?: ToastVariant;
  /** Milliseconds. Loading toasts and Infinity persist until updated or dismissed. */
  duration?: number;
  dismissible?: boolean;
  closeButton?: boolean;
  action?: ToastAction;
  cancel?: ToastAction;
  onDismiss?: (id: ToastId) => void;
  onAutoClose?: (id: ToastId) => void;
}

export interface ToastUpdate extends Omit<ToastOptions, "id" | "toasterId"> {
  message?: ComponentChildren;
}

export interface ToastPromiseOptions<T> extends Omit<ToastOptions, "variant"> {
  loading: ComponentChildren;
  success: ComponentChildren | ((value: T) => ComponentChildren);
  error: ComponentChildren | ((error: unknown) => ComponentChildren);
}

export interface ToastPromise<T> {
  id: ToastId;
  unwrap: () => Promise<T>;
}

export interface ToastRecord extends ToastOptions {
  id: ToastId;
  message: ComponentChildren;
  variant: ToastVariant;
  revision: number;
  dismissed: boolean;
}
