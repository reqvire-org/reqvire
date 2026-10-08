# Elements

### Implementation Coverage Rollup Shape

Defines SHACL constraints for implementation coverage assessment records and rollup rule vocabulary.

The shape constrains computed assessment records, not the completeness of authored requirements. An uncovered requirement is a valid assessment. Requirement dependency completeness and recursive evaluation follow the implementation coverage logic specification; these shapes do not execute the evaluation or materialize its records.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:ImplementationCoverageShape a sh:NodeShape ;
  sh:targetClass reqvire:RequirementImplementationCoverage, reqvire:CapabilityImplementationCoverage ;
  sh:property [ sh:path reqvire:implementationCovered ; sh:minCount 1 ; sh:maxCount 1 ; sh:datatype xsd:boolean ] ;
  sh:property [ sh:path reqvire:supportingImplementationArtifact ; sh:class reqvire:Artifact ] ;
  sh:property [ sh:path reqvire:requiredImplementationCoverage ; sh:class reqvire:ImplementationCoverage ] ;
  sh:property [ sh:path reqvire:implementationBlockedByRequirement ; sh:class reqvire:Requirement ] ;
  sh:or (
    [ sh:property [ sh:path reqvire:implementationCovered ; sh:hasValue false ] ]
    [
      sh:property [ sh:path reqvire:implementationCovered ; sh:hasValue true ] ;
      sh:property [ sh:path reqvire:supportingImplementationArtifact ; sh:minCount 1 ] ;
      sh:property [ sh:path reqvire:implementationBlockedByRequirement ; sh:maxCount 0 ] ;
      sh:property [
        sh:path reqvire:requiredImplementationCoverage ;
        sh:node [ sh:property [ sh:path reqvire:implementationCovered ; sh:minCount 1 ; sh:maxCount 1 ; sh:hasValue true ] ] ;
      ] ;
    ]
  ) .

reqvire:RequirementImplementationCoverageShape a sh:NodeShape ;
  sh:targetClass reqvire:RequirementImplementationCoverage ;
  sh:property [ sh:path reqvire:implementationRequirement ; sh:minCount 1 ; sh:maxCount 1 ; sh:class reqvire:Requirement ] ;
  sh:property [ sh:path reqvire:implementationCapability ; sh:maxCount 0 ] ;
  sh:property [ sh:path reqvire:assessedAsTerminal ; sh:minCount 1 ; sh:maxCount 1 ; sh:datatype xsd:boolean ] ;
  sh:property [ sh:path reqvire:directImplementationArtifact ; sh:class reqvire:Artifact ] ;
  sh:property [ sh:path reqvire:requiredImplementationCoverage ; sh:class reqvire:RequirementImplementationCoverage ] ;
  sh:or (
    [
      sh:property [ sh:path reqvire:assessedAsTerminal ; sh:hasValue true ] ;
      sh:property [ sh:path reqvire:requiredImplementationCoverage ; sh:maxCount 0 ] ;
      sh:or (
        [
          sh:property [ sh:path reqvire:implementationCovered ; sh:hasValue true ] ;
          sh:property [ sh:path reqvire:directImplementationArtifact ; sh:minCount 1 ] ;
        ]
        [
          sh:property [ sh:path reqvire:implementationCovered ; sh:hasValue false ] ;
          sh:property [ sh:path reqvire:directImplementationArtifact ; sh:maxCount 0 ] ;
        ]
      ) ;
    ]
    [
      sh:property [ sh:path reqvire:assessedAsTerminal ; sh:hasValue false ] ;
      sh:property [ sh:path reqvire:requiredImplementationCoverage ; sh:minCount 1 ] ;
      sh:or (
        [ sh:property [ sh:path reqvire:implementationCovered ; sh:hasValue true ] ]
        [
          sh:property [ sh:path reqvire:implementationCovered ; sh:hasValue false ] ;
          sh:property [
            sh:path reqvire:requiredImplementationCoverage ;
            sh:qualifiedValueShape [ sh:property [ sh:path reqvire:implementationCovered ; sh:hasValue false ] ] ;
            sh:qualifiedMinCount 1 ;
          ] ;
        ]
      ) ;
    ]
  ) .

reqvire:CapabilityImplementationCoverageShape a sh:NodeShape ;
  sh:targetClass reqvire:CapabilityImplementationCoverage ;
  sh:property [ sh:path reqvire:implementationCapability ; sh:minCount 1 ; sh:maxCount 1 ; sh:class reqvire:Capability ] ;
  sh:property [ sh:path reqvire:implementationRequirement ; sh:maxCount 0 ] ;
  sh:property [ sh:path reqvire:assessedAsTerminal ; sh:maxCount 0 ] ;
  sh:property [ sh:path reqvire:directImplementationArtifact ; sh:maxCount 0 ] .

reqvire:ImplementationRollupRuleShape a sh:NodeShape ;
  sh:targetClass reqvire:ImplementationRollupRule ;
  sh:property [ sh:path rdfs:comment ; sh:minCount 1 ; sh:datatype xsd:string ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Implementation Coverage Evaluation](../Implementation/Traceability/ImplementationTraceabilityRequirements.md#implementation-coverage-evaluation)
  * use: [Reqvire Implementation Rollup Ontology](#reqvire-implementation-rollup-ontology)
---

### Reqvire Implementation Rollup Ontology

The implementation rollup ontology defines implementation coverage assessments, terminal requirements, supporting artifacts, and required contributions from requirement and capability hierarchies and shared-contract consumers.

The vocabulary distinguishes which subject is assessed, whether it is covered, which other assessments it depends on, which artifacts support it, and which requirements remain uncovered. Assessments describe a complete validated model snapshot; they are computed records, not authored satisfaction relations or additional model element types.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix concept: <https://www.reqvire.org/concepts#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:ImplementationCoverage a owl:Class ;
  reqvire:mapsToConcept concept:ImplementationCoverage ;
  rdfs:comment "Computed implementation coverage assessment of a requirement or capability." .
reqvire:RequirementImplementationCoverage a owl:Class ;
  rdfs:subClassOf reqvire:ImplementationCoverage ;
  owl:disjointWith reqvire:CapabilityImplementationCoverage ;
  rdfs:comment "Implementation assessment of one requirement, including its direct evidence and required contributions." .
reqvire:CapabilityImplementationCoverage a owl:Class ;
  rdfs:subClassOf reqvire:ImplementationCoverage ;
  rdfs:comment "Implementation assessment rolled up from specifying requirements and child capabilities; no direct capability satisfaction." .
reqvire:TerminalRequirement a owl:Class ;
  rdfs:subClassOf reqvire:Requirement ;
  reqvire:mapsToConcept concept:TerminalRequirement ;
  rdfs:comment "Requirement with no immediate requirement children through derive and no requirements binding its owned contracts. Terminal status is determined from the complete validated graph, not inferred from missing OWL facts." .
reqvire:ImplementationRollupRule a owl:Class ;
  rdfs:comment "Stable semantic rule governing implementation coverage assessment." .

reqvire:implementationRequirement a owl:ObjectProperty ;
  rdfs:domain reqvire:RequirementImplementationCoverage ;
  rdfs:range reqvire:Requirement ;
  rdfs:comment "Requirement assessed by this coverage record." .
reqvire:implementationCapability a owl:ObjectProperty ;
  rdfs:domain reqvire:CapabilityImplementationCoverage ;
  rdfs:range reqvire:Capability ;
  rdfs:comment "Capability assessed by this coverage record." .
reqvire:implementationCovered a owl:DatatypeProperty ;
  rdfs:domain reqvire:ImplementationCoverage ;
  rdfs:range xsd:boolean ;
  rdfs:comment "Whether the subject's implementation obligations are covered under the implementation rollup rules." .
reqvire:assessedAsTerminal a owl:DatatypeProperty ;
  rdfs:domain reqvire:RequirementImplementationCoverage ;
  rdfs:range xsd:boolean ;
  rdfs:comment "Whether the assessed requirement is terminal in the complete model snapshot." .
reqvire:directImplementationArtifact a owl:ObjectProperty ;
  rdfs:domain reqvire:RequirementImplementationCoverage ;
  rdfs:range reqvire:Artifact ;
  rdfs:comment "Artifact directly linked from the assessed requirement by satisfiedBy; its presence alone does not establish nonterminal coverage." .
reqvire:supportingImplementationArtifact a owl:ObjectProperty ;
  rdfs:domain reqvire:ImplementationCoverage ;
  rdfs:range reqvire:Artifact ;
  rdfs:comment "Direct or recursively traced implementation artifact retained as supporting evidence, including for incomplete assessments." .
reqvire:requiredImplementationCoverage a owl:ObjectProperty ;
  rdfs:domain reqvire:ImplementationCoverage ;
  rdfs:range reqvire:ImplementationCoverage ;
  rdfs:comment "Immediate assessment contribution required for coverage: requirement children and consumers of owned contracts, or specifying requirements and child capabilities." .
reqvire:implementationBlockedByRequirement a owl:ObjectProperty ;
  rdfs:domain reqvire:ImplementationCoverage ;
  rdfs:range reqvire:Requirement ;
  rdfs:comment "Uncovered requirement explaining an outstanding implementation obligation." .

reqvire:terminalRequirementImplementationRule a owl:NamedIndividual, reqvire:ImplementationRollupRule ;
  rdfs:comment "A terminal requirement is implementation-covered if and only if it has at least one direct satisfiedBy artifact. Owning an unused contract does not make a requirement nonterminal." .
reqvire:childRequirementImplementationRule a owl:NamedIndividual, reqvire:ImplementationRollupRule ;
  rdfs:comment "Every immediate requirement child through derive is a required contribution, evaluated recursively. One implemented descendant is insufficient." .
reqvire:contractConsumerImplementationRule a owl:NamedIndividual, reqvire:ImplementationRollupRule ;
  rdfs:comment "Every distinct requirement binding any contract owned by the assessed requirement is a required contribution, evaluated recursively. A binding on a parent is fulfilled through that parent's complete implementation rollup; inherited bindings do not create duplicate contributions." .
reqvire:combinedImplementationRollupRule a owl:NamedIndividual, reqvire:ImplementationRollupRule ;
  rdfs:comment "A nonterminal requirement is implementation-covered if and only if all requirement children and all required contract consumers are covered. These contributions collectively fulfill the requirement without requiring direct owner evidence." .
reqvire:directParentImplementationEvidenceRule a owl:NamedIndividual, reqvire:ImplementationRollupRule ;
  rdfs:comment "Direct evidence of a nonterminal requirement remains visible but cannot override an uncovered child or contract consumer." .
reqvire:capabilityImplementationRollupRule a owl:NamedIndividual, reqvire:ImplementationRollupRule ;
  rdfs:comment "Capability implementation coverage rolls up from specifying requirements and child capabilities. Capabilities have no direct satisfaction; an empty capability supplies no implementation evidence." .
reqvire:implementationCoverageUnitRule a owl:NamedIndividual, reqvire:ImplementationRollupRule ;
  rdfs:comment "Implementation percentages count distinct terminal requirements among the selected subjects. Specifications and other owned contracts supply neither coverage units nor implementation evidence; their bindings identify required consumers. External contributors retain their coverage and evidence without entering selected subject counts." .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Relation Ontology](RelationsAndImpact.md#reqvire-relation-ontology)
---
