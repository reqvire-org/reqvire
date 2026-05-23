## Submodels

Independent feature-rooted subgraphs resolved via feature ownership relations.

### [Feature One](specifications/Requirements.md#feature-one)
  * Type: feature
  * Requirements: 5
---

### [Feature Two](specifications/Requirements.md#feature-two)
  * Type: feature
  * Requirements: 3
---

## Cross-Submodel Couplings

Requirement-to-requirement relations where source and target belong to different feature roots.

  * [Invoice Requirement](specifications/Requirements.md#invoice-requirement) --trace--> [Identity Requirement](specifications/Requirements.md#identity-requirement) (Feature One -> Feature Two)
  * [Session Requirement](specifications/Requirements.md#session-requirement) --trace--> [Payments Requirement](specifications/Requirements.md#payments-requirement) (Feature Two -> Feature One)

## Summary

- **Submodels:** 2
- **Requirements:** 8
- **Cross-Submodel Couplings:** 2
