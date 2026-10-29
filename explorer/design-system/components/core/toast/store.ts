import { type ComponentChildren } from "preact";
import type { ToastId, ToastOptions, ToastPromise, ToastPromiseOptions, ToastRecord, ToastUpdate, ToastVariant } from "./types";

const EMPTY: readonly ToastRecord[] = [];

/** Internal observable store; React reads immutable snapshots with useSyncExternalStore. */
export class ToastStore {
  private records: readonly ToastRecord[] = EMPTY;
  private listeners = new Set<() => void>();
  private sequence = 0;

  subscribe = (listener: () => void) => {
    this.listeners.add(listener);
    return () => { this.listeners.delete(listener); };
  };

  getSnapshot = () => this.records;
  getServerSnapshot = () => EMPTY;

  private publish(records: readonly ToastRecord[]) {
    this.records = records;
    this.listeners.forEach((listener) => listener());
  }

  add(message: ComponentChildren, options: ToastOptions = {}): ToastId {
    let id = options.id ?? `toast-${++this.sequence}`;
    if (options.id === undefined) {
      while (this.records.some((record) => record.id === id)) id = `toast-${++this.sequence}`;
    }
    const existing = this.records.find((record) => record.id === id);
    const record: ToastRecord = {
      ...existing,
      ...options,
      id,
      message,
      variant: options.variant ?? existing?.variant ?? "default",
      revision: ++this.sequence,
      dismissed: false,
    };
    this.publish(existing
      ? this.records.map((item) => item.id === id ? record : item)
      : [record, ...this.records]);
    return id;
  }

  update(id: ToastId, update: ToastUpdate): ToastId | undefined {
    const existing = this.records.find((record) => record.id === id && !record.dismissed);
    if (!existing) return undefined;
    this.publish(this.records.map((record) => record.id === id
      ? { ...record, ...update, revision: ++this.sequence }
      : record));
    return id;
  }

  dismiss(id?: ToastId, automatic = false) {
    const dismissed = this.records.filter((record) => !record.dismissed && (id === undefined || record.id === id));
    if (!dismissed.length) return;
    this.publish(this.records.map((record) => dismissed.includes(record) ? { ...record, dismissed: true } : record));
    for (const record of dismissed) {
      if (automatic) record.onAutoClose?.(record.id);
      else record.onDismiss?.(record.id);
    }
  }

  remove(id: ToastId, revision: number) {
    const records = this.records.filter((record) => record.id !== id || record.revision !== revision || !record.dismissed);
    if (records.length !== this.records.length) this.publish(records);
  }

  clearScope(toasterId: string) {
    const records = this.records.filter((record) => (record.toasterId ?? "default") !== toasterId);
    if (records.length !== this.records.length) this.publish(records);
  }
}

export function createToastApi(store: ToastStore) {
  const show = (message: ComponentChildren, options?: ToastOptions) => store.add(message, options);
  const variant = (tone: ToastVariant) => (message: ComponentChildren, options?: ToastOptions) =>
    show(message, { ...options, variant: tone });

  return Object.assign(show, {
    success: variant("success"),
    error: variant("danger"),
    warning: variant("warning"),
    info: variant("info"),
    loading: variant("loading"),
    update: (id: ToastId, update: ToastUpdate) => store.update(id, update),
    dismiss: (id?: ToastId) => store.dismiss(id),
    promise<T>(task: Promise<T> | (() => Promise<T>), options: ToastPromiseOptions<T>): ToastPromise<T> {
      const { loading, success, error, ...toastOptions } = options;
      const id = show(loading, { ...toastOptions, variant: "loading" });
      const revision = store.getSnapshot().find((record) => record.id === id)?.revision;
      const result = typeof task === "function" ? Promise.resolve().then(task) : task;
      const settle = (message: ComponentChildren, tone: ToastVariant) => {
        if (store.getSnapshot().some((record) => record.id === id && record.revision === revision && !record.dismissed)) {
          store.update(id, { message, variant: tone });
        }
      };
      // Preserve the result for unwrap(), and prevent stale promises from updating a replacement toast.
      const observed = result.then(
        (value) => {
          settle(typeof success === "function" ? success(value) : success, "success");
          return value;
        },
        (reason: unknown) => {
          settle(typeof error === "function" ? error(reason) : error, "danger");
          throw reason;
        },
      );
      void observed.catch(() => {});
      return { id, unwrap: () => observed };
    },
  });
}

export const toastStore = new ToastStore();
export const toast = createToastApi(toastStore);
