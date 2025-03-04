# Requirements in ReqFlow

## What Are Requirements?

In ReqFlow, a requirement represents a stakeholder's need, system capability, or constraint that the system must fulfill. 
Requirements define **what the system must do** (functional requirements) and **how well it must perform** (non-functional requirements), serving as the foundation for system design, development, and verification.

ReqFlow mandates a specific format for organizing and presenting requirements to ensure consistency across the project. 
However, it does not impose a specific syntax for expressing the content of requirements, offering flexibility to tailor expressions based on the project's needs and the team's preferences. 
Structured syntaxes like **EARS (Easy Approach to Requirements Syntax)** are encouraged to improve clarity and consistency, but their use is not required.


### Requirements document format
 
Requirements are organizied within requirements documents: a markdow document with specific format.


```
# Document title

## Grouping Title: User Interface Requirements

### Requirement Title: Login Functionality

The system shall provide a login interface that allows users to authenticate using a username and password.

Relations:
 * tracedFrom: specifications/UserStories.md/authentication
 * verifiedBy: specifications/tests.md/testLogin

## Other group

### Requirement 4

### Requirement 5

### Requirement 6

```


#### Document title

The **Document Title** provides a descriptive name for the entire requirements document. 
It identifies the scope or subject of the requirements contained within and is placed at the very beginning of the file.

Expected Markdown:

```
# <Descriptive Title for the Document>
```

#### Grouping Title

Each group of related requirements is introduced by a **Grouping Title**. This serves as a high-level categorization or organizational header for the set of requirements.

Expected Markdown:

```
## <Descriptive Title for the Group>

```


### Requirements format


Example:
```
### Login Functionality

The system shall provide a login interface that allows users to authenticate using a username and password.

Relations:
 * tracedFrom: specifications/UserStories.md/Authentication


### Password Recovery

The system shall provide a mechanism for users to recover forgotten passwords.

#### Relations
 * tracedFrom: specifications/UserStories.md/Security
 * verifiedBy: specifications/tests.md/testPasswordRecovery
  
#### Metadata
 * Priority: High
 * Criticality: Moderate

#### Some other additional context

```

#### Requirement Name

Each individual requirement is identified with a Requirement Name. This provides a brief and descriptive name for the requirement and must be unique within a document itself.

```
### <Requirement name>

```

#### Requirement Text

The Requirement Text describes the specific functionality, constraint, or need in detail. This is the main body of the requirement.

#### Relations

The **"#### Relations** section documents links between this requirement and other system elements,requirements, specifications, or validation methods.

Relations used for the requirements are a subset of relations that exist in the **Reqflow**:
 * containedBy
 * derivedFrom 
 * refine
 * satisfiedBy 
 * verifiedBy
 * tracedFrom

Relations foher the requirements are mandatory: at least 1 relation to parent object must exist.
The parent objects depends on type of requirement and is explained further down.


#### Metadata

The **#### Metadata** section is optional and provides additional key-value pair details about the requirement. These details help define attributes or characteristics of the requirement.

The keys and values are expressed as bullet points. Accepted key-value pairs include:

- **Priority**: High | Medium | Low
- **Criticality**: High | Moderate | Low


Expected markdow:
```
#### Metadata
  * Key: value
```

#### Additional context section

The Additional Context Section is optional and allows for any supplementary information that might be relevant or useful for understanding the requirement. It provides flexibility to include detailed explanations, clarifications, or related information.

Content in this section should follow valid Markdown syntax and cannot include headers with a level less than ####.
Expected markdow:
```
#### <A section title>

Markdown valid text which cannot have headers less than ####

```

## Structure of Requirements in ReqFlow

The diagram below demonstrates how requirements, their relationships, and hierarchical structures are organized within the **ReqFlow methodology**. 
It showcases the connection between stakeholder needs, user requirements, mission requirements, system requirements, and their links to test cases, specification documents, and other system elements.

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

Requirements in ReqFlow are divided into three main categories:
 * User Requirements
 * Mission Requirements
 * System Requirements

  
### User Requirements

User requirements describe the specific functionalities and capabilities that users expect from the system. They focus on *what* the system must provide to fulfill user needs, improve user experience, and achieve business objectives as well as capture specific needs of end-users.

User requirements are directly related to user story: they **refine** user story.

User requirements are expected to be documented in the `specifications/UserRequirement.md` file.


### Mission Requirements

Mission and System Requirements are expected to be documented in the `specifications/MissionRequirement.md` file.

These requirements represent the high-level mission / enterprise  objectives, needs and measures of effectiveness, that a system must fulfill to align with the strategic goals of the organization and satisfy stakeholder expectations. 

**mission requirement** must at least have a relation to specific MOE.

User requirements are expected to be documented in the `specifications/MissionRequirement.md` file.

### System  Requirements

System requirements define the detailed technical and functional specifications that the system must meet to fulfill the user and mission requirements. They describe *how* the system will achieve the objectives set by the user and mission requirements.

System requirements are structured to map to specific subsystems or components of the overall system. Each subsystem or component has its own dedicated folder in the `specifications/systemRequirements` directory, ensuring modularity and clarity.

#### Organization and File Structure

1. **Location**: System requirements must be documented in the `specifications/systemRequirements` directory.
2. **Subfolders**: Each subsystem has its own subfolder. Subfolders can nest further into sub-subsystems as needed.
3. **File Name**: Each folder must contain exactly one `Requirements.md` file, where the requirements for that specific subsystem are documented.

#### Traceability

System requirements must trace back to their respective user and mission requirements to ensure alignment with user needs and mission objectives. Traceability is documented through relations like `tracedFrom` and `verifiedBy`.

#### Expected File Structure for System Requirements

- **Top-Level File**: 
  - `specifications/systemRequirements/Requirements.md` – Contains high-level system requirements that apply to the overall system.
- **Subsystem Folders**:
  - `specifications/systemRequirements/<subsystem>/Requirements.md`
  - Subsystems can have nested folders, as shown:
    ```plaintext
    specifications/systemRequirements/subsystem/subsubsystem/Requirements.md
    specifications/systemRequirements/othersubsystem/subsystem/Requirements.md
    ```

Each `Requirements.md` file contains requirements specific to the subsystem or component it corresponds to.



## Requirements Diagrams

mermaid's requirementsDiagram which is based on SysML is not that flexible in regards to picking colors and making links for docRefs thefor ReqFlow uses graphTD diagram for the time being.




