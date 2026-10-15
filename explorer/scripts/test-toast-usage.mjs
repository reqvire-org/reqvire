import { describe, it } from "node:test";
import { RuleTester } from "oxlint/plugins-dev";
import { toastRules } from "../design-system/components/core/toast/toast.oxlint.mjs";

RuleTester.describe = describe;
RuleTester.it = it;
const tester = new RuleTester({ languageOptions: { parserOptions: { lang: "tsx" } } });
const toastImport = 'import { toast } from "@ds";';
const toasterImport = 'import { Toaster } from "@ds";';
const effectImport = 'import { useEffect } from "preact/hooks";';
const root = "src/App.tsx";

tester.run("toast-client-usage", toastRules["toast-client-usage"], {
  valid: [
    `${toastImport} function App() { return <button onClick={() => toast.success("Saved")}>Save</button>; }`,
    `${toastImport} ${effectImport} function App() { useEffect(() => toast("Connected", { id: "connection" }), []); return null; }`,
    `${toastImport} function notifySaved() { toast.success("Saved"); }`,
    `${toastImport} export default function notifySaved() { toast.success("Saved"); }`,
    `${toastImport} function App() { const toast = () => {}; toast(); return null; }`,
    'import { toast } from "another-library"; toast("Other API");',
  ],
  invalid: [
    { code: `${toastImport} toast("Top level");`, errors: [{ messageId: "render" }] },
    { code: `${toastImport} function App() { toast("During render"); return null; }`, errors: [{ messageId: "render" }] },
    { code: `${toastImport} const App = () => { toast.loading("During render"); return null; };`, errors: [{ messageId: "render" }] },
    { code: `${toastImport} export default () => { toast("During render"); return null; };`, errors: [{ messageId: "render" }] },
    { code: 'import { toast as notify } from "@ds"; function App() { notify.error("During render"); return null; }', errors: [{ messageId: "render" }] },
    { code: 'import * as ds from "@ds"; function App() { ds.toast.success("During render"); return null; }', errors: [{ messageId: "render" }] },
    { code: '"use server"; import { toast } from "@ds"; export async function save() { toast.success("Saved"); }', errors: [{ messageId: "server" }] },
  ],
});

tester.run("toast-effect-id", toastRules["toast-effect-id"], {
  valid: [
    `${toastImport} ${effectImport} function App() { useEffect(() => toast("Connected", { id: "connection" }), []); }`,
    `${toastImport} ${effectImport} function App() { useEffect(() => toast.update("connection", { message: "Ready" }), []); }`,
    `${toastImport} ${effectImport} function App() { useEffect(() => toast.dismiss("connection"), []); }`,
    `${toastImport} ${effectImport} function App() { useEffect(() => { window.addEventListener("online", () => toast("Connected")); }, []); }`,
    `${toastImport} ${effectImport} function App() { useEffect(() => toast("Connected", options), []); }`,
    `${toastImport} ${effectImport} function App() { useEffect(() => toast("Connected", { ...options }), []); }`,
    `${toastImport} function useEffect(fn) { fn(); } useEffect(() => toast("Not React"));`,
  ],
  invalid: [
    { code: `${toastImport} ${effectImport} function App() { useEffect(() => toast("Connected"), []); }`, errors: [{ messageId: "id" }] },
    { code: `${toastImport} ${effectImport} function App() { useEffect(() => toast.success("Connected", { description: "Details" }), []); }`, errors: [{ messageId: "id" }] },
    { code: `${toastImport} import { useEffect as effect } from "react"; function App() { effect(() => toast("Connected"), []); }`, errors: [{ messageId: "id" }] },
    { code: `${toastImport} import * as React from "react"; function App() { React.useEffect(() => toast("Connected"), []); }`, errors: [{ messageId: "id" }] },
    { code: `${toastImport} ${effectImport} function App() { useEffect(() => toast.promise(save(), { loading: "Saving", success: "Saved", error: "Failed" }), []); }`, errors: [{ messageId: "id" }] },
  ],
});

tester.run("toaster-root-mount", toastRules["toaster-root-mount"], {
  valid: [
    { filename: root, code: `${toasterImport} function App() { return <><Toaster /><Content /></>; }` },
    { filename: root, code: `${toasterImport} function App() { return <><Toaster id="one" /><Toaster id="two" /></>; }` },
    { filename: "src/CustomRoot.tsx", options: [{ rootFiles: ["src/CustomRoot.tsx"] }], code: `${toasterImport} function App() { return <Toaster />; }` },
    { filename: "src/Page.tsx", code: 'function Toaster() { return null; } function Page() { return <Toaster />; }' },
  ],
  invalid: [
    { filename: "src/Page.tsx", code: `${toasterImport} function Page() { return <Toaster />; }`, errors: [{ messageId: "root" }] },
    { filename: root, code: `${toasterImport} function App() { return <>{open && <Toaster />}</>; }`, errors: [{ messageId: "conditional" }] },
    { filename: root, code: `${toasterImport} function App() { return open ? <Toaster /> : null; }`, errors: [{ messageId: "conditional" }] },
    { filename: root, code: `${toasterImport} function App() { return <><Toaster /><Toaster /></>; }`, errors: [{ messageId: "duplicate" }] },
    { filename: root, code: `${toasterImport} function App() { return <><Toaster id="same" /><Toaster id={"same"} /></>; }`, errors: [{ messageId: "duplicate" }] },
    { filename: "src/Page.tsx", code: 'import { Toaster as Notices } from "@ds"; function Page() { return <Notices />; }', errors: [{ messageId: "root" }] },
    { filename: "src/Page.tsx", code: 'import * as ds from "@ds"; function Page() { return <ds.Toaster />; }', errors: [{ messageId: "root" }] },
  ],
});
