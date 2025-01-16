# Physical Architecture for ReqFlow

The Physical Architecture represents the concrete systems, services, and components that implement the functionality of ReqFlow. It defines the deployment-level structure of the tool, detailing how various components interact.


## Architecture Diagram


```mermaid
classDiagram
    class ReqFlow {
        <<system>>
    }

    class ReqFlowTool {
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
    ReqFlow --> ReqFlowTool
    ReqFlow --> Integrations
    

    ReqFlowTool --> UserInterface
    ReqFlowTool --> Storage

    UserInterface--> CLI
    UserInterface--> ChatOps    

    ReqFlowTool --> ModelManagement
    ReqFlowTool --> ValidationAndReporting

    Integrations --> GitHubIntegration
    Integrations --> CICDIntegration

    CICDIntegration --> AIWorkflows

 
```

Logical to physical architecture mapping:
```mermaid
graph TD
    %% Root System
    ReqFlow["ReqFlow (system)"]

    %% Subsystems under ReqFlow
    subgraph ReqFlowTool["ReqFlowTool (subsystem)"]
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
    ReqFlow --> ReqFlowTool
    ReqFlow --> Integrations
    ReqFlow --> AIWorkflows

    ReqFlowTool --> UserInterface
    ReqFlowTool --> ModelManagement
    ReqFlowTool --> ValidationAndReporting
    ReqFlowTool --> Storage

    UserInterface --> CLI["CLI (component)"]
    UserInterface --> ChatOps["ChatOps (component)"]

    Integrations --> GitHubIntegration
    Integrations --> CICDIntegration

    CICDIntegration --> AIWorkflowsComponent

    AIWorkflowsComponent --> AI

    SystemOfInterest --> Storage

```
