Square icon-only button — explorer chrome, toolbars, card actions, modal close. Always pass `aria-label`.

```jsx
<IconButton aria-label="Search"><Icon name="search" /></IconButton>
<IconButton aria-label="Grid view" active><Icon name="grid" /></IconButton>
<IconButton aria-label="Close" size="sm" tone="ghost"><Icon name="x" /></IconButton>
```

Sizes: `sm` · `md`. `tone="ghost"` removes the hover fill. `active` paints the rose tint (used for selected rail items).
