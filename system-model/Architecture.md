# Architecture

## Physical Architecture

### Physical Architecture Block

The Physical Architecture represents the concrete systems, services, and components that implement the functionality of Reqvire. It defines the deployment-level structure of the tool, detailing how various components interact and are organized in the actual system.

```mermaid
graph TD
    subgraph Reqvire["Reqvire System"]
        subgraph ReqvireTool["ReqvireTool Subsystem"]
            subgraph UserInterface["UserInterface"]
                CLI["CLI"]
                WebInterface["WebInterface"]
                MCPServer["MCPServer"]
            end
            SharedToolContracts["SharedToolContracts"]
            MutationSafety["MutationSafety"]
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
        SharedToolContracts["SharedToolContracts (component)"]
        MutationSafety["MutationSafety (component)"]
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
    UserInterface --> WebInterface["WebInterface (component)"]
    UserInterface --> MCPServer["MCPServer (component)"]
    MCPServer --> SharedToolContracts
    SharedToolContracts --> MutationSafety
    SharedToolContracts --> ModelManagement
    SharedToolContracts --> ValidationAndReporting

    Integrations --> GitHubIntegration
    Integrations --> CICDIntegration

    CICDIntegration --> AIWorkflowsComponent

    AIWorkflowsComponent --> AI

    SystemOfInterest --> Storage
```

#### Metadata
  * type: block
---

## Logical Architecture

### Logical Architecture Block

The Logical Architecture for Reqvire defines the high-level functional organization of the tool, focusing on the main components that deliver its core functionalities. This architecture serves as the foundation for further contract into physical architecture and implementation-facing requirements.

```mermaid
classDiagram
    class UserInteraction {
        +CLIInterface
        +WebInterface
        +MCPInterface
    }
    class CLI {
        +InteractWithModelManagement()
    }
    class WebInterface {
        +BrowseGeneratedDocumentation()
        +NavigateModelReports()
    }
    class MCPInterface {
        +ExposeTypedToolContracts()
        +ExposeReadOnlyResources()
        +GateMutationRequests()
    }
    class SharedToolContracts {
        +TypedRequests()
        +TypedResults()
        +StructuredErrors()
        +EvidenceMetadata()
    }
    class MutationSafety {
        +RequireDryRun()
        +FlushFilesystemWrites()
        +RefreshInternalGraph()
        +ReturnDiffs()
        +SyncGraphFromCoreMutation()
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
        +CallMCPTools()
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
    UserInteraction --> WebInterface : "Browses Explorer"
    UserInteraction --> AI : "Reviews AI suggestions"
    CLI --> ModelManagement
    CLI --> ValidationAndReporting : "Triggers validation/fixing/reporting"
    CLI --> Storage : "Reads/Writes Model Data"
    WebInterface --> ValidationAndReporting : "Displays report projections"
    WebInterface --> Storage : "Reads Project Store data"
    MCPInterface --> SharedToolContracts : "Adapts MCP protocol"
    SharedToolContracts --> ModelManagement : "Calls core operations"
    SharedToolContracts --> ValidationAndReporting : "Calls report operations"
    SharedToolContracts --> MutationSafety : "Applies write policy"
    MutationSafety --> ModelManagement : "Executes approved mutations"
    AI --> MCPInterface : "Calls typed MCP tools"
    AI --> ValidationAndReporting : "Provides AI Validation Suggestions"
    AI --> Storage : "Saves approved code changes"
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
    AIWorkflow --> MCPInterface : "Uses typed model tools"
```

#### Metadata
  * type: block
---

## Implementation Architecture

# Elements

### CRUD Operations Delegation Pattern

The system shall implement all CRUD operations using a delegation pattern where the CRUD layer orchestrates user requests and delegates validation and execution to the graph_registry layer.

#### Details
All element manipulation operations follow this architectural pattern:
- CRUD layer (crud.rs) provides public API and orchestration logic
- Graph registry layer (graph_registry.rs) performs validation and executes changes
- CRUD delegates to graph_registry for all model mutations

**Operation Delegation Mapping:**
```
crud.add_element() → graph_registry.add_element_to_file()
crud.remove_element() → graph_registry.remove_element_with_cleanup()
crud.move_element() → graph_registry.move_element_comprehensive()
crud.merge_elements() → graph_registry.merge_elements()
crud.rename_element() → graph_registry.rename_element()
crud.link() → graph_registry.add_element_relation_full()
crud.unlink() → graph_registry.remove_element_relation_full()
```

**Benefits:**
- Clear separation of concerns between orchestration and execution
- Centralized validation logic in graph_registry
- Consistent error handling across all operations
- Maintainable and testable code structure

#### Metadata
  * type: requirement

#### Relations
  * satisfiedBy: [crud.rs](core/src/crud.rs)
  * satisfiedBy: [graph_registry.rs](core/src/graph_registry.rs)
---

### Shared Utility Functions

The system shall extract common code patterns into shared utility functions to reduce duplication and maintain consistency across modules.

#### Details
When a code pattern appears in multiple locations, it should be extracted into a shared utility function. This follows the DRY (Don't Repeat Yourself) principle and improves maintainability.

**Example: Parent Directory Extraction**
The `get_parent_dir()` utility function provides consistent parent directory extraction logic used across crud.rs and graph_registry.rs, replacing 6 instances of duplicate code.

```rust
pub fn get_parent_dir(file_path: &str) -> PathBuf {
    PathBuf::from(file_path).parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_default()
}
```

**Benefits:**
- Eliminates code duplication
- Single source of truth for common operations
- Easier to test and maintain
- Consistent behavior across modules

#### Metadata
  * type: requirement

#### Relations
  * satisfiedBy: [utils.rs](core/src/utils.rs)
  * satisfiedBy: [crud.rs](core/src/crud.rs)
  * satisfiedBy: [graph_registry.rs](core/src/graph_registry.rs)
---
