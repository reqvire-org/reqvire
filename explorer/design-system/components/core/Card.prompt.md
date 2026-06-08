Base surface for model-browser tiles (folder / file / element) and panels. Compose the inside yourself.

```jsx
<Card interactive onClick={openFolder}>
  <div style={{display:'flex',alignItems:'center',gap:8}}>
    <ElementIcon type="capability" />
    <strong>Capabilities</strong>
  </div>
  <Badge>8 children</Badge>
</Card>

<Card selected>…the active tile…</Card>
```

Props: `interactive` (hover lift), `selected` (dark ring), `padded` (default true), `accentColor` (left stripe).
