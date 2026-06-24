#!/bin/bash
set -uo pipefail

MCP_PROTOCOL_VERSION="2025-11-25"
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

fail() {
  echo "❌ FAILED: $1"
  if [ -n "${2:-}" ] && [ -f "$2" ]; then
    echo ""
    cat "$2"
  fi
  exit 1
}

pick_port() {
  python3 - <<'PY'
import socket
s = socket.socket()
s.bind(("127.0.0.1", 0))
print(s.getsockname()[1])
s.close()
PY
}

json_line() {
  local file="$1"
  local line_number="$2"
  sed -n "${line_number}p" "$file"
}

assert_jq_line() {
  local file="$1"
  local line_number="$2"
  local filter="$3"
  local description="$4"

  if ! json_line "$file" "$line_number" | jq -e "$filter" >/dev/null 2>&1; then
    echo "❌ FAILED: $description"
    echo "Line $line_number:"
    json_line "$file" "$line_number"
    exit 1
  fi
}

start_http_mcp() {
  local port="$1"
  local output_prefix="$2"
  shift 2
  (cd "$TEST_DIR" && "$REQVIRE_BIN" mcp --host 127.0.0.1 --port "$port" "$@") > "${output_prefix}.stdout" 2> "${output_prefix}.stderr" &
  HTTP_MCP_PID=$!
}

stop_http_mcp() {
  if [ -n "${HTTP_MCP_PID:-}" ]; then
    kill "$HTTP_MCP_PID" >/dev/null 2>&1 || true
    wait "$HTTP_MCP_PID" >/dev/null 2>&1 || true
    HTTP_MCP_PID=""
  fi
}

wait_for_http_mcp() {
  local port="$1"
  local output_file="$2"
  local request
  request="$(init_request)"

  for _ in $(seq 1 50); do
    if curl -sS -o "$output_file" \
      -H 'Content-Type: application/json' \
      -H 'Accept: application/json, text/event-stream' \
      --data "$request" \
      "http://127.0.0.1:${port}/mcp" >/dev/null 2>&1; then
      if jq -e '.result.protocolVersion == "2025-11-25"' "$output_file" >/dev/null 2>&1; then
        return 0
      fi
    fi
    sleep 0.1
  done

  return 1
}

http_mcp_call() {
  local port="$1"
  local request="$2"
  local output_file="$3"
  shift 3
  curl -sS -o "$output_file" \
    -H 'Content-Type: application/json' \
    -H 'Accept: application/json, text/event-stream' \
    -H "Mcp-Protocol-Version: ${MCP_PROTOCOL_VERSION}" \
    "$@" \
    --data "$request" \
    "http://127.0.0.1:${port}/mcp"
}

run_http_mcp_sequence() {
  local port="$1"
  local output_file="$2"
  local tmp_file="${output_file}.tmp"
  shift 2
  : > "$output_file"
  for request in "$@"; do
    http_mcp_call "$port" "$request" "$tmp_file" || return 1
    cat "$tmp_file" >> "$output_file"
    printf '\n' >> "$output_file"
  done
}

init_request() {
  jq -n -c --arg version "$MCP_PROTOCOL_VERSION" \
    '{jsonrpc:"2.0",id:1,method:"initialize",params:{protocolVersion:$version,capabilities:{},clientInfo:{name:"reqvire-test",version:"0"}}}'
}

tools_list_request() {
  jq -n -c '{jsonrpc:"2.0",id:2,method:"tools/list",params:{}}'
}

resources_list_request() {
  jq -n -c '{jsonrpc:"2.0",id:3,method:"resources/list",params:{}}'
}

prompts_list_request() {
  jq -n -c '{jsonrpc:"2.0",id:25,method:"prompts/list",params:{}}'
}

semantic_query_prompt_request() {
  jq -n -c '{jsonrpc:"2.0",id:26,method:"prompts/get",params:{name:"reqvire.semantic.query",arguments:{question:"How many requirements are verified?",scope:"MCP interface"}}}'
}

workflow_explore_prompt_request() {
  jq -n -c '{jsonrpc:"2.0",id:27,method:"prompts/get",params:{name:"reqvire.workflow.explore_model",arguments:{question:"Show relevant requirements"}}}'
}

unknown_prompt_request() {
  jq -n -c '{jsonrpc:"2.0",id:28,method:"prompts/get",params:{name:"reqvire.unknown.prompt"}}'
}

workspace_status_request() {
  jq -n -c '{jsonrpc:"2.0",id:4,method:"tools/call",params:{name:"reqvire.workspace_status",arguments:{}}}'
}

read_element_request() {
  jq -n -c '{jsonrpc:"2.0",id:5,method:"tools/call",params:{name:"reqvire.read_element",arguments:{name:"Test Requirement Beta"}}}'
}

semantic_requirement_request() {
  jq -n -c '{jsonrpc:"2.0",id:11,method:"tools/call",params:{name:"reqvire.read_element",arguments:{name:"MCP Semantic Requirement"}}}'
}

ontology_search_request() {
  jq -n -c '{jsonrpc:"2.0",id:12,method:"tools/call",params:{name:"reqvire.search",arguments:{filter_type:"ontology"}}}'
}

ontologies_request() {
  jq -n -c '{jsonrpc:"2.0",id:14,method:"tools/call",params:{name:"reqvire.semantic.ontologies",arguments:{}}}'
}

ontologies_jsonld_request() {
  jq -n -c '{jsonrpc:"2.0",id:15,method:"tools/call",params:{name:"reqvire.semantic.ontologies",arguments:{format:"jsonld"}}}'
}

semantic_graph_full_request() {
  jq -n -c '{jsonrpc:"2.0",id:16,method:"tools/call",params:{name:"reqvire.semantic.graph",arguments:{full:true}}}'
}

ontologies_include_external_request() {
  jq -n -c '{jsonrpc:"2.0",id:29,method:"tools/call",params:{name:"reqvire.semantic.ontologies",arguments:{include_external:true}}}'
}

semantic_shapes_request() {
  jq -n -c '{jsonrpc:"2.0",id:20,method:"tools/call",params:{name:"reqvire.semantic.shapes",arguments:{}}}'
}

semantic_concepts_request() {
  jq -n -c '{jsonrpc:"2.0",id:21,method:"tools/call",params:{name:"reqvire.semantic.concepts",arguments:{include_mappings:true}}}'
}

semantic_graph_include_external_request() {
  jq -n -c '{jsonrpc:"2.0",id:34,method:"tools/call",params:{name:"reqvire.semantic.graph",arguments:{include_external:true}}}'
}

semantic_prefixes_request() {
  jq -n -c '{jsonrpc:"2.0",id:19,method:"tools/call",params:{name:"reqvire.semantic.prefixes",arguments:{}}}'
}

semantic_prefixes_include_external_request() {
  jq -n -c '{jsonrpc:"2.0",id:30,method:"tools/call",params:{name:"reqvire.semantic.prefixes",arguments:{include_external:true}}}'
}

semantic_vocabulary_all_request() {
  jq -n -c '{jsonrpc:"2.0",id:22,method:"tools/call",params:{name:"reqvire.semantic.vocabulary",arguments:{section:"all"}}}'
}

semantic_vocabulary_authored_document_classes_request() {
  jq -n -c '{jsonrpc:"2.0",id:36,method:"tools/call",params:{name:"reqvire.semantic.vocabulary",arguments:{section:"classes",ontology_base:"https://example.test/ontology"}}}'
}

semantic_vocabulary_external_properties_request() {
  jq -n -c '{jsonrpc:"2.0",id:31,method:"tools/call",params:{name:"reqvire.semantic.vocabulary",arguments:{section:"properties",include_external:true,filter:"ExternalCode"}}}'
}

semantic_vocabulary_external_document_properties_request() {
  jq -n -c '{jsonrpc:"2.0",id:37,method:"tools/call",params:{name:"reqvire.semantic.vocabulary",arguments:{section:"properties",include_external:true,ontology_document:"https://example.test/mcp-external"}}}'
}

semantic_vocabulary_unused_external_classes_request() {
  jq -n -c '{jsonrpc:"2.0",id:35,method:"tools/call",params:{name:"reqvire.semantic.vocabulary",arguments:{section:"classes",include_external:true,filter:"ExternalResource"}}}'
}

semantic_vocabulary_relation_families_request() {
  jq -n -c '{jsonrpc:"2.0",id:23,method:"tools/call",params:{name:"reqvire.semantic.vocabulary",arguments:{section:"relation_families",limit:1}}}'
}

semantic_vocabulary_query_patterns_request() {
  jq -n -c '{jsonrpc:"2.0",id:24,method:"tools/call",params:{name:"reqvire.semantic.vocabulary",arguments:{section:"query_patterns",include_examples:true}}}'
}

semantic_vocabulary_concepts_request() {
  jq -n -c '{jsonrpc:"2.0",id:42,method:"tools/call",params:{name:"reqvire.semantic.vocabulary",arguments:{section:"concepts"}}}'
}

concept_schemes_list_request() {
  jq -n -c '{jsonrpc:"2.0",id:43,method:"tools/call",params:{name:"reqvire.concept_schemes.list",arguments:{}}}'
}

concepts_list_request() {
  jq -n -c '{jsonrpc:"2.0",id:44,method:"tools/call",params:{name:"reqvire.concepts.list",arguments:{scheme_iri:"https://example.test/concepts#MCPConcepts"}}}'
}

concept_get_request() {
  jq -n -c '{jsonrpc:"2.0",id:45,method:"tools/call",params:{name:"reqvire.concepts.get",arguments:{iri:"https://example.test/concepts#AccessToken"}}}'
}

concept_mappings_list_request() {
  jq -n -c '{jsonrpc:"2.0",id:46,method:"tools/call",params:{name:"reqvire.concept_mappings.list",arguments:{}}}'
}

sparql_request() {
  jq -n -c --arg query 'PREFIX owl: <http://www.w3.org/2002/07/owl#>
PREFIX reqvire: <https://www.reqvire.org/ontology#>
SELECT ?requirement ?verification ?relation WHERE {
  ?requirement a reqvire:Requirement ;
    reqvire:elementId "mcp-semantic-requirement" ;
    reqvire:requirementVerifiedByVerification ?verification .
  ?verification reqvire:elementId "mcp-semantic-verification" .
  ?relation a reqvire:ModelRelation ;
    reqvire:relationSource ?requirement ;
    reqvire:relationTarget ?verification ;
    reqvire:relationType "verifiedBy" .
}' '{jsonrpc:"2.0",id:17,method:"tools/call",params:{name:"reqvire.semantic.sparql",arguments:{query:$query}}}'
}

invalid_sparql_request() {
  jq -n -c '{jsonrpc:"2.0",id:18,method:"tools/call",params:{name:"reqvire.semantic.sparql",arguments:{query:"SELECT WHERE"}}}'
}

sparql_external_default_request() {
  jq -n -c --arg query 'PREFIX owl: <http://www.w3.org/2002/07/owl#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
PREFIX ext: <https://example.test/mcp-external#>
SELECT ?datatype WHERE {
  ?datatype a rdfs:Datatype .
  FILTER(?datatype = ext:ExternalCode)
}' '{jsonrpc:"2.0",id:32,method:"tools/call",params:{name:"reqvire.semantic.sparql",arguments:{query:$query}}}'
}

sparql_external_include_request() {
  jq -n -c --arg query 'PREFIX owl: <http://www.w3.org/2002/07/owl#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
PREFIX ext: <https://example.test/mcp-external#>
SELECT ?datatype WHERE {
  ?datatype a rdfs:Datatype .
  FILTER(?datatype = ext:ExternalCode)
}' '{jsonrpc:"2.0",id:33,method:"tools/call",params:{name:"reqvire.semantic.sparql",arguments:{query:$query,include_external:true}}}'
}

sparql_unused_external_include_request() {
  jq -n -c --arg query 'PREFIX owl: <http://www.w3.org/2002/07/owl#>
PREFIX ext: <https://example.test/mcp-external#>
SELECT ?unused WHERE {
  ?unused a owl:Class .
  FILTER(?unused = ext:ExternalResource)
}' '{jsonrpc:"2.0",id:36,method:"tools/call",params:{name:"reqvire.semantic.sparql",arguments:{query:$query,include_external:true}}}'
}

sparql_authored_graph_request() {
  jq -n -c --arg query 'PREFIX owl: <http://www.w3.org/2002/07/owl#>
PREFIX testonto: <https://example.test/ontology#>
SELECT ?s WHERE {
  GRAPH <urn:reqvire:semantic-graph:authored-ontology> {
    ?s a owl:Class .
    FILTER(?s = testonto:AccessToken)
  }
}' '{jsonrpc:"2.0",id:38,method:"tools/call",params:{name:"reqvire.semantic.sparql",arguments:{query:$query}}}'
}

sparql_model_context_graph_request() {
  jq -n -c --arg query 'PREFIX reqvire: <https://www.reqvire.org/ontology#>
SELECT ?projection WHERE {
  GRAPH <urn:reqvire:semantic-graph:generated> {
    ?projection a reqvire:OntologyProjectionGraph .
  }
}' '{jsonrpc:"2.0",id:39,method:"tools/call",params:{name:"reqvire.semantic.sparql",arguments:{query:$query,full:true}}}'
}

sparql_external_subset_graph_request() {
  jq -n -c --arg query 'PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
PREFIX ext: <https://example.test/mcp-external#>
SELECT ?datatype WHERE {
  GRAPH <urn:reqvire:semantic-graph:external-used-subset> {
    ?datatype a rdfs:Datatype .
    FILTER(?datatype = ext:ExternalCode)
  }
}' '{jsonrpc:"2.0",id:40,method:"tools/call",params:{name:"reqvire.semantic.sparql",arguments:{query:$query,include_external:true}}}'
}

sparql_raw_external_graph_request() {
  jq -n -c --arg query 'PREFIX owl: <http://www.w3.org/2002/07/owl#>
PREFIX ext: <https://example.test/mcp-external#>
SELECT ?unused WHERE {
  GRAPH <urn:reqvire:semantic-graph:raw-external-source> {
    ?unused a owl:Class .
    FILTER(?unused = ext:ExternalResource)
  }
}' '{jsonrpc:"2.0",id:41,method:"tools/call",params:{name:"reqvire.semantic.sparql",arguments:{query:$query,full:true,include_external:true}}}'
}

model_request() {
  jq -n -c '{jsonrpc:"2.0",id:13,method:"tools/call",params:{name:"reqvire.model",arguments:{from:"Test Requirement Alpha"}}}'
}

collect_request() {
  jq -n -c '{jsonrpc:"2.0",id:6,method:"tools/call",params:{name:"reqvire.collect",arguments:{element_name:"Test Requirement Beta"}}}'
}

schema_error_request() {
  jq -n -c '{jsonrpc:"2.0",id:7,method:"tools/call",params:{name:"reqvire.collect",arguments:{}}}'
}

unknown_tool_request() {
  jq -n -c '{jsonrpc:"2.0",id:8,method:"tools/call",params:{name:"reqvire.validate",arguments:{}}}'
}

resource_read_request() {
  jq -n -c '{jsonrpc:"2.0",id:9,method:"resources/read",params:{uri:"reqvire://workspace/status"}}'
}

format_fix_rejected_request() {
  jq -n -c '{jsonrpc:"2.0",id:10,method:"tools/call",params:{name:"reqvire.format",arguments:{fix:true}}}'
}

cp -a "$TEST_SCRIPT_DIR/../test-json-file-output/specifications" "$TEST_DIR/"
cp -a "$TEST_SCRIPT_DIR/../test-json-file-output/docs" "$TEST_DIR/"
mkdir -p "$TEST_DIR/specifications/references"
cat > "$TEST_DIR/specifications/references/mcp-external.ttl" <<'EOF'
@prefix ext: <https://example.test/mcp-external#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

<https://example.test/mcp-external> a owl:Ontology ;
  rdfs:label "MCP external test vocabulary" .

ext:ExternalResource a owl:Class ;
  rdfs:label "MCP external resource" .

ext:ExternalCode a rdfs:Datatype ;
  rdfs:label "MCP external code datatype" .
EOF

cat > "$TEST_DIR/specifications/references/mcp-external.jsonld" <<'EOF'
{
  "@context": {
    "jsonext": "https://example.test/mcp-jsonld-external#",
    "owl": "http://www.w3.org/2002/07/owl#",
    "rdfs": "http://www.w3.org/2000/01/rdf-schema#"
  },
  "@graph": [
    {
      "@id": "https://example.test/mcp-jsonld-external",
      "@type": "owl:Ontology",
      "rdfs:label": "MCP JSON-LD external vocabulary"
    },
    {
      "@id": "jsonext:JsonExternalResource",
      "@type": "owl:Class",
      "rdfs:label": "MCP JSON-LD external resource"
    }
  ]
}
EOF

cat > "$TEST_DIR/specifications/references/mcp-external.rdf" <<'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF
  xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
  xmlns:rdfs="http://www.w3.org/2000/01/rdf-schema#"
  xmlns:owl="http://www.w3.org/2002/07/owl#">
  <owl:Ontology rdf:about="https://example.test/mcp-rdf-external">
    <rdfs:label>MCP RDF/XML external vocabulary</rdfs:label>
  </owl:Ontology>

  <owl:Class rdf:about="https://example.test/mcp-rdf-external#RdfExternalResource">
    <rdfs:label>MCP RDF/XML external resource</rdfs:label>
  </owl:Class>
</rdf:RDF>
EOF

cat >> "$TEST_DIR/specifications/Requirements.md" <<'EOF'

### MCP Semantic Capability

MCP semantic capability for readable Access Token model concepts.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [MCP Semantic Requirement](#mcp-semantic-requirement)
---

### MCP Access Token Ontology

Access token ontology for MCP semantic evidence.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### External Ontology
  * prefix: ext
  * namespace: https://example.test/mcp-external#
  * resource: https://example.test/mcp-external
  * source: references/mcp-external.ttl
  * format: turtle

#### External Ontology
  * prefix: jsonext
  * namespace: https://example.test/mcp-jsonld-external#
  * resource: https://example.test/mcp-jsonld-external
  * source: references/mcp-external.jsonld
  * format: jsonld

#### External Ontology
  * prefix: rdfext
  * namespace: https://example.test/mcp-rdf-external#
  * resource: https://example.test/mcp-rdf-external
  * source: references/mcp-external.rdf
  * format: rdf

#### Ontology
```turtle
@prefix concept: <https://example.test/concepts#> .
@prefix testonto: <https://example.test/ontology#> .
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
testonto:AccessToken a owl:Class ;
  reqvire:mapsToConcept concept:AccessToken .
testonto:subject a owl:ObjectProperty .
testonto:testVerificationRelationFamily a reqvire:RelationFamily ;
  reqvire:relationFamilyName "test-verification" ;
  reqvire:relationFamilyMeaning "Test verification relation family." ;
  reqvire:relationFamilyForwardProperty testonto:subject ;
  reqvire:relationFamilyInverseProperty testonto:subject .
testonto:testVerificationRelationRule a reqvire:RelationRule ;
  reqvire:relationName "testVerifiedBy" ;
  reqvire:relationFamily testonto:testVerificationRelationFamily ;
  reqvire:relationDirection "forward" ;
  reqvire:allowedSourceType "requirement" ;
  reqvire:allowedTargetType "verification" .
testonto:testContractRelationFamily a reqvire:RelationFamily ;
  reqvire:relationFamilyName "test-contract" ;
  reqvire:relationFamilyMeaning "Test contract relation family." ;
  reqvire:relationFamilyForwardProperty testonto:subject ;
  reqvire:relationFamilyInverseProperty testonto:subject .
testonto:testContractRelationRule a reqvire:RelationRule ;
  reqvire:relationName "testDefinedBy" ;
  reqvire:relationFamily testonto:testContractRelationFamily ;
  reqvire:relationDirection "forward" ;
  reqvire:allowedSourceType "requirement" ;
  reqvire:allowedTargetType "contract" .
testonto:code a owl:DatatypeProperty .
```
---

### MCP Concepts

Native concept scheme for MCP semantic evidence.

Concept scheme for MCP semantic evidence.

#### Metadata
  * type: concept-scheme
  * concept_base: https://example.test/concepts
  * concept_prefix: concept
---
### Access Token

Native concept for access tokens.

A credential-like token used to authorize access.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [MCP Concepts](#mcp-concepts)
---
### MCP Semantic Requirement

The system shall expose MCP semantic evidence for readable Access Token model concepts.

#### Metadata
  * type: requirement

#### Concept References
  * [Access Token](#access-token)

#### Relations
  * specify: [MCP Semantic Capability](#mcp-semantic-capability)
  * constrainedBy: [MCP Access Token Shape Contract](#mcp-access-token-shape-contract)
  * verifiedBy: [MCP Semantic Verification](#mcp-semantic-verification)
---

### MCP Semantic Verification Objective

Verification objective for MCP semantic relation-family query projection.

#### Metadata
  * type: verification-objective

#### Relations
  * derive: [MCP Semantic Verification](#mcp-semantic-verification)
---

### MCP Semantic Verification

Verifies that MCP semantic query projection exposes normalized verification relations.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [MCP Semantic Verification Objective](#mcp-semantic-verification-objective)
  * verify: [MCP Semantic Requirement](#mcp-semantic-requirement)
  * satisfiedBy: [asset.txt](../docs/asset.txt)
---

### MCP Access Token Shape Contract

MCP access token shape contract.

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [MCP Semantic Requirement](#mcp-semantic-requirement)
  * use: [MCP Access Token Ontology](#mcp-access-token-ontology)

#### Shapes
```turtle
@prefix testonto: <https://example.test/ontology#> .
@prefix ext: <https://example.test/mcp-external#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

testonto:AccessTokenShape
  a sh:NodeShape ;
  sh:targetClass testonto:AccessToken ;
  sh:property [
    sh:path testonto:subject ;
  ] ;
  sh:property [
    sh:path testonto:code ;
    sh:datatype ext:ExternalCode ;
  ] .
```
---
EOF

(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json --filter-name "Test Requirement Beta") > "$TEST_DIR/output/binary-search.json" 2>&1 \
  || fail "Reqvire binary search failed" "$TEST_DIR/output/binary-search.json"
jq -e '.global_counters.total_elements == 1' "$TEST_DIR/output/binary-search.json" >/dev/null \
  || fail "Reqvire binary search should find Test Requirement Beta" "$TEST_DIR/output/binary-search.json"

set +e
(cd "$TEST_DIR" && "$REQVIRE_BIN" mcp --transport stdio) > "$TEST_DIR/output/mcp-stdio-removed.stdout" 2> "$TEST_DIR/output/mcp-stdio-removed.stderr"
STDIO_EXIT=$?
set +e
if [ "$STDIO_EXIT" -eq 0 ]; then
  fail "stdio MCP transport should not be accepted"
fi

DEFAULT_OUTPUT="$TEST_DIR/output/mcp-default.jsonl"
DEFAULT_PORT="$(pick_port)"
DEFAULT_OUTPUT_PREFIX="$TEST_DIR/output/mcp-default"
start_http_mcp "$DEFAULT_PORT" "$DEFAULT_OUTPUT_PREFIX"
trap stop_http_mcp EXIT
wait_for_http_mcp "$DEFAULT_PORT" "$TEST_DIR/output/mcp-default-init.json" || fail "default MCP HTTP server did not start" "${DEFAULT_OUTPUT_PREFIX}.stderr"
run_http_mcp_sequence "$DEFAULT_PORT" "$DEFAULT_OUTPUT" \
  "$(init_request)" \
  "$(tools_list_request)" \
  "$(resources_list_request)" \
  "$(workspace_status_request)" \
  "$(read_element_request)" \
  "$(collect_request)" \
  "$(schema_error_request)" \
  "$(unknown_tool_request)" \
  "$(resource_read_request)" \
  "$(format_fix_rejected_request)" \
  "$(semantic_requirement_request)" \
  "$(ontology_search_request)" \
  "$(ontologies_request)" \
  "$(ontologies_jsonld_request)" \
  "$(semantic_graph_full_request)" \
  "$(sparql_request)" \
  "$(invalid_sparql_request)" \
  "$(semantic_prefixes_request)" \
  "$(semantic_shapes_request)" \
  "$(semantic_concepts_request)" \
  "$(semantic_vocabulary_all_request)" \
  "$(semantic_vocabulary_relation_families_request)" \
  "$(semantic_vocabulary_query_patterns_request)" \
  "$(prompts_list_request)" \
  "$(semantic_query_prompt_request)" \
  "$(workflow_explore_prompt_request)" \
  "$(unknown_prompt_request)" \
  "$(ontologies_include_external_request)" \
  "$(semantic_prefixes_include_external_request)" \
  "$(semantic_vocabulary_external_properties_request)" \
  "$(sparql_external_default_request)" \
  "$(sparql_external_include_request)" \
  "$(semantic_graph_include_external_request)" \
  "$(sparql_authored_graph_request)" \
  "$(sparql_model_context_graph_request)" \
  "$(sparql_external_subset_graph_request)" \
  "$(sparql_raw_external_graph_request)" \
  "$(semantic_vocabulary_unused_external_classes_request)" \
  "$(sparql_unused_external_include_request)" \
  "$(semantic_vocabulary_authored_document_classes_request)" \
  "$(semantic_vocabulary_external_document_properties_request)" \
  "$(semantic_vocabulary_concepts_request)" \
  "$(concept_schemes_list_request)" \
  "$(concepts_list_request)" \
  "$(concept_get_request)" \
  "$(concept_mappings_list_request)" || fail "default MCP HTTP request sequence failed"
stop_http_mcp
trap - EXIT

assert_jq_line "$DEFAULT_OUTPUT" 1 '.result.protocolVersion == "2025-11-25"' "initialize reports supported protocol"
assert_jq_line "$DEFAULT_OUTPUT" 1 '.result.capabilities.tools | type == "object"' "initialize reports standard tools capability"
assert_jq_line "$DEFAULT_OUTPUT" 1 '.result.capabilities.resources | type == "object"' "initialize reports standard resources capability"
assert_jq_line "$DEFAULT_OUTPUT" 1 '.result.capabilities.prompts | type == "object"' "initialize reports standard prompts capability"
assert_jq_line "$DEFAULT_OUTPUT" 1 '.result.capabilities.logging == null and .result.capabilities.completions == null and .result.capabilities.tasks == null' "initialize does not advertise unsupported capabilities"
assert_jq_line "$DEFAULT_OUTPUT" 1 '.result.serverInfo.name == "reqvire"' "initialize reports serverInfo"

assert_jq_line "$DEFAULT_OUTPUT" 2 '[.result.tools[].name] | index("reqvire.search") != null' "tools/list includes read tools"
assert_jq_line "$DEFAULT_OUTPUT" 2 '[.result.tools[].name] | index("reqvire.semantic.ontologies") != null' "tools/list includes semantic ontology tool"
assert_jq_line "$DEFAULT_OUTPUT" 2 '[.result.tools[].name] | index("reqvire.semantic.shapes") != null' "tools/list includes semantic shapes tool"
assert_jq_line "$DEFAULT_OUTPUT" 2 '[.result.tools[].name] | index("reqvire.semantic.concepts") != null' "tools/list includes semantic concepts tool"
assert_jq_line "$DEFAULT_OUTPUT" 2 '[.result.tools[].name] | index("reqvire.concepts.list") != null and index("reqvire.concepts.get") != null and index("reqvire.concept_schemes.list") != null and index("reqvire.concept_mappings.list") != null and index("reqvire.concepts.validate_mappings") == null' "tools/list includes standalone concept tools"
assert_jq_line "$DEFAULT_OUTPUT" 2 '[.result.tools[].name] | index("reqvire.semantic.graph") != null' "tools/list includes semantic graph tool"
assert_jq_line "$DEFAULT_OUTPUT" 2 '[.result.tools[].name] | index("reqvire.semantic.sparql") != null' "tools/list includes semantic query tool"
assert_jq_line "$DEFAULT_OUTPUT" 2 '[.result.tools[].name] | index("reqvire.semantic.prefixes") != null' "tools/list includes semantic prefix registry tool"
assert_jq_line "$DEFAULT_OUTPUT" 2 '[.result.tools[].name] | index("reqvire.semantic.vocabulary") != null' "tools/list includes semantic vocabulary tool"
assert_jq_line "$DEFAULT_OUTPUT" 2 '[.result.tools[].name] | index("reqvire.add_element") == null and index("reqvire.link") == null' "default tools/list omits mutation tools"
assert_jq_line "$DEFAULT_OUTPUT" 2 '[.result.tools[].name] | index("reqvire.mcp") == null and index("reqvire.serve") == null and index("reqvire.validate") == null' "tools/list omits server and validate commands"
assert_jq_line "$DEFAULT_OUTPUT" 2 '[.result.tools[].name] | index("reqvire.command") == null and index("reqvire.shell") == null and index("reqvire.sout") == null' "tools/list omits shell-style tools"
assert_jq_line "$DEFAULT_OUTPUT" 2 'all(.result.tools[]; (.name|type=="string") and (.description|type=="string") and (.inputSchema.type=="object") and (.outputSchema|type=="object") and (.annotations|type=="object"))' "each tool has MCP tool contract fields"
assert_jq_line "$DEFAULT_OUTPUT" 2 'all(.result.tools[]; ((.inputSchema.properties // {}) | has("json") | not) and ((.inputSchema.properties // {}) | has("output") | not))' "tool schemas omit CLI transport options"
assert_jq_line "$DEFAULT_OUTPUT" 2 '.result.tools[] | select(.name=="reqvire.search") | .inputSchema.properties | has("filter_status") and has("filter_priority") and has("filter_risk") and has("filter_owner")' "search tool schema advertises governance metadata filters"
assert_jq_line "$DEFAULT_OUTPUT" 2 '.result.tools[] | select(.name=="reqvire.semantic.ontologies") | .inputSchema.properties.include_external.default == false' "semantic ontologies schema advertises include_external flag"
assert_jq_line "$DEFAULT_OUTPUT" 2 '.result.tools[] | select(.name=="reqvire.semantic.prefixes") | .inputSchema.properties.include_external.default == false' "semantic prefixes schema advertises include_external flag"
assert_jq_line "$DEFAULT_OUTPUT" 2 '.result.tools[] | select(.name=="reqvire.semantic.vocabulary") | .inputSchema.properties.include_external.default == false' "semantic vocabulary schema advertises include_external flag"
assert_jq_line "$DEFAULT_OUTPUT" 2 '.result.tools[] | select(.name=="reqvire.semantic.vocabulary") | .inputSchema.properties | has("ontology_document") and has("ontology_base")' "semantic vocabulary schema advertises ontology document filters"
assert_jq_line "$DEFAULT_OUTPUT" 2 '.result.tools[] | select(.name=="reqvire.semantic.sparql") | .inputSchema.properties.include_external.default == false' "semantic sparql schema advertises include_external flag"
assert_jq_line "$DEFAULT_OUTPUT" 2 '.result.tools[] | select(.name=="reqvire.format") | .annotations.readOnlyHint == true and .inputSchema.properties.fix.enum == [false]' "format is preview-only in default mode"

assert_jq_line "$DEFAULT_OUTPUT" 3 '[.result.resources[].uri] | index("reqvire://workspace/status") != null' "resources/list exposes workspace status"
assert_jq_line "$DEFAULT_OUTPUT" 4 '.result.structuredContent.workspace_root | type == "string"' "workspace_status returns workspace root"
assert_jq_line "$DEFAULT_OUTPUT" 4 '.result.structuredContent.size_estimates_enabled == false' "workspace_status reports size estimates disabled by default"
assert_jq_line "$DEFAULT_OUTPUT" 4 '.result.structuredContent.git.head | type == "string"' "workspace_status returns git HEAD"
assert_jq_line "$DEFAULT_OUTPUT" 4 '.result.structuredContent.model.valid == true and (.result.structuredContent.model.fingerprint | type == "string")' "workspace_status returns model validity and fingerprint"
assert_jq_line "$DEFAULT_OUTPUT" 5 '.result.structuredContent.name == "Test Requirement Beta"' "read_element returns authoritative element"
assert_jq_line "$DEFAULT_OUTPUT" 5 '.result.structuredContent | has("size_estimate") | not' "read_element omits size estimate by default"
assert_jq_line "$DEFAULT_OUTPUT" 6 '.result.structuredContent.starting_element == "specifications/Requirements.md#test-requirement-beta" and (.result.structuredContent.items[] | select(.name=="Test Requirement Beta"))' "collect returns structured content"
assert_jq_line "$DEFAULT_OUTPUT" 7 '.error.code == -32602 and (.error.data.message | contains("element_name"))' "schema-invalid tool arguments return protocol error"
assert_jq_line "$DEFAULT_OUTPUT" 8 '.error.code == -32602' "unknown or unadvertised tool returns protocol error"
assert_jq_line "$DEFAULT_OUTPUT" 9 '.result.contents[0].uri == "reqvire://workspace/status" and .result.contents[0].mimeType == "application/json"' "resources/read returns JSON resource content"
assert_jq_line "$DEFAULT_OUTPUT" 10 '.error.code == -32602' "format fix is rejected by default schema"
assert_jq_line "$DEFAULT_OUTPUT" 11 '.result.structuredContent.concept_references[0].label == "Access Token" and .result.structuredContent.concept_references[0].target == "#access-token"' "read_element returns concept references"
assert_jq_line "$DEFAULT_OUTPUT" 12 '.result.structuredContent.files[]?.elements[] | select(.name=="MCP Access Token Ontology") | .ontology.ontology.content | contains("testonto:AccessToken")' "search returns ontology ADT content"
assert_jq_line "$DEFAULT_OUTPUT" 13 '.result.structuredContent.format == "turtle" and .result.structuredContent.semantic_layer == "ontologies" and (.result.structuredContent.content | contains("testonto:AccessToken")) and (.result.structuredContent.content | contains("testonto:AccessTokenShape") | not)' "semantic ontologies tool returns ontology-only Turtle content"
assert_jq_line "$DEFAULT_OUTPUT" 13 '.result.structuredContent.content | contains("testonto:AccessToken rdfs:isDefinedBy <https://example.test/ontology>")' "semantic ontologies tool returns generated isDefinedBy edge for authored ontology term"
assert_jq_line "$DEFAULT_OUTPUT" 13 '.result.structuredContent.summary.ontology_blocks >= 1 and .result.structuredContent.summary.shape_blocks == 0' "ontologies tool returns ontology-only semantic index summary"
assert_jq_line "$DEFAULT_OUTPUT" 13 '.result.structuredContent.blocks[] | select(.source_name=="MCP Access Token Ontology" and .kind=="ontology")' "ontologies tool returns ontology block metadata"
assert_jq_line "$DEFAULT_OUTPUT" 13 '.result.structuredContent.include_external == false and (.result.structuredContent.content | contains("MCP external resource") | not) and (.result.structuredContent.content | contains("MCP external code datatype") | not) and (.result.structuredContent.content | contains("MCP JSON-LD external resource") | not) and (.result.structuredContent.content | contains("MCP RDF/XML external resource") | not)' "semantic ontologies default excludes external source triples"
assert_jq_line "$DEFAULT_OUTPUT" 13 '.result.structuredContent.include_external == false and (.result.structuredContent.ontology_declarations | tostring | contains("mcp-external") | not)' "semantic ontologies default excludes external declarations"
assert_jq_line "$DEFAULT_OUTPUT" 13 '.result.structuredContent.graph_layers | type == "array" and length >= 6 and (any(.[]; .role=="authored-ontology" and .included == true)) and (any(.[]; .role=="raw-external-source" and .included == false))' "semantic ontologies returns active graph layer metadata"
assert_jq_line "$DEFAULT_OUTPUT" 14 '.result.structuredContent.format == "jsonld" and (.result.structuredContent.jsonld | type == "array") and (.result.structuredContent.jsonld | length) > 0' "ontologies tool returns JSON-LD semantic content"
assert_jq_line "$DEFAULT_OUTPUT" 15 '.result.structuredContent.semantic_layer == "graph" and .result.structuredContent.full == true and (.result.structuredContent.content | contains("reqvire:conceptReference")) and (.result.structuredContent.content | contains("urn:reqvire:element:mcp-semantic-requirement")) and (.result.structuredContent.content | contains("reqvire:OntologyProjectionGraph"))' "semantic graph tool returns full model context triples and ontology projection facts"
assert_jq_line "$DEFAULT_OUTPUT" 16 '.result.structuredContent.result_type == "select" and .result.structuredContent.full == true and .result.structuredContent.row_count == 1 and .result.structuredContent.variables == ["requirement","verification","relation"]' "sparql tool returns SELECT result metadata"
assert_jq_line "$DEFAULT_OUTPUT" 16 '.result.structuredContent.bindings[0].requirement.iri == "urn:reqvire:element:mcp-semantic-requirement" and .result.structuredContent.bindings[0].verification.iri == "urn:reqvire:element:mcp-semantic-verification" and (.result.structuredContent.bindings[0].relation.iri | startswith("urn:reqvire:model-relation:"))' "sparql tool queries normalized relation-family facts from built semantic store"
assert_jq_line "$DEFAULT_OUTPUT" 16 '.result.structuredContent.summary.ontology_blocks >= 1 and (.result.structuredContent.model_fingerprint | type == "string")' "sparql tool returns semantic summary and model fingerprint"
assert_jq_line "$DEFAULT_OUTPUT" 17 '.result.isError == true and (.result.structuredContent.error.message | contains("Invalid SPARQL query"))' "invalid sparql returns MCP tool error"
assert_jq_line "$DEFAULT_OUTPUT" 18 '.result.structuredContent.prefixes[] | select(.prefix=="testonto" and .namespace=="https://example.test/ontology#") | .source.content == "Access token ontology for MCP semantic evidence."' "semantic prefixes returns ontology-defined namespace with source prose content"
assert_jq_line "$DEFAULT_OUTPUT" 18 '.result.structuredContent.prefixes[] | select(.prefix=="testonto") | .ontology_base == "https://example.test/ontology" and .term_namespace == "https://example.test/ontology#" and .ontology_document_iri == "https://example.test/ontology"' "semantic prefixes returns ontology base and term namespace"
assert_jq_line "$DEFAULT_OUTPUT" 18 '.result.structuredContent.prefixes[] | select(.prefix=="testonto") | .source.element_identifier == "specifications/Requirements.md#mcp-access-token-ontology" and .source.element_name == "MCP Access Token Ontology" and (.source.file_path | endswith("specifications/Requirements.md")) and (.source.line_number | type == "number")' "semantic prefixes returns source element provenance"
assert_jq_line "$DEFAULT_OUTPUT" 18 '.result.structuredContent.prefixes[] | select(.prefix=="concept" and .namespace=="https://example.test/concepts#") | .concept_schemes[] | select(.scheme_element_identifier=="specifications/Requirements.md#mcp-concepts" and .scheme_element_name=="MCP Concepts" and .scheme_iri=="https://example.test/concepts#MCPConcepts")' "semantic prefixes returns concept scheme namespace context"
assert_jq_line "$DEFAULT_OUTPUT" 18 '.result.structuredContent.sparql_prefix_block | contains("PREFIX testonto: <https://example.test/ontology#>")' "semantic prefixes returns SPARQL prefix block"
assert_jq_line "$DEFAULT_OUTPUT" 18 '.result.structuredContent.prefixes[] | select(.prefix=="testonto") | (.source.content | contains("@prefix") | not)' "semantic prefixes source content omits Turtle prefix block"
assert_jq_line "$DEFAULT_OUTPUT" 18 '.result.structuredContent.include_external == false and ([.result.structuredContent.prefixes[].prefix] | index("ext") == null)' "semantic prefixes default excludes external prefixes"
assert_jq_line "$DEFAULT_OUTPUT" 19 '.result.structuredContent.semantic_layer == "shapes" and .result.structuredContent.summary.ontology_blocks == 0 and .result.structuredContent.summary.shape_blocks >= 1 and (.result.structuredContent.content | contains("testonto:AccessTokenShape")) and (.result.structuredContent.content | contains("owl:Ontology") | not)' "semantic shapes tool returns SHACL without ontology declarations"
assert_jq_line "$DEFAULT_OUTPUT" 20 '.result.structuredContent.semantic_layer == "concepts" and .result.structuredContent.include_mappings == true and (.result.structuredContent.content | contains("concept:AccessToken")) and (.result.structuredContent.content | contains("reqvire:mapsToConcept")) and (.result.structuredContent.content | contains("testonto:AccessTokenShape") | not) and (.result.structuredContent.external_blocks | length == 0)' "semantic concepts tool returns SKOS concepts and optional mappings"
assert_jq_line "$DEFAULT_OUTPUT" 20 '.result.structuredContent.summary.ontology_blocks == 0 and .result.structuredContent.summary.total_blocks >= 1 and all(.result.structuredContent.blocks[]; .kind == "concepts") and (.result.structuredContent.ontology_declarations | length == 0)' "semantic concepts tool returns native concept block provenance"
assert_jq_line "$DEFAULT_OUTPUT" 20 '.result.structuredContent.concepts[] | select(.iri=="https://example.test/concepts#MCPConcepts" and .kind=="concept-scheme" and .generated_from_markdown == true and .pref_label == "MCP Concepts" and .scheme_iri=="https://example.test/concepts#MCPConcepts" and .namespace_base=="https://example.test/concepts" and .namespace_prefix=="concept" and .namespace_iri=="https://example.test/concepts#" and .source_element_identifier=="specifications/Requirements.md#mcp-concepts" and .source_element_type=="concept-scheme" and .source_element.element_type=="concept-scheme")' "semantic concepts tool returns structured native concept scheme data"
assert_jq_line "$DEFAULT_OUTPUT" 20 '.result.structuredContent.concepts[] | select(.iri=="https://example.test/concepts#AccessToken" and .kind=="concept" and .generated_from_markdown == true and .pref_label == "Access Token" and .concept_iri=="https://example.test/concepts#AccessToken" and .scheme_iri=="https://example.test/concepts#MCPConcepts" and .scheme_element_identifier=="specifications/Requirements.md#mcp-concepts" and .namespace_base=="https://example.test/concepts" and .namespace_prefix=="concept" and (.in_scheme | index("concept:MCPConcepts") != null) and .source_element_identifier=="specifications/Requirements.md#access-token" and .source_element_type=="concept" and .source_element.element_type=="concept")' "semantic concepts tool returns structured native concept data"
assert_jq_line "$DEFAULT_OUTPUT" 21 '.result.structuredContent.section == "all" and .result.structuredContent.summary.relation_families >= 1 and (.result.structuredContent.prefixes[] | select(.prefix=="testonto")) and (.result.structuredContent.sparql_prefix_block | contains("PREFIX testonto: <https://example.test/ontology#>"))' "semantic vocabulary all section returns counts and prefixes"
assert_jq_line "$DEFAULT_OUTPUT" 21 '.result.structuredContent.include_external == false and ([.result.structuredContent.prefixes[].prefix] | index("ext") == null)' "semantic vocabulary default excludes external prefixes"
assert_jq_line "$DEFAULT_OUTPUT" 22 '.result.structuredContent.section == "relation_families" and .result.structuredContent.items[0].raw_relations and (.result.structuredContent.items[0] | has("forward_property")) and .result.structuredContent.paging.has_more == true and (.result.structuredContent.paging.next_cursor | type == "string")' "semantic vocabulary relation families are paged with normalized properties"
assert_jq_line "$DEFAULT_OUTPUT" 23 '.result.structuredContent.section == "query_patterns" and (.result.structuredContent.items[] | select(.id=="verified_requirements" and (.sparql | contains("requirementVerifiedByVerification")))) and (.result.structuredContent.prefixes[] | select(.prefix=="testonto"))' "semantic vocabulary query patterns include SPARQL examples and prefixes"
assert_jq_line "$DEFAULT_OUTPUT" 24 '[.result.prompts[].name] | index("reqvire.semantic.query") != null and index("reqvire.semantic.verification_search") != null and index("reqvire.workflow.explore_model") != null and index("reqvire.workflow.verify_coverage") != null' "prompts/list includes semantic and regular Reqvire prompts"
assert_jq_line "$DEFAULT_OUTPUT" 24 '.result.prompts[] | select(.name=="reqvire.semantic.query") | .title == "Reqvire Semantic Query" and (.arguments[] | select(.name=="question"))' "prompts/list returns prompt metadata and arguments"
assert_jq_line "$DEFAULT_OUTPUT" 25 '.result.messages[0].role == "user" and (.result.messages[0].content.text | contains("reqvire.semantic.vocabulary") and contains("reqvire.semantic.prefixes") and contains("reqvire.semantic.sparql") and contains("ontology_document") and contains("Client arguments") and contains("used external subset") and contains("raw full external ontology dependencies") and contains("o-kernel") and contains("external-used-subset"))' "prompts/get returns semantic query guidance"
assert_jq_line "$DEFAULT_OUTPUT" 26 '.result.messages[0].role == "user" and (.result.messages[0].content.text | contains("reqvire.workspace_status") and contains("reqvire.search") and contains("reqvire.read_element"))' "prompts/get returns regular Reqvire workflow guidance"
assert_jq_line "$DEFAULT_OUTPUT" 27 '.error.code == -32602 and (.error.data.message | contains("Unknown MCP prompt"))' "unknown prompt returns protocol error"
assert_jq_line "$DEFAULT_OUTPUT" 28 '.result.structuredContent.include_external == true and (.result.structuredContent.content | contains("MCP external code datatype")) and (.result.structuredContent.content | contains("ext:ExternalCode")) and (.result.structuredContent.content | contains("MCP external resource") | not) and (.result.structuredContent.content | contains("MCP JSON-LD external resource") | not) and (.result.structuredContent.content | contains("MCP RDF/XML external resource") | not) and (.result.structuredContent.content | contains("ext:ExternalResource") | not) and (.result.structuredContent.content | contains("jsonext:JsonExternalResource") | not) and (.result.structuredContent.content | contains("rdfext:RdfExternalResource") | not)' "semantic ontologies include_external materializes only used external subset triples"
assert_jq_line "$DEFAULT_OUTPUT" 28 '.result.structuredContent.include_external == true and (.result.structuredContent.content | contains("ext:ExternalCode rdfs:isDefinedBy") | not)' "semantic ontologies include_external does not generate isDefinedBy for external terms"
assert_jq_line "$DEFAULT_OUTPUT" 28 '.result.structuredContent.include_external == true and (.result.structuredContent.ontology_declarations["https://example.test/mcp-external#ExternalCode"][] | select(.external == true))' "semantic ontologies include_external marks used external declaration"
assert_jq_line "$DEFAULT_OUTPUT" 28 '.result.structuredContent.external_materialization == "used_subset" and .result.structuredContent.external_counts.declared_external_source_count == 4 and .result.structuredContent.external_counts.used_external_source_count == 2 and .result.structuredContent.external_counts.visible_external_term_declaration_count == 3 and .result.structuredContent.external_counts.materialized_external_triple_count > 0 and .result.structuredContent.external_counts.raw_external_triple_count > .result.structuredContent.external_counts.materialized_external_triple_count and (.result.structuredContent.external_blocks[] | select(.external_materialization == "used_subset" and .source == "reqvire:external-used-subset")) and (.result.structuredContent.ontology_declarations | tostring | contains("ExternalResource") | not)' "semantic ontologies include_external reports used-subset metadata and prunes unused declarations"
assert_jq_line "$DEFAULT_OUTPUT" 28 '.result.structuredContent.graph_layers | any(.[]; .role=="external-used-subset" and .included == true) and any(.[]; .role=="raw-external-source" and .included == false)' "semantic ontologies include_external response includes graph role metadata"
assert_jq_line "$DEFAULT_OUTPUT" 34 '.result.structuredContent.row_count == 1 and .result.structuredContent.bindings[0].s.iri == "https://example.test/ontology#AccessToken"' "sparql generated authored graph query hits expected ontology class"
assert_jq_line "$DEFAULT_OUTPUT" 35 '.result.structuredContent.full == true and .result.structuredContent.row_count >= 1 and any(.result.structuredContent.bindings[]; .projection.iri | startswith("urn:reqvire:ontology-projection:graph:"))' "sparql generated graph query returns ontology projection facts"
assert_jq_line "$DEFAULT_OUTPUT" 36 '.result.structuredContent.include_external == true and (.result.structuredContent.graph_layers | any(.[]; .role=="external-used-subset" and .included == true)) and .result.structuredContent.row_count >= 1 and .result.structuredContent.bindings[0].datatype.iri == "https://example.test/mcp-external#ExternalCode"' "sparql graph query uses external-used-subset for external terms"
assert_jq_line "$DEFAULT_OUTPUT" 37 '.result.structuredContent.row_count == 0 and .result.structuredContent.include_external == true' "sparql raw external graph is not exposed in public MCP store"
assert_jq_line "$DEFAULT_OUTPUT" 29 '.result.structuredContent.include_external == true and (.result.structuredContent.prefixes[] | select(.prefix=="ext" and .external == true and .namespace=="https://example.test/mcp-external#")) and ([.result.structuredContent.prefixes[].prefix] | index("jsonext") == null and index("rdfext") == null) and (.result.structuredContent.sparql_prefix_block | contains("PREFIX ext: <https://example.test/mcp-external#>")) and (.result.structuredContent.sparql_prefix_block | contains("PREFIX jsonext:") | not) and (.result.structuredContent.sparql_prefix_block | contains("PREFIX rdfext:") | not)' "semantic prefixes include_external includes only used external subset prefixes"
assert_jq_line "$DEFAULT_OUTPUT" 29 '.result.structuredContent.external_materialization == "used_subset" and .result.structuredContent.external_counts.declared_external_source_count == 4 and .result.structuredContent.external_counts.used_external_source_count == 2 and (.result.structuredContent.prefixes[] | select(.prefix=="ext" and .external_materialization=="used_subset" and .source_declaration=="declared"))' "semantic prefixes include_external distinguishes declared sources from used subset materialization"
assert_jq_line "$DEFAULT_OUTPUT" 30 '.result.structuredContent.include_external == true and .result.structuredContent.section == "properties" and (.result.structuredContent.items[] | select(.curie=="ext:ExternalCode" and .external == true and .label=="MCP external code datatype" and (.source.external == true)))' "semantic vocabulary include_external includes marked used external datatype"
assert_jq_line "$DEFAULT_OUTPUT" 30 '([.result.structuredContent.prefixes[].prefix] | index("ext") != null and index("jsonext") == null and index("rdfext") == null)' "semantic vocabulary include_external prefixes are limited to used external subset"
assert_jq_line "$DEFAULT_OUTPUT" 30 '.result.structuredContent.external_materialization == "used_subset" and .result.structuredContent.external_counts.visible_external_term_declaration_count == 3 and (.result.structuredContent.items[] | select(.curie=="ext:ExternalCode" and .materialized_in_used_subset == true and .external_materialization == "used_subset"))' "semantic vocabulary include_external reports used-subset metadata on external terms"
assert_jq_line "$DEFAULT_OUTPUT" 31 '.result.structuredContent.include_external == false and .result.structuredContent.row_count == 0' "semantic sparql default excludes external subset triples"
assert_jq_line "$DEFAULT_OUTPUT" 32 '.result.structuredContent.include_external == true and .result.structuredContent.row_count == 1 and .result.structuredContent.bindings[0].datatype.iri == "https://example.test/mcp-external#ExternalCode"' "semantic sparql include_external queries used external subset triples"
assert_jq_line "$DEFAULT_OUTPUT" 32 '.result.structuredContent.external_materialization == "used_subset" and .result.structuredContent.external_counts.used_external_source_count == 2 and .result.structuredContent.external_counts.materialized_external_triple_count > 0' "semantic sparql include_external reports used-subset metadata"
assert_jq_line "$DEFAULT_OUTPUT" 33 '.result.structuredContent.semantic_layer == "graph" and .result.structuredContent.include_external == true and (.result.structuredContent.content | contains("MCP external code datatype")) and (.result.structuredContent.content | contains("testonto:AccessTokenShape"))' "semantic graph include_external materializes used external subset with authored shapes"
assert_jq_line "$DEFAULT_OUTPUT" 38 '.result.structuredContent.include_external == true and .result.structuredContent.section == "classes" and .result.structuredContent.paging.total == 0 and (.result.structuredContent.items | length) == 0 and .result.structuredContent.external_materialization == "used_subset" and .result.structuredContent.external_counts.visible_external_term_declaration_count == 3 and ([.result.structuredContent.prefixes[].prefix] | index("skos") != null)' "semantic vocabulary include_external excludes external classes while reporting used SKOS subset metadata"
assert_jq_line "$DEFAULT_OUTPUT" 39 '.result.structuredContent.include_external == true and .result.structuredContent.row_count == 0 and .result.structuredContent.external_materialization == "used_subset" and .result.structuredContent.external_counts.materialized_external_triple_count > 0' "semantic sparql include_external cannot query unused raw external classes"
assert_jq_line "$DEFAULT_OUTPUT" 40 '.result.structuredContent.include_external == false and .result.structuredContent.ontology_document_filter == "https://example.test/ontology" and .result.structuredContent.section == "classes" and (.result.structuredContent.items[] | select(.iri=="https://example.test/ontology#AccessToken" and .ontology_document=="https://example.test/ontology" and .external == false))' "semantic vocabulary filters authored classes by ontology document"
assert_jq_line "$DEFAULT_OUTPUT" 41 '.result.structuredContent.include_external == true and .result.structuredContent.ontology_document_filter == "https://example.test/mcp-external" and .result.structuredContent.section == "properties" and (.result.structuredContent.items[] | select(.curie=="ext:ExternalCode" and .ontology_document=="https://example.test/mcp-external" and .external == true and .external_materialization=="used_subset"))' "semantic vocabulary filters used external subset properties by ontology document"
assert_jq_line "$DEFAULT_OUTPUT" 42 '.result.structuredContent.section == "concepts" and (.result.structuredContent.items[] | select(.iri=="https://example.test/concepts#MCPConcepts" and .kind=="concept-scheme" and .generated_from_markdown == true and .pref_label == "MCP Concepts" and (.definition | contains("Concept scheme for MCP semantic evidence.")) and (.top_concepts | index("concept:AccessToken") != null) and .scheme_element_identifier=="specifications/Requirements.md#mcp-concepts" and .source_element_type=="concept-scheme"))' "semantic vocabulary concepts section exposes generated concept scheme data and provenance"
assert_jq_line "$DEFAULT_OUTPUT" 42 '.result.structuredContent.section == "concepts" and (.result.structuredContent.items[] | select(.iri=="https://example.test/concepts#AccessToken" and .kind=="concept" and .generated_from_markdown == true and .pref_label == "Access Token" and (.definition | contains("A credential-like token used to authorize access.")) and .concept_iri=="https://example.test/concepts#AccessToken" and .scheme_iri=="https://example.test/concepts#MCPConcepts" and .scheme_element_name=="MCP Concepts" and (.in_scheme | index("concept:MCPConcepts") != null) and .source_element_identifier=="specifications/Requirements.md#access-token" and .source_element_type=="concept"))' "semantic vocabulary concepts section exposes generated concept data and provenance"
assert_jq_line "$DEFAULT_OUTPUT" 43 '.result.structuredContent.count >= 1 and (.result.structuredContent.concept_schemes[] | select(.iri=="https://example.test/concepts#MCPConcepts" and .kind=="concept-scheme" and .namespace_base=="https://example.test/concepts" and .namespace_prefix=="concept"))' "concept schemes list tool returns standalone scheme namespace context"
assert_jq_line "$DEFAULT_OUTPUT" 44 '.result.structuredContent.count >= 1 and all(.result.structuredContent.concepts[]; .scheme_iri=="https://example.test/concepts#MCPConcepts") and (.result.structuredContent.concepts[] | select(.iri=="https://example.test/concepts#AccessToken" and .kind=="concept" and .pref_label=="Access Token"))' "concepts list tool returns scheme-filtered native concepts"
assert_jq_line "$DEFAULT_OUTPUT" 45 '.result.structuredContent.concept.iri == "https://example.test/concepts#AccessToken" and .result.structuredContent.concept.kind == "concept" and .result.structuredContent.concept.source_element_identifier == "specifications/Requirements.md#access-token"' "concept get tool resolves generated concept IRI"
assert_jq_line "$DEFAULT_OUTPUT" 46 '.result.structuredContent.count >= 1 and (.result.structuredContent.mappings[] | select(.source_iri=="https://example.test/ontology#AccessToken" and .source_curie=="testonto:AccessToken" and .target_iri=="https://example.test/concepts#AccessToken" and .target_curie=="concept:AccessToken" and .predicate_curie=="reqvire:mapsToConcept" and .target_concept.kind=="concept" and .target_concept.source_element_identifier=="specifications/Requirements.md#access-token"))' "concept mappings list tool returns native bridge inventory"

UNSUPPORTED_PORT="$(pick_port)"
UNSUPPORTED_OUTPUT_PREFIX="$TEST_DIR/output/mcp-unsupported-protocol"
start_http_mcp "$UNSUPPORTED_PORT" "$UNSUPPORTED_OUTPUT_PREFIX"
trap stop_http_mcp EXIT
wait_for_http_mcp "$UNSUPPORTED_PORT" "$TEST_DIR/output/mcp-unsupported-startup-init.json" || fail "unsupported protocol test MCP HTTP server did not start" "${UNSUPPORTED_OUTPUT_PREFIX}.stderr"
UNSUPPORTED_HEADER_STATUS="$(curl -sS -o "$TEST_DIR/output/mcp-unsupported-protocol.txt" -w "%{http_code}" \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json, text/event-stream' \
  -H 'Mcp-Protocol-Version: 1900-01-01' \
  --data "$(tools_list_request)" \
  "http://127.0.0.1:${UNSUPPORTED_PORT}/mcp")"
stop_http_mcp
trap - EXIT
if [ "$UNSUPPORTED_HEADER_STATUS" != "400" ]; then
  fail "unsupported MCP-Protocol-Version header should be rejected" "$TEST_DIR/output/mcp-unsupported-protocol.txt"
fi

SIZE_OUTPUT="$TEST_DIR/output/mcp-size-estimates.jsonl"
SIZE_PORT="$(pick_port)"
SIZE_OUTPUT_PREFIX="$TEST_DIR/output/mcp-size-estimates"
start_http_mcp "$SIZE_PORT" "$SIZE_OUTPUT_PREFIX" --with-size-estimates
trap stop_http_mcp EXIT
wait_for_http_mcp "$SIZE_PORT" "$TEST_DIR/output/mcp-size-estimates-init.json" || fail "size-estimates MCP HTTP server did not start" "${SIZE_OUTPUT_PREFIX}.stderr"
run_http_mcp_sequence "$SIZE_PORT" "$SIZE_OUTPUT" \
  "$(init_request)" \
  "$(workspace_status_request)" \
  "$(read_element_request)" \
  "$(model_request)" \
  "$(resource_read_request)" || fail "size-estimates MCP HTTP request sequence failed"
stop_http_mcp
trap - EXIT

assert_jq_line "$SIZE_OUTPUT" 2 '.result.structuredContent.size_estimates_enabled == true' "workspace_status reports size estimates enabled"
assert_jq_line "$SIZE_OUTPUT" 3 '.result.structuredContent.size_estimate.content_bytes >= 0 and .result.structuredContent.size_estimate.rendered_context_bytes > 0 and .result.structuredContent.size_estimate.estimated_tokens > 0' "read_element includes size estimate when enabled"
assert_jq_line "$SIZE_OUTPUT" 4 '[.result.structuredContent.elements[]? | .. | objects | select(has("identifier") and has("name"))] as $elements | ($elements | length) > 0 and all($elements[]; (.size_estimate.content_bytes | type == "number") and (.size_estimate.rendered_context_bytes | type == "number") and (.size_estimate.estimated_tokens | type == "number"))' "model tool includes size estimates when enabled"
assert_jq_line "$SIZE_OUTPUT" 5 '.result.contents[0].text | fromjson | .size_estimates_enabled == true' "workspace status resource reports size estimates enabled"

DRY_RUN_OUTPUT="$TEST_DIR/output/mcp-mutation-dry-run.jsonl"
ADD_CONTENT="$(< "$TEST_SCRIPT_DIR/fixtures/mcp-added-requirement.md")"
DRY_RUN_PORT="$(pick_port)"
DRY_RUN_OUTPUT_PREFIX="$TEST_DIR/output/mcp-mutation-dry-run"
start_http_mcp "$DRY_RUN_PORT" "$DRY_RUN_OUTPUT_PREFIX" --enable-mutations
trap stop_http_mcp EXIT
wait_for_http_mcp "$DRY_RUN_PORT" "$TEST_DIR/output/mcp-mutation-dry-run-init.json" || fail "dry-run MCP HTTP server did not start" "${DRY_RUN_OUTPUT_PREFIX}.stderr"
run_http_mcp_sequence "$DRY_RUN_PORT" "$DRY_RUN_OUTPUT" \
  "$(init_request)" \
  "$(tools_list_request)" \
  "$(jq -n -c --arg content "$ADD_CONTENT" '{jsonrpc:"2.0",id:3,method:"tools/call",params:{name:"reqvire.add_element",arguments:{file:"specifications/Requirements.md",content:$content,dry_run:true}}}')" || fail "dry-run MCP HTTP request sequence failed"
stop_http_mcp
trap - EXIT

assert_jq_line "$DRY_RUN_OUTPUT" 2 '[.result.tools[].name] | index("reqvire.add_element") != null and index("reqvire.link") != null' "mutation mode advertises mutation tools"
assert_jq_line "$DRY_RUN_OUTPUT" 2 '.result.tools[] | select(.name=="reqvire.add_element") | .annotations.readOnlyHint == false' "mutation tools are non-read-only"
assert_jq_line "$DRY_RUN_OUTPUT" 2 '.result.tools[] | select(.name=="reqvire.format") | .inputSchema.properties.fix.type == "boolean"' "mutation mode exposes format fix argument"
assert_jq_line "$DRY_RUN_OUTPUT" 3 '.result.structuredContent.dry_run == true and (.result.structuredContent.diffs | length) >= 1' "dry-run mutation returns diffs without execution"
if grep -q "MCP Added Requirement" "$TEST_DIR/specifications/Requirements.md"; then
  fail "dry-run mutation modified the fixture file"
fi

MUTATION_OUTPUT="$TEST_DIR/output/mcp-mutation-execute.jsonl"
MUTATION_PORT="$(pick_port)"
MUTATION_OUTPUT_PREFIX="$TEST_DIR/output/mcp-mutation-execute"
start_http_mcp "$MUTATION_PORT" "$MUTATION_OUTPUT_PREFIX" --enable-mutations
trap stop_http_mcp EXIT
wait_for_http_mcp "$MUTATION_PORT" "$TEST_DIR/output/mcp-mutation-execute-init.json" || fail "mutation MCP HTTP server did not start" "${MUTATION_OUTPUT_PREFIX}.stderr"
run_http_mcp_sequence "$MUTATION_PORT" "$MUTATION_OUTPUT" \
  "$(init_request)" \
  "$(jq -n -c --arg content "$ADD_CONTENT" '{jsonrpc:"2.0",id:4,method:"tools/call",params:{name:"reqvire.add_element",arguments:{file:"specifications/Requirements.md",content:$content,dry_run:false}}}')" \
  "$(jq -n -c '{jsonrpc:"2.0",id:5,method:"tools/call",params:{name:"reqvire.read_element",arguments:{name:"MCP Added Requirement"}}}')" || fail "mutation MCP HTTP request sequence failed"
stop_http_mcp
trap - EXIT

assert_jq_line "$MUTATION_OUTPUT" 2 '.result.structuredContent.dry_run == false and (.result.structuredContent.diffs | length) >= 1' "executing mutation returns persisted diffs"
grep -q "MCP Added Requirement" "$TEST_DIR/specifications/Requirements.md" || fail "executing mutation did not update the fixture file"
assert_jq_line "$MUTATION_OUTPUT" 3 '.result.structuredContent.name == "MCP Added Requirement"' "post-mutation read observes refreshed model state"

HTTP_PORT="$(pick_port)"
HTTP_OUTPUT_PREFIX="$TEST_DIR/output/mcp-http"
start_http_mcp "$HTTP_PORT" "$HTTP_OUTPUT_PREFIX"
trap stop_http_mcp EXIT
wait_for_http_mcp "$HTTP_PORT" "$TEST_DIR/output/mcp-http-init.json" || fail "HTTP MCP server did not start" "${HTTP_OUTPUT_PREFIX}.stderr"

http_mcp_call "$HTTP_PORT" "$(tools_list_request)" "$TEST_DIR/output/mcp-http-tools.json" || fail "HTTP tools/list request failed"
jq -r '.result.tools[].name' "$TEST_DIR/output/mcp-http-tools.json" > "$TEST_DIR/output/mcp-http-tools.txt"
if ! diff -u "$TEST_SCRIPT_DIR/expected/http-read-tools.txt" "$TEST_DIR/output/mcp-http-tools.txt"; then
  fail "HTTP default tools/list does not match expected read-only tool set"
fi
jq -e '[.result.tools[].name] | index("reqvire.search") != null and index("reqvire.add_element") == null' "$TEST_DIR/output/mcp-http-tools.json" >/dev/null \
  || fail "HTTP default tools/list should expose read-only tools and omit mutation tools" "$TEST_DIR/output/mcp-http-tools.json"
jq -e 'all(.result.tools[]; (.inputSchema.type=="object") and (.outputSchema|type=="object") and (.annotations|type=="object"))' "$TEST_DIR/output/mcp-http-tools.json" >/dev/null \
  || fail "HTTP tools/list should expose schema and annotation fields" "$TEST_DIR/output/mcp-http-tools.json"

http_mcp_call "$HTTP_PORT" "$(resources_list_request)" "$TEST_DIR/output/mcp-http-resources.json" || fail "HTTP resources/list request failed"
jq -e '[.result.resources[].uri] | index("reqvire://workspace/status") != null' "$TEST_DIR/output/mcp-http-resources.json" >/dev/null \
  || fail "HTTP resources/list should expose workspace status" "$TEST_DIR/output/mcp-http-resources.json"

http_mcp_call "$HTTP_PORT" "$(workspace_status_request)" "$TEST_DIR/output/mcp-http-status.json" || fail "HTTP workspace_status request failed"
jq -e '.result.structuredContent.model.valid == true and (.result.structuredContent.model.fingerprint | type == "string")' "$TEST_DIR/output/mcp-http-status.json" >/dev/null \
  || fail "HTTP workspace_status should return structured model state" "$TEST_DIR/output/mcp-http-status.json"

http_mcp_call "$HTTP_PORT" "$(workspace_status_request)" "$TEST_DIR/output/mcp-http-loopback-origin.json" -H "Origin: http://localhost:9999" \
  || fail "HTTP loopback Origin should be accepted"
jq -e '.result.structuredContent.workspace_root | type == "string"' "$TEST_DIR/output/mcp-http-loopback-origin.json" >/dev/null \
  || fail "HTTP loopback Origin response should execute request" "$TEST_DIR/output/mcp-http-loopback-origin.json"

INVALID_ORIGIN_STATUS="$(curl -sS -o "$TEST_DIR/output/mcp-http-invalid-origin.txt" -w "%{http_code}" \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json, text/event-stream' \
  -H "Mcp-Protocol-Version: ${MCP_PROTOCOL_VERSION}" \
  -H 'Origin: https://evil.example' \
  --data "$(workspace_status_request)" \
  "http://127.0.0.1:${HTTP_PORT}/mcp")"
if [ "$INVALID_ORIGIN_STATUS" != "403" ]; then
  fail "HTTP non-loopback Origin should be rejected before tool execution" "$TEST_DIR/output/mcp-http-invalid-origin.txt"
fi

NULL_ORIGIN_STATUS="$(curl -sS -o "$TEST_DIR/output/mcp-http-null-origin.txt" -w "%{http_code}" \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json, text/event-stream' \
  -H "Mcp-Protocol-Version: ${MCP_PROTOCOL_VERSION}" \
  -H 'Origin: null' \
  --data "$(workspace_status_request)" \
  "http://127.0.0.1:${HTTP_PORT}/mcp")"
if [ "$NULL_ORIGIN_STATUS" != "403" ]; then
  fail "HTTP null Origin should be rejected before tool execution" "$TEST_DIR/output/mcp-http-null-origin.txt"
fi

GET_STATUS="$(curl -sS -o "$TEST_DIR/output/mcp-http-get.txt" -w "%{http_code}" "http://127.0.0.1:${HTTP_PORT}/mcp")"
if [ "$GET_STATUS" != "405" ]; then
  fail "HTTP GET without SSE streaming should return method-not-allowed" "$TEST_DIR/output/mcp-http-get.txt"
fi

stop_http_mcp

HTTP_MUTATION_PORT="$(pick_port)"
HTTP_MUTATION_OUTPUT_PREFIX="$TEST_DIR/output/mcp-http-mutations"
start_http_mcp "$HTTP_MUTATION_PORT" "$HTTP_MUTATION_OUTPUT_PREFIX" --enable-mutations
wait_for_http_mcp "$HTTP_MUTATION_PORT" "$TEST_DIR/output/mcp-http-mutations-init.json" || fail "HTTP mutation MCP server did not start" "${HTTP_MUTATION_OUTPUT_PREFIX}.stderr"

http_mcp_call "$HTTP_MUTATION_PORT" "$(tools_list_request)" "$TEST_DIR/output/mcp-http-mutation-tools.json" || fail "HTTP mutation tools/list request failed"
jq -r '.result.tools[].name' "$TEST_DIR/output/mcp-http-mutation-tools.json" > "$TEST_DIR/output/mcp-http-mutation-tools.txt"
if ! diff -u "$TEST_SCRIPT_DIR/expected/http-mutation-tools.txt" "$TEST_DIR/output/mcp-http-mutation-tools.txt"; then
  fail "HTTP mutation tools/list does not match expected mutation tool set"
fi
jq -e '[.result.tools[].name] | index("reqvire.add_element") != null and index("reqvire.link") != null' "$TEST_DIR/output/mcp-http-mutation-tools.json" >/dev/null \
  || fail "HTTP mutation mode should advertise mutation tools" "$TEST_DIR/output/mcp-http-mutation-tools.json"

HTTP_CONCURRENT_CONTENT_A="$(< "$TEST_SCRIPT_DIR/fixtures/http-concurrent-requirement-a.md")"
HTTP_CONCURRENT_CONTENT_B="$(< "$TEST_SCRIPT_DIR/fixtures/http-concurrent-requirement-b.md")"

HTTP_CONCURRENT_REQUEST_A="$(jq -n -c --arg content "$HTTP_CONCURRENT_CONTENT_A" '{jsonrpc:"2.0",id:11,method:"tools/call",params:{name:"reqvire.add_element",arguments:{file:"specifications/Requirements.md",content:$content,dry_run:false}}}')"
HTTP_CONCURRENT_REQUEST_B="$(jq -n -c --arg content "$HTTP_CONCURRENT_CONTENT_B" '{jsonrpc:"2.0",id:12,method:"tools/call",params:{name:"reqvire.add_element",arguments:{file:"specifications/Requirements.md",content:$content,dry_run:false}}}')"

http_mcp_call "$HTTP_MUTATION_PORT" "$HTTP_CONCURRENT_REQUEST_A" "$TEST_DIR/output/mcp-http-concurrent-a.json" &
HTTP_CURL_PID_A=$!
http_mcp_call "$HTTP_MUTATION_PORT" "$HTTP_CONCURRENT_REQUEST_B" "$TEST_DIR/output/mcp-http-concurrent-b.json" &
HTTP_CURL_PID_B=$!
wait "$HTTP_CURL_PID_A" || fail "first concurrent HTTP mutation failed" "$TEST_DIR/output/mcp-http-concurrent-a.json"
wait "$HTTP_CURL_PID_B" || fail "second concurrent HTTP mutation failed" "$TEST_DIR/output/mcp-http-concurrent-b.json"

jq -e '.result.structuredContent.dry_run == false' "$TEST_DIR/output/mcp-http-concurrent-a.json" >/dev/null \
  || fail "first concurrent HTTP mutation should execute" "$TEST_DIR/output/mcp-http-concurrent-a.json"
jq -e '.result.structuredContent.dry_run == false' "$TEST_DIR/output/mcp-http-concurrent-b.json" >/dev/null \
  || fail "second concurrent HTTP mutation should execute" "$TEST_DIR/output/mcp-http-concurrent-b.json"
grep '^### MCP HTTP Concurrent Requirement' "$TEST_DIR/specifications/Requirements.md" > "$TEST_DIR/output/mcp-http-concurrent-requirement-headings.txt"
if ! diff -u "$TEST_SCRIPT_DIR/expected/http-concurrent-requirement-headings.txt" "$TEST_DIR/output/mcp-http-concurrent-requirement-headings.txt"; then
  fail "serialized HTTP mutations should preserve both expected filesystem writes"
fi

http_mcp_call "$HTTP_MUTATION_PORT" "$(jq -n -c '{jsonrpc:"2.0",id:13,method:"tools/call",params:{name:"reqvire.read_element",arguments:{name:"MCP HTTP Concurrent Requirement A"}}}')" "$TEST_DIR/output/mcp-http-post-mutation-read.json" \
  || fail "HTTP post-mutation read request failed"
jq -e '.result.structuredContent.name == "MCP HTTP Concurrent Requirement A"' "$TEST_DIR/output/mcp-http-post-mutation-read.json" >/dev/null \
  || fail "HTTP post-mutation read should observe refreshed model state" "$TEST_DIR/output/mcp-http-post-mutation-read.json"

stop_http_mcp
trap - EXIT

INVALID_DIR="$(mktemp -d -t reqvire-mcp-invalid-XXXXXX)"
cp -a "$TEST_SCRIPT_DIR/fixtures/invalid-startup/." "$INVALID_DIR/"

set +e
INVALID_PORT="$(pick_port)"
(cd "$INVALID_DIR" && "$REQVIRE_BIN" mcp --host 127.0.0.1 --port "$INVALID_PORT") > "$TEST_DIR/output/mcp-invalid-startup.stdout" 2> "$TEST_DIR/output/mcp-invalid-startup.stderr" &
INVALID_PID=$!
for _ in $(seq 1 50); do
  if ! kill -0 "$INVALID_PID" >/dev/null 2>&1; then
    break
  fi
  sleep 0.1
done
if kill -0 "$INVALID_PID" >/dev/null 2>&1; then
  kill "$INVALID_PID" >/dev/null 2>&1 || true
fi
wait "$INVALID_PID"
INVALID_EXIT=$?
set +e

if [ "$INVALID_EXIT" -eq 0 ]; then
  fail "invalid model should prevent MCP startup"
fi
if ! grep -q "MCP startup validation failed" "$TEST_DIR/output/mcp-invalid-startup.stderr"; then
  fail "startup validation diagnostics should mention MCP startup validation" "$TEST_DIR/output/mcp-invalid-startup.stderr"
fi

exit 0
