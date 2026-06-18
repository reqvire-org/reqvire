# Elements

### Billing Capability
Billing capability used by the contract relation migration fixture.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [Invoice Number Requirement](#invoice-number-requirement)

---

### Invoice Number Requirement
The system shall create stable invoice numbers.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Billing Capability](#billing-capability)
  * refinedBy: [Invoice Numbering Specification](#invoice-numbering-specification)

---

### Invoice Numbering Specification
Invoice numbers must be sequential per fiscal year and immutable after issue.

#### Metadata
  * type: specification

#### Relations
  * refine: [Invoice Number Requirement](#invoice-number-requirement)

---

