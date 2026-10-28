"""Exercise the actual dependency-free Cargo build script without a full Cargo build."""
import os
from pathlib import Path
import shutil
import subprocess
import sys

root = Path(__file__).resolve().parents[2]
output = Path(sys.argv[1]).resolve()
output.mkdir(parents=True, exist_ok=True)
build = output / "build-script"
# Resolve the selected toolchain before fixtures override PATH. Hermit launchers
# rebuild PATH when invoked and would hide the fixture's fake Bun from Cargo.
rust_bin = Path(subprocess.check_output(["rustc", "--print", "sysroot"], text=True).strip()) / "bin"
subprocess.run([str(rust_bin / "rustc"), str(root / "crates/reqvire-core/build.rs"), "-o", str(build)], check=True)
checks = []

def run_case(name, check):
    try:
        check()
        checks.append(f"PASS {name}")
    except Exception as error:
        checks.append(f"FAIL {name}")
        print(f"{name}: {error}", file=sys.stderr)

def fixture(name):
    repo = output / name
    manifest = repo / "crates/reqvire-core"
    manifest.mkdir(parents=True)
    dist = repo / "explorer/dist"
    (dist / "assets").mkdir(parents=True)
    (repo / "explorer/src").mkdir()
    (dist / "index.html").write_text("old index")
    (dist / "assets/app.js").write_text("old app")
    out = repo / "out"
    out.mkdir()
    bin_dir = repo / "bin"
    bin_dir.mkdir()
    env = {**os.environ, "CARGO_MANIFEST_DIR": str(manifest), "OUT_DIR": str(out), "PATH": str(bin_dir)}
    env.pop("REQVIRE_BUILD_EXPLORER", None)
    return repo, dist, out, bin_dir, env

def invoke(env):
    return subprocess.run([str(build)], env=env, text=True, capture_output=True)

def require(condition, message):
    assert condition, message

def failed_requested_build(missing=False):
    _, _, out, bin_dir, env = fixture("missing-bun" if missing else "failed-bun")
    if not missing:
        bun = bin_dir / "bun"
        bun.write_text("#!/bin/sh\nexit 19\n")
        bun.chmod(0o755)
    env["REQVIRE_BUILD_EXPLORER"] = "1"
    result = invoke(env)
    require(result.returncode != 0, "explicit failed rebuild was accepted")
    require(not (out / "explorer_bundle_manifest.rs").exists(), "stale manifest was published")

def prebuilt():
    _, _, out, _, env = fixture("prebuilt")
    require(invoke(env).returncode == 0, "prebuilt mode requires bun")
    require((out / "explorer_bundle/assets/app.js").read_text() == "old app", "prebuilt asset missing")

def unchanged():
    _, _, out, _, env = fixture("unchanged")
    require(invoke(env).returncode == 0, "initial embedding failed")
    paths = [out / "explorer_bundle/index.html", out / "explorer_bundle/assets/app.js", out / "explorer_bundle_manifest.rs"]
    for path in paths:
        os.utime(path, ns=(1_000_000_000, 1_000_000_000))
    require(invoke(env).returncode == 0, "second embedding failed")
    require(all(path.stat().st_mtime_ns == 1_000_000_000 for path in paths), "identical output was rewritten")

def changed():
    _, dist, out, _, env = fixture("changed")
    require(invoke(env).returncode == 0, "initial embedding failed")
    (dist / "assets/app.js").unlink()
    (dist / "assets/new.js").write_text("new app")
    (dist / "index.html").write_text("new index")
    require(invoke(env).returncode == 0, "updated embedding failed")
    require(not (out / "explorer_bundle/assets/app.js").exists(), "removed asset remained")
    require((out / "explorer_bundle/assets/new.js").read_text() == "new app", "new asset absent")
    require((out / "explorer_bundle/index.html").read_text() == "new index", "changed asset stale")
    manifest = (out / "explorer_bundle_manifest.rs").read_text()
    require('assets/new.js' in manifest and 'assets/app.js' not in manifest, "manifest is stale")

def missing_bundle():
    _, dist, _, _, env = fixture("missing-bundle")
    shutil.rmtree(dist)
    require(invoke(env).returncode != 0, "missing bundle was accepted")

def successful_requested_build():
    _, _, out, bin_dir, env = fixture("success")
    bun = bin_dir / "bun"
    bun.write_text('#!/bin/sh\n[ "$1 $2" = "run build" ] || exit 2\nprintf fresh > dist/index.html\n')
    bun.chmod(0o755)
    env["REQVIRE_BUILD_EXPLORER"] = "1"
    require(invoke(env).returncode == 0, "successful rebuild failed")
    require((out / "explorer_bundle/index.html").read_text() == "fresh", "did not embed rebuilt output")

def watches():
    repo, dist, _, bin_dir, env = fixture("watches")
    (repo / "explorer/design-system/dist-showcase").mkdir(parents=True)
    (repo / "explorer/design-system/dist-kit").mkdir()
    result = invoke(env)
    watched = [Path(line.split("=", 1)[1]) for line in result.stdout.splitlines() if line.startswith("cargo:rerun-if-changed=")]
    require(dist in watched, "prebuilt output not watched")
    for generated in ["dist-kit", "dist-showcase"]:
        target = repo / "explorer/design-system" / generated
        require(not any(w == target or w in target.parents for w in watched), "generated design output is watched")
    bun = bin_dir / "bun"
    bun.write_text("#!/bin/sh\nexit 0\n")
    bun.chmod(0o755)
    env["REQVIRE_BUILD_EXPLORER"] = "1"
    result = invoke(env)
    watched = [Path(line.split("=", 1)[1]) for line in result.stdout.splitlines() if line.startswith("cargo:rerun-if-changed=")]
    require(not any(w == dist or dist in w.parents or w in dist.parents for w in watched), "requested rebuild watches generated production output")
    require(repo / "explorer/src" in watched, "source is not watched")

def cargo_fresh(rebuild=False):
    repo, dist, _, bin_dir, env = fixture("cargo-rebuild" if rebuild else "cargo-prebuilt")
    manifest = repo / "crates/reqvire-core"
    shutil.copy(root / "crates/reqvire-core/build.rs", manifest / "build.rs")
    (manifest / "src").mkdir()
    (manifest / "src/lib.rs").write_text("pub fn fixture() {}")
    (manifest / "Cargo.toml").write_text('[package]\nname="embed-fixture"\nversion="0.1.0"\nedition="2021"\n')
    public = repo / "explorer/public"
    public.mkdir()
    (public / ".gitkeep").touch()
    authored = ["ds.config.json", "biome.css.json", "vitest.config.ts", "design-system/showcase/fixture.ts", "design-system/vite.showcase.config.ts"]
    for name in authored:
        source = repo / "explorer" / name
        source.parent.mkdir(parents=True, exist_ok=True)
        source.write_text("initial source")
    bun = bin_dir / "bun"
    bun.write_text('#!/bin/sh\nprintf fresh > dist/index.html\n')
    bun.chmod(0o755)
    env["PATH"] += os.pathsep + os.environ["PATH"]
    env["CARGO_TARGET_DIR"] = str(repo / "target")
    env["RUSTC"] = str(rust_bin / "rustc")
    if rebuild:
        env["REQVIRE_BUILD_EXPLORER"] = "1"
    cargo_runs = 0
    def cargo():
        nonlocal cargo_runs
        cargo_runs += 1
        # CI enables Cargo colors; keep the verbose output parsed below plain.
        result = subprocess.run([str(rust_bin / "cargo"), "check", "--offline", "--verbose", "--color", "never", "--manifest-path", str(manifest / "Cargo.toml")], env=env, text=True, capture_output=True)
        (repo / f"cargo-{cargo_runs}.log").write_text(result.stderr)
        require(result.returncode == 0, result.stderr)
        return result.stderr
    def embedded(log):
        return any("Running `" in line and "/build-script-build`" in line for line in log.splitlines())
    require(embedded(cargo()), "initial embedding did not run")
    # Generated showcase/kit output must not invalidate production embedding.
    for name in ["dist-showcase", "dist-kit"]:
        generated = repo / "explorer/design-system" / name
        generated.mkdir(parents=True, exist_ok=True)
        (generated / "index.html").write_text("generated output")
    second = cargo()
    (repo / "second-cargo.log").write_text(second)
    require(not embedded(second), "unchanged Cargo build reran build script")
    if not rebuild:
        (dist / "index.html").write_text("changed production asset")
        require(embedded(cargo()), "prebuilt production changes ignored")
    require(not embedded(cargo()), "successive unchanged Cargo build is not fresh")
    (public / "new-asset.txt").write_text("new public input")
    require(embedded(cargo()), "adding an asset to the empty tracked public root was ignored")
    require(not embedded(cargo()), "unchanged public assets repeated embedding")
    for name in authored:
        (repo / "explorer" / name).write_text("updated source")
        require(embedded(cargo()), f"authored build input ignored: {name}")
        require(not embedded(cargo()), f"unchanged build repeated after {name}")

for name, check in [
    ("failed-requested-rebuild", failed_requested_build),
    ("missing-bun-requested-rebuild", lambda: failed_requested_build(True)),
    ("prebuilt-without-bun", prebuilt), ("identical-outputs-untouched", unchanged),
    ("changed-and-removed-assets", changed), ("missing-bundle", missing_bundle),
    ("successful-requested-rebuild", successful_requested_build), ("input-watches", watches),
    ("cargo-prebuilt-incremental", cargo_fresh),
    ("cargo-requested-incremental", lambda: cargo_fresh(True)),
]:
    run_case(name, check)
(output / "checks.txt").write_text("\n".join(checks) + "\n")
sys.exit(any(check.startswith("FAIL") for check in checks))
