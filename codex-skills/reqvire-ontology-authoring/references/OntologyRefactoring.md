# Ontology Refactoring and Improvement

Use this reference only when improving existing Reqvire ontology files. Do not load it for greenfield ontology creation unless the user explicitly asks to refactor, audit, clean up, or improve existing ontology content.

## Refactoring Goal

Refactoring should make the ontology more coherent without changing valid system meaning accidentally.

Preserve:

- stable IRIs and model identifiers unless the user explicitly approves a rename
- capability attachments and requirement reachability
- SHACL contracts that validate existing requirements
- controlled-vocabulary tokens used by Markdown metadata, CLI output, API payloads, reports, or validation
- source traceability, verification links, and existing evidence

Improve:

- class hierarchy correctness
- property domain/range precision
- OWL/SHACL separation
- datatype ranges and XSD usage
- controlled-vocabulary modeling
- ontology dependency hierarchy
- visualization usefulness in HTML ontology exploration

## Refactoring Workflow

1. Inventory the existing ontology plane before editing.
   - Run `reqvire search --filter-type=ontology --short`.
   - Read the affected `requirements/Ontologies/*.md` files and nearby semantic-contract shape profiles.
   - Check which capabilities attach the ontology and which requirements inherit it.
2. Identify the refactoring driver.
   - unclear class hierarchy
   - duplicate or synonym classes
   - missing domain/range
   - validation facets mixed into ontology
   - SHACL shapes referencing undeclared or unreachable terms
   - controlled-vocabulary tokens modeled as generic labels
   - graph visualization noise or missing ontology constructs
3. Make the smallest semantic correction that resolves the driver.
4. Update SHACL shape profiles when property names, domains, ranges, or target classes change.
5. Update requirements/specifications/verifications when the refactor changes the model contract.
6. Validate before broadening the refactor.

## Audit Checks

### OWL / SHACL Split

Ontology blocks should define OWL/RDFS terms and stable semantics:

- classes and subclass hierarchy
- object properties and datatype properties
- `rdfs:domain` and `rdfs:range` when stable
- XSD ranges for datatype properties
- true OWL axioms such as inverse properties, disjointness, equivalence, restrictions, or property chains
- typed controlled-vocabulary individuals

SHACL shape blocks should define closed-world validation:

- `sh:targetClass`
- `sh:path`
- `sh:datatype`
- `sh:class`
- `sh:minCount` / `sh:maxCount`
- `sh:pattern`
- `sh:in`
- `sh:minInclusive` / `sh:maxInclusive`
- `sh:message`

Do not move operational validation facets into ontology blocks. Do not redeclare OWL classes or properties inside SHACL blocks just to make a shape parse.

### Datatype Properties

Every `owl:DatatypeProperty` should have:

- a stable `rdfs:domain`
- an XSD `rdfs:range`, such as `xsd:string`, `xsd:boolean`, `xsd:integer`, `xsd:decimal`, `xsd:float`, `xsd:double`, `xsd:dateTime`, `xsd:date`, or `xsd:anyURI`
- an `rdfs:comment` when the meaning is not obvious

If the literal is a canonical token used by Reqvire behavior, keep it as a domain property. Do not replace it with `rdfs:label`.

### Object Properties

Every `owl:ObjectProperty` should have:

- a stable `rdfs:domain`
- a stable `rdfs:range`
- an `owl:inverseOf` when a real inverse relation exists and both directions are modeled
- property characteristics only when they are true for the domain, not just useful for display

Avoid overly broad domains/ranges such as a root class unless every instance in that class can genuinely participate.

### Class Hierarchy

Check that `rdfs:subClassOf` means "is a kind of":

- every instance of the subclass must also be an instance of the superclass
- sibling classes should be at the same generality level
- singular/plural variants should not both exist as separate classes
- synonyms should be comments, labels, or aliases, not duplicate classes
- avoid cycles unless deliberate equivalence is intended

Use multiple inheritance only when every instance truly belongs to every parent class.

### Controlled Vocabulary

Typed individuals are appropriate for stable enum-like records, report kinds, governance values, relation rules, issue kinds, and symbol records. During refactoring, prefer explicit `owl:NamedIndividual` plus the domain class for these records.

Formal meaning comes from:

- IRI identity
- explicit `owl:NamedIndividual` typing
- `rdf:type`
- class hierarchy
- axioms
- domain properties validated by SHACL

Use `rdfs:label` and `rdfs:comment` for optional presentation metadata. Keep custom `*Name`, `*Meaning`, or similar properties only when the value is a canonical token, payload field, validation target, report value, or queryable model contract. If SHACL validates a presentation-only custom name/meaning field, refactor the SHACL `sh:path` to `rdfs:label` or `rdfs:comment` instead of keeping the custom property.

Refactor this:

```turtle
ex:collectReportKind a ex:ReportKind ;
  ex:reportKindName "collect" ;
  rdfs:comment "Report kind that gathers element context." .
```

Into this:

```turtle
ex:collectReportKind a owl:NamedIndividual, ex:ReportKind ;
  ex:reportKindName "collect" ;
  rdfs:comment "Report kind that gathers element context." .
```

Do not replace `ex:reportKindName "collect"` with only `rdfs:label "collect"` when `collect` is a CLI/report/API/validation/query token.

### Ontology Dependencies

If an ontology element uses terms declared by another ontology element, its `derivedFrom` relation should point to the nearest ontology dependency, not only to the root ontology.

Examples:

- an ontology using `RequirementOwnedRefinement` should derive from the requirement ontology
- an ontology using relation-rule vocabulary should derive from the relation ontology
- a projection/export ontology can derive from the report or core ontology that defines the reused terms

Do not use capability attachments as a substitute for ontology hierarchy.

### SHACL Reachability

When refactoring ontology terms:

- update `sh:targetClass`, `sh:path`, and `sh:class` references
- keep referenced terms reachable from the semantic-contract owner through capability ontology context
- keep built-in annotation paths such as `rdfs:label` and `rdfs:comment` only when annotation validation is the intended contract

## Common Refactors

### Move Validation Out of OWL

If an ontology block contains operational validation language or SHACL-like constraints, move that behavior to a requirement-owned `semantic-contract`.

Keep in OWL:

```turtle
ex:hasAge a owl:DatatypeProperty ;
  rdfs:domain ex:Employee ;
  rdfs:range xsd:integer .
```

Move to SHACL:

```turtle
ex:EmployeeShape a sh:NodeShape ;
  sh:targetClass ex:Employee ;
  sh:property [
    sh:path ex:hasAge ;
    sh:datatype xsd:integer ;
    sh:minInclusive 18 ;
    sh:maxInclusive 65 ;
  ] .
```

### Align Ownership Classes

If prose says a class is owned by a specific model element type, the OWL hierarchy should reflect that when the owner class exists.

For example, requirement-owned semantic refinements should subclass `reqvire:RequirementOwnedRefinement`, not only generic `reqvire:Refinement`.

### Preserve Tokens

Do not replace domain token properties with `rdfs:label` during cleanup.

Keep:

```turtle
reqvire:mediumRisk a reqvire:RiskValue ;
  reqvire:governanceValueName "medium" ;
  rdfs:comment "Requirement realization has manageable uncertainty." .
```

Use labels/comments only if the literal is purely presentation metadata.

## Verification

After refactoring, run focused checks before broad validation:

```bash
cargo run -q -p cli -- validate
cargo run -q -p cli -- ontologies
```

If shapes or output fixtures changed, also run the affected test slice, usually:

```bash
./tests/run_tests.sh test-ontologies-command
./tests/run_tests.sh test-semantic-contract-sanity
```
