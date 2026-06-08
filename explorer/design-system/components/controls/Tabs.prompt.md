Primary mode switcher — the top header tabs for the explorer's modes (Model · Knowledge Graph · Ontologies · Search · Traces). Icons clarify each mode; a trailing `badge` can show counts.

```jsx
<Tabs
  value={mode}
  onChange={setMode}
  items={[
    { value: 'model', label: 'Model', icon: <Icon name="folder" size={15} /> },
    { value: 'graph', label: 'Knowledge Graph', icon: <Icon name="network" size={15} /> },
    { value: 'ontologies', label: 'Ontologies', icon: <Icon name="globe" size={15} /> },
    { value: 'search', label: 'Search', icon: <Icon name="search" size={15} /> },
    { value: 'traces', label: 'Traces', icon: <Icon name="activity" size={15} />, badge: 145 },
  ]}
/>
```

`variant="underline"` (default) for the header bar; `variant="pill"` for a compact inline segmented switch.
