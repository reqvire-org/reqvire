# Architecture

This document defines the Reqvire architecture using block diagrams that show the logical and physical structure of the system.

## Logical Architecture
### Logical Architecture Block

The Logical Architecture for Reqvire defines the high-level functional organization of the tool, focusing on the main components that deliver its core functionalities. This architecture serves as the foundation for further refinement into physical architecture and system requirements.

```mermaid
classDiagram
    class UserInteraction {
        +CLIInterface
        +ChatOpsInterface
    }
    class CLI {
        +InteractWithModelManagement()
    }
    class ModelManagement {
        +ManageModel()
        +GenerateDiagrams()
        +AnalyzeRelations()
        +ValidateStructure()
        +TraceChanges()
        +GenerateTraceabilityMatrix()
    }
    class AI {
        +ProvideAISuggestions()
        +DevelopSystemOfInterest()
        +ApplyAISuggestions()
        +CallModelManagementFunctions()
    }
    class AIWorkflow {
        +DriveAIAgentsDevelopment()
        +ProvidesCollaborationFeedback()
        +TrackSystemOfInterestProgress()
    }
    class ValidationAndReporting {
        +ValidateMarkdownStructure()
        +ValidateFilesystemStructure()
        +ValidateModelConsistency()
        +FixModelIssues()
        +GenerateReports()
    }
    class Storage {
        +GitRepository
        +ModelStorage
    }
    class Integrations {
        +CollaboratesWithGitHub()
        +CollaboratesWithCICD()
    }

    class GitHubIntegration {
    }

    class CICDIntegration {
        +TriggerBuilds()
        +RunTests()
        +RunActions()
        +TriggerValidations()
    }

    class SystemOfInterest {
        +MBSEModel
        +DevelopedSystem
    }

    %% Relationships
    UserInteraction --> CLI : "Interacts via CLI"
    UserInteraction --> AI : "Interacts via ChatOps"
    CLI --> ModelManagement
    AI --> ModelManagement : "Calls Functions for Model Management"
    CLI --> ValidationAndReporting : "Triggers validation/fixing/reporting"
    CLI --> Storage : "Reads/Writes Model Data"
    AI --> ValidationAndReporting : "Provides AI Validation Suggestions"
    AI --> Storage : "Saves Approved Changes"
    ValidationAndReporting --> Storage : "Accesses Model Data"
    Integrations --> ValidationAndReporting : "Triggers Validations"
    Integrations --> GitHubIntegration : "Manages GitHub-related tasks"
    Integrations --> CICDIntegration : "Manages CI/CD workflows"
    GitHubIntegration --> Storage : "Syncs changes with Git Repository"
    GitHubIntegration --> AIWorkflow : "Triggers AI-driven commits/changes"
    GitHubIntegration --> ModelManagement : "Facilitates version control of model"
    CICDIntegration --> GitHubIntegration : "Triggers actions based on PR/Merge status"
    CICDIntegration --> ValidationAndReporting : "Performs automated validations during builds"
    CICDIntegration --> AIWorkflow : "Enables AI-driven tests and deployment"

    Storage --> SystemOfInterest : "Stores MBSE Model and Developed System"

    %% New AIWorkflow Component Relationships
    AIWorkflow --> Integrations : "Collaborates with CI/CD and Github"
    AIWorkflow --> SystemOfInterest : "Guides System Development"
    AIWorkflow --> AI : "Drives AI Agent Actions"
    AIWorkflow --> ModelManagement : "Interacts with Model Management Functions"
```

#### Metadata
  * type: block

#### Relations
  * trace: [Managing MBSE Models](UserStories.md#managing-mbse-models)
  * trace: [AI-Assisted MBSE Model Management](UserStories.md#ai-assisted-mbse-model-management)
  * trace: [CLI interface](UserRequirements.md#cli-interface)
  * trace: [Browse Model via Web Interface](UserRequirements.md#browse-model-via-web-interface)
  * trace: [Integrate with GitHub Workflows](UserStories.md#integrate-with-github-workflows)
  * trace: [Model Reports](UserRequirements.md#model-reports)
  * trace: [Validating Structures](UserStories.md#validating-structures)

---

## Physical Architecture
### Physical Architecture Block

The Physical Architecture represents the concrete systems, services, and components that implement the functionality of Reqvire. It defines the deployment-level structure of the tool, detailing how various components interact and are organized in the actual system.

```mermaid
graph TD
    subgraph Reqvire["Reqvire System"]
        subgraph ReqvireTool["ReqvireTool Subsystem"]
            subgraph UserInterface["UserInterface"]
                CLI["CLI"]
                ChatOps["ChatOps"]
            end
            ModelManagement["ModelManagement"]
            ValidationAndReporting["ValidationAndReporting"]
            Storage["Storage"]
        end

        subgraph Integrations["Integrations Subsystem"]
            GitHubIntegration["GitHubIntegration"]
            subgraph CICDIntegration["CICDIntegration"]
                AIWorkflows["AIWorkflows"]
            end
        end
    end
```

**Logical to Physical Architecture Mapping:**

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

#### Metadata
  * type: block

#### Relations
  * trace: [Managing MBSE Models](UserStories.md#managing-mbse-models)
  * trace: [AI-Assisted MBSE Model Management](UserStories.md#ai-assisted-mbse-model-management)
  * trace: [CLI interface](UserRequirements.md#cli-interface)
  * trace: [Browse Model via Web Interface](UserRequirements.md#browse-model-via-web-interface)
  * trace: [Integrate with GitHub Workflows](UserStories.md#integrate-with-github-workflows)
  * trace: [Model Reports](UserRequirements.md#model-reports)
  * trace: [Validating Structures](UserStories.md#validating-structures)

---