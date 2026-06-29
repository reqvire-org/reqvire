#!/bin/bash
set -uo pipefail

set +e
TTL_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" ontologies 2>&1)
TTL_EXIT=$?
set -e

if [ $TTL_EXIT -ne 0 ]; then
  echo "FAILED: ontologies command failed"
  echo "$TTL_OUTPUT"
  exit 1
fi

set +e
SEMANTIC_ONTOLOGIES_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" semantic export --layer ontologies 2>&1)
SEMANTIC_ONTOLOGIES_EXIT=$?
SEMANTIC_SHAPES_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" semantic export --layer shapes 2>&1)
SEMANTIC_SHAPES_EXIT=$?
SEMANTIC_CONCEPTS_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" semantic export --layer concepts 2>&1)
SEMANTIC_CONCEPTS_EXIT=$?
SEMANTIC_MODEL_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" semantic export --layer model 2>&1)
SEMANTIC_MODEL_EXIT=$?
SEMANTIC_PREFIXES_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" semantic export --layer prefixes 2>&1)
SEMANTIC_PREFIXES_EXIT=$?
CONCEPTS_EXPORT_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" concepts export --include-mappings 2>&1)
CONCEPTS_EXPORT_EXIT=$?
CONCEPTS_VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" concepts validate 2>&1)
CONCEPTS_VALIDATE_EXIT=$?
SEMANTIC_GRAPH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" semantic export 2>&1)
SEMANTIC_GRAPH_EXIT=$?
NATIVE_CONCEPT_SEARCH_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-type concept --json 2>&1)
NATIVE_CONCEPT_SEARCH_EXIT=$?
NATIVE_SCHEME_SEARCH_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-type concept-scheme --json 2>&1)
NATIVE_SCHEME_SEARCH_EXIT=$?
NATIVE_CONCEPT_MODEL_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model --filter-type concept-scheme 2>&1)
NATIVE_CONCEPT_MODEL_EXIT=$?
NATIVE_CONCEPT_COLLECT_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" collect "API Endpoint Requirement" --json 2>&1)
NATIVE_CONCEPT_COLLECT_EXIT=$?
set -e

if [ $SEMANTIC_ONTOLOGIES_EXIT -ne 0 ]; then
  echo "FAILED: semantic export --layer ontologies command failed"
  echo "$SEMANTIC_ONTOLOGIES_OUTPUT"
  exit 1
fi

if [ $SEMANTIC_SHAPES_EXIT -ne 0 ]; then
  echo "FAILED: semantic export --layer shapes command failed"
  echo "$SEMANTIC_SHAPES_OUTPUT"
  exit 1
fi

if [ $SEMANTIC_CONCEPTS_EXIT -ne 0 ]; then
  echo "FAILED: semantic export --layer concepts command failed"
  echo "$SEMANTIC_CONCEPTS_OUTPUT"
  exit 1
fi

if [ $SEMANTIC_MODEL_EXIT -ne 0 ]; then
  echo "FAILED: semantic export --layer model command failed"
  echo "$SEMANTIC_MODEL_OUTPUT"
  exit 1
fi

if [ $SEMANTIC_PREFIXES_EXIT -ne 0 ]; then
  echo "FAILED: semantic export --layer prefixes command failed"
  echo "$SEMANTIC_PREFIXES_OUTPUT"
  exit 1
fi

if [ $CONCEPTS_EXPORT_EXIT -ne 0 ]; then
  echo "FAILED: concepts export command failed"
  echo "$CONCEPTS_EXPORT_OUTPUT"
  exit 1
fi

if [ $CONCEPTS_VALIDATE_EXIT -ne 0 ]; then
  echo "FAILED: concepts validate command failed"
  echo "$CONCEPTS_VALIDATE_OUTPUT"
  exit 1
fi

if [ $SEMANTIC_GRAPH_EXIT -ne 0 ]; then
  echo "FAILED: default semantic export command failed"
  echo "$SEMANTIC_GRAPH_OUTPUT"
  exit 1
fi

if [ $NATIVE_CONCEPT_SEARCH_EXIT -ne 0 ]; then
  echo "FAILED: search --filter-type concept command failed"
  echo "$NATIVE_CONCEPT_SEARCH_JSON"
  exit 1
fi

if [ $NATIVE_SCHEME_SEARCH_EXIT -ne 0 ]; then
  echo "FAILED: search --filter-type concept-scheme command failed"
  echo "$NATIVE_SCHEME_SEARCH_JSON"
  exit 1
fi

if [ $NATIVE_CONCEPT_MODEL_EXIT -ne 0 ]; then
  echo "FAILED: model --filter-type concept-scheme command failed"
  echo "$NATIVE_CONCEPT_MODEL_JSON"
  exit 1
fi

if [ $NATIVE_CONCEPT_COLLECT_EXIT -ne 0 ]; then
  echo "FAILED: collect with native concept reference command failed"
  echo "$NATIVE_CONCEPT_COLLECT_JSON"
  exit 1
fi

if ! grep -q "testonto:ServiceEndpoint a owl:Class" <<< "$SEMANTIC_ONTOLOGIES_OUTPUT"; then
  echo "FAILED: semantic export --layer ontologies output missing ontology class"
  echo "$SEMANTIC_ONTOLOGIES_OUTPUT"
  exit 1
fi

if grep -q "testonto:ServiceEndpointShape" <<< "$SEMANTIC_ONTOLOGIES_OUTPUT"; then
  echo "FAILED: semantic export --layer ontologies output must not include semantic-contract SHACL shapes"
  echo "$SEMANTIC_ONTOLOGIES_OUTPUT"
  exit 1
fi

if ! grep -q "testonto:ServiceEndpointShape" <<< "$SEMANTIC_SHAPES_OUTPUT"; then
  echo "FAILED: semantic export --layer shapes output missing SHACL shape"
  echo "$SEMANTIC_SHAPES_OUTPUT"
  exit 1
fi

if grep -q "testonto:ServiceEndpoint a owl:Class" <<< "$SEMANTIC_SHAPES_OUTPUT"; then
  echo "FAILED: semantic export --layer shapes output must not include authored ontology classes"
  echo "$SEMANTIC_SHAPES_OUTPUT"
  exit 1
fi

if grep -q "concept:TraceabilityConcept" <<< "$SEMANTIC_CONCEPTS_OUTPUT"; then
  echo "FAILED: semantic export --layer concepts output must not include legacy Turtle-authored SKOS concepts"
  echo "$SEMANTIC_CONCEPTS_OUTPUT"
  exit 1
fi

if grep -q "reqvire:mapsToConcept" <<< "$SEMANTIC_CONCEPTS_OUTPUT"; then
  echo "FAILED: semantic export --layer concepts output must not include authored ontology concept bridge triples"
  echo "$SEMANTIC_CONCEPTS_OUTPUT"
  exit 1
fi

if ! grep -q "reqvire:mapsToConcept" <<< "$SEMANTIC_ONTOLOGIES_OUTPUT"; then
  echo "FAILED: semantic export --layer ontologies output missing authored concept bridge"
  echo "$SEMANTIC_ONTOLOGIES_OUTPUT"
  exit 1
fi

if ! grep -q "reqvire:mapsToConcept" <<< "$CONCEPTS_EXPORT_OUTPUT"; then
  echo "FAILED: concepts export --include-mappings output missing concept bridge"
  echo "$CONCEPTS_EXPORT_OUTPUT"
  exit 1
fi

if ! grep -q "concept:NativeTraceability a skos:Concept" <<< "$CONCEPTS_EXPORT_OUTPUT"; then
  echo "FAILED: concepts export output missing Markdown-generated native concept RDF"
  echo "$CONCEPTS_EXPORT_OUTPUT"
  exit 1
fi

for native_concept_token in \
  "concept:NativeConcepts" \
  "skos:ConceptScheme" \
  "Native concept scheme authored as Reqvire Markdown." \
  "skos:hasTopConcept" \
  "concept:EngineeringKnowledge" \
  "concept:NativeTraceability" \
  "concept:ServiceEndpoint" \
  "skos:Concept" \
  "skos:inScheme" \
  "\"Native Traceability\"" \
  "skos:altLabel" \
  "\"Trace link analysis\"" \
  "skos:broader" \
  "skos:related" \
  "concept:VerificationEvidence"; do
  if ! grep -qF "$native_concept_token" <<< "$SEMANTIC_CONCEPTS_OUTPUT"; then
    echo "FAILED: semantic export --layer concepts output missing native Markdown concept token: $native_concept_token"
    echo "$SEMANTIC_CONCEPTS_OUTPUT"
    exit 1
  fi
done

if grep -q "concept:NativeTraceability a skos:Concept" <<< "$SEMANTIC_ONTOLOGIES_OUTPUT"; then
  echo "FAILED: semantic export --layer ontologies output must not include Markdown-generated native concept RDF"
  echo "$SEMANTIC_ONTOLOGIES_OUTPUT"
  exit 1
fi

if ! grep -q "concept:NativeTraceability a skos:Concept" <<< "$SEMANTIC_GRAPH_OUTPUT"; then
  echo "FAILED: default semantic export output missing Markdown-generated native concept RDF"
  echo "$SEMANTIC_GRAPH_OUTPUT"
  exit 1
fi

if ! jq -e '
  any(.files[].elements[];
    .name == "Native Traceability"
    and .concept.pref_label == "Native Traceability"
    and (.concept.definition | contains("engineering intent"))
    and any(.relations[]; .relation_type == "broader" and (.target.target | endswith("SemanticContracts.md#engineering-knowledge")))
    and any(.relations[]; .relation_type == "related" and (.target.target | endswith("SemanticContracts.md#verification-evidence"))))
  and any(.files[].elements[]; .name == "Engineering Knowledge")
  and any(.files[].elements[]; .name == "Verification Evidence")
' >/dev/null 2>&1 <<< "$NATIVE_CONCEPT_SEARCH_JSON"; then
  echo "FAILED: search --filter-type concept JSON missing native concept payloads or concept relations"
  echo "$NATIVE_CONCEPT_SEARCH_JSON"
  exit 1
fi

if ! jq -e '
  any(.files[].elements[];
    .name == "Native Concepts"
    and .concept_scheme.pref_label == "Native Concepts"
    and (.concept_scheme.definition | contains("Native concept scheme"))
    and (.concept_scheme.top_concepts | length == 3))
' >/dev/null 2>&1 <<< "$NATIVE_SCHEME_SEARCH_JSON"; then
  echo "FAILED: search --filter-type concept-scheme JSON missing native concept scheme payload"
  echo "$NATIVE_SCHEME_SEARCH_JSON"
  exit 1
fi

if ! jq -e '
  any(.elements[];
    .name == "Native Concepts"
    and .element_type == "concept-scheme"
    and any(.relations[]; .relation_type == "derive" and .element.name == "Engineering Knowledge"))
' >/dev/null 2>&1 <<< "$NATIVE_CONCEPT_MODEL_JSON"; then
  echo "FAILED: model --filter-type concept-scheme JSON missing native concept root hierarchy"
  echo "$NATIVE_CONCEPT_MODEL_JSON"
  exit 1
fi

if ! jq -e '
  .metadata.concept_context_count >= 1
  and any(.items[]; .name == "Native Traceability" and .source_type == "concept_context")
  and any(.items[]; .name == "Engineering Knowledge" and .source_type == "concept_context")
' >/dev/null 2>&1 <<< "$NATIVE_CONCEPT_COLLECT_JSON"; then
  echo "FAILED: collect JSON missing Markdown-native concept context for concept references"
  echo "$NATIVE_CONCEPT_COLLECT_JSON"
  exit 1
fi

if ! grep -q "reqvire:OntologyProjectionGraph" <<< "$SEMANTIC_GRAPH_OUTPUT"; then
  echo "FAILED: semantic export output missing generated ontology projection facts"
  echo "$SEMANTIC_GRAPH_OUTPUT"
  exit 1
fi

if ! grep -q "urn:reqvire:element:api-capability" <<< "$SEMANTIC_MODEL_OUTPUT"; then
  echo "FAILED: semantic export --layer model output missing generated model element facts"
  echo "$SEMANTIC_MODEL_OUTPUT"
  exit 1
fi

if grep -q "reqvire:TurtlePrefixDeclaration" <<< "$SEMANTIC_MODEL_OUTPUT"; then
  echo "FAILED: semantic export --layer model output must not include prefix projection facts"
  echo "$SEMANTIC_MODEL_OUTPUT"
  exit 1
fi

if ! grep -q "reqvire:TurtlePrefixDeclaration" <<< "$SEMANTIC_PREFIXES_OUTPUT"; then
  echo "FAILED: semantic export --layer prefixes output missing prefix projection facts"
  echo "$SEMANTIC_PREFIXES_OUTPUT"
  exit 1
fi

count_occurrences() {
  { grep -oF "$1" <<< "$2" || true; } | wc -l | tr -d ' '
}

if ! grep -q "testonto:ServiceEndpoint a owl:Class" <<< "$TTL_OUTPUT"; then
  echo "FAILED: Turtle output missing ontology class"
  echo "$TTL_OUTPUT"
  exit 1
fi

if ! grep -q "<https://example.test/ontology> a owl:Ontology" <<< "$TTL_OUTPUT"; then
  echo "FAILED: Turtle output missing generated ontology document declaration"
  echo "$TTL_OUTPUT"
  exit 1
fi

if grep -q "<https://example.test/ontology/api-ontology> a owl:Ontology" <<< "$TTL_OUTPUT"; then
  echo "FAILED: Turtle output should declare the ontology document at ontology_base, not per element"
  echo "$TTL_OUTPUT"
  exit 1
fi

if ! grep -q "reqvire:termNamespace \"https://example.test/ontology#\"" <<< "$TTL_OUTPUT"; then
  echo "FAILED: Turtle output missing generated ontology term namespace"
  echo "$TTL_OUTPUT"
  exit 1
fi

if ! grep -q "reqvire:ontologyPrefix \"testonto\"" <<< "$TTL_OUTPUT"; then
  echo "FAILED: Turtle output missing generated ontology prefix"
  echo "$TTL_OUTPUT"
  exit 1
fi

if ! grep -q "testonto:ServiceEndpoint rdfs:isDefinedBy <https://example.test/ontology>" <<< "$TTL_OUTPUT"; then
  echo "FAILED: Turtle output missing generated rdfs:isDefinedBy edge from authored ontology term to ontology document"
  echo "$TTL_OUTPUT"
  exit 1
fi

set +e
FILTERED_CONCEPT_TTL_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" ontologies --namespace-base https://example.test/ontology/conceptual 2>&1)
FILTERED_CONCEPT_EXIT=$?
set -e

if [ $FILTERED_CONCEPT_EXIT -ne 0 ]; then
  echo "FAILED: ontologies --namespace-base command failed"
  echo "$FILTERED_CONCEPT_TTL_OUTPUT"
  exit 1
fi

if ! grep -q "<https://example.test/ontology/conceptual> a owl:Ontology" <<< "$FILTERED_CONCEPT_TTL_OUTPUT"; then
  echo "FAILED: namespace-filtered Turtle output missing selected ontology document declaration"
  echo "$FILTERED_CONCEPT_TTL_OUTPUT"
  exit 1
fi

if ! grep -q "conceptual:TraceabilityConstruct a owl:Class" <<< "$FILTERED_CONCEPT_TTL_OUTPUT"; then
  echo "FAILED: namespace-filtered Turtle output missing selected namespace term"
  echo "$FILTERED_CONCEPT_TTL_OUTPUT"
  exit 1
fi

if grep -q "testonto:ServiceEndpoint a owl:Class" <<< "$FILTERED_CONCEPT_TTL_OUTPUT"; then
  echo "FAILED: namespace-filtered Turtle output included non-selected ontology term"
  echo "$FILTERED_CONCEPT_TTL_OUTPUT"
  exit 1
fi

set +e
FILTERED_API_TTL_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" ontologies --namespace-base https://example.test/ontology# 2>&1)
FILTERED_API_EXIT=$?
set -e

if [ $FILTERED_API_EXIT -ne 0 ]; then
  echo "FAILED: ontologies --namespace-base term namespace command failed"
  echo "$FILTERED_API_TTL_OUTPUT"
  exit 1
fi

if ! grep -q "testonto:ServiceEndpoint a owl:Class" <<< "$FILTERED_API_TTL_OUTPUT"; then
  echo "FAILED: term-namespace-filtered Turtle output missing selected ontology term"
  echo "$FILTERED_API_TTL_OUTPUT"
  exit 1
fi

if grep -q "concept:TraceabilityConcept a skos:Concept" <<< "$FILTERED_API_TTL_OUTPUT"; then
  echo "FAILED: term-namespace-filtered Turtle output included non-selected ontology term"
  echo "$FILTERED_API_TTL_OUTPUT"
  exit 1
fi

set +e
FILTERED_FULL_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" ontologies --full --namespace-base https://example.test/ontology# 2>&1)
FILTERED_FULL_EXIT=$?
set -e

if [ $FILTERED_FULL_EXIT -eq 0 ]; then
  echo "FAILED: ontologies --full --namespace-base should be rejected until model-layer filtering is specified"
  echo "$FILTERED_FULL_OUTPUT"
  exit 1
fi

if ! grep -q -- "--namespace-base filters clean authored semantic exports" <<< "$FILTERED_FULL_OUTPUT"; then
  echo "FAILED: ontologies --full --namespace-base rejection did not explain clean export boundary"
  echo "$FILTERED_FULL_OUTPUT"
  exit 1
fi

for authored_term in \
  "ProductionEndpoint" \
  "SecondaryEndpoint"; do
  if ! grep -q "testonto:$authored_term rdfs:isDefinedBy <https://example.test/ontology>" <<< "$TTL_OUTPUT"; then
    echo "FAILED: Turtle output missing generated rdfs:isDefinedBy edge for authored ontology named subject: $authored_term"
    echo "$TTL_OUTPUT"
    exit 1
  fi
done

for prefix in \
  "@prefix owl:" \
  "@prefix rdf:" \
  "@prefix rdfs:" \
  "@prefix reqvire:" \
  "@prefix testonto:" \
  "@prefix ext:" \
  "@prefix sh:" \
  "@prefix xs:" \
  "@prefix xsd:"; do
  PREFIX_COUNT=$(count_occurrences "$prefix" "$TTL_OUTPUT")
  if [ "$PREFIX_COUNT" -gt 1 ]; then
    echo "FAILED: Turtle output contains duplicate prefix declaration for $prefix"
    echo "$TTL_OUTPUT"
    exit 1
  fi
done

ONTOLOGY_DECL_COUNT=$(( \
  $(count_occurrences "<https://example.test/ontology> a owl:Ontology" "$TTL_OUTPUT") \
))
if [ "$ONTOLOGY_DECL_COUNT" -ne 1 ]; then
  echo "FAILED: Turtle output should contain exactly one ontology document type declaration"
  echo "$TTL_OUTPUT"
  exit 1
fi

IMPORT_COUNT=$(count_occurrences "<https://example.test/imported>" "$TTL_OUTPUT")
if [ "$IMPORT_COUNT" -ne 1 ]; then
  echo "FAILED: Turtle output should contain the duplicated authored owl:imports statement exactly once"
  echo "$TTL_OUTPUT"
  exit 1
fi

if ! grep -q "owl:someValuesFrom testonto:Response" <<< "$TTL_OUTPUT"; then
  echo "FAILED: Turtle output missing ontology restriction construct fixture"
  echo "$TTL_OUTPUT"
  exit 1
fi

if ! grep -q "sh:targetClass testonto:ServiceEndpoint" <<< "$TTL_OUTPUT"; then
  echo "FAILED: Turtle output missing SHACL target class"
  echo "$TTL_OUTPUT"
  exit 1
fi

if ! grep -q "sh:datatype ext:ExternalCode" <<< "$TTL_OUTPUT"; then
  echo "FAILED: Turtle output missing SHACL reference to external custom datatype"
  echo "$TTL_OUTPUT"
  exit 1
fi

for external_source_token in \
  "<https://example.test/external> a owl:Ontology" \
  "ext:ExternalResource a owl:Class" \
  "ext:externalCode a owl:DatatypeProperty" \
  "External code datatype" \
  "External code property" \
  "External resource" \
  "Unused external resource" \
  "Unused external code property" \
  "<https://example.test/jsonld-external> a owl:Ontology" \
  "jsonext:JsonExternalResource a owl:Class" \
  "jsonext:jsonExternalCode a owl:DatatypeProperty" \
  "JSON-LD external code datatype" \
  "JSON-LD external code property" \
  "JSON-LD external resource" \
  "Unused JSON-LD external resource" \
  "Unused JSON-LD external code property" \
  "<https://example.test/rdf-external> a owl:Ontology" \
  "rdfext:RdfExternalResource a owl:Class" \
  "rdfext:rdfExternalCode a owl:DatatypeProperty" \
  "RDF/XML external code datatype" \
  "RDF/XML external code property" \
  "RDF/XML external resource" \
  "Unused RDF/XML external resource" \
  "Unused RDF/XML external code property" \
  "An idea or notion; a unit of thought." \
  "A set of concepts, optionally including statements about semantic relationships between those concepts." \
  "The preferred lexical label for a resource, in a given language."; do
  if grep -qF "$external_source_token" <<< "$TTL_OUTPUT"; then
    echo "FAILED: default Turtle output must not include external ontology source triples: $external_source_token"
    echo "$TTL_OUTPUT"
    exit 1
  fi
done

set +e
EXTERNAL_TTL_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" ontologies --include-external 2>&1)
EXTERNAL_TTL_EXIT=$?
set -e

if [ $EXTERNAL_TTL_EXIT -ne 0 ]; then
  echo "FAILED: ontologies --include-external command failed"
  echo "$EXTERNAL_TTL_OUTPUT"
  exit 1
fi

for used_external_subset_token in \
  "ext:ExternalCode a rdfs:Datatype" \
  "ext:ExternalResource a owl:Class" \
  "ext:externalCode a owl:DatatypeProperty" \
  "External code datatype" \
  "External code property" \
  "External resource" \
  "jsonext:JsonExternalCode a rdfs:Datatype" \
  "jsonext:JsonExternalResource a owl:Class" \
  "jsonext:jsonExternalCode a owl:DatatypeProperty" \
  "JSON-LD external code datatype" \
  "JSON-LD external code property" \
  "JSON-LD external resource" \
  "rdfext:RdfExternalCode a rdfs:Datatype" \
  "rdfext:RdfExternalResource a owl:Class" \
  "rdfext:rdfExternalCode a owl:DatatypeProperty" \
  "RDF/XML external code datatype" \
  "RDF/XML external code property" \
  "RDF/XML external resource" \
  "skos:Concept a owl:Class" \
  "skos:ConceptScheme a owl:Class" \
  "An idea or notion; a unit of thought." \
  "A set of concepts, optionally including statements about semantic relationships between those concepts."; do
  if ! grep -qF "$used_external_subset_token" <<< "$EXTERNAL_TTL_OUTPUT"; then
    echo "FAILED: --include-external Turtle output missing used external subset triple: $used_external_subset_token"
    echo "$EXTERNAL_TTL_OUTPUT"
    exit 1
  fi
done

for unused_external_source_token in \
  "<https://example.test/external> a owl:Ontology" \
  "ext:UnusedExternalResource a owl:Class" \
  "ext:unusedExternalCode a owl:DatatypeProperty" \
  "Unused external resource" \
  "Unused external code property" \
  "<https://example.test/jsonld-external> a owl:Ontology" \
  "jsonext:UnusedJsonExternalResource a owl:Class" \
  "jsonext:unusedJsonExternalCode a owl:DatatypeProperty" \
  "Unused JSON-LD external resource" \
  "Unused JSON-LD external code property" \
  "<https://example.test/rdf-external> a owl:Ontology" \
  "rdfext:UnusedRdfExternalResource a owl:Class" \
  "rdfext:unusedRdfExternalCode a owl:DatatypeProperty" \
  "Unused RDF/XML external resource" \
  "Unused RDF/XML external code property" \
  "skos:OrderedCollection a owl:Class" \
  "An ordered collection of concepts, where both the grouping and the ordering are meaningful."; do
  if grep -qF "$unused_external_source_token" <<< "$EXTERNAL_TTL_OUTPUT"; then
    echo "FAILED: --include-external Turtle output must not include unused external source triple: $unused_external_source_token"
    echo "$EXTERNAL_TTL_OUTPUT"
    exit 1
  fi
done

for external_definition_token in \
  "ext:ExternalCode rdfs:isDefinedBy" \
  "ext:ExternalCode rdfs:isDefinedBy" \
  "jsonext:JsonExternalCode rdfs:isDefinedBy" \
  "jsonext:JsonExternalCode rdfs:isDefinedBy" \
  "rdfext:RdfExternalCode rdfs:isDefinedBy" \
  "rdfext:RdfExternalCode rdfs:isDefinedBy"; do
  if grep -qF "$external_definition_token" <<< "$EXTERNAL_TTL_OUTPUT"; then
    echo "FAILED: --include-external Turtle output must not generate rdfs:isDefinedBy for external source terms: $external_definition_token"
    echo "$EXTERNAL_TTL_OUTPUT"
    exit 1
  fi
done

if ! grep -qF "# Source: reqvire:external-used-subset" <<< "$EXTERNAL_TTL_OUTPUT"; then
  echo "FAILED: --include-external Turtle output should include the external-used-subset source section"
  echo "$EXTERNAL_TTL_OUTPUT"
  exit 1
fi

if grep -qF "reqvire:ExternalOntologySource" <<< "$EXTERNAL_TTL_OUTPUT" || \
   grep -qF "reqvire:externalOntologyResource" <<< "$EXTERNAL_TTL_OUTPUT"; then
  echo "FAILED: --include-external Turtle output must not dump raw external ontology source graph triples"
  echo "$EXTERNAL_TTL_OUTPUT"
  exit 1
fi

set +e
FULL_EXTERNAL_TTL_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" ontologies --full --include-external 2>&1)
FULL_EXTERNAL_TTL_EXIT=$?
set -e

if [ $FULL_EXTERNAL_TTL_EXIT -ne 0 ]; then
  echo "FAILED: ontologies --full --include-external command failed"
  echo "$FULL_EXTERNAL_TTL_OUTPUT"
  exit 1
fi

for used_external_subset_token in \
  "ext:ExternalCode a rdfs:Datatype" \
  "ext:externalCode a owl:DatatypeProperty" \
  "jsonext:JsonExternalCode a rdfs:Datatype" \
  "jsonext:jsonExternalCode a owl:DatatypeProperty" \
  "rdfext:RdfExternalCode a rdfs:Datatype" \
  "rdfext:rdfExternalCode a owl:DatatypeProperty"; do
  if ! grep -qF "$used_external_subset_token" <<< "$FULL_EXTERNAL_TTL_OUTPUT"; then
    echo "FAILED: full external Turtle output missing used external subset triple: $used_external_subset_token"
    echo "$FULL_EXTERNAL_TTL_OUTPUT"
    exit 1
  fi
done

for unused_external_source_token in \
  "ext:UnusedExternalResource a owl:Class" \
  "ext:unusedExternalCode a owl:DatatypeProperty" \
  "jsonext:UnusedJsonExternalResource a owl:Class" \
  "jsonext:unusedJsonExternalCode a owl:DatatypeProperty" \
  "rdfext:UnusedRdfExternalResource a owl:Class" \
  "rdfext:unusedRdfExternalCode a owl:DatatypeProperty"; do
  if grep -qF "$unused_external_source_token" <<< "$FULL_EXTERNAL_TTL_OUTPUT"; then
    echo "FAILED: full external Turtle output must not include unused external source term: $unused_external_source_token"
    echo "$FULL_EXTERNAL_TTL_OUTPUT"
    exit 1
  fi
done

if ! grep -qF "reqvire:OntologyProjectionGraph" <<< "$FULL_EXTERNAL_TTL_OUTPUT"; then
  echo "FAILED: full external Turtle output missing ontology projection facts"
  echo "$FULL_EXTERNAL_TTL_OUTPUT"
  exit 1
fi

for forbidden in \
  "reqvire:OntologyProjectionGraph" \
  "reqvire:OntologyConstruct" \
  "urn:reqvire:ontology-construct" \
  "urn:reqvire:ontology-member" \
  "urn:reqvire:ontology-symbol" \
  "reqvire:projectionDerivationMode"; do
  if grep -qF "$forbidden" <<< "$TTL_OUTPUT"; then
    echo "FAILED: default Turtle output must not contain generated ontology projection marker: $forbidden"
    echo "$TTL_OUTPUT"
    exit 1
  fi
done

set +e
JSONLD_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" ontologies --jsonld 2>&1)
JSONLD_EXIT=$?
set -e

if [ $JSONLD_EXIT -ne 0 ]; then
  echo "FAILED: ontologies --jsonld command failed"
  echo "$JSONLD_OUTPUT"
  exit 1
fi

if ! jq . >/dev/null 2>&1 <<< "$JSONLD_OUTPUT"; then
  echo "FAILED: JSON-LD output should be valid JSON"
  echo "$JSONLD_OUTPUT"
  exit 1
fi

if ! jq -e 'any(.[]; .["@id"] == "https://example.test/ontology" and ((.["http://www.w3.org/1999/02/22-rdf-syntax-ns#type"] // []) | map(.["@id"]) | index("http://www.w3.org/2002/07/owl#Ontology")))' >/dev/null 2>&1 <<< "$JSONLD_OUTPUT"; then
  echo "FAILED: JSON-LD output missing generated ontology document declaration"
  echo "$JSONLD_OUTPUT"
  exit 1
fi

if jq -e 'any(.[]; .["@id"] == "https://example.test/ontology/api-ontology" and ((.["http://www.w3.org/1999/02/22-rdf-syntax-ns#type"] // []) | map(.["@id"]) | index("http://www.w3.org/2002/07/owl#Ontology")))' >/dev/null 2>&1 <<< "$JSONLD_OUTPUT"; then
  echo "FAILED: JSON-LD output should declare the ontology document at ontology_base, not per element"
  echo "$JSONLD_OUTPUT"
  exit 1
fi

if ! grep -qF '"@value":"testonto"' <<< "$JSONLD_OUTPUT"; then
  echo "FAILED: JSON-LD output missing generated ontology prefix"
  echo "$JSONLD_OUTPUT"
  exit 1
fi

if ! jq -e 'any(.[]; .["@id"] == "https://example.test/ontology#ServiceEndpoint" and ((.["http://www.w3.org/2000/01/rdf-schema#isDefinedBy"] // []) | map(.["@id"]) | index("https://example.test/ontology")))' >/dev/null 2>&1 <<< "$JSONLD_OUTPUT"; then
  echo "FAILED: JSON-LD output missing generated rdfs:isDefinedBy edge from authored ontology term to ontology document"
  echo "$JSONLD_OUTPUT"
  exit 1
fi

for authored_term in \
  "ProductionEndpoint" \
  "SecondaryEndpoint"; do
  if ! jq -e --arg iri "https://example.test/ontology#$authored_term" 'any(.[]; .["@id"] == $iri and ((.["http://www.w3.org/2000/01/rdf-schema#isDefinedBy"] // []) | map(.["@id"]) | index("https://example.test/ontology")))' >/dev/null 2>&1 <<< "$JSONLD_OUTPUT"; then
    echo "FAILED: JSON-LD output missing generated rdfs:isDefinedBy edge for authored ontology named subject: $authored_term"
    echo "$JSONLD_OUTPUT"
    exit 1
  fi
done

JSONLD_IMPORT_COUNT=$(jq '[.[] | select(.["@id"] == "https://example.test/ontology") | .["http://www.w3.org/2002/07/owl#imports"][]? | select(.["@id"] == "https://example.test/imported")] | length' <<< "$JSONLD_OUTPUT")
if [ "$JSONLD_IMPORT_COUNT" -ne 1 ]; then
  echo "FAILED: JSON-LD output should contain the duplicated authored owl:imports statement exactly once"
  echo "$JSONLD_OUTPUT"
  exit 1
fi

for forbidden in \
  "OntologyProjectionGraph" \
  "OntologyConstruct" \
  "urn:reqvire:ontology-construct" \
  "urn:reqvire:ontology-member" \
  "projectionDerivationMode"; do
  if grep -qF "$forbidden" <<< "$JSONLD_OUTPUT"; then
    echo "FAILED: default JSON-LD output must not contain generated ontology projection marker: $forbidden"
    echo "$JSONLD_OUTPUT"
    exit 1
  fi
done

if grep -qF "External code property" <<< "$JSONLD_OUTPUT" || \
   grep -qF "JSON-LD external code property" <<< "$JSONLD_OUTPUT" || \
   grep -qF "RDF/XML external code property" <<< "$JSONLD_OUTPUT"; then
  echo "FAILED: default JSON-LD output must not include external ontology source labels"
  echo "$JSONLD_OUTPUT"
  exit 1
fi

set +e
EXTERNAL_JSONLD_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" ontologies --include-external --jsonld 2>&1)
EXTERNAL_JSONLD_EXIT=$?
set -e

if [ $EXTERNAL_JSONLD_EXIT -ne 0 ]; then
  echo "FAILED: ontologies --include-external --jsonld command failed"
  echo "$EXTERNAL_JSONLD_OUTPUT"
  exit 1
fi

if ! jq . >/dev/null 2>&1 <<< "$EXTERNAL_JSONLD_OUTPUT"; then
  echo "FAILED: include-external JSON-LD output should be valid JSON"
  echo "$EXTERNAL_JSONLD_OUTPUT"
  exit 1
fi

for used_external_jsonld_token in \
  "https://example.test/external#externalCode" \
  "https://example.test/external#ExternalResource" \
  "External code property" \
  "https://example.test/jsonld-external#jsonExternalCode" \
  "https://example.test/jsonld-external#JsonExternalResource" \
  "JSON-LD external code property" \
  "https://example.test/rdf-external#rdfExternalCode" \
  "https://example.test/rdf-external#RdfExternalResource" \
  "RDF/XML external code property" \
  "http://www.w3.org/2004/02/skos/core#Concept" \
  "http://www.w3.org/2004/02/skos/core#ConceptScheme" \
  "An idea or notion; a unit of thought."; do
  if ! grep -qF "$used_external_jsonld_token" <<< "$EXTERNAL_JSONLD_OUTPUT"; then
    echo "FAILED: include-external JSON-LD output missing used external subset token: $used_external_jsonld_token"
    echo "$EXTERNAL_JSONLD_OUTPUT"
    exit 1
  fi
done

for unused_external_jsonld_token in \
  "Unused external resource" \
  "unusedExternalCode" \
  "Unused JSON-LD external resource" \
  "unusedJsonExternalCode" \
  "Unused RDF/XML external resource" \
  "unusedRdfExternalCode" \
  "http://www.w3.org/2004/02/skos/core#OrderedCollection" \
  "An ordered collection of concepts, where both the grouping and the ordering are meaningful."; do
  if grep -qF "$unused_external_jsonld_token" <<< "$EXTERNAL_JSONLD_OUTPUT"; then
    echo "FAILED: include-external JSON-LD output must not include unused external source token: $unused_external_jsonld_token"
    echo "$EXTERNAL_JSONLD_OUTPUT"
    exit 1
  fi
done

if grep -qF '"externalOntologySource"' <<< "$EXTERNAL_JSONLD_OUTPUT" || \
   grep -qF '"externalOntologyResource"' <<< "$EXTERNAL_JSONLD_OUTPUT"; then
  echo "FAILED: include-external JSON-LD output must not dump raw external ontology source graph triples"
  echo "$EXTERNAL_JSONLD_OUTPUT"
  exit 1
fi

set +e
FULL_TTL_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" ontologies --full 2>&1)
FULL_TTL_EXIT=$?
set -e

if [ $FULL_TTL_EXIT -ne 0 ]; then
  echo "FAILED: ontologies --full command failed"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

if ! grep -q "urn:reqvire:element:api-capability" <<< "$FULL_TTL_OUTPUT"; then
  echo "FAILED: full Turtle output missing capability element IRI"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

for named_individual_fact in \
  "<urn:reqvire:element:api-capability> a owl:NamedIndividual , reqvire:Element , reqvire:Capability" \
  "<urn:reqvire:element:api-endpoint-requirement> a owl:NamedIndividual , reqvire:Element , reqvire:Requirement" \
  "<urn:reqvire:element:api-endpoint-shape-contract> a owl:NamedIndividual , reqvire:Element , reqvire:SemanticContract"; do
  if ! grep -qF "$named_individual_fact" <<< "$FULL_TTL_OUTPUT"; then
    echo "FAILED: full Turtle output missing model owl:NamedIndividual fact: $named_individual_fact"
    echo "$FULL_TTL_OUTPUT"
    exit 1
  fi
done

if ! grep -q "reqvire:conceptReference concept:ServiceEndpoint" <<< "$FULL_TTL_OUTPUT"; then
  echo "FAILED: full Turtle output missing concept-reference term edge"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

if grep -q "reqvire:bindsContract <urn:reqvire:element:api-ontology>" <<< "$FULL_TTL_OUTPUT"; then
  echo "FAILED: full Turtle output must not contain capability ontology contract_bindings edge"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

if ! grep -qF "reqvire:bindsContract <urn:reqvire:element:api-endpoint-contract>" <<< "$FULL_TTL_OUTPUT"; then
  echo "FAILED: full Turtle output missing contract_bindings normalized forward predicate"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

if ! grep -qF "reqvire:boundByContract <urn:reqvire:element:api-client-requirement>" <<< "$FULL_TTL_OUTPUT"; then
  echo "FAILED: full Turtle output missing contract_bindings normalized inverse predicate"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

if grep -q "requirementBindsContract\\|contractBoundBy\\|reqvire:reuse" <<< "$FULL_TTL_OUTPUT"; then
  echo "FAILED: full Turtle output contains legacy contract binding projection vocabulary"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

if ! grep -q "reqvire:specifiedBy <urn:reqvire:element:api-endpoint-requirement>" <<< "$FULL_TTL_OUTPUT"; then
  echo "FAILED: full Turtle output missing capability requirement specifiedBy edge"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

if ! grep -q "reqvire:declaresTerm .*testonto:ServiceEndpoint" <<< "$FULL_TTL_OUTPUT"; then
  echo "FAILED: full Turtle output missing ontology term declaration edge"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

if ! grep -q "reqvire:referencesTerm .*testonto:ServiceEndpoint" <<< "$FULL_TTL_OUTPUT"; then
  echo "FAILED: full Turtle output missing semantic-contract reference edge"
  echo "$FULL_TTL_OUTPUT"
  exit 1
fi

for relation_fact in \
  "reqvire:constrainedBy <urn:reqvire:element:api-endpoint-shape-contract>" \
  "reqvire:constrain <urn:reqvire:element:api-endpoint-requirement>" \
  "reqvire:use <urn:reqvire:element:api-ontology>" \
  "reqvire:usedBy <urn:reqvire:element:api-endpoint-shape-contract>"; do
  if ! grep -qF "$relation_fact" <<< "$FULL_TTL_OUTPUT"; then
    echo "FAILED: full Turtle output missing semantic-contract relation fact: $relation_fact"
    echo "$FULL_TTL_OUTPUT"
    exit 1
  fi
done

for token in \
  "reqvire:OntologyProjectionGraph" \
  "reqvire:OntologyConstructProjection" \
  "reqvire:OntologyConstruct" \
  "urn:reqvire:ontology-construct" \
  "urn:reqvire:ontology-member" \
  "reqvire:projectionDerivationMode \"direct-authored\"" \
  "reqvire:constructFamily \"property-domain-range\"" \
  "reqvire:constructKind \"property-chain\"" \
  "reqvire:constructKind \"restriction\"" \
  "reqvire:restrictionKind \"existential\"" \
  "reqvire:constructSubject" \
  "reqvire:constructPredicate" \
  "reqvire:constructObject" \
  "reqvire:constructSourceBlock" \
  "reqvire:constructProvenance" \
  "reqvire:constructMember" \
  "reqvire:constructSequenceIndex"; do
  if ! grep -qF "$token" <<< "$FULL_TTL_OUTPUT"; then
    echo "FAILED: full Turtle output missing ontology projection fact: $token"
    echo "$FULL_TTL_OUTPUT"
    exit 1
  fi
done

set +e
FULL_JSONLD_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" ontologies --full --jsonld 2>&1)
FULL_JSONLD_EXIT=$?
set -e

if [ $FULL_JSONLD_EXIT -ne 0 ]; then
  echo "FAILED: ontologies --full --jsonld command failed"
  echo "$FULL_JSONLD_OUTPUT"
  exit 1
fi

if ! jq . >/dev/null 2>&1 <<< "$FULL_JSONLD_OUTPUT"; then
  echo "FAILED: full JSON-LD output should be valid JSON"
  echo "$FULL_JSONLD_OUTPUT"
  exit 1
fi

if ! grep -q "urn:reqvire:element:api-capability" <<< "$FULL_JSONLD_OUTPUT"; then
  echo "FAILED: full JSON-LD output missing model context element"
  echo "$FULL_JSONLD_OUTPUT"
  exit 1
fi

for token in \
  "OntologyProjectionGraph" \
  "OntologyConstruct" \
  "urn:reqvire:ontology-construct" \
  "urn:reqvire:ontology-member" \
  "projectionDerivationMode"; do
  if ! grep -qF "$token" <<< "$FULL_JSONLD_OUTPUT"; then
    echo "FAILED: full JSON-LD output missing generated ontology projection marker: $token"
    echo "$FULL_JSONLD_OUTPUT"
    exit 1
  fi
done

set +e
SEARCH_JSON_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-name "API Endpoint Requirement|API Endpoint Shape Contract|API Ontology" --json 2>&1)
SEARCH_JSON_EXIT=$?
set -e

if [ $SEARCH_JSON_EXIT -ne 0 ]; then
  echo "FAILED: search --json command failed"
  echo "$SEARCH_JSON_OUTPUT"
  exit 1
fi

if ! jq -e '
  any(.files[].elements[];
    .name == "API Endpoint Requirement"
    and any(.relations[]; .relation_type == "constrainedBy" and (.target.target | endswith("SemanticContracts.md#api-endpoint-shape-contract"))))
  and any(.files[].elements[];
    .name == "API Endpoint Shape Contract"
    and (.semantic_contract.shapes.content | contains("sh:NodeShape"))
    and any(.relations[]; .relation_type == "constrain" and (.target.target | endswith("SemanticContracts.md#api-endpoint-requirement")))
    and any(.relations[]; .relation_type == "use" and (.target.target | endswith("SemanticContracts.md#api-ontology"))))
  and any(.files[].elements[];
    .name == "API Ontology"
    and any(.relations[]; .relation_type == "usedBy" and (.target.target | endswith("SemanticContracts.md#api-endpoint-shape-contract"))))
' >/dev/null 2>&1 <<< "$SEARCH_JSON_OUTPUT"; then
  echo "FAILED: search JSON output missing semantic-contract shape artifacts or relation edges"
  echo "$SEARCH_JSON_OUTPUT"
  exit 1
fi

set +e
MODEL_JSON_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model --from "API Endpoint Requirement" 2>&1)
MODEL_JSON_EXIT=$?
set -e

if [ $MODEL_JSON_EXIT -ne 0 ]; then
  echo "FAILED: model command failed"
  echo "$MODEL_JSON_OUTPUT"
  exit 1
fi

if ! jq -e '
  any(.. | objects; .relation_type? == "constrainedBy")
  and any(.. | objects; .relation_type? == "use")
' >/dev/null 2>&1 <<< "$MODEL_JSON_OUTPUT"; then
  echo "FAILED: model JSON output missing semantic-contract constrainedBy/use relation chain"
  echo "$MODEL_JSON_OUTPUT"
  exit 1
fi

for forbidden in \
  "OntologyProjectionGraph" \
  "OntologyConstruct" \
  "urn:reqvire:ontology-construct" \
  "urn:reqvire:ontology-member" \
  "urn:reqvire:ontology-symbol" \
  "projectionDerivationMode"; do
  if grep -qF "$forbidden" <<< "$TTL_OUTPUT"; then
    echo "FAILED: default Turtle output must include document declarations and authored ontology/SHACL only, without projection facts: $forbidden"
    exit 1
  fi
done

# Default Turtle output must carry the representative OWL/RDFS constructs.
for construct in \
  "propertyChainAxiom" \
  "inverseOf" \
  "equivalentClass" \
  "equivalentProperty" \
  "sameAs" \
  "domain" \
  "range"; do
  if ! grep -q "$construct" <<< "$TTL_OUTPUT"; then
    echo "FAILED: default Turtle output missing OWL/RDFS construct: $construct"
    exit 1
  fi
done

# Representative standards reserved vocabulary terms must survive serialization without requiring local External Ontology sources.
for reserved_term in \
  "rdf:PlainLiteral" \
  "rdfs:label" \
  "rdfs:comment" \
  "rdfs:Literal" \
  "owl:rational" \
  "owl:real" \
  "owl:Thing" \
  "xsd:anyURI" \
  "xsd:string" \
  "xsd:boolean" \
  "xsd:integer" \
  "sh:NodeShape" \
  "sh:targetClass" \
  "sh:datatype"; do
  if ! grep -qF "$reserved_term" <<< "$TTL_OUTPUT"; then
    echo "FAILED: default Turtle output missing standards reserved vocabulary term: $reserved_term"
    echo "$TTL_OUTPUT"
    exit 1
  fi
done

if grep -A4 "#### External Ontology" "$TEST_DIR/specifications/SemanticContracts.md" | grep -Eq "prefix: (rdf|rdfs|owl|xs|xsd)$"; then
  echo "FAILED: standards reserved vocabulary fixture must not use External Ontology declarations"
  exit 1
fi

if grep -A4 "#### External Ontology" "$TEST_DIR/specifications/SemanticContracts.md" | grep -Eq "prefix: skos$"; then
  echo "FAILED: built-in SKOS fixture must not use a local External Ontology declaration"
  exit 1
fi

cp "$TEST_DIR/specifications/SemanticContracts.md" "$TEST_DIR/specifications/SemanticContracts.md.bak"
perl -0pi -e 's/testonto:SecondaryEndpoint a testonto:ServiceEndpoint ;/testonto:SecondaryEndpoint a testonto:ServiceEndpoint ;\n  rdfs:isDefinedBy <https:\/\/example.test\/wrong-ontology> ;/g' "$TEST_DIR/specifications/SemanticContracts.md"
set +e
INVALID_DEFINED_BY_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
INVALID_DEFINED_BY_EXIT=$?
set -e
mv "$TEST_DIR/specifications/SemanticContracts.md.bak" "$TEST_DIR/specifications/SemanticContracts.md"

if [ $INVALID_DEFINED_BY_EXIT -eq 0 ]; then
  echo "FAILED: validate should reject conflicting authored rdfs:isDefinedBy target"
  echo "$INVALID_DEFINED_BY_OUTPUT"
  exit 1
fi

if ! grep -q "conflicting rdfs:isDefinedBy target" <<< "$INVALID_DEFINED_BY_OUTPUT"; then
  echo "FAILED: conflicting rdfs:isDefinedBy validation error missing expected guidance"
  echo "$INVALID_DEFINED_BY_OUTPUT"
  exit 1
fi

cp "$TEST_DIR/specifications/fixtures/builtin-skos-invalid.txt" "$TEST_DIR/specifications/BuiltinSkosInvalid.md"
set +e
INVALID_SKOS_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
INVALID_SKOS_EXIT=$?
set -e
rm -f "$TEST_DIR/specifications/BuiltinSkosInvalid.md"

if [ $INVALID_SKOS_EXIT -eq 0 ]; then
  echo "FAILED: validate should reject references to missing built-in SKOS terms"
  echo "$INVALID_SKOS_OUTPUT"
  exit 1
fi

if ! grep -q "NotARealSkosClass" <<< "$INVALID_SKOS_OUTPUT"; then
  echo "FAILED: missing built-in SKOS term validation error should name the missing term"
  echo "$INVALID_SKOS_OUTPUT"
  exit 1
fi

cp "$TEST_DIR/specifications/fixtures/invalid-reserved-vocabulary.txt" "$TEST_DIR/specifications/InvalidReservedVocabulary.md"
set +e
INVALID_RESERVED_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
INVALID_RESERVED_EXIT=$?
set -e
rm -f "$TEST_DIR/specifications/InvalidReservedVocabulary.md"

if [ $INVALID_RESERVED_EXIT -eq 0 ]; then
  echo "FAILED: validate should reject fake terms in standards reserved vocabulary namespaces"
  echo "$INVALID_RESERVED_OUTPUT"
  exit 1
fi

if ! grep -q "NotARealShapeClass" <<< "$INVALID_RESERVED_OUTPUT"; then
  echo "FAILED: missing standards reserved vocabulary term validation error should name the missing term"
  echo "$INVALID_RESERVED_OUTPUT"
  exit 1
fi

exit 0
