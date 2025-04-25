# Physical Architecture for Reqvire

The Physical Architecture represents the concrete systems, services, and components that implement the functionality of Reqvire. It defines the deployment-level structure of the tool, detailing how various components interact.


## Architecture Diagram


```mermaid
classDiagram
    class Reqvire {
        <<system>>
    }

    class ReqvireTool {
        <<subsystem>>
    }

    class UserInterface {
        <<component>>
    }

    class CLI {
        <<component>>
    }
    class ChatOps {
        <<component>>
    }    

    class ModelManagement {
        <<component>>
    }

    class ValidationAndReporting {
        <<component>>
    }

    class Integrations {
        <<subsystem>>
    }

    class GitHubIntegration {
        <<component>>
    }

    class CICDIntegration {
        <<component>>
    }

    class AIWorkflows {
        <<workflow>>
    }

    class Storage {
        <<component>>
    }

 
    %% Hierarchical Relationships
    Reqvire --> ReqvireTool
    Reqvire --> Integrations
    

    ReqvireTool --> UserInterface
    ReqvireTool --> Storage

    UserInterface--> CLI
    UserInterface--> ChatOps    

    ReqvireTool --> ModelManagement
    ReqvireTool --> ValidationAndReporting

    Integrations --> GitHubIntegration
    Integrations --> CICDIntegration

    CICDIntegration --> AIWorkflows

 
```

Logical to physical architecture mapping:
```mermaid
graph TD
    %% Root System
    Reqvire["Reqvire (system)"]

    %% Subsystems under Reqvire
    subgraph ReqvireTool["ReqvireTool (subsystem)"]
        UserInterface["UserInterface (component)"]
        ModelManagement["ModelManagement (component)"]
        ValidationAndReporting["ValidationAndReporting (component)"]
        Storage["Storage (component)"]
    end

    subgraph Integrations["Integrations (subsystem)"]
        GitHubIntegration["GitHubIntegration (component)"]
        CICDIntegration["CICDIntegration (component)"]
    end

    subgraph AIWorkflows["AIWorkflows (workflow)"]
        AIWorkflowsComponent["AIWorkflows (workflow)"]
    end

    %% AI component (added based on the logical architecture)
    AI["AI (component)"]

    %% Systems
    SystemOfInterest["SystemOfInterest (system)"]

    %% Hierarchical Structure
    Reqvire --> ReqvireTool
    Reqvire --> Integrations
    Reqvire --> AIWorkflows

    ReqvireTool --> UserInterface
    ReqvireTool --> ModelManagement
    ReqvireTool --> ValidationAndReporting
    ReqvireTool --> Storage

    UserInterface --> CLI["CLI (component)"]
    UserInterface --> ChatOps["ChatOps (component)"]

    Integrations --> GitHubIntegration
    Integrations --> CICDIntegration

    CICDIntegration --> AIWorkflowsComponent

    AIWorkflowsComponent --> AI

    SystemOfInterest --> Storage

```
