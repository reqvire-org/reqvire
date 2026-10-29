# Elements

### Reqvire Worktree Session Ontology

Defines the structural vocabulary for independently addressed worktree contexts, manifest-selected component repositories, accepted and observed commit provenance, accepted model snapshots, and the input dependencies governing a HEAD refresh.

#### Details
Competency questions:
- Which worktree and accepted model snapshot belong to a context?
- Which commit is accepted, and which commit is currently observed?
- Which input observations protect model content or source resolution?
- Which referenced artifacts have existence dependencies without frozen content?
- Which context is the subject of a HEAD refresh?
- Which Central worktree and manifest define a Repo workspace?
- Which project paths and resolved commits supply that workspace's component sources?
- Which Git repository owns each changed file independently of the Central branch selector?

Command names, eligibility algorithms, blocker codes and publication transitions belong to requirement-owned contracts. The vocabulary describes those domain distinctions without imposing an operational state machine or claiming runtime session instances are already exported.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix concept: <https://www.reqvire.org/concepts#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:WorkspaceModelSnapshot a owl:Class ;
  rdfs:comment "Complete model inputs and repository revision selections for one workspace composition, independently of a live checkout." .
reqvire:RepositoryRevisionSelection a owl:Class ;
  rdfs:comment "A repository identity, selected commit and workspace-relative mount path belonging to one model snapshot." .
reqvire:snapshotRepositorySelection a owl:ObjectProperty ;
  rdfs:domain reqvire:WorkspaceModelSnapshot ; rdfs:range reqvire:RepositoryRevisionSelection .
reqvire:selectionRepository a owl:ObjectProperty ;
  rdfs:domain reqvire:RepositoryRevisionSelection ; rdfs:range reqvire:GitRepository .
reqvire:selectionCommit a owl:ObjectProperty ;
  rdfs:domain reqvire:RepositoryRevisionSelection ; rdfs:range reqvire:GitCommit .
reqvire:selectionWorkspacePath a owl:DatatypeProperty ;
  rdfs:domain reqvire:RepositoryRevisionSelection ; rdfs:range xsd:string .
reqvire:snapshotManifestInput a owl:ObjectProperty ;
  rdfs:domain reqvire:WorkspaceModelSnapshot ; rdfs:range reqvire:ModelInputDependency .

reqvire:WorktreeContext a owl:Class ;
  reqvire:mapsToConcept concept:MCPWorktreeContext ;
  rdfs:comment "An independently addressed accepted-model session rooted in one owning Git worktree; a Central worktree's manifest can select component sources within that same context." .
reqvire:GitCommit a owl:Class ;
  rdfs:comment "A Git commit that can be accepted or observed by a worktree context." .
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
reqvire:commitObjectId a owl:DatatypeProperty ;
  rdfs:domain reqvire:GitCommit ;
  rdfs:range xsd:string .
reqvire:GitRepository a owl:Class ;
  rdfs:comment "Git history and object ownership for a source repository." .
reqvire:RepoWorkspace a owl:Class ;
  reqvire:mapsToConcept concept:RepoWorkspace ;
  rdfs:comment "Workspace whose Central worktree and versioned Repo manifest select the component model view." .
reqvire:RepoManifest a owl:Class ;
  reqvire:mapsToConcept concept:RepoManifest ;
  rdfs:comment "Versioned project inventory and revision selection belonging to a Central worktree." .
reqvire:RevisionLockedManifest a owl:Class ;
  rdfs:subClassOf reqvire:RepoManifest ;
  reqvire:mapsToConcept concept:RevisionLockedManifest ;
  rdfs:comment "Manifest whose selected component revisions are exact commit identifiers." .
reqvire:ManifestProject a owl:Class ;
  reqvire:mapsToConcept concept:ManifestProject ;
  rdfs:comment "Repository and checkout-path declaration in an effective Repo manifest." .
reqvire:contextRepoWorkspace a owl:ObjectProperty ;
  rdfs:domain reqvire:WorktreeContext ; rdfs:range reqvire:RepoWorkspace .
reqvire:centralWorktree a owl:ObjectProperty ;
  rdfs:domain reqvire:RepoWorkspace ; rdfs:range reqvire:GitWorktree .
reqvire:workspaceManifest a owl:ObjectProperty ;
  rdfs:domain reqvire:RepoWorkspace ; rdfs:range reqvire:RepoManifest .
reqvire:manifestDeclaresProject a owl:ObjectProperty ;
  rdfs:domain reqvire:RepoManifest ; rdfs:range reqvire:ManifestProject .
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
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---
