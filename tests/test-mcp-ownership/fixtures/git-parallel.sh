#!/usr/bin/env bash
# Block the selected observation until the parent releases concurrent requests.
set -euo pipefail
base=$REQVIRE_WORKER_TEST_BASE
if [[ $# == 2 && "$1" == status && "$2" == --porcelain && -e "$base/armed" ]]; then
    printf ready > "$base/entered-$$"
    deadline=$((SECONDS+15))
    while [[ ! -e "$base/release" ]]; do
        if ((SECONDS>=deadline)); then exit 99; fi
        sleep 0.005
    done
fi
exec "$REAL_TEST_GIT" "$@"
