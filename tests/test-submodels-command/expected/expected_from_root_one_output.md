## Submodels

Independent requirement hierarchies resolved via `derivedFrom` relations.

### [Billing Requirement](specifications/Requirements.md#billing-requirement)
  * Type: requirement
  * Requirements: 2
---

### [Payments Requirement](specifications/Requirements.md#payments-requirement)
  * Type: requirement
  * Requirements: 2
---

## Cross-Submodel Couplings

Requirement-to-requirement relations where source and target belong to different top roots.

  * [Invoice Requirement](specifications/Requirements.md#invoice-requirement) --trace--> [Identity Requirement](specifications/Requirements.md#identity-requirement) (Root One -> Root Two)
  * [Session Requirement](specifications/Requirements.md#session-requirement) --trace--> [Payments Requirement](specifications/Requirements.md#payments-requirement) (Root Two -> Root One)

## Summary

- **Submodels:** 2
- **Requirements:** 4
- **Cross-Submodel Couplings:** 2
