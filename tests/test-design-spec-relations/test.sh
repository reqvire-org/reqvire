#!/bin/bash

# Test: Validation of Relations to Excluded Folder Documents
# ------------------------------------------------------------
# Acceptance Criteria:
# - System should correctly validate relations to files in excluded folders (like DesignSpecifications)
# - System should maintain relations even when folders are excluded from main processing
#
# Test Criteria:
# - Command should correctly validate relations to files in excluded_filename_patterns
# - Using '**/DesignSpecifications/**/*.md' pattern to exclude processing but not for relations
#


OUTPUT=$(cd "$TEST_DIR" && "$REQFLOW_BIN" --config "${TEST_DIR}/reqflow.yaml"  --validate 2>&1)
EXIT_CODE=$?


printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"
exit $EXIT_CODE
