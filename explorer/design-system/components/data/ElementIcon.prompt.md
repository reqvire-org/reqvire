Colored type glyph for a model element. Color + shape derive from `type`; `capability` → dark hub with a blue pip, requirement/verification/ontology/resource → type-colored square, and refinement-family types → orange diamond with a subtype mark (`source`, `specification`, `constraint`, `behavior`, `state`, `input-output`, `semantic-contract`).

```jsx
<ElementIcon type="capability" />
<ElementIcon type="verification" size="sm" />
<ElementIcon type="semantic-contract" />   {/* diamond */}
```

Element types: `capability`, `requirement`, `user-story`, `refinement`, `verification`, `specification`, `semantic-contract`, `semantic-query-contract`, `ontology`, `resource`, `other`. Override the glyph with `shape`. Sizes `sm` · `md` · `lg`.
