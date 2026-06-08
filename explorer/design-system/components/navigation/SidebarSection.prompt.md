Labeled sidebar group — uppercase title (+ optional action) wrapping a stack of controls. Builds the explorer sidebar's SUMMARY / SHOW / OVERLAYS / TYPES sections.

```jsx
<SidebarSection title="Show">
  <ToggleRow label="Requirement" color="var(--requirement)" />
  <ToggleRow label="Verification" color="var(--verification)" />
</SidebarSection>

<SidebarSection title="Filters" action={<Button tone="link" size="sm">Reset</Button>}>
  …
</SidebarSection>
```
