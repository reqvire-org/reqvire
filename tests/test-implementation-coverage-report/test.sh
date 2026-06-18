#!/usr/bin/env bash
set -euo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Test: Requirement Implementation Coverage Report
# ------------------------------------------------
# Verifies: system-model/Functional/Output/Reporting.md#requirement-implementation-coverage-report

run_text_report_test() {
  local output
  output=$(cd "$TEST_DIR" && "$REQVIRE_BIN" coverage 2>&1)

  local section
  section=$(printf "%s\n" "$output" | sed -n '/^### Requirement Implementation Coverage$/,$p')

  if [ -z "$section" ]; then
    echo "❌ FAILED: implementation coverage section missing from text report"
    exit 1
  fi

  if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected_output.md" <(printf "%s\n" "$section"); then
    echo "❌ FAILED: implementation coverage text output does not match expected output"
    exit 1
  fi
}

run_json_report_test() {
  local output
  output=$(cd "$TEST_DIR" && "$REQVIRE_BIN" coverage --json 2>&1)

  if ! echo "$output" | jq . >/dev/null 2>&1; then
    echo "❌ FAILED: coverage --json did not produce valid JSON"
    exit 1
  fi

  if ! echo "$output" | jq '.summary | has("total_requirements_in_scope")' | grep -q true; then
    echo "❌ FAILED: JSON summary missing total_requirements_in_scope"
    exit 1
  fi

  if ! echo "$output" | jq '.summary | has("coverage_sources")' | grep -q true; then
    echo "❌ FAILED: JSON summary missing coverage_sources"
    exit 1
  fi

  if ! echo "$output" | jq 'has("covered_requirements")' | grep -q true; then
    echo "❌ FAILED: JSON missing covered_requirements section"
    exit 1
  fi

  if ! echo "$output" | jq 'has("uncovered_requirements")' | grep -q true; then
    echo "❌ FAILED: JSON missing uncovered_requirements section"
    exit 1
  fi

  local actual_json
  actual_json=$(echo "$output" | jq '{
    summary: {
      total_requirements_in_scope: .summary.total_requirements_in_scope,
      covered_requirements: .summary.covered_requirements,
      uncovered_requirements: .summary.uncovered_requirements,
      implementation_coverage_percentage: .summary.implementation_coverage_percentage,
      coverage_sources: .summary.coverage_sources
    },
    covered_requirements: .covered_requirements,
    uncovered_requirements: .uncovered_requirements
  }')

  if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected_output.json" <(echo "$actual_json" | jq -S .); then
    echo "❌ FAILED: implementation coverage JSON output does not match expected output"
    exit 1
  fi
}

run_text_report_test
run_json_report_test

echo "✅ PASSED: Requirement implementation coverage report"
