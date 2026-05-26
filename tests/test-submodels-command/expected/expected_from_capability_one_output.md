## Submodels

Independent capability-rooted subgraphs resolved via capability ownership relations.

### [Capability One](specifications/Requirements.md#capability-one)
  * Type: capability
  * Requirements: 5
---

## Cross-Submodel Couplings

Requirement-to-requirement relations where source and target belong to different capability roots.

  * [Invoice Requirement](specifications/Requirements.md#invoice-requirement) --trace--> [Identity Requirement](specifications/Requirements.md#identity-requirement) (Capability One -> Capability Two)
  * [Session Requirement](specifications/Requirements.md#session-requirement) --trace--> [Payments Requirement](specifications/Requirements.md#payments-requirement) (Capability Two -> Capability One)

## Summary

- **Submodels:** 1
- **Requirements:** 5
- **Cross-Submodel Couplings:** 2
