//! Build script that stages the compiled Explorer SPA bundle for embedding.
//!
//! The exported/served `index.html` is the Vite/React/Radix Explorer bundle
//! (built from `explorer/`), not a runtime-assembled page. This script copies
//! the built bundle (`explorer/dist/index.html` + `assets/explorer.{js,css}`)
//! into `OUT_DIR` so `src/export.rs` can `include_bytes!`/`include_str!` it at
//! compile time.
//!
//! The bundle is a required build input. CI and `make` build the Explorer first
//! (`cd explorer && npm ci && npm run build`) so release artifacts always embed
//! the real bundle.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bundle_out = out_dir.join("explorer_bundle");
    fs::create_dir_all(&bundle_out).expect("create explorer_bundle out dir");

    // explorer/dist relative to the workspace root (core/ is one level down).
    let dist = manifest_dir.join("../explorer/dist");
    let index = dist.join("index.html");
    let js = dist.join("assets/explorer.js");
    let css = dist.join("assets/explorer.css");

    // Rebuild embedding whenever the built bundle changes.
    println!("cargo:rerun-if-changed={}", index.display());
    println!("cargo:rerun-if-changed={}", js.display());
    println!("cargo:rerun-if-changed={}", css.display());

    if !(index.is_file() && js.is_file() && css.is_file()) {
        panic!(
            "explorer/dist bundle not found at {}. Run `cd explorer && npm run build` before `cargo build`.",
            dist.display()
        );
    }

    copy(&index, &bundle_out.join("index.html"));
    copy(&js, &bundle_out.join("explorer.js"));
    copy(&css, &bundle_out.join("explorer.css"));
}

fn copy(src: &Path, dest: &Path) {
    fs::copy(src, dest)
        .unwrap_or_else(|e| panic!("copy {} -> {}: {e}", src.display(), dest.display()));
}
