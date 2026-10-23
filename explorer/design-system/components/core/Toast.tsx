// Sonner-derived motion: see toast/README.md and toast/LICENSE.sonner.md.
import { forwardRef } from "preact/compat";
import { useEffect, useState } from "preact/hooks";
import { type HTMLAttributes, type ComponentChildren } from "preact";
import { css, cx } from "@linaria/atomic";
import { Button } from "./Button";
import { Icon, type IconName } from "./Icon";
import { IconButton } from "./IconButton";
import { Spinner } from "./Spinner";
import type { ToastAction, ToastVariant } from "./toast/types";

export interface ToastProps extends Omit<(HTMLAttributes<HTMLDivElement> & { className?: string }), "style" | "title"> {
  title: ComponentChildren;
  description?: ComponentChildren;
  variant?: ToastVariant;
  action?: ToastAction;
  cancel?: ToastAction;
  onDismiss?: () => void;
  closeButton?: boolean;
  closeLabel?: string;
}

const baseUX = css`
  box-sizing: border-box;
  width: min(100%, var(--ds-toast-width, var(--toast-width)));
  height: var(--ds-toast-card-height, auto);
  overflow: hidden;
  border: var(--border-w) solid var(--ds-toast-border, var(--border-default));
  border-radius: var(--radius-md);
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  line-height: var(--leading-snug);
  overflow-wrap: anywhere;

  .ds-toast__body {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-8);
  }

  .ds-toast__icon {
    position: relative;
    display: flex;
    flex: 0 0 auto;
    align-items: center;
    justify-content: center;
    width: var(--icon-md);
    height: var(--icon-md);
    color: var(--ds-toast-icon-color, var(--text-secondary));
  }

  .ds-toast__loader {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    pointer-events: none;
    opacity: 1;
    transform: scale(1);
    transition: opacity var(--dur-toast-loader) var(--ease-toast-stack), transform var(--dur-toast-loader) var(--ease-toast-stack);
  }

  .ds-toast__loader[data-visible="false"] {
    opacity: 0;
    transform: scale(0.8);
    .ds-spinner { animation: none; }
  }

  .ds-toast__content {
    display: flex;
    flex: 1 1 auto;
    min-width: 0;
    flex-direction: column;
    gap: var(--space-1);
  }

  .ds-toast__title { font-weight: var(--weight-medium); }
  .ds-toast__description { color: var(--text-secondary); }

  &[data-has-loaded="true"] .ds-toast__icon > .ds-icon {
    animation: ds-toast-icon-enter var(--dur-toast-icon) var(--ease-toast-stack) both;
  }

  @keyframes ds-toast-icon-enter {
    from { opacity: 0; transform: scale(0.8); }
    to { opacity: 1; transform: scale(1); }
  }

  @media (prefers-reduced-motion: reduce) {
    .ds-toast__loader { transition: none; }
    &[data-has-loaded="true"] .ds-toast__icon > .ds-icon { animation: none; }
  }

  .ds-toast__actions {
    display: flex;
    flex: 0 0 auto;
    flex-wrap: wrap;
    align-items: center;
    justify-content: flex-end;
    gap: var(--space-2);
    max-width: 45%;
  }

  @media (max-width: 420px) {
    .ds-toast__body { flex-wrap: wrap; }
    .ds-toast__actions { max-width: 100%; margin-inline-start: auto; }
  }
`;

const skinX = css`
  color: var(--text-strong);
  background: var(--bg-overlay);
  box-shadow: var(--shadow-md);

  &[data-variant="success"] { --ds-toast-icon-color: var(--success); }
  &[data-variant="danger"] { --ds-toast-icon-color: var(--danger); }
  &[data-variant="warning"] { --ds-toast-icon-color: var(--warning); }
  &[data-variant="info"] { --ds-toast-icon-color: var(--info); }
  &[data-variant="loading"] { --ds-toast-icon-color: var(--text-muted); }
`;

const ICONS: Partial<Record<ToastVariant, IconName>> = {
  success: "circle-check",
  danger: "circle-x",
  warning: "alert-triangle",
  info: "info",
};

export const Toast = forwardRef<HTMLDivElement, ToastProps>(function Toast({
  title,
  description,
  variant = "default",
  action,
  cancel,
  onDismiss,
  closeButton = true,
  closeLabel = "Dismiss notification",
  className = "",
  role = "status",
  ...props
}, ref) {
  const icon = ICONS[variant];
  const loading = variant === "loading";
  const [hasLoaded, setHasLoaded] = useState(loading);
  useEffect(() => { if (loading) setHasLoaded(true); }, [loading]);
  return (
    <div ref={ref} className={cx("ds-toast", baseUX, skinX, className)} data-variant={variant} data-has-loaded={hasLoaded} role={role} {...props}>
      <div className="ds-toast__body">
        {(icon || loading) && (
          <span className="ds-toast__icon" aria-hidden="true">
            {hasLoaded && <span className="ds-toast__loader" data-visible={loading}><Spinner size="md" /></span>}
            {icon && <Icon name={icon} size={17} />}
          </span>
        )}
        <div className="ds-toast__content">
          <div className="ds-toast__title">{title}</div>
          {description !== undefined && <div className="ds-toast__description">{description}</div>}
        </div>
        {(action || cancel || (closeButton && onDismiss)) && (
          <div className="ds-toast__actions">
            {cancel && <Button size="sm" tone="ghost" disabled={loading} onClick={(event) => {
              cancel.onClick(event);
              if (!event.defaultPrevented) onDismiss?.();
            }}>{cancel.label}</Button>}
            {action && <Button size="sm" tone="primary" disabled={loading} onClick={(event) => {
              action.onClick(event);
              if (!event.defaultPrevented) onDismiss?.();
            }}>{action.label}</Button>}
            {closeButton && onDismiss && <IconButton size="sm" tone="ghost" aria-label={closeLabel} disabled={loading} onClick={onDismiss}>
              <Icon name="x" />
            </IconButton>}
          </div>
        )}
      </div>
    </div>
  );
});
