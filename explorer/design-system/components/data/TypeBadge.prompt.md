Lowercase type-slug chip — the small "capability" / "specification" / "semantic-contract" tag seen on element tiles and headers.

```jsx
<TypeBadge type="capability" />
<TypeBadge type="semantic-contract" />
<TypeBadge type="verification" tinted />
<TypeBadge type="requirement" dot={false}>requirement</TypeBadge>
```

Neutral gray with a colored dot by default; `tinted` fills with the type hue. Label defaults to the type slug — override via children.
