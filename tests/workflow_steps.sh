#!/bin/bash
# Read the named steps and literal run blocks used by the owned CI workflows.
# This deliberately rejects unsupported folded/chomped run-block styles.
workflow_steps() {
    local workflow=$1 destination=$2
    mkdir -p "$destination"
    awk -v destination="$destination" '
      function flush(    path) {
        if (name != "" && has_run) {
          path = sprintf("%s/step-%03d", destination, count++)
          print name > (path ".name")
          printf "%s", block > (path ".block")
          print command > (path ".run")
          close(path ".name"); close(path ".block"); close(path ".run")
        }
      }
      /^      - name: / {
        flush(); name=$0; sub(/^      - name: /, "", name)
        block=""; command=""; has_run=0; literal=0; next
      }
      name != "" {
        block=block $0 "\n"
        if ($0 ~ /^        run: /) {
          command=$0; sub(/^        run: /, "", command); has_run=1
          if (command == "|") { command=""; literal=1 }
          else if (command ~ /^[>|]/) {
            print "Unsupported run block for " name > "/dev/stderr"; failed=1; exit 1
          }
          next
        }
        if (literal && $0 ~ /^          /) {
          line=substr($0,11)
          command=command (command == "" ? "" : "\n") line
        }
      }
      END { if (!failed) flush() }
    ' "$workflow"
}

workflow_job() {
    local workflow=$1 job=$2
    awk -v job="$job" '
      $0 ~ "^  " job ":[[:space:]]*$" { found=1; next }
      found && /^  [[:alnum:]_][[:alnum:]_-]*:[[:space:]]*$/ { exit }
      found { print }
      END { if (!found) exit 1 }
    ' "$workflow"
}
