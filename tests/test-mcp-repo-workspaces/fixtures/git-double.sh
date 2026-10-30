#!/usr/bin/env bash
# Resolve fixture remotes locally and record push order without using a network.
set -euo pipefail
real_git=$REQVIRE_REPO_TEST_GIT_BINARY
args=("$@")
network=false
for arg in "$@"; do case "$arg" in push|ls-remote) network=true ;; esac; done
if [[ "$network" == true ]]; then
    targets=()
    for i in "${!args[@]}"; do
        arg=${args[$i]}
        case "$arg" in origin|local)
            if url=$("$real_git" remote get-url "$arg" 2>/dev/null) &&
                jq -e --arg url "$url" 'has($url)' <<< "$REQVIRE_REPO_TEST_REMOTES" >/dev/null; then arg=$url; fi ;;
        esac
        if jq -e --arg url "$arg" 'has($url)' <<< "$REQVIRE_REPO_TEST_REMOTES" >/dev/null; then
            targets+=("$arg")
            args[$i]=$(jq -r --arg url "$arg" '.[$url]' <<< "$REQVIRE_REPO_TEST_REMOTES")
        else args[$i]=$arg; fi
    done
    for arg in "$@"; do
        if [[ "$arg" == push ]]; then
            jq -cn --arg cwd "$PWD" --args '{cwd:$cwd,targets:$ARGS.positional}' -- "${targets[@]}" >> "$REQVIRE_REPO_TEST_PUSHES"
            if [[ -f "$REQVIRE_REPO_TEST_FAIL_COMPONENT" ]]; then
                for target in "${targets[@]}"; do
                    if [[ "$target" == *component* ]]; then printf 'Injected component remote denial\n' >&2; exit 1; fi
                done
            fi
            break
        fi
    done
    for arg in "${args[@]}"; do
        case "$arg" in https://*|ssh://*|git@*) printf 'Network forbidden in offline Repo test\n' >&2; exit 99 ;; esac
    done
fi
exec "$real_git" "${args[@]}"
