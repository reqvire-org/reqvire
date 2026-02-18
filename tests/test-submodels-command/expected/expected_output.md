## Submodels

Independent requirement hierarchies resolved via `derivedFrom` relations.

### [Root One](specifications/Requirements.md#root-one)
  * Type: user-requirement
  * Requirements: 5
---

### [Root Two](specifications/Requirements.md#root-two)
  * Type: user-requirement
  * Requirements: 3
---

## Cross-Submodel Couplings

Requirement-to-requirement relations where source and target belong to different top roots.

  * [Invoice Requirement](specifications/Requirements.md#invoice-requirement) --trace--> [Identity Requirement](specifications/Requirements.md#identity-requirement) (Root One -> Root Two)
  * [Session Requirement](specifications/Requirements.md#session-requirement) --trace--> [Payments Requirement](specifications/Requirements.md#payments-requirement) (Root Two -> Root One)

## Summary

- **Submodels:** 2
- **Requirements:** 8
- **Cross-Submodel Couplings:** 2
