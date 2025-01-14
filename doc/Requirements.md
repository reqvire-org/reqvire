# Requirements in ReqFlow

## What Are Requirements?

In **ReqFlow**, a requirement represents a stakeholder's need, system capability, or constraint that the system must fulfill. Requirements define **what the system must do** (functional requirements) and **how well it must perform** (non-functional requirements), serving as the foundation for system design, development, and verification.

Mention EARS format but say that ReqFlow doesn't mandate specific format for requirements expression.


## Structure of Requirements in ReqFlow

The diagram below provides an example of how requirements, relationships, and their hierarchical structure are organized within the **ReqFlow methodology**. It illustrates the integration of stakeholder requirements, system requirements, and their connections to system elements, test cases, and other elements. 

```mermaid
graph TD
    subgraph Requirements Design
  

      subgraph System Requirements        
        subgraph IDP

            REQ_ENCRYPT["**REQ_ID**: 2.1<br>**Text**: The system shall encrypt authentication data."]
            style REQ_ENCRYPT font-color:#000000,fill:#FFCCCC,stroke:#FF0000,stroke-width:2px;


            REQ_SESSION["**REQ_ID**: 2.2<br>**Text**: The system shall manage user sessions securely."]
            style REQ_SESSION font-color:#000000,fill:#FFCCCC,stroke:#FF0000,stroke-width:2px;



            REQ_PASSWORD["**REQ_ID**: 1.1<br>**Text**: The system shall support password-based authentication."]
            style REQ_PASSWORD font-color:#000000,fill:#FFCCCC,stroke:#FF0000,stroke-width:2px;

            REQ_OAUTH["**REQ_ID**: 1.2<br>**Text**: The system shall support federated login using OAuth."]
            style REQ_OAUTH font-color:#000000,fill:#FFCCCC,stroke:#FF0000,stroke-width:2px;


            %% Test Case (verifies)
            TestPasswordStrength["Test Case: Password Strength Validation"]
            style TestPasswordStrength fill:#CCFFCC,stroke:#008000,stroke-width:2px;

        end 

      end

      subgraph Stakeholder Needs

        subgraph Mission Requirements

                MOE_CPD["MOE_CPD: Decrease Costs and Increase Profitability"]
    
                REQ_LIMITS["**REQ_ID**: 3<br>**Text**:Specification Design Document for Resource Rates and Limits"]
                style REQ_LIMITS font-color:#000001,fill:#FFCCCC,stroke:#FF0000,stroke-width:2px;

                SDD_LIMITS["SDD_LIMITS_AND_RATES: Specification Design Document: Limits and Rates"]
                style SDD_LIMITS font-color:#000001,fill:#FFA500,stroke:#db9d00,stroke-width:2px;

        end
        subgraph User Requirements

                MOE_CR["MOE_CR: Maintain High Customer Retention by Reducing Churn"]
                USER_STORY_PASSWORD["User Story: User Sign-Up and Sign-In"]
                style USER_STORY_PASSWORD fill:#FFFF99,stroke:#FFD700,stroke-width:2px;
    
                REQ_AUTH["**REQ_ID**: 1<br>**Text**: The system shall provide user authentication."]
                style REQ_AUTH font-color:#000000,fill:#FFCCCC,stroke:#FF0000,stroke-width:2px;  
    
                REQ_SECURITY["**REQ_ID**: 2<br>**Text**: The system shall ensure authentication security."]
                style REQ_SECURITY font-color:#000001,fill:#FFCCCC,stroke:#FF0000,stroke-width:2px;

        end    
    end 

    MOE_CPD -.->|trace| REQ_LIMITS
    SDD_LIMITS -.->|satisy| REQ_LIMITS

    MOE_CR -.->|trace| USER_STORY_PASSWORD
    USER_STORY_PASSWORD -.->|refine| REQ_AUTH
    USER_STORY_PASSWORD -.->|refine| REQ_SECURITY
    
    %% Relationships
    REQ_ENCRYPT -.->|derive| REQ_SECURITY 
    REQ_SESSION -.->|derive| REQ_SECURITY


    REQ_AUTH -.->|contains| REQ_PASSWORD
    REQ_AUTH -.->|contains| REQ_OAUTH


    TestPasswordStrength -.->|verify| REQ_PASSWORD


    %% Click Actions
    click TestPasswordStrength href "https://example.com/docs/test-case-222" "Test Case Documentation"
    click USER_STORY_PASSWORD href "https://example.com/docs/user-story-password-login" "User Story Documentation"
    click USER_STORY_OAUTH href "https://example.com/docs/user-story-google-login" "User Story Documentation"
    click AuthenticationSubsystem href "https://example.com/docs/authentication-subsystem" "Subsystem Documentation"
    click LoginBehavior href "https://example.com/docs/login-behavior" "Behavior Documentation"




end

```

Requirements in ReqFlow are structurally divided into 3 categories:
 * User Requirements
 * Mission Requirements
 * System Requirements
 
Format of User Requirements is different from the later two.


### User Requirements

User requirements describe the specific functionalities and capabilities that users expect from the system. They focus on *what* the system must provide to fulfill user needs, improve user experience, and achieve business objectives as well as capture specific needs of end-users.

User requirements are directly related to user story.

User requirements are expected to be documented in the `specifications/UserRequirement.md` file.

Each user requirement is associated with a **user story**, and a single user story may contain multiple requirements. 
**user story** must at least have a relation to specific MOE, while relations for individual requirements within the user story are optional.

A user requirement in ReqFlow includes the following elements:
- **## USER STORY TITLE**
  - The name acts as a unique identifier, and ReqFlow tools ensure uniqueness is maintained.
  - Names must be unique within the document for user stories
- **User story Text**  
  - FILL IN 
- **User story Relations**  
  - Each user story must define at least one relation, and that one being **tracedFrom** linking to MOE element
- One or more requrements
- **### REQUIREMENT TITLE**
  - The name acts as a unique identifier, and ReqFlow tools ensure uniqueness is maintained.
  - Names must be unique within the document, as ReqFlow internally uses the file path and document name as part of its uniqueness mechanism.
- **Requirement Text**
  - The text describing the requirement comes directly below the name.
- **Relations**
  - Relations are optional for user requirements

Example format:
```markdown
## User story title

Relations:
 * tracedFrom: specifications/MOEs.md/elementName

As a user, I want to:
   * some user story
   * another related user story
   
### Requirement title

Requirement text.

### Requirement title

Requirement text.
```

### Mission and System Requirements

Mission and System Requirements are expected to be documented in the `specifications/MissionRequirement.md` file.

These requirements represent the high-level mission / enterprise  objectives, needs and measures of effectiveness, that a system must fulfill to align with the strategic goals of the organization and satisfy stakeholder expectations. 

**mission requirement** must at least have a relation to specific MOE.

A Mission or System Requirement includes the following elements:
- **### NAME**
  - The name acts as a unique identifier, and ReqFlow tools ensure uniqueness.
  - Names must be unique within the document, with ReqFlow leveraging the file path and document name as part of its uniqueness logic.
- **Requirement Text**
  - A concise description of the requirement, ideally written using structured methods such as **EARS (Easy Approach to Requirements Syntax)**.
- **Relations**
  - Each requirement must define at least one relation, such as:
    - **derivedFrom**: Linking to higher-level requirements or MOEs.
    - **satisfiedBy**: Linking to design specifications or test cases.

Example format:
```markdown

### Requirement name

A concise text describing the requirement, preferably using structured syntax like EARS.

Relations:
 * derivedFrom: path/documentName.md/ElementName
 * satisfiedBy: path/otherDocumentName.md/OtherElementName

```

# Requirements Diagrams

mermaid's requirementsDiagram which is bases on SysML is not that flexible in regards to picking colors and making links for docRefs thefor ReqFlow uses graphTD diagram.

## Requirements relations

In ReqFlow, requirements can be linked using the following types of relationships:
 * containedBy
 * contain
 * derivedFrom 
 * derive
 * refine
 * satisfiedBy
 * satisfy
 * verifiedBy
 * verify 
 * tracedFrom
 * trace 
  

### `contains` and `derives`

Both `contains` and `derives` relationships in SysML allow breaking down requirements, and their implementation may often look similar. However, the difference lies in the **modeling intent**:

- **`contains`**: Groups **independent, standalone sub-requirements** under a broader parent requirement.
- **`derives`**: Decomposes a **high-level requirement** into more specific, actionable requirements to show how the overall goal is achieved.

The choice depends on how you want to structure and trace your requirements:
- Use `contains` for **organizational grouping** eg. per feature sub-requirements.
- Use `derives` for **breaking down abstract requirements into detailed ones**.

While both relationships allow sub-requirements to be implemented independently, the clarity and traceability differ based on the intent of their relationship.

---

#### Practical Comparison

| Aspect                  | **`contains`**                           | **`derives`**                           |
|--------------------------|------------------------------------------|------------------------------------------|
| **Purpose**             | Groups **independent sub-requirements** under a parent. | Decomposes a general requirement into **specific actionable requirements**. |
| **Relationship**         | Parent is **loosely coupled** to sub-requirements. | Derived requirements are **traceable back** to the high-level requirement. |
| **Dependency**           | No logical or contextual dependency implied. | Derived requirements **logically flow** from the parent. |
| **Use Case**             | Organizing independent tasks or features. | Breaking down a general goal into specific system requirements. |
| **Example**              | User management grouped under a parent. | SaaS subscription management decomposed into feature requirements. |

---

#### Examples

**Contains**:
  * Parent requirement: **"The system shall support multiple user roles."**
  * Sub-requirements (distinct, independent features):
     *  **"The system shall support administrator roles."**
     *  **"The system shall support editor roles."**
     *  **"The system shall support viewer roles."**

**Why `contains` fits**:
- Each sub-requirement is a **separate feature** within the broader scope of user roles.
- Sub-requirements don’t explain **how roles are managed**; they are distinct components grouped under a common parent.
- Implementation of each sub-requirement is **independent** of the others but collectively satisfies the parent.

```mermaid
requirementDiagram

    %% Parent Requirement
    requirement REQ_PARENT {
        id: 1
        text: "The system shall support multiple user roles."
    }

    %% Sub-requirements
    requirement REQ_ADMIN {
        id: 1.1
        text: "The system shall support administrator roles."
    }

    requirement REQ_EDITOR {
        id: 1.2
        text: "The system shall support editor roles."
    }

    requirement REQ_VIEWER {
        id: 1.3
        text: "The system shall support viewer roles."
    }

    %% Relationship
    REQ_PARENT - contains -> REQ_ADMIN
    REQ_PARENT - contains -> REQ_EDITOR
    REQ_PARENT - contains -> REQ_VIEWER
```

**Derives**:
  * Parent requirement: **"The system shall support SaaS subscription management."**
  * Derived requirements (specific, actionable details):
     *  **"The system shall enable subscription plan selection."**
     *  **"The system shall process subscription payments securely."**
     *  **"The system shall send subscription renewal reminders."**

**Why `derives` fits**:
- These derived requirements **decompose the parent requirement** into specific, actionable tasks.
- Each derived requirement explains **how the parent goal is achieved**.
- Together, they trace back to the broader objective of subscription management.

```mermaid
requirementDiagram

    %% Parent Requirement
    requirement REQ_PARENT {
        id: 2
        text: "The system shall support SaaS subscription management."
    }

    %% Derived Requirements
    requirement REQ_PLAN {
        id: 2.1
        text: "The system shall enable subscription plan selection."
    }

    requirement REQ_PAYMENT {
        id: 2.2
        text: "The system shall process subscription payments securely."
    }

    requirement REQ_REMINDERS {
        id: 2.3
        text: "The system shall send subscription renewal reminders."
    }

    %% Relationship
    REQ_PARENT - derives -> REQ_PLAN
    REQ_PARENT - derives -> REQ_PAYMENT
    REQ_PARENT - derives -> REQ_REMINDERS
```

---


#### `copies`
- **Definition**: Creates a duplicate of an existing requirement, retaining the same text as the master requirement. The copied requirement references the original for traceability but may be applied in a different context.
- **When to Use**: Use `copies` when a requirement needs to be reused in another context, ensuring consistency in description while maintaining traceability to the original.
- **Example**: 
  - Original Requirement: "The system shall support user authentication."
  - Copied Requirement: "The system shall support user authentication." (applied to the mobile subsystem).

```mermaid
requirementDiagram
    %% Master Requirement
    requirement REQ_MASTER {
        id: 1
        text: "The system shall support user authentication."
    }

    %% Copied Requirement
    requirement REQ_COPIED {
        id: 1.1
        text: "The system shall support user authentication."
    }

    %% Relationship
    REQ_COPIED - copies -> REQ_MASTER
```
---


#### `refines`

- **Definition**: A `refines` relationship is a dependency that describes how a model element (e.g., use case, activity diagram, or text) provides further detail or context to a requirement. It can also describe how a text-based requirement refines a model element, elaborating on its purpose or functionality.

- **Purpose**: Use `refines` to illustrate how a requirement or model element is **further detailed or elaborated** to make its meaning clearer. This is particularly useful for connecting requirements to modeling elements (like use cases, activity diagrams, or blocks) that provide implementation or operational context.

- **When to Use**: Use `refines` when:
  1. A requirement needs to be connected to a more detailed model element (e.g., activity diagrams, use cases, or state machines).
  2. A text-based requirement refines a higher-level model element to provide more descriptive detail.

#### Example

### Updated Example with User Story Refining a Requirement

**Scenario**: A SaaS platform has a **user story** that refines a **functional requirement** for login, and this refinement adds more context and details.

- **Functional Requirement**: "The system shall support federated login using Google OAuth."
- **User Story**: "As a user, I want to log in to the system using my Google account so that I can access my personal dashboard."


```mermaid
requirementDiagram

    %% Functional Requirement
    requirement REQ_FEDERATED_LOGIN {
        id: 1
        text: "The system shall support federated login using Google OAuth."
    }

    %% Refining User Story
    element USER_STORY_FEDERATED_LOGIN {
        type: userstory
        docRef: "https://github.com/GrapheneDB/red/blob/reqman/README.md"
    }  



    %% Relationships
    USER_STORY_FEDERATED_LOGIN - refines -> REQ_FEDERATED_LOGIN

```
---


#### `satisfies`
- **Definition**: Links a system element (e.g., block, behavior) to the requirement it fulfills.
- **When to Use**: Use `satisfies` to trace the implementation of requirements to specific system elements.
- **Example**: 
  - Requirement: "The system shall support user authentication."
  - System Element: `AuthenticationSubsystem`.

```mermaid
requirementDiagram
    requirement REQ_AUTH {
        id: 3
        text: "The system shall support user authentication."
    }
    element AuthenticationSubsystem {
        type: "Block"
    }
    AuthenticationSubsystem - satisfies -> REQ_AUTH

```
---


### `verifies`
- **Definition**: Links a test case to a requirement to indicate how the requirement is validated.
- **When to Use**: Use `verifies` when you need to demonstrate or test that a requirement is met.
- **Example**: A test case "Verify system can encrypt files with AES-256" verifies "The system shall encrypt all uploaded files."

---

### `traces`
- **Definition**: Establishes a generic relationship between requirements or between a requirement and a system element, without specifying a strict semantic meaning.
- **When to Use**: Use `traces` when a relationship exists but doesn’t fit into the specific categories like `satisfies`, `verifies`, or `refines`.
- **Example**: "The system shall notify users of critical events" traces to the behavior "CriticalNotificationBehavior."

---

### Summary Table of Relationships

| Relationship Type | Definition                                                                 | Typical Use Case                                               |
|--------------------|---------------------------------------------------------------------------|----------------------------------------------------------------|
| **`contains`**     | Groups independent, parallel sub-requirements under a broader requirement. | Organizational grouping of standalone features or tasks.       |
| **`copies`**       | Creates a duplicate of an existing requirement for reuse while maintaining traceability to the original. | Reusing and adapting requirements in different contexts.       |
| **`derives`**      | Breaks down a high-level requirement into more specific ones.             | Decomposing general goals into specific system requirements.   |
| **`satisfies`**    | Links a system element (block, behavior) to the requirement it fulfills.  | Tracing system design elements to requirements.                |
| **`verifies`**     | Links a test case to a requirement for validation.                        | Testing or demonstrating that a requirement is met.            |
| **`refines`**      | Describes how a model element or set of elements further elaborates a requirement. | Detailing a requirement through use cases, activity diagrams, or textual elaboration. |
| **`traces`**       | Establishes a general dependency or relationship.                        | Linking requirements to elements without strict semantics.      |

---

### Conclusion

SysML requirement relationships provide powerful ways to structure, trace, and validate system requirements. Choosing the correct relationship depends on the intent and context:

- Use **`contains`** for grouping independent tasks.
- Use **`copies`** for reusing requirements in different contexts while maintaining traceability.
- Use **`derives`** for breaking down high-level requirements into specific actionable components.
- Use **`satisfies`** to ensure system components fulfill requirements.
- Use **`verifies`** to connect test cases for validation of requirements.
- Use **`refines`** for providing additional detail or elaboration to a requirement.
- Use **`traces`** for general-purpose, flexible dependency mapping.

Each relationship ensures traceability and alignment between requirements and system design, facilitating clear communication and robust development processes.





