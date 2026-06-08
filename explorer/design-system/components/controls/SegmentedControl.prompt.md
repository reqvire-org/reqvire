Compact 2–4 option switch with a dark-ink active segment. Used for List/Grid and chart/table view toggles.

```jsx
<SegmentedControl
  value={view}
  onChange={setView}
  items={[
    { value: 'list', label: 'List', icon: <Icon name="list" size={14} /> },
    { value: 'grid', label: 'Grid', icon: <Icon name="grid" size={14} /> },
  ]}
/>
```
