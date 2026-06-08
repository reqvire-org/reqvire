A typed relation row — a monospace "kind" tag next to a clickable target chip. Used in the element detail view's incoming/outgoing relations.

```jsx
<RelationPill kind="specifiedBy" target="Git Repository as Project Root" type="specification" onOpen={open} />
<RelationPill kind="verifiedBy" target="CLI Search Command Test" type="verification" />
```

Common kinds: `derivedFrom`, `refinedBy`, `satisfiedBy`, `verifiedBy`, `specifiedBy`, `trace`. Pass `type` to color the target pip.
