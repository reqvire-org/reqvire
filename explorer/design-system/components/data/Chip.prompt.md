Filter / facet pill — icon + label (+ optional count) that toggles to a dark filled state. Used for the browser's "folder / source file / modeled element" facets and result-type filters.

```jsx
<Chip icon={<Icon name="folder" size={13} />}>folder</Chip>
<Chip icon={<Icon name="box" size={13} />} active>modeled element</Chip>
<Chip count={50}>Elements</Chip>
```
