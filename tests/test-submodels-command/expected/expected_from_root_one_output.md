## Submodels

Independent capability-rooted subgraphs resolved via capability ownership relations.

### [Billing Requirement](specifications/Requirements.md#billing-requirement)
  * Type: requirement
  * Requirements: 2
---

### [Payments Requirement](specifications/Requirements.md#payments-requirement)
  * Type: requirement
  * Requirements: 2
---

## Cross-Submodel Couplings

Requirement-to-requirement relations where source and target belong to different capability roots.

  * [Invoice Requirement](specifications/Requirements.md#invoice-requirement) --trace--> [Identity Requirement](specifications/Requirements.md#identity-requirement) (Capability One -> Capability Two)
  * [Session Requirement](specifications/Requirements.md#session-requirement) --trace--> [Payments Requirement](specifications/Requirements.md#payments-requirement) (Capability Two -> Capability One)

## Summary

- **Submodels:** 2
- **Requirements:** 4
- **Cross-Submodel Couplings:** 2
