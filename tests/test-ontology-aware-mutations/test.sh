#!/usr/bin/env bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

strip_trailing_blank_lines() {
  awk '
    {
      lines[++count] = $0
    }
    END {
      while (count > 0 && lines[count] == "") {
        count--
      }
      for (i = 1; i <= count; i++) {
        print lines[i]
      }
    }
  '
}

cat > "$TEST_DIR/specifications.md" <<'EOF'
# Elements

### Billing Ontology

Defines billing payload vocabulary.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/old-billing
  * ontology_prefix: oldbilling

#### Relations
  * derivedFrom: [Old Foundation Ontology](#old-foundation-ontology)
  * usedBy: [Billing Shape Contract](#billing-shape-contract)

#### Ontology
```turtle
@prefix oldbilling: <https://example.test/old-billing#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/old-billing> a owl:Ontology ;
  owl:imports <https://example.test/old-foundation> .
oldbilling:BillingPayload a owl:Class .
oldbilling:billingId a owl:DatatypeProperty .
```

---

### Billing Capability

Billing capability.

#### Metadata
  * type: capability

---

### Billing Requirement

The system shall validate billing payloads.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Billing Capability](#billing-capability)
  * constrainedBy: [Billing Shape Contract](#billing-shape-contract)

#### Concept References
  * Billing payload class: oldbilling:BillingPayload
  * Billing payload IRI: <https://example.test/old-billing#billingId>

---

### Billing Shape Contract

Defines billing payload shape.

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Billing Requirement](#billing-requirement)
  * use: [Billing Ontology](#billing-ontology)

#### Shapes
```turtle
@prefix oldbilling: <https://example.test/old-billing#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

oldbilling:BillingPayloadShape
  a sh:NodeShape ;
  sh:targetClass oldbilling:BillingPayload ;
  sh:property [
    sh:path oldbilling:billingId ;
    sh:minCount 1 ;
  ] .
```

---

### Old Foundation Ontology

Old imported ontology.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/old-foundation
  * ontology_prefix: oldfoundation

#### Ontology
```turtle
@prefix oldfoundation: <https://example.test/old-foundation#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/old-foundation> a owl:Ontology .
oldfoundation:FoundationThing a owl:Class .
```

---

### Platform Ontology

Platform imported ontology.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/platform
  * ontology_prefix: platform

#### Relations
  * derivedFrom: [Old Foundation Ontology](#old-foundation-ontology)

#### Ontology
```turtle
@prefix platform: <https://example.test/platform#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/platform> a owl:Ontology ;
  owl:imports <https://example.test/old-foundation> .
platform:PlatformThing a owl:Class .
```

---

### API Ontology

API ontology with an explicit base and a cross-boundary parent.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/api
  * ontology_prefix: api

#### Relations
  * derivedFrom: [Old Foundation Ontology](#old-foundation-ontology)

#### Ontology
```turtle
@prefix api: <https://example.test/api#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/api> a owl:Ontology ;
  owl:imports <https://example.test/old-foundation> .
api:ApiThing a owl:Class .
```
EOF

set +e
VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATE_EXIT=$?
set -e
if [ $VALIDATE_EXIT -ne 0 ]; then
  echo "FAILED: initial ontology-aware mutation fixture should validate"
  echo "$VALIDATE_OUTPUT"
  exit 1
fi

UPDATED_BILLING_ONTOLOGY='### Billing Ontology

Defines billing payload vocabulary after a base change.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/new-billing
  * ontology_prefix: newbilling

#### Relations
  * derivedFrom: [Old Foundation Ontology](#old-foundation-ontology)
  * usedBy: [Billing Shape Contract](#billing-shape-contract)

#### Ontology
```turtle
@prefix newbilling: <https://example.test/new-billing#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/new-billing> a owl:Ontology ;
  owl:imports <https://example.test/old-foundation> .
newbilling:BillingPayload a owl:Class .
newbilling:billingId a owl:DatatypeProperty .
```'

set +e
OVERRIDE_OUTPUT=$(cd "$TEST_DIR" && printf "%s\n" "$UPDATED_BILLING_ONTOLOGY" | "$REQVIRE_BIN" add specifications.md --override 2>&1)
OVERRIDE_EXIT=$?
set -e
if [ $OVERRIDE_EXIT -ne 0 ]; then
  echo "FAILED: ontology base override should rewrite dependent SHACL references"
  echo "$OVERRIDE_OUTPUT"
  exit 1
fi

if ! grep -Fq "@prefix newbilling: <https://example.test/new-billing#> ." "$TEST_DIR/specifications.md"; then
  echo "FAILED: rewritten SHACL prefix binding was not persisted"
  exit 1
fi
if ! grep -Fq "sh:targetClass newbilling:BillingPayload" "$TEST_DIR/specifications.md"; then
  echo "FAILED: SHACL target class CURIE was not rewritten"
  exit 1
fi
if ! grep -Fq "sh:path newbilling:billingId" "$TEST_DIR/specifications.md"; then
  echo "FAILED: SHACL path CURIE was not rewritten"
  exit 1
fi
if ! grep -Fq "Billing payload class: newbilling:BillingPayload" "$TEST_DIR/specifications.md"; then
  echo "FAILED: concept reference CURIE was not rewritten"
  exit 1
fi
if ! grep -Fq "Billing payload IRI: <https://example.test/new-billing#billingId>" "$TEST_DIR/specifications.md"; then
  echo "FAILED: concept reference IRI was not rewritten"
  exit 1
fi
if grep -Fq "oldbilling:BillingPayload" "$TEST_DIR/specifications.md"; then
  echo "FAILED: old billing CURIE reference remained after boundary rewrite"
  exit 1
fi

set +e
RELINK_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" relink "API Ontology" derivedFrom "Old Foundation Ontology" "Platform Ontology" 2>&1)
RELINK_EXIT=$?
set -e
if [ $RELINK_EXIT -ne 0 ]; then
  echo "FAILED: ontology derivedFrom relink should add required cross-boundary import"
  echo "$RELINK_OUTPUT"
  exit 1
fi

if ! grep -Fq "<https://example.test/api> <http://www.w3.org/2002/07/owl#imports> <https://example.test/platform> ." "$TEST_DIR/specifications.md"; then
  echo "FAILED: relink did not add required platform owl:imports statement"
  exit 1
fi

set +e
FINAL_VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
FINAL_VALIDATE_EXIT=$?
set -e
if [ $FINAL_VALIDATE_EXIT -ne 0 ]; then
  echo "FAILED: ontology-aware mutation fixture should validate after rewrites"
  echo "$FINAL_VALIDATE_OUTPUT"
  exit 1
fi

cat > "$TEST_DIR/specifications.md" <<'EOF'
# Elements

### Root Ontology

Root ontology.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/root
  * ontology_prefix: root

#### Ontology
```turtle
@prefix root: <https://example.test/root#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/root> a owl:Ontology .
root:RootThing a owl:Class .
```

---

### Child Ontology

Child ontology contribution.

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Root Ontology](#root-ontology)

#### Ontology
```turtle
@prefix root: <https://example.test/root#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
root:ChildThing a owl:Class .
```
EOF

set +e
INITIAL_VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
INITIAL_VALIDATE_EXIT=$?
set -e
if [ $INITIAL_VALIDATE_EXIT -ne 0 ]; then
  echo "FAILED: inherited boundary fixture should validate before conversion"
  echo "$INITIAL_VALIDATE_OUTPUT"
  exit 1
fi

CONVERT_CHILD_TO_BOUNDARY='### Child Ontology

Child ontology boundary.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/child
  * ontology_prefix: child

#### Relations
  * derivedFrom: [Root Ontology](#root-ontology)

#### Ontology
```turtle
@prefix child: <https://example.test/child#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/child> a owl:Ontology ;
  owl:imports <https://example.test/root> .
child:ChildThing a owl:Class .
```'

set +e
CONVERT_OUTPUT=$(cd "$TEST_DIR" && printf "%s\n" "$CONVERT_CHILD_TO_BOUNDARY" | "$REQVIRE_BIN" add specifications.md --override 2>&1)
CONVERT_EXIT=$?
set -e
if [ $CONVERT_EXIT -ne 0 ]; then
  echo "FAILED: converting a contributor into an explicit ontology boundary should succeed"
  echo "$CONVERT_OUTPUT"
  exit 1
fi

if ! grep -Fq "@prefix child: <https://example.test/child#> ." "$TEST_DIR/specifications.md"; then
  echo "FAILED: converted child boundary did not persist new prefix binding"
  exit 1
fi
if ! grep -Fq "<https://example.test/child> a owl:Ontology ;" "$TEST_DIR/specifications.md"; then
  echo "FAILED: converted child boundary did not persist owl:Ontology declaration"
  exit 1
fi
if ! grep -Fq "owl:imports <https://example.test/root>" "$TEST_DIR/specifications.md" && \
   ! grep -Fq "<http://www.w3.org/2002/07/owl#imports> <https://example.test/root>" "$TEST_DIR/specifications.md"; then
  echo "FAILED: converted child boundary did not persist required import"
  exit 1
fi

set +e
CONVERT_VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
CONVERT_VALIDATE_EXIT=$?
set -e
if [ $CONVERT_VALIDATE_EXIT -ne 0 ]; then
  echo "FAILED: converted child boundary should validate"
  echo "$CONVERT_VALIDATE_OUTPUT"
  exit 1
fi

CONVERT_CHILD_BACK='### Child Ontology

Child ontology contribution again.

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Root Ontology](#root-ontology)

#### Ontology
```turtle
@prefix root: <https://example.test/root#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
root:ChildThing a owl:Class .
```'

set +e
REVERT_OUTPUT=$(cd "$TEST_DIR" && printf "%s\n" "$CONVERT_CHILD_BACK" | "$REQVIRE_BIN" add specifications.md --override 2>&1)
REVERT_EXIT=$?
set -e
if [ $REVERT_EXIT -ne 0 ]; then
  echo "FAILED: converting the explicit boundary back into a contributor should succeed"
  echo "$REVERT_OUTPUT"
  exit 1
fi

if grep -Fq "owl:imports <https://example.test/root>" "$TEST_DIR/specifications.md" || \
   grep -Fq "<http://www.w3.org/2002/07/owl#imports> <https://example.test/root>" "$TEST_DIR/specifications.md"; then
  echo "FAILED: reverted child contributor should not keep a cross-boundary import"
  exit 1
fi
if ! grep -Fq "@prefix root: <https://example.test/root#> ." "$TEST_DIR/specifications.md"; then
  echo "FAILED: reverted child contributor lost inherited prefix binding"
  exit 1
fi

set +e
REVERT_VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
REVERT_VALIDATE_EXIT=$?
set -e
if [ $REVERT_VALIDATE_EXIT -ne 0 ]; then
  echo "FAILED: reverted child contributor should validate"
  echo "$REVERT_VALIDATE_OUTPUT"
  exit 1
fi

cp "$TEST_SCRIPT_DIR/fixtures/01-nested-child-boundary.txt" "$TEST_DIR/specifications.md"

set +e
GRANDCHILD_INITIAL_VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
GRANDCHILD_INITIAL_VALIDATE_EXIT=$?
set -e
if [ $GRANDCHILD_INITIAL_VALIDATE_EXIT -ne 0 ]; then
  echo "FAILED: nested inherited boundary fixture should validate before conversion"
  echo "$GRANDCHILD_INITIAL_VALIDATE_OUTPUT"
  exit 1
fi

CONVERT_GRANDCHILD_TO_BOUNDARY='### Grandchild Ontology

Grandchild ontology boundary.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/grandchild
  * ontology_prefix: grandchild

#### Relations
  * derivedFrom: [Child Ontology](#child-ontology)

#### Ontology
```turtle
@prefix grandchild: <https://example.test/grandchild#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/grandchild> a owl:Ontology ;
  owl:imports <https://example.test/root> .
grandchild:GrandchildThing a owl:Class .
```'

set +e
GRANDCHILD_CONVERT_OUTPUT=$(cd "$TEST_DIR" && printf "%s\n" "$CONVERT_GRANDCHILD_TO_BOUNDARY" | "$REQVIRE_BIN" add specifications.md --override 2>&1)
GRANDCHILD_CONVERT_EXIT=$?
set -e
if [ $GRANDCHILD_CONVERT_EXIT -ne 0 ]; then
  echo "FAILED: converting a grandchild contributor into an explicit ontology boundary should succeed"
  echo "$GRANDCHILD_CONVERT_OUTPUT"
  exit 1
fi

if ! diff -u <(strip_trailing_blank_lines < "$TEST_SCRIPT_DIR/expected/01-nested-child-boundary-after-grandchild-boundary.txt") <(strip_trailing_blank_lines < "$TEST_DIR/specifications.md"); then
  echo "FAILED: grandchild boundary rewrite does not match expected ontology hierarchy and imports"
  exit 1
fi

set +e
GRANDCHILD_CONVERT_VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
GRANDCHILD_CONVERT_VALIDATE_EXIT=$?
set -e
if [ $GRANDCHILD_CONVERT_VALIDATE_EXIT -ne 0 ]; then
  echo "FAILED: converted grandchild boundary should validate"
  echo "$GRANDCHILD_CONVERT_VALIDATE_OUTPUT"
  exit 1
fi

CONVERT_GRANDCHILD_BACK='### Grandchild Ontology

Grandchild ontology contribution again.

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Child Ontology](#child-ontology)

#### Ontology
```turtle
@prefix child: <https://example.test/child#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
child:GrandchildThing a owl:Class .
```'

set +e
GRANDCHILD_REVERT_OUTPUT=$(cd "$TEST_DIR" && printf "%s\n" "$CONVERT_GRANDCHILD_BACK" | "$REQVIRE_BIN" add specifications.md --override 2>&1)
GRANDCHILD_REVERT_EXIT=$?
set -e
if [ $GRANDCHILD_REVERT_EXIT -ne 0 ]; then
  echo "FAILED: converting the grandchild boundary back into a contributor should succeed"
  echo "$GRANDCHILD_REVERT_OUTPUT"
  exit 1
fi

if ! diff -u <(strip_trailing_blank_lines < "$TEST_SCRIPT_DIR/expected/01-nested-child-boundary-after-grandchild-revert.txt") <(strip_trailing_blank_lines < "$TEST_DIR/specifications.md"); then
  echo "FAILED: grandchild revert does not match expected inherited-boundary hierarchy"
  exit 1
fi

set +e
GRANDCHILD_REVERT_VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
GRANDCHILD_REVERT_VALIDATE_EXIT=$?
set -e
if [ $GRANDCHILD_REVERT_VALIDATE_EXIT -ne 0 ]; then
  echo "FAILED: reverted grandchild contributor should validate"
  echo "$GRANDCHILD_REVERT_VALIDATE_OUTPUT"
  exit 1
fi

cp "$TEST_SCRIPT_DIR/fixtures/02-ontology-mutation-ops.txt" "$TEST_DIR/specifications.md"

set +e
ONTOLOGY_RM_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" rm "Child Ontology" 2>&1)
ONTOLOGY_RM_EXIT=$?
set -e
if [ $ONTOLOGY_RM_EXIT -ne 0 ]; then
  echo "FAILED: deleting an ontology contributor should succeed"
  echo "$ONTOLOGY_RM_OUTPUT"
  exit 1
fi
if grep -Fxq "### Child Ontology" "$TEST_DIR/specifications.md"; then
  echo "FAILED: deleted ontology contributor still appears in the model"
  exit 1
fi
set +e
ONTOLOGY_RM_VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
ONTOLOGY_RM_VALIDATE_EXIT=$?
set -e
if [ $ONTOLOGY_RM_VALIDATE_EXIT -ne 0 ]; then
  echo "FAILED: ontology contributor deletion should validate"
  echo "$ONTOLOGY_RM_VALIDATE_OUTPUT"
  exit 1
fi

cp "$TEST_SCRIPT_DIR/fixtures/02-ontology-mutation-ops.txt" "$TEST_DIR/specifications.md"

set +e
ONTOLOGY_RENAME_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" rename "Child Ontology" "Child Ontology Renamed" 2>&1)
ONTOLOGY_RENAME_EXIT=$?
set -e
if [ $ONTOLOGY_RENAME_EXIT -ne 0 ]; then
  echo "FAILED: renaming an ontology contributor should succeed"
  echo "$ONTOLOGY_RENAME_OUTPUT"
  exit 1
fi
if ! grep -Fq "### Child Ontology Renamed" "$TEST_DIR/specifications.md"; then
  echo "FAILED: renamed ontology contributor heading was not updated"
  exit 1
fi
if grep -Fxq "### Child Ontology" "$TEST_DIR/specifications.md"; then
  echo "FAILED: old ontology contributor heading still exists after rename"
  exit 1
fi
if ! grep -Fq "@prefix root: <https://example.test/root#> ." "$TEST_DIR/specifications.md"; then
  echo "FAILED: renamed ontology contributor lost inherited prefix binding"
  exit 1
fi
set +e
ONTOLOGY_RENAME_VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
ONTOLOGY_RENAME_VALIDATE_EXIT=$?
set -e
if [ $ONTOLOGY_RENAME_VALIDATE_EXIT -ne 0 ]; then
  echo "FAILED: ontology contributor rename should validate"
  echo "$ONTOLOGY_RENAME_VALIDATE_OUTPUT"
  exit 1
fi

cp "$TEST_SCRIPT_DIR/fixtures/02-ontology-mutation-ops.txt" "$TEST_DIR/specifications.md"

set +e
ONTOLOGY_MV_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv "Child Ontology" "specifications/Moved.md" 2>&1)
ONTOLOGY_MV_EXIT=$?
set -e
if [ $ONTOLOGY_MV_EXIT -ne 0 ]; then
  echo "FAILED: moving an ontology contributor should succeed"
  echo "$ONTOLOGY_MV_OUTPUT"
  exit 1
fi
if grep -Fxq "### Child Ontology" "$TEST_DIR/specifications.md"; then
  echo "FAILED: moved ontology contributor still appears in the source file"
  exit 1
fi
if ! grep -Fq "### Child Ontology" "$TEST_DIR/specifications/Moved.md"; then
  echo "FAILED: moved ontology contributor was not written to the target file"
  exit 1
fi
if ! grep -Fq "@prefix root: <https://example.test/root#> ." "$TEST_DIR/specifications/Moved.md"; then
  echo "FAILED: moved ontology contributor lost inherited prefix binding"
  exit 1
fi
set +e
ONTOLOGY_MV_VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
ONTOLOGY_MV_VALIDATE_EXIT=$?
set -e
if [ $ONTOLOGY_MV_VALIDATE_EXIT -ne 0 ]; then
  echo "FAILED: ontology contributor move should validate"
  echo "$ONTOLOGY_MV_VALIDATE_OUTPUT"
  exit 1
fi

cp "$TEST_SCRIPT_DIR/fixtures/02-ontology-mutation-ops.txt" "$TEST_DIR/specifications.md"
rm -f "$TEST_DIR/specifications/Moved.md"

set +e
ONTOLOGY_MERGE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" merge "Billing Ontology" "Platform Ontology" 2>&1)
ONTOLOGY_MERGE_EXIT=$?
set -e
if [ $ONTOLOGY_MERGE_EXIT -ne 0 ]; then
  echo "FAILED: ontology merge should fold source Turtle into the target ontology block"
  echo "$ONTOLOGY_MERGE_OUTPUT"
  exit 1
fi
if ! grep -Fq "newbilling:BillingPayload a owl:Class ." "$TEST_DIR/specifications.md"; then
  echo "FAILED: target ontology declarations were lost during merge"
  exit 1
fi
if ! grep -Fq "newbilling:PlatformThing a owl:Class ." "$TEST_DIR/specifications.md"; then
  echo "FAILED: source ontology Turtle was not rewritten into the target ontology boundary"
  exit 1
fi
if grep -Fq "### Platform Ontology" "$TEST_DIR/specifications.md"; then
  echo "FAILED: merged source ontology element still appears in the model"
  exit 1
fi
if ! grep -Fq "#### Ontology" "$TEST_DIR/specifications.md"; then
  echo "FAILED: merged ontology target lost its ontology block"
  exit 1
fi
if ! grep -Fq "@prefix newbilling: <https://example.test/new-billing#> ." "$TEST_DIR/specifications.md"; then
  echo "FAILED: merged ontology target lost the rewritten prefix binding"
  exit 1
fi
if ! grep -Fq "<https://example.test/new-billing> a owl:Ontology" "$TEST_DIR/specifications.md"; then
  echo "FAILED: merged ontology target lost the rewritten ontology document declaration"
  exit 1
fi
if ! grep -Fq "<https://example.test/new-billing> <http://www.w3.org/2002/07/owl#imports> <https://example.test/root> ." "$TEST_DIR/specifications.md" && \
   ! grep -Fq "owl:imports <https://example.test/root>" "$TEST_DIR/specifications.md"; then
  echo "FAILED: merged ontology target lost required owl:imports"
  exit 1
fi
set +e
ONTOLOGY_MERGE_VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
ONTOLOGY_MERGE_VALIDATE_EXIT=$?
set -e
if [ $ONTOLOGY_MERGE_VALIDATE_EXIT -ne 0 ]; then
  echo "FAILED: ontology merge should validate"
  echo "$ONTOLOGY_MERGE_VALIDATE_OUTPUT"
  exit 1
fi
