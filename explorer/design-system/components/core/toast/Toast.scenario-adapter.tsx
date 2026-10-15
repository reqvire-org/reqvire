import { ShowcaseRegion } from "@ds";
import { useState } from "preact/hooks";
import { Button, ToggleRow, Toast, toast, type ToastOptions, type ToastPosition } from "@ds";
import { useToastDemoSettings } from "./settings.scenario-adapter";
const TARGET: ToastOptions = {};
const POSITIONS: ToastPosition[] = ["bottom-right", "bottom-center", "bottom-left", "top-right", "top-center", "top-left"];
export function ToastDemo() {
    const { position, setPosition, expand, setExpand } = useToastDemoSettings();
    const [undone, setUndone] = useState(false);
    const promise = (fail: boolean) => {
        toast.promise(() => new Promise<string>((resolve, reject) => {
            window.setTimeout(() => fail ? reject(new Error("The connection timed out. Please try again.")) : resolve("Your changes are now up to date."), 1400);
        }), {
            ...TARGET,
            loading: "Saving changes…",
            success: (message) => message,
            error: (error) => error instanceof Error ? error.message : "Something went wrong.",
        });
    };
    return (<ShowcaseRegion regions={["col"]}>
      <ShowcaseRegion regions={["row", "row--center"]}>
        {POSITIONS.map((value) => <Button key={value} size="sm" aria-pressed={position === value} onClick={() => setPosition(value)}>{value}</Button>)}
        <ToggleRow label="Always expand toasts" on={expand} onToggle={() => setExpand(!expand)} />
        <Button size="sm" tone="ghost" onClick={() => toast.dismiss()}>Clear toasts</Button>
      </ShowcaseRegion>
      <ShowcaseRegion regions={["row"]}>
        <Button onClick={() => toast("Event has been created", TARGET)}>Default toast</Button>
        <Button onClick={() => toast.success("Changes saved", { ...TARGET, description: "Your workspace is up to date." })}>Success toast</Button>
        <Button onClick={() => toast.error("Could not save changes", { ...TARGET, description: "Check your connection and try again." })}>Error toast</Button>
        <Button onClick={() => toast.warning("Your session expires soon", TARGET)}>Warning toast</Button>
        <Button onClick={() => toast.info("A new version is available", TARGET)}>Info toast</Button>
      </ShowcaseRegion>
      <ShowcaseRegion regions={["row"]}>
        <Button onClick={() => {
            toast("First notification", { ...TARGET, description: "A short message." });
            toast.success("Second notification", { ...TARGET, description: "This taller card demonstrates how the stack expands to each card’s natural height, even when the messages wrap over several lines." });
            toast.info("Third notification", TARGET);
        }}>Stack three</Button>
        <Button onClick={() => toast("A longer notification", {
            ...TARGET,
            description: "This notification has a longer description so you can compare the compact stack with its expanded layout. Hover over the stack, use Alt+T to focus it, or swipe a card toward the edge to dismiss it.",
        })}>Long message</Button>
        <Button onClick={() => {
            setUndone(false);
            toast("Item archived", { ...TARGET, action: { label: "Undo", onClick: () => setUndone(true) } });
        }}>Undo action</Button>
        <Button onClick={() => toast("Waiting for your review", { ...TARGET, duration: Infinity, closeButton: true, description: "This notification stays until you dismiss it." })}>Persistent toast</Button>
        <Button onClick={() => promise(false)}>Loading → success</Button>
        <Button onClick={() => promise(true)}>Loading → error</Button>
      </ShowcaseRegion>
      <ShowcaseRegion regions={["label"]} role="status">
        {undone ? "Archive undone." : "Hover to expand · swipe toward an edge to dismiss · Alt+T to focus · Escape to return"}
      </ShowcaseRegion>
      <ShowcaseRegion regions={["col"]}>
        <Toast title="Changes saved" description="Your workspace is up to date." variant="success"/>
        <Toast title="Connection interrupted" description="Try again in a moment." variant="danger"/>
        <Toast title="Preparing your export…" variant="loading"/>
      </ShowcaseRegion>
    </ShowcaseRegion>);
}
