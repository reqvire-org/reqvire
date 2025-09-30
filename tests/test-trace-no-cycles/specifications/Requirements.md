# Trace Relations No Cycles Test Requirements

This document contains requirements with trace relations that form cycles to verify they don't cause circular dependency errors.

## Requirements

### Requirement Alpha

This requirement has trace relations that would form a cycle if trace relations were considered for dependency checking.

#### Metadata
  * type: user-requirement

#### Relations
  * trace: [Requirement Beta](#requirement-beta)
  * derive: [Child Requirement Alpha](#child-requirement-alpha)

---

### Requirement Beta

This requirement traces back to Alpha, forming a trace cycle that should not be detected as a circular dependency.

#### Metadata
  * type: user-requirement

#### Relations
  * trace: [Requirement Gamma](#requirement-gamma)
  * derive: [Child Requirement Beta](#child-requirement-beta)

---

### Requirement Gamma

This requirement completes the trace cycle back to Alpha, which should be allowed since trace is non-directional.

#### Metadata
  * type: user-requirement

#### Relations
  * trace: [Requirement Alpha](#requirement-alpha)
  * derive: [Child Requirement Gamma](#child-requirement-gamma)

---

### Child Requirement Alpha

This is a valid child requirement of Alpha.

#### Metadata
  * type: system-requirement

#### Relations
  * derivedFrom: [Requirement Alpha](#requirement-alpha)

---

### Child Requirement Beta

This is a valid child requirement of Beta.

#### Metadata
  * type: system-requirement

#### Relations
  * derivedFrom: [Requirement Beta](#requirement-beta)

---

### Child Requirement Gamma

This is a valid child requirement of Gamma.

#### Metadata
  * type: system-requirement

#### Relations
  * derivedFrom: [Requirement Gamma](#requirement-gamma)

---

### Complex Trace Scenario

This requirement has multiple trace relations forming complex patterns that should not trigger circular dependency errors.

#### Metadata
  * type: user-requirement

#### Relations
  * trace: [Requirement Alpha](#requirement-alpha)
  * trace: [Requirement Beta](#requirement-beta)
  * trace: [Requirement Gamma](#requirement-gamma)
  * derive: [Child Complex Requirement](#child-complex-requirement)

---

### Child Complex Requirement

This child requirement also has trace relations.

#### Metadata
  * type: system-requirement

#### Relations
  * derivedFrom: [Complex Trace Scenario](#complex-trace-scenario)
  * trace: [Child Requirement Alpha](#child-requirement-alpha)
  * trace: [Child Requirement Beta](#child-requirement-beta)

---