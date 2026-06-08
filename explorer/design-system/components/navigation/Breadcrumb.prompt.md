Path trail above the browser grid. Last item is the current location; earlier items are clickable.

```jsx
<Breadcrumb items={[
  { label: 'Reqvire root', onClick: () => goTo('/') },
  { label: 'requirements', onClick: () => goTo('/requirements') },
  { label: 'Capabilities.md' },
]} />
```
