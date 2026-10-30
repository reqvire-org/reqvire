#!/bin/bash
# Route only the declared fixture remote to local Git; refuse other transports.
set -euo pipefail
network=false
for arg in "$@"; do case "$arg" in push|ls-remote) network=true ;; esac; done
args=()
for arg in "$@"; do
    if [[ "$network" == true ]]; then
        if [[ "$arg" == https://github.com/test/project.git ]]; then arg=$REQVIRE_GIT_TEST_REMOTE; fi
        case "$arg" in https://*|ssh://*|git@*) exit 99 ;; esac
    fi
    args+=("$arg")
done
exec "$REQVIRE_GIT_TEST_BINARY" "${args[@]}"
