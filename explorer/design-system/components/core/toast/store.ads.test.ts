import assert from "node:assert/strict";
import { test } from "vitest";
import { ToastStore, createToastApi } from "./store";

function fixture() {
  const store = new ToastStore();
  return { store, toast: createToastApi(store) };
}

test("snapshots are stable until changed, subscriptions clean up, and SSR stays empty", () => {
  const { store, toast } = fixture();
  const initial = store.getSnapshot();
  assert.equal(initial, store.getSnapshot());
  let updates = 0;
  const unsubscribe = store.subscribe(() => updates++);
  toast("Hello");
  assert.equal(updates, 1);
  assert.notEqual(initial, store.getSnapshot());
  assert.equal(store.getServerSnapshot().length, 0);
  unsubscribe();
  toast("Another");
  assert.equal(updates, 1);
});

test("generated ids cannot overwrite custom ids", () => {
  const { store, toast } = fixture();
  toast("Custom", { id: "toast-2" });
  const generated = toast("Generated");
  assert.notEqual(generated, "toast-2");
  assert.equal(store.getSnapshot().length, 2);
});

test("updating an id preserves options and cannot be removed by an old exit", () => {
  const { store, toast } = fixture();
  const id = toast("First", { duration: Infinity, description: "Details" });
  const original = store.getSnapshot()[0];
  toast.dismiss(id);
  toast.success("Replacement", { id });
  store.remove(id, original.revision);
  const replacement = store.getSnapshot()[0];
  assert.equal(replacement.message, "Replacement");
  assert.equal(replacement.description, "Details");
  assert.equal(replacement.duration, Infinity);
  assert.equal(replacement.dismissed, false);
  assert.equal(replacement.variant, "success");
});

test("manual and automatic dismissal invoke the right callback once", () => {
  const { store, toast } = fixture();
  let manual = 0;
  let automatic = 0;
  const options = { onDismiss: () => manual++, onAutoClose: () => automatic++ };
  const first = toast("Manual", options);
  const second = toast("Automatic", options);
  toast.dismiss(first);
  toast.dismiss(first);
  store.dismiss(second, true);
  store.dismiss(second, true);
  assert.equal(manual, 1);
  assert.equal(automatic, 1);
  const record = store.getSnapshot().find((item) => item.id === first)!;
  store.remove(first, record.revision);
  assert.equal(store.getSnapshot().length, 1);
});

test("toaster teardown clears its scope and preserves other scopes", () => {
  const { store, toast } = fixture();
  toast("Default");
  toast("Scoped", { toasterId: "other" });
  store.clearScope("other");
  assert.equal(store.getSnapshot().length, 1);
  assert.equal(store.getSnapshot()[0].message, "Default");
});

test("a successful promise updates the same card and unwrap preserves its value", async () => {
  const { store, toast } = fixture();
  const handle = toast.promise(Promise.resolve(42), { loading: "Working", success: (value) => `Result ${value}`, error: "Failed" });
  assert.equal(store.getSnapshot()[0].variant, "loading");
  assert.equal(await handle.unwrap(), 42);
  assert.equal(store.getSnapshot().length, 1);
  assert.equal(store.getSnapshot()[0].id, handle.id);
  assert.equal(store.getSnapshot()[0].message, "Result 42");
  assert.equal(store.getSnapshot()[0].variant, "success");
});

test("promise rejection and synchronous factory errors remain available through unwrap", async () => {
  const { store, toast } = fixture();
  const error = new Error("Offline");
  const handle = toast.promise(() => { throw error; }, { loading: "Working", success: "Done", error: (reason) => reason instanceof Error ? reason.message : String(reason) });
  await assert.rejects(handle.unwrap(), (reason) => reason === error);
  assert.equal(store.getSnapshot()[0].message, "Offline");
  assert.equal(store.getSnapshot()[0].variant, "danger");
});

test("settling a promise cannot resurrect a dismissed toast", async () => {
  const { store, toast } = fixture();
  let resolve!: (value: string) => void;
  const handle = toast.promise(new Promise<string>((done) => { resolve = done; }), { loading: "Working", success: "Done", error: "Failed" });
  const revision = store.getSnapshot()[0].revision;
  toast.dismiss(handle.id);
  store.remove(handle.id, revision);
  resolve("Finished");
  assert.equal(await handle.unwrap(), "Finished");
  assert.equal(store.getSnapshot().length, 0);
});

test("an older promise cannot overwrite a newer request with the same id", async () => {
  const { store, toast } = fixture();
  let resolve!: (value: string) => void;
  const older = toast.promise(new Promise<string>((done) => { resolve = done; }), { id: "save", loading: "Old request", success: "Old result", error: "Failed" });
  const newer = toast.promise(Promise.resolve("New result"), { id: "save", loading: "New request", success: (value) => value, error: "Failed" });
  await newer.unwrap();
  resolve("Old result");
  await older.unwrap();
  assert.equal(store.getSnapshot()[0].message, "New result");
});
