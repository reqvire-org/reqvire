import { useEffect, useRef, useLayoutEffect, useState } from "preact/hooks";
import type { ComponentChildren } from "preact";

import { css, cx } from "@linaria/atomic";
import { SearchInput } from "../../components/controls/SearchInput";

export type TokenSampleName = `--${string}`;
export interface TokenSampleBinding { token: TokenSampleName; property: string; probe: string; }
export type TokenSampleKind = "color" | "text" | "typography" | "spacing" | "shadow" | "shape" | "value";
export interface TokenSamplesProps {
  title: string;
  samples: readonly TokenSampleBinding[];
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
  .ux-token-grid[data-kind="value"] .ux-token-summary { display: grid; grid-template-columns: minmax(0, var(--space-showcase-input-demo-lg-w)) minmax(0, 1fr) minmax(0, 1fr); align-items: center; }
  .ux-token-grid[data-kind="value"] .ux-token-name { grid-column: 1; grid-row: 1; white-space: normal; overflow-wrap: anywhere; }
  .ux-token-grid[data-kind="value"] [data-effect] { grid-column: 2; grid-row: 1; }
  .ux-token-grid[data-kind="value"] .ux-token-note { grid-column: 3; grid-row: 1; font-family: var(--font-mono); overflow-wrap: anywhere; }
  .ux-token-grid[data-kind="shadow"] .ux-token-summary { flex-direction: column; align-items: stretch; }
  .ux-token-grid[data-kind="shadow"] .ux-token-name { white-space: normal; overflow-wrap: anywhere; }
  .ux-token-value { margin: var(--space-6) 0; padding: var(--space-6); border: var(--border-w) solid var(--border-subtle); border-radius: var(--radius-xs); overflow-wrap: anywhere; font-family: var(--font-mono); font-size: var(--text-caption); }
  .ux-token-contexts { display: inline-flex; gap: var(--space-1); flex: none; }
  .ux-token-light, .ux-token-dark { padding: var(--space-1) var(--space-2); border: var(--border-w) solid var(--border-default); border-radius: var(--radius-xs); }
  .ux-token-light { background: var(--color-paper); }
  .ux-token-dark { background: var(--color-ink); }
  .ux-token-note { color: var(--text-muted); font-size: var(--text-caption); }
  .ux-token-secondary { margin-top: var(--space-12); }
  .ux-token-secondary > summary { cursor: pointer; padding-block: var(--space-6); color: var(--text-muted); }
  @media (max-width: 600px) {
    .ux-token-summary { flex-wrap: wrap; }
    .ux-token-grid[data-kind="typography"] .ux-token-name, .ux-token-grid[data-kind="spacing"] .ux-token-name { flex-basis: 100%; }
    .ux-token-grid[data-kind="value"] .ux-token-summary { grid-template-columns: minmax(0, 1fr); }
    .ux-token-grid[data-kind="value"] .ux-token-summary > * { grid-column: auto; grid-row: auto; }
  }
`;

const sampleStyle = css`
  position: relative;
  &[data-effect="z-index"] { z-index: var(--ux-token-sample-value); }
  &[data-effect="filter"] { filter: var(--ux-token-sample-value); }
  &[data-effect="animation-duration"] { animation-duration: var(--ux-token-sample-value); }
  &[data-effect="animation-timing-function"] { animation-timing-function: var(--ux-token-sample-value); }
  &[data-effect="opacity"] { opacity: var(--ux-token-sample-value); }
  &[data-effect="margin"] { margin: var(--ux-token-sample-value); }
  &[data-effect="margin-top"] { margin-top: var(--ux-token-sample-value); }
  &[data-effect="grid-template-columns"] { display: inline-grid; grid-template-columns: var(--ux-token-sample-value); gap: var(--space-2); }
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
  /* Column counts draw that many columns: stripes with a gap, filling the row. */
  &[data-effect="column-count"] { display: block; width: 100%; height: var(--space-8); column-count: var(--ux-token-sample-value); background: repeating-linear-gradient(to right, var(--accent) 0 calc((100% + var(--space-2)) / var(--ux-token-sample-value) - var(--space-2)), transparent 0 calc((100% + var(--space-2)) / var(--ux-token-sample-value))); }
`;

function Sample({ token, property, children, className, ...props }: { token: TokenSampleName; property: string; children?: ComponentChildren; className?: string; "data-effect": string; "aria-hidden"?: "true" }) {
  const ref = useRef<HTMLSpanElement>(null);
  useLayoutEffect(() => { ref.current?.style.setProperty("--ux-token-sample-value", `var(${token})`); }, [token]);
  return <span ref={ref} className={cx(sampleStyle, className)} data-ads-token={token} data-ads-property={property} {...props}>{children}</span>;
}

function effect(property: string) {
  const roles: Record<string, string> = { "background-color": "color", "color": "text", "font-family": "font", "font-size": "size", "font-weight": "weight", "line-height": "leading", "letter-spacing": "tracking", "width": "spacing", "box-shadow": "shadow", "border-radius": "radius" };
  return roles[property] ?? property;
}

/** Project-owned review pattern. Inputs name real tokens; foundation CSS supplies values. */
export function TokenSamples({ title, samples, kind, text, compactText, multilineText }: TokenSamplesProps) {
  const tokens = samples.map(sample => sample.token);
  const [search, setSearch] = useState("");
  const [values, setValues] = useState<Record<string, string>>({});
  useEffect(() => {
    const read = () => { const style = getComputedStyle(document.documentElement); setValues(Object.fromEntries(tokens.map(token => [token, style.getPropertyValue(token).trim()]))); };
    read();
    const observer = new MutationObserver(read);
    observer.observe(document.documentElement, { attributes: true, attributeFilter: ["data-theme"] });
    return () => observer.disconnect();
  }, [samples]);
  const visible = tokens.filter(token => token.toLowerCase().includes(search.toLowerCase()));
  const primary = kind === "spacing" ? visible.filter(token => /^--space-\d+$/.test(token)) : kind === "typography" ? visible.filter(token => /^--(?:text-|font-)/.test(token)) : visible;
  const secondary = visible.filter(token => !primary.includes(token));
  function renderToken(token: TokenSampleName) {
      const value = values[token] ?? "";
      const property = samples.find(sample => sample.token === token)!.property;
      const mode = effect(property);
      return <details key={token} data-token={token}>
        <summary className="ux-token-summary" title={`${token}: ${value}`}>
          {mode === "color" && <Sample token={token} property={property} data-effect={mode} aria-hidden="true" />}
          {mode === "text" && <Sample token={token} property={property} data-effect={mode} className="ux-token-contexts" aria-hidden="true"><span className="ux-token-light">{compactText}</span><span className="ux-token-dark">{compactText}</span></Sample>}
          <code className="ux-token-name">{token}{mode === "spacing" ? ` — ${value}` : ""}</code>
          {["size", "font", "weight", "leading", "tracking"].includes(mode) && <Sample token={token} property={property} data-effect={mode}>{mode === "leading" ? multilineText : text}</Sample>}
          {["spacing", "shadow", "radius", "column-count"].includes(mode) && <Sample token={token} property={property} data-effect={mode} aria-hidden="true" />}
          {mode === "grid-template-columns" && <Sample token={token} property={property} data-effect={mode}><span>{compactText}</span><span>{compactText}</span></Sample>}
          {!["color", "text", "size", "font", "weight", "leading", "tracking", "spacing", "shadow", "radius", "column-count", "grid-template-columns"].includes(mode) && <Sample token={token} property={property} data-effect={mode}>{compactText}</Sample>}
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
