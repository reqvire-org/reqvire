import { useEffect, useRef, useLayoutEffect, useState } from "preact/hooks";
import type { ComponentChildren } from "preact";

import { css, cx } from "@linaria/atomic";
import { SearchInput } from "../../components/controls/SearchInput";

export type TokenSampleName = `--${string}`;
export type TokenSampleKind = "color" | "text" | "typography" | "spacing" | "shadow" | "shape" | "value";
export interface TokenSamplesProps {
  title: string;
  tokens: readonly TokenSampleName[];
  kind: TokenSampleKind;
  text: string;
  compactText: string;
  multilineText: string;
}

const layout = css`
  max-width: var(--space-showcase-page-max-w);
  margin-inline: auto;
  padding: var(--space-16) var(--space-12);
  .ux-token-header { display: flex; flex-wrap: wrap; align-items: center; gap: var(--space-8); padding-bottom: var(--space-8); border-bottom: var(--border-w) solid var(--border-subtle); margin-bottom: var(--space-12); }
  .ux-token-title { margin: 0 auto 0 0; font-size: var(--text-lg); font-weight: var(--weight-semibold); }
  .ux-token-search { max-width: var(--space-showcase-input-demo-lg-w); }
  .ux-token-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(min(var(--space-showcase-input-demo-lg-w), 100%), 1fr)); gap: var(--space-6) var(--space-8); }
  .ux-token-grid[data-kind="typography"], .ux-token-grid[data-kind="spacing"], .ux-token-grid[data-kind="value"] { display: flex; flex-direction: column; }
  .ux-token-summary { display: flex; align-items: center; gap: var(--space-6); cursor: pointer; list-style: none; }
  .ux-token-summary::-webkit-details-marker { display: none; }
  .ux-token-summary:focus-visible { outline: var(--focus-w) solid var(--border-focus); outline-offset: var(--focus-offset); }
  .ux-token-name { font-family: var(--font-mono); font-size: var(--text-caption); color: var(--text-muted); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; min-width: 0; }
  .ux-token-grid[data-kind="typography"] .ux-token-name, .ux-token-grid[data-kind="spacing"] .ux-token-name { flex: 0 0 var(--space-showcase-input-demo-lg-w); white-space: normal; overflow-wrap: anywhere; }
  .ux-token-grid[data-kind="value"] .ux-token-summary { display: grid; grid-template-columns: minmax(0, var(--space-showcase-input-demo-lg-w)) minmax(0, 1fr); align-items: start; }
  .ux-token-grid[data-kind="value"] .ux-token-name { white-space: normal; overflow-wrap: anywhere; }
  .ux-token-grid[data-kind="value"] .ux-token-note { font-family: var(--font-mono); overflow-wrap: anywhere; }
  .ux-token-grid[data-kind="value"] [data-effect="color"] { display: none; }
  .ux-token-grid[data-kind="shadow"] .ux-token-summary { flex-direction: column; align-items: stretch; }
  .ux-token-grid[data-kind="shadow"] .ux-token-name { white-space: normal; overflow-wrap: anywhere; }
  .ux-token-value { margin: var(--space-6) 0; padding: var(--space-6); border: var(--border-w) solid var(--border-subtle); border-radius: var(--radius-xs); overflow-wrap: anywhere; font-family: var(--font-mono); font-size: var(--text-caption); }
  .ux-token-contexts { display: inline-flex; gap: var(--space-1); flex: none; }
  .ux-token-light, .ux-token-dark { padding: var(--space-1) var(--space-2); border: var(--border-w) solid var(--border-default); border-radius: var(--radius-xs); }
  .ux-token-light { background: var(--slate-0); }
  .ux-token-dark { background: var(--slate-950); }
  .ux-token-note { color: var(--text-muted); font-size: var(--text-caption); }
  .ux-token-secondary { margin-top: var(--space-12); }
  .ux-token-secondary > summary { cursor: pointer; padding-block: var(--space-6); color: var(--text-muted); }
  @media (max-width: 600px) {
    .ux-token-summary { flex-wrap: wrap; }
    .ux-token-grid[data-kind="typography"] .ux-token-name, .ux-token-grid[data-kind="spacing"] .ux-token-name { flex-basis: 100%; }
    .ux-token-grid[data-kind="value"] .ux-token-summary { grid-template-columns: minmax(0, 1fr); }
  }
`;

const sampleStyle = css`
  display: inline-block;
  flex: none;
  max-width: 100%;
  overflow-wrap: anywhere;
  &[data-effect="size"], &[data-effect="font"], &[data-effect="weight"], &[data-effect="leading"], &[data-effect="tracking"] { flex: 0 1 auto; min-width: 0; }
  &[data-effect="color"] { width: var(--space-6); height: var(--space-6); background: var(--ux-token-sample-value); border: var(--border-w) solid var(--border-default); border-radius: var(--radius-xs); }
  &[data-effect="text"] { color: var(--ux-token-sample-value); }
  &[data-effect="size"] { font-size: var(--ux-token-sample-value); }
  &[data-effect="font"] { font-family: var(--ux-token-sample-value); }
  &[data-effect="weight"] { font-weight: var(--ux-token-sample-value); }
  &[data-effect="leading"] { line-height: var(--ux-token-sample-value); white-space: pre-line; }
  &[data-effect="tracking"] { letter-spacing: var(--ux-token-sample-value); }
  &[data-effect="spacing"] { width: min(var(--ux-token-sample-value), 100%); max-width: 100%; height: var(--space-5); background: var(--accent); border-radius: var(--radius-xs); }
  &[data-effect="shadow"] { width: 100%; height: var(--space-32); background: var(--bg-surface); border-radius: var(--radius-sm); box-shadow: var(--ux-token-sample-value); }
  &[data-effect="radius"] { width: var(--space-32); height: var(--space-32); border: var(--border-w) solid var(--border-default); border-radius: var(--ux-token-sample-value); }
`;

function Sample({ token, children, className, ...props }: { token: TokenSampleName; children?: ComponentChildren; className?: string; "data-effect": string; "aria-hidden"?: "true" }) {
  const ref = useRef<HTMLSpanElement>(null);
  useLayoutEffect(() => { ref.current?.style.setProperty("--ux-token-sample-value", `var(${token})`); }, [token]);
  return <span ref={ref} className={cx(sampleStyle, className)} {...props}>{children}</span>;
}

function effect(token: TokenSampleName, value: string, kind: TokenSampleKind) {
  if (typeof CSS === "undefined" || !value) return "value";
  if (CSS.supports("color", value)) return kind === "text" ? "text" : "color";
  if (kind === "typography") {
    if (token.startsWith("--font-") && CSS.supports("font-family", value)) return "font";
    if ((token.startsWith("--weight-") || token.endsWith("-weight")) && CSS.supports("font-weight", value)) return "weight";
    if (token.startsWith("--leading-") && CSS.supports("line-height", value)) return "leading";
    if (token.startsWith("--tracking-") && CSS.supports("letter-spacing", value)) return "tracking";
    if (CSS.supports("font-size", value)) return "size";
  }
  if (kind === "shadow" && CSS.supports("box-shadow", value)) return "shadow";
  if (kind === "shape" && token.startsWith("--radius-") && CSS.supports("border-radius", value)) return "radius";
  if (kind === "spacing" && CSS.supports("width", value)) return "spacing";
  return "value";
}

/** Project-owned review pattern. Inputs name real tokens; foundation CSS supplies values. */
export function TokenSamples({ title, tokens, kind, text, compactText, multilineText }: TokenSamplesProps) {
  const [search, setSearch] = useState("");
  const [values, setValues] = useState<Record<string, string>>({});
  useEffect(() => {
    const read = () => { const style = getComputedStyle(document.documentElement); setValues(Object.fromEntries(tokens.map(token => [token, style.getPropertyValue(token).trim()]))); };
    read();
    const observer = new MutationObserver(read);
    observer.observe(document.documentElement, { attributes: true, attributeFilter: ["data-theme"] });
    return () => observer.disconnect();
  }, [tokens]);
  const visible = tokens.filter(token => token.toLowerCase().includes(search.toLowerCase()));
  const primary = kind === "spacing" ? visible.filter(token => /^--space-\d+$/.test(token)) : kind === "typography" ? visible.filter(token => /^--(?:text-|font-)/.test(token)) : visible;
  const secondary = visible.filter(token => !primary.includes(token));
  function renderToken(token: TokenSampleName) {
      const value = values[token] ?? "";
      const mode = effect(token, value, kind);
      return <details key={token} data-token={token}>
        <summary className="ux-token-summary" title={`${token}: ${value}`}>
          {mode === "color" && <Sample token={token} data-effect={mode} aria-hidden="true" />}
          {mode === "text" && <Sample token={token} data-effect={mode} className="ux-token-contexts" aria-hidden="true"><span className="ux-token-light">{compactText}</span><span className="ux-token-dark">{compactText}</span></Sample>}
          <code className="ux-token-name">{token}{mode === "spacing" ? ` — ${value}` : ""}</code>
          {["size", "font", "weight", "leading", "tracking"].includes(mode) && <Sample token={token} data-effect={mode}>{mode === "leading" ? multilineText : text}</Sample>}
          {["spacing", "shadow", "radius"].includes(mode) && <Sample token={token} data-effect={mode} aria-hidden="true" />}
          {(mode === "value" || kind === "value") && <span className="ux-token-note">{value || "Conditional or unset"}</span>}
        </summary>
        <p className="ux-token-value">{value || "Conditional or unset"}</p>
      </details>;
  }
  return <section className={layout}>
    <header className="ux-token-header"><h1 className="ux-token-title">{title}</h1><div className="ux-token-search"><SearchInput aria-label="Find token" placeholder="Find a token" value={search} onInput={event => { setSearch(event.currentTarget.value); }} /></div></header>
    <div className="ux-token-grid" data-kind={kind}>{primary.map(renderToken)}</div>
    {!!secondary.length && <details className="ux-token-secondary" open={search ? true : undefined}><summary>Other token roles and dimensions ({secondary.length})</summary><div className="ux-token-grid" data-kind={kind}>{secondary.map(renderToken)}</div></details>}
    {!visible.length && <p className="ux-token-note">No matching tokens.</p>}
  </section>;
}
