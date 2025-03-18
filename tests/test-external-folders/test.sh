#!/bin/bash

OUTPUT=$(cd "$TEST_DIR" && "$REQFLOW_BIN" --config "${TEST_DIR}/reqflow-valid.yaml"  --validate 2>&1)
EXIT_CODE=$?


printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"


if [[ $status -ne 0 ]]; then
    exit $EXIT_CODE
fi

OUTPUT=$(cd "$TEST_DIR" && "$REQFLOW_BIN" --config "${TEST_DIR}/reqflow-invalid.yaml"  --validate 2>&1)
EXIT_CODE=$?

printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"


if echo "$OUTPUT" | grep -q "Validation Summary validation failed with 3 error(s):"; then
    # Worked
    exit 0
else
   exit 1
fi



