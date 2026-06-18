#!/bin/bash
set -euo pipefail

REQVIRE="${REQVIRE_BIN:-reqvire}"

cd "$TEST_DIR"

VALIDATE_OUTPUT="$($REQVIRE validate 2>&1 || true)"
if ! echo "$VALIDATE_OUTPUT" | grep -q "v1.0-contract-relations"; then
  echo "FAILED: legacy relation validation should report contract relation migration candidate"
  echo "$VALIDATE_OUTPUT"
  exit 1
fi

DRY_RUN_OUTPUT="$($REQVIRE migrate 2>&1)"
if ! echo "$DRY_RUN_OUTPUT" | grep -q "2 contract relation rewrite"; then
  echo "FAILED: migrate dry-run should preview two contract relation rewrites"
  echo "$DRY_RUN_OUTPUT"
  exit 1
fi

if ! grep -q "refinedBy" specifications/Requirements.md; then
  echo "FAILED: dry-run migration should not rewrite fixture files"
  exit 1
fi

$REQVIRE migrate --fix > migrate-fix.out 2>&1

if grep -q "refinedBy\\|refine:" specifications/Requirements.md; then
  echo "FAILED: migrate --fix should remove legacy relation names"
  cat specifications/Requirements.md
  exit 1
fi

if ! grep -q "definedBy: \\[Invoice Numbering Specification\\]" specifications/Requirements.md; then
  echo "FAILED: migrated requirement should use definedBy"
  cat specifications/Requirements.md
  exit 1
fi

if ! grep -q "define: \\[Invoice Number Requirement\\]" specifications/Requirements.md; then
  echo "FAILED: migrated specification should use define"
  cat specifications/Requirements.md
  exit 1
fi

$REQVIRE validate > validate-after.out 2>&1

