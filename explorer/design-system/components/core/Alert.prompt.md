Inline status or validation message. Use for local feedback inside panes, modals, and forms; keep the message short and actionable.

```jsx
<Alert>Search index is still loading.</Alert>
<Alert variant="warning">Some verification traces could not be rendered.</Alert>
<Alert variant="danger">Project Store data is missing.</Alert>
<Alert variant="success">Explorer bundle generated successfully.</Alert>
```

Variants: `default`, `danger`, `warning`, `success`. The default `role` is `alert`; set `role="status"` for non-urgent progress updates. Do not use Alert as a card, page banner, or decorative container.
