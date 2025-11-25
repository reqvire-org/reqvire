# Elements

This file should be ignored by Reqvire based on the configuration.


```mermaid
graph TD;
    A[Ignored Diagram] --> B[Should Not Be Removed];
    B --> C[This is in Excluded File];
```

### Some Requirement

This looks like a real requirement but should be ignored.

#### Relations
  * verifiedBy: [Some Test](#some-test)

---


```mermaid
graph LR;
    X[Another Ignored Diagram] --> Y[Also Should Stay];
    Y --> Z[Because File is Excluded];
```

### Some Test

This verification should also be ignored.

#### Metadata
  * type: verification

---
