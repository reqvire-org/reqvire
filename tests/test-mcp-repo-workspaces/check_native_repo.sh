#!/bin/bash
# Native selection uses real local Git remotes and a root-owned TOML manifest.
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

native_fixture() {
    local root_model=${2:-model} layout=${3:-flat} source_name=reqvire-workspace.toml
    native_case="$TEST_DIR/native-$1"
    mkdir -p "$native_case/remotes" "$native_case/component" "$native_case/central"
    export GIT_ALLOW_PROTOCOL=file GIT_TERMINAL_PROMPT=0
    export GIT_AUTHOR_NAME='Repo E2E' GIT_AUTHOR_EMAIL='repo-e2e@example.invalid'
    export GIT_COMMITTER_NAME="$GIT_AUTHOR_NAME" GIT_COMMITTER_EMAIL="$GIT_AUTHOR_EMAIL"
    git -C "$native_case/component" init -qb main
    cp "$TEST_SCRIPT_DIR/fixtures/component.md.txt" "$native_case/component/Model.md"
    git -C "$native_case/component" add .
    git -C "$native_case/component" -c commit.gpgSign=false commit -qm alpha
    alpha=$(git -C "$native_case/component" rev-parse HEAD)
    sed -i 's/behavior is alpha/behavior is omega/' "$native_case/component/Model.md"
    git -C "$native_case/component" -c commit.gpgSign=false commit -qam omega
    omega=$(git -C "$native_case/component" rev-parse HEAD)
    git clone -q --bare "$native_case/component" "$native_case/remotes/component"
    git -C "$native_case/remotes/component" branch next "$omega"
    git -C "$native_case/remotes/component" update-ref refs/heads/main "$alpha"
    central="$native_case/central"
    git -C "$central" init -qb main
    if [[ "$root_model" == model ]]; then
        cp "$TEST_SCRIPT_DIR/fixtures/central.md.txt" "$central/Model.md"
    fi
    printf '.repo/\nrepositories/\n' > "$central/.gitignore"
    mkdir -p "$(dirname "$central/$source_name")"
    fixture_manifest > "$central/$source_name"
    fixture_project component "$alpha" >> "$central/$source_name"
    git -C "$central" add .
    git -C "$central" -c commit.gpgSign=false commit -qm Central
    git clone -q --bare "$central" "$native_case/remotes/central"
    git -C "$central" remote add origin "file://$native_case/remotes/central"
    (cd "$central" && "$REQVIRE_BIN" workspace sync) > "$native_case/sync.log" 2>&1
    source_manifest="$central/$source_name"
    native="$source_manifest"
}

# Keep fixture declarations explicit. Paths are relative to the configured
# repositories_dir, which is "." here to preserve the prior checkout layout.
fixture_manifest() {
    printf 'version = 1\nrepositories_dir = "."\n'
}
fixture_project() {
    printf '\n[[repositories]]\nname = "%s"\npath = "repositories/%s"\nurl = "%s%s"\nref = "refs/heads/main"\ncommit = "%s"\n' \
        "$1" "${3:-$1}" "${4:-file://$native_case/remotes/}" "$1" "$2"
}
# Remove a complete table without dropping any adjacent declarations.
fixture_remove() {
    local file=$1 name=$2
    awk -v name="$name" 'BEGIN {RS="\n\\[\\[repositories\\]\\]"; ORS=""}
      NR==1 {print;next} !index($0, "name = \"" name "\"") {print "\n[[repositories]]" $0}' "$file" > "$file.next"
    mv "$file.next" "$file"
}
