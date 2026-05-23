## Submodels

Independent feature-rooted subgraphs resolved via feature ownership relations.

### [Billing Requirement](specifications/Requirements.md#billing-requirement)
  * Type: requirement
  * Requirements: 2
---

### [Payments Requirement](specifications/Requirements.md#payments-requirement)
  * Type: requirement
  * Requirements: 2
---

## Cross-Submodel Couplings

Requirement-to-requirement relations where source and target belong to different feature roots.

  * [Invoice Requirement](specifications/Requirements.md#invoice-requirement) --trace--> [Identity Requirement](specifications/Requirements.md#identity-requirement) (Feature One -> Feature Two)
  * [Session Requirement](specifications/Requirements.md#session-requirement) --trace--> [Payments Requirement](specifications/Requirements.md#payments-requirement) (Feature Two -> Feature One)

## Summary

- **Submodels:** 2
- **Requirements:** 4
- **Cross-Submodel Couplings:** 2
