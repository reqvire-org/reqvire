Text action button — defaults to a quiet outlined `secondary`; use one `primary` (dark ink) per view, `accent` for brand moments.

```jsx
<Button tone="primary" iconLeft={<Icon name="plus" />}>Add element</Button>
<Button tone="secondary">Reset layout</Button>
<Button tone="ghost" size="sm">Cancel</Button>
```

Tones: `primary` (dark slate fill), `accent`, `secondary` (outlined surface, the default), `ghost` (transparent), `link` (inline), `danger`. Sizes: `sm` · `md` · `lg`. Props: `iconLeft`, `iconRight`, `block`. Inherits all native `<button>` attributes (`onClick`, `disabled`, ...).
