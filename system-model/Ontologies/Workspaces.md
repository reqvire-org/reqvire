# Elements

### Reqvire Workspace Ontology

Defines the shared structural vocabulary for workspace compositions, repository revision selections, checkout locations and branch sets, together with model-session observations over those compositions.

#### Details
Competency questions:
- Which repository and branch own the authoritative workspace manifest?
- Which configured parent directory and component paths locate dependent checkouts?
- Which exact repository revisions and named branches belong to one composition?
- Which tracking refs supply explicit updates independently of working branches and frozen pins?
- Which legacy manifest was converted into a native workspace manifest?
- Which root checkpoint records the pins captured from committed component work?
- Which repository-specific PRs review one coordinated change?
- Which worktree and accepted model snapshot belong to a context?
- Which commit is accepted and which is currently observed?
- Which input observations protect model content or source resolution?

This shared structural vocabulary is independent of manifest serialization, command names, API fields and transaction algorithms. Requirement-owned contracts define those obligations. Runtime context/provenance instances are not implied to be exported.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix concept: <https://www.reqvire.org/concepts#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:RepositoryRevisionSelection a owl:Class ;
  rdfs:comment "A repository identity, selected commit and workspace-relative mount path belonging to one model snapshot." .

reqvire:selectionRepository a owl:ObjectProperty ;
  rdfs:domain reqvire:RepositoryRevisionSelection ; rdfs:range reqvire:GitRepository .

reqvire:selectionCommit a owl:ObjectProperty ;
  rdfs:domain reqvire:RepositoryRevisionSelection ; rdfs:range reqvire:GitCommit .

reqvire:selectionWorkspacePath a owl:DatatypeProperty ;
  rdfs:domain reqvire:RepositoryRevisionSelection ; rdfs:range xsd:string .

reqvire:GitCommit a owl:Class ;
  rdfs:comment "A committed revision owned by one Git repository." .

reqvire:commitObjectId a owl:DatatypeProperty ;
  rdfs:domain reqvire:GitCommit ;
  rdfs:range xsd:string .

reqvire:GitRepository a owl:Class ;
  rdfs:comment "Git history and object ownership for a source repository." .

reqvire:WorkspaceComposition a owl:Class ;
  reqvire:mapsToConcept concept:WorkspaceComposition ;
  rdfs:comment "An owning repository revision and its declared dependent repository revision selections." .

reqvire:WorkspaceManifest a owl:Class ;
  reqvire:mapsToConcept concept:WorkspaceManifest ;
  rdfs:comment "A versioned repository inventory, checkout layout and exact commit selection owned by one repository branch." .

reqvire:WorkspaceManifestSource a owl:Class ;
  rdfs:subClassOf reqvire:File ;
  rdfs:comment "A tracked authoritative manifest source in the manifest-owning root repository." .

reqvire:manifestSource a owl:ObjectProperty ;
  rdfs:domain reqvire:WorkspaceManifest ; rdfs:range reqvire:WorkspaceManifestSource .

reqvire:RevisionLockedManifest a owl:Class ;
  rdfs:subClassOf reqvire:WorkspaceManifest ;
  reqvire:mapsToConcept concept:RevisionLockedManifest ;
  rdfs:comment "Manifest whose selected component revisions are exact commit identifiers." .

reqvire:ManifestProject a owl:Class ;
  reqvire:mapsToConcept concept:ManifestProject ;
  rdfs:comment "A dependent repository and checkout-path declaration in a workspace manifest." .

reqvire:workspaceOwningWorktree a owl:ObjectProperty ;
  rdfs:domain reqvire:WorkspaceComposition ; rdfs:range reqvire:GitWorktree .

reqvire:workspaceManifest a owl:ObjectProperty ;
  rdfs:domain reqvire:WorkspaceComposition ; rdfs:range reqvire:WorkspaceManifest .

reqvire:manifestDeclaresProject a owl:ObjectProperty ;
  rdfs:domain reqvire:WorkspaceManifest ; rdfs:range reqvire:ManifestProject .

reqvire:projectRepository a owl:ObjectProperty ;
  rdfs:domain reqvire:ManifestProject ; rdfs:range reqvire:GitRepository .

reqvire:projectCheckout a owl:ObjectProperty ;
  rdfs:domain reqvire:ManifestProject ; rdfs:range reqvire:GitWorktree .

reqvire:projectResolvedCommit a owl:ObjectProperty ;
  rdfs:domain reqvire:ManifestProject ; rdfs:range reqvire:GitCommit .

reqvire:fileOwnedByRepository a owl:ObjectProperty ;
  rdfs:domain reqvire:File ; rdfs:range reqvire:GitRepository .

reqvire:projectPath a owl:DatatypeProperty ;
  rdfs:domain reqvire:ManifestProject ; rdfs:range xsd:string .

reqvire:projectRevision a owl:DatatypeProperty ;
  rdfs:domain reqvire:ManifestProject ; rdfs:range xsd:string .

reqvire:WorkspaceBranchComposition a owl:Class ;
  rdfs:comment "A logical authoring branch composed of repository-specific branch and revision selections." .

reqvire:GitBranch a owl:Class ;
  rdfs:comment "A named branch owned by one Git repository; equal names in different repositories identify distinct branches." .

reqvire:RepositoryBranchSelection a owl:Class ;
  rdfs:subClassOf reqvire:RepositoryRevisionSelection ;
  rdfs:comment "A repository revision selection associated with a named authoring branch." .

reqvire:CompositionCheckpoint a owl:Class ;
  rdfs:comment "A root Git revision containing the authoritative source manifest, optional root model content and exact pins selecting component commits." .

reqvire:ManifestRevisionSelection a owl:Class ;
  rdfs:subClassOf reqvire:RepositoryRevisionSelection ;
  rdfs:comment "An authoritative manifest source selected from the owning root repository branch revision, independently of live or derived working copies." .

reqvire:CoordinatedPullRequestSet a owl:Class ;
  rdfs:comment "Repository-specific pull requests reviewing one composed model change." .

reqvire:RepositoryPullRequest a owl:Class .

reqvire:ReviewBaseline a owl:Class ;
  rdfs:comment "The root comparison revision selected for a review, independently of the branch creation base." .

reqvire:compositionBranchSelection a owl:ObjectProperty ;
  rdfs:domain reqvire:WorkspaceBranchComposition ; rdfs:range reqvire:RepositoryBranchSelection .

reqvire:selectionBranch a owl:ObjectProperty ;
  rdfs:domain reqvire:RepositoryBranchSelection ; rdfs:range reqvire:GitBranch .

reqvire:branchRepository a owl:ObjectProperty ;
  rdfs:domain reqvire:GitBranch ; rdfs:range reqvire:GitRepository .

reqvire:branchName a owl:DatatypeProperty ;
  rdfs:domain reqvire:GitBranch ; rdfs:range xsd:string .

reqvire:compositionCheckpoint a owl:ObjectProperty ;
  rdfs:domain reqvire:WorkspaceBranchComposition ; rdfs:range reqvire:CompositionCheckpoint .

reqvire:checkpointRootCommit a owl:ObjectProperty ;
  rdfs:domain reqvire:CompositionCheckpoint ; rdfs:range reqvire:GitCommit .

reqvire:checkpointManifestSelection a owl:ObjectProperty ;
  rdfs:domain reqvire:CompositionCheckpoint ; rdfs:range reqvire:ManifestRevisionSelection .

reqvire:pullRequestSetMember a owl:ObjectProperty ;
  rdfs:domain reqvire:CoordinatedPullRequestSet ; rdfs:range reqvire:RepositoryPullRequest .

reqvire:pullRequestRepository a owl:ObjectProperty ;
  rdfs:domain reqvire:RepositoryPullRequest ; rdfs:range reqvire:GitRepository .

reqvire:reviewBaselineCommit a owl:ObjectProperty ;
  rdfs:domain reqvire:ReviewBaseline ; rdfs:range reqvire:GitCommit .

reqvire:compositionReviewBaseline a owl:ObjectProperty ;
  rdfs:domain reqvire:WorkspaceBranchComposition ; rdfs:range reqvire:ReviewBaseline .

reqvire:RepositoriesDirectory a owl:Class ;
  reqvire:mapsToConcept concept:RepositoriesDirectory ;
  rdfs:comment "A configured parent location for the dependent checkouts of a workspace manifest." .
reqvire:manifestRepositoriesDirectory a owl:ObjectProperty ;
  rdfs:domain reqvire:WorkspaceManifest ; rdfs:range reqvire:RepositoriesDirectory .
reqvire:repositoriesDirectoryPath a owl:DatatypeProperty ;
  rdfs:domain reqvire:RepositoriesDirectory ; rdfs:range xsd:string .
reqvire:WorkspaceFreeze a owl:Class ;
  reqvire:mapsToConcept concept:WorkspaceFreeze ;
  rdfs:comment "A recording of committed dependent repository revisions as composition pins." .
reqvire:freezeComposition a owl:ObjectProperty ;
  rdfs:domain reqvire:WorkspaceFreeze ; rdfs:range reqvire:WorkspaceComposition .
reqvire:freezeRepositorySelection a owl:ObjectProperty ;
  rdfs:domain reqvire:WorkspaceFreeze ; rdfs:range reqvire:RepositoryRevisionSelection .

reqvire:WorkspaceModelSnapshot a owl:Class ;
  rdfs:comment "Complete model inputs and repository revision selections for one workspace composition, independently of a live checkout." .

reqvire:snapshotRepositorySelection a owl:ObjectProperty ;
  rdfs:domain reqvire:WorkspaceModelSnapshot ; rdfs:range reqvire:RepositoryRevisionSelection .

reqvire:snapshotManifestInput a owl:ObjectProperty ;
  rdfs:domain reqvire:WorkspaceModelSnapshot ; rdfs:range reqvire:ModelInputDependency .

reqvire:WorktreeContext a owl:Class ;
  reqvire:mapsToConcept concept:MCPWorktreeContext ;
  rdfs:comment "An independently addressed accepted-model session rooted in one owning Git worktree; a root worktree's manifest can select component sources within that same context." .

reqvire:AcceptedModelSnapshot a owl:Class ;
  rdfs:comment "A complete validated model retained as authoritative by a context." .

reqvire:ModelInputDependency a owl:Class ;
  rdfs:comment "A content, existence or source-selection observation on which an accepted model depends." .

reqvire:ProtectedModelInput a owl:Class ;
  rdfs:subClassOf reqvire:ModelInputDependency ;
  reqvire:mapsToConcept concept:ProtectedModelInput ;
  rdfs:comment "A currently eligible model source, or a semantic source or resolution observation with a currently eligible declaring owner, whose change requires full validated admission." .

reqvire:ReferenceTargetDependency a owl:Class ;
  rdfs:subClassOf reqvire:ModelInputDependency ;
  rdfs:comment "A declared local reference's dependency on target existence and eligibility, independent of the target's content role." .

reqvire:SourceResolutionDependency a owl:Class ;
  rdfs:subClassOf reqvire:ProtectedModelInput ;
  rdfs:comment "An observation of a candidate path, including absence, that determines which local semantic source is selected." .

reqvire:HeadRefresh a owl:Class ;
  reqvire:mapsToConcept concept:HEADRefresh ;
  rdfs:comment "A guarded reload of an observed committed branch tip through normal model loading." .

reqvire:contextWorktree a owl:ObjectProperty ;
  rdfs:domain reqvire:WorktreeContext ;
  rdfs:range reqvire:GitWorktree .

reqvire:acceptedHeadCommit a owl:ObjectProperty ;
  rdfs:domain reqvire:WorktreeContext ;
  rdfs:range reqvire:GitCommit ;
  reqvire:mapsToConcept concept:AcceptedHEAD .

reqvire:observedHeadCommit a owl:ObjectProperty ;
  rdfs:domain reqvire:WorktreeContext ;
  rdfs:range reqvire:GitCommit ;
  reqvire:mapsToConcept concept:ObservedHEAD .

reqvire:acceptedModelSnapshot a owl:ObjectProperty ;
  rdfs:domain reqvire:WorktreeContext ;
  rdfs:range reqvire:AcceptedModelSnapshot .

reqvire:modelInputDependency a owl:ObjectProperty ;
  rdfs:domain reqvire:AcceptedModelSnapshot ;
  rdfs:range reqvire:ModelInputDependency .

reqvire:refreshContext a owl:ObjectProperty ;
  rdfs:domain reqvire:HeadRefresh ;
  rdfs:range reqvire:WorktreeContext .

reqvire:dependencyPath a owl:DatatypeProperty ;
  rdfs:domain reqvire:ModelInputDependency ;
  rdfs:range xsd:string .

reqvire:contextWorkspaceComposition a owl:ObjectProperty ;
  rdfs:domain reqvire:WorktreeContext ; rdfs:range reqvire:WorkspaceComposition .

reqvire:contextBranchComposition a owl:ObjectProperty ;
  rdfs:domain reqvire:WorktreeContext ; rdfs:range reqvire:WorkspaceBranchComposition .
reqvire:projectTrackingRef a owl:DatatypeProperty ;
  rdfs:domain reqvire:ManifestProject ; rdfs:range xsd:string ;
  reqvire:mapsToConcept concept:TrackingRef ;
  rdfs:comment "The configured revision source for explicit updates, independently of a working branch and exact commit pin." .
reqvire:WorkspaceSynchronization a owl:Class ;
  rdfs:comment "Preparation of a workspace composition from recorded commits or explicitly refreshed tracking references." .
reqvire:synchronizationComposition a owl:ObjectProperty ;
  rdfs:domain reqvire:WorkspaceSynchronization ; rdfs:range reqvire:WorkspaceComposition .
reqvire:synchronizationRepositorySelection a owl:ObjectProperty ;
  rdfs:domain reqvire:WorkspaceSynchronization ; rdfs:range reqvire:RepositoryRevisionSelection .
reqvire:WorkspaceMigration a owl:Class ;
  reqvire:mapsToConcept concept:WorkspaceMigration ;
  rdfs:comment "An explicit conversion from an existing composition description to a native workspace manifest." .
reqvire:migrationSourceManifestPath a owl:DatatypeProperty ;
  rdfs:domain reqvire:WorkspaceMigration ; rdfs:range xsd:string .
reqvire:migrationTargetManifest a owl:ObjectProperty ;
  rdfs:domain reqvire:WorkspaceMigration ; rdfs:range reqvire:WorkspaceManifestSource .

```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---
