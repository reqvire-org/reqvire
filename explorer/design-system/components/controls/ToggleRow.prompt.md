Full-width legend toggle (colored swatch + label + optional count). The explorer's SHOW / TYPES panels use these to filter the graph by element type; `line` renders an edge-style swatch for OVERLAYS legends.

```jsx
<ToggleRow label="Requirement" color="var(--requirement)" meta={420} on={show.req} onToggle={() => toggle('req')} />
<ToggleRow label="Verification" color="var(--verification)" on={false} onToggle={…} />
<ToggleRow label="Trace" color="var(--edge-trace)" line />
```
