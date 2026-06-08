Label + value pair for sidebar SUMMARY rows. Wrap several in `StatRow` for the inline "Submodels 13  Elements 640" treatment, or use `stacked` for a KPI.

```jsx
<StatRow>
  <Stat label="Submodels" value={13} />
  <Stat label="Elements" value={640} />
  <Stat label="Relations" value={1090} />
</StatRow>

<Stat label="Verifications" value={145} stacked />
```
