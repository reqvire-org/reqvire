# Elements

### Verification Traceability

As a **V&V Engineer**, I want Reqvire to trace verification evidence through capabilities, requirements, and capability roots, so that I can see which abilities and obligations are verified, which are blocked, and which capabilities still have coverage gaps.

#### Details
Verification traceability is the capability for verification objectives, concrete verification elements, verification evidence, direct capability verification, and requirement verification rollup.

Verification objectives organize mandatory verification planning hierarchy. Every concrete verification element must derive from a verification objective parent. Concrete verification elements verify capabilities or requirements. Capabilities may be directly verified; capability coverage status also rolls up from requirements that specify the capability.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: high
  * risk: high
  * status: approved

#### Relations
  * specifiedBy: [Verification Upward Traceability](Traceability/VerificationTracesRequirements.md#verification-upward-traceability)
---

