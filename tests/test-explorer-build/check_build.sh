#!/bin/bash
# Exercise the actual dependency-free Cargo build script in disposable fixtures.
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$SCRIPT_DIR/../.." && pwd)"
mkdir -p "$1"
output="$(cd "$1" && pwd)"
build="$output/build-script"
# Resolve the toolchain before fixture PATH overrides hide the real tools.
rust_bin="$(rustc --print sysroot)/bin"
original_path=$PATH
fail() { printf '%s\n' "$*" >&2; exit 1; }

fixture() {
    repo="$output/$1"
    manifest="$repo/crates/reqvire-core"
    dist="$repo/explorer/dist"
    out="$repo/out"
    bin_dir="$repo/bin"
    rebuild=false
    mkdir -p "$manifest" "$dist/assets" "$repo/explorer/src" "$out" "$bin_dir"
    printf 'old index' > "$dist/index.html"
    printf 'old app' > "$dist/assets/app.js"
}

invoke() {
    local options=(-u REQVIRE_BUILD_EXPLORER)
    [[ "$rebuild" != true ]] || options+=(REQVIRE_BUILD_EXPLORER=1)
    local result=0
    env "${options[@]}" CARGO_MANIFEST_DIR="$manifest" OUT_DIR="$out" PATH="$bin_dir" \
        "$build" > "$repo/build.stdout" 2> "$repo/build.stderr" || result=$?
    if [[ "$result" != 0 ]]; then cat "$repo/build.stderr" >&2; fi
    return "$result"
}

failed_requested_build() {
    local missing=${1:-false}
    if [[ "$missing" == true ]]; then fixture missing-bun; else
        fixture failed-bun
        printf '#!/bin/sh\nexit 19\n' > "$bin_dir/bun"
        chmod +x "$bin_dir/bun"
    fi
    rebuild=true
    if invoke; then fail 'explicit failed rebuild was accepted'; fi
    [[ ! -e "$out/explorer_bundle_manifest.rs" ]] || fail 'stale manifest was published'
}

prebuilt() {
    fixture prebuilt
    invoke
    [[ $(cat "$out/explorer_bundle/assets/app.js") == 'old app' ]] || fail 'prebuilt asset missing'
}

unchanged() {
    fixture unchanged
    invoke
    local paths=("$out/explorer_bundle/index.html" "$out/explorer_bundle/assets/app.js" "$out/explorer_bundle_manifest.rs")
    TZ=UTC touch -t 197001010000.01 "$repo/reference-time"
    touch -r "$repo/reference-time" "${paths[@]}"
    invoke
    local path
    for path in "${paths[@]}"; do
        [[ ! "$path" -nt "$repo/reference-time" && ! "$repo/reference-time" -nt "$path" ]] || fail 'identical output was rewritten'
    done
}

changed() {
    fixture changed
    invoke
    rm "$dist/assets/app.js"
    printf 'new app' > "$dist/assets/new.js"
    printf 'new index' > "$dist/index.html"
    invoke
    [[ ! -e "$out/explorer_bundle/assets/app.js" ]] || fail 'removed asset remained'
    [[ $(cat "$out/explorer_bundle/assets/new.js") == 'new app' ]] || fail 'new asset absent'
    [[ $(cat "$out/explorer_bundle/index.html") == 'new index' ]] || fail 'changed asset stale'
    grep -Fq assets/new.js "$out/explorer_bundle_manifest.rs"
    if grep -Fq assets/app.js "$out/explorer_bundle_manifest.rs"; then fail 'manifest is stale'; fi
}

missing_bundle() {
    fixture missing-bundle
    rm -rf "$dist"
    if invoke; then fail 'missing bundle was accepted'; fi
}

successful_requested_build() {
    fixture success
    cat > "$bin_dir/bun" <<'BUN'
#!/bin/sh
[ "$1 $2" = 'run build' ] || exit 2
printf fresh > dist/index.html
BUN
    chmod +x "$bin_dir/bun"
    rebuild=true
    invoke
    [[ $(cat "$out/explorer_bundle/index.html") == fresh ]] || fail 'did not embed rebuilt output'
}

watches() {
    fixture watches
    mkdir -p "$repo/explorer/design-system/dist-showcase" "$repo/explorer/design-system/dist-kit"
    invoke
    sed -n 's/^cargo:rerun-if-changed=//p' "$repo/build.stdout" > "$repo/watched.txt"
    grep -Fxq "$dist" "$repo/watched.txt"
    local generated watched target
    for generated in dist-kit dist-showcase; do
        target="$repo/explorer/design-system/$generated"
        while IFS= read -r watched; do
            [[ "$watched" != "$target" && "$target" != "$watched/"* ]] || fail 'generated design output is watched'
        done < "$repo/watched.txt"
    done
    printf '#!/bin/sh\nexit 0\n' > "$bin_dir/bun"
    chmod +x "$bin_dir/bun"
    rebuild=true
    invoke
    sed -n 's/^cargo:rerun-if-changed=//p' "$repo/build.stdout" > "$repo/watched.txt"
    while IFS= read -r watched; do
        [[ "$watched" != "$dist" && "$watched" != "$dist/"* && "$dist" != "$watched/"* ]] || fail 'requested rebuild watches generated production output'
    done < "$repo/watched.txt"
    grep -Fxq "$repo/explorer/src" "$repo/watched.txt"
}

cargo_run() {
    cargo_runs=$((cargo_runs + 1))
    cargo_log="$repo/cargo-$cargo_runs.log"
    local options=(-u REQVIRE_BUILD_EXPLORER)
    [[ "$rebuild" != true ]] || options+=(REQVIRE_BUILD_EXPLORER=1)
    if ! env "${options[@]}" PATH="$bin_dir:$original_path" RUSTC="$rust_bin/rustc" \
        CARGO_TARGET_DIR="$repo/target" "$rust_bin/cargo" check --offline --verbose --color never \
        --manifest-path "$manifest/Cargo.toml" > "$repo/cargo.stdout" 2> "$cargo_log"; then
        cat "$cargo_log" >&2
        fail 'fixture Cargo check failed'
    fi
}

embedded() { grep -q 'Running `.*\/build-script-build`' "$cargo_log"; }

cargo_fresh() {
    local requested=${1:-false}
    if [[ "$requested" == true ]]; then fixture cargo-rebuild; else fixture cargo-prebuilt; fi
    rebuild=$requested
    cp "$root/crates/reqvire-core/build.rs" "$manifest/build.rs"
    mkdir -p "$manifest/src" "$repo/explorer/public"
    printf 'pub fn fixture() {}' > "$manifest/src/lib.rs"
    printf '[package]\nname="embed-fixture"\nversion="0.1.0"\nedition="2021"\n' > "$manifest/Cargo.toml"
    touch "$repo/explorer/public/.gitkeep"
    local authored=(ds.config.json biome.css.json vitest.config.ts design-system/showcase/fixture.ts design-system/vite.showcase.config.ts) name
    for name in "${authored[@]}"; do
        mkdir -p "$(dirname "$repo/explorer/$name")"
        printf 'initial source' > "$repo/explorer/$name"
    done
    printf '#!/bin/sh\nprintf fresh > dist/index.html\n' > "$bin_dir/bun"
    chmod +x "$bin_dir/bun"
    cargo_runs=0
    cargo_run; embedded || fail 'initial embedding did not run'
    for name in dist-showcase dist-kit; do
        mkdir -p "$repo/explorer/design-system/$name"
        printf 'generated output' > "$repo/explorer/design-system/$name/index.html"
    done
    cargo_run
    cp "$cargo_log" "$repo/second-cargo.log"
    if embedded; then fail 'unchanged Cargo build reran build script'; fi
    if [[ "$rebuild" != true ]]; then
        printf 'changed production asset' > "$dist/index.html"
        cargo_run; embedded || fail 'prebuilt production changes ignored'
    fi
    cargo_run
    if embedded; then fail 'successive unchanged Cargo build is not fresh'; fi
    printf 'new public input' > "$repo/explorer/public/new-asset.txt"
    cargo_run; embedded || fail 'adding an asset to the empty tracked public root was ignored'
    cargo_run
    if embedded; then fail 'unchanged public assets repeated embedding'; fi
    for name in "${authored[@]}"; do
        printf 'updated source' > "$repo/explorer/$name"
        cargo_run; embedded || fail "authored build input ignored: $name"
        cargo_run
        if embedded; then fail "unchanged build repeated after $name"; fi
    done
}

if [[ "${2:-}" == --case ]]; then
    shift 2
    "$@"
    exit
fi
"$rust_bin/rustc" "$root/crates/reqvire-core/build.rs" -o "$build"
: > "$output/checks.txt"
status=0
while read -r name function argument; do
    args=("$function")
    [[ -z "$argument" ]] || args+=("$argument")
    if bash "$SCRIPT_DIR/check_build.sh" "$output" --case "${args[@]}" > "$output/$name.log" 2>&1; then
        printf 'PASS %s\n' "$name" >> "$output/checks.txt"
    else
        printf 'FAIL %s\n' "$name" >> "$output/checks.txt"
        cat "$output/$name.log" >&2
        status=1
    fi
done <<'CASES'
failed-requested-rebuild failed_requested_build false
missing-bun-requested-rebuild failed_requested_build true
prebuilt-without-bun prebuilt
identical-outputs-untouched unchanged
changed-and-removed-assets changed
missing-bundle missing_bundle
successful-requested-rebuild successful_requested_build
input-watches watches
cargo-prebuilt-incremental cargo_fresh false
cargo-requested-incremental cargo_fresh true
CASES
exit "$status"
