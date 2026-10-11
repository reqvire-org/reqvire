Minimal fixture suite for model-element prose-link validation: 10 scenarios.

Parsing/read (5): one combined valid-authoring control; inline and reference-style
links with aggregated requirement/concept guidance; a file-only link in a single-
element document; capability guidance; and a model-report publication gate.
CLI CRUD (3): atomic create rejection; override dry-run rejection; and creation
that makes otherwise untouched prose resolve to an element in the candidate.
MCP (1): atomic override rejection, structured guidance, accepted-state preservation,
and a valid repair after rejection.
Formatting (1): preview/fix preserve authored decisions and surface contract-source
guidance; explicit repair produces the expected file.

base/: small valid model copied into each isolated Git test workspace.
sources/: exact input documents copied without runtime Markdown generation.
payloads/: exact create/override candidates and explicit author repairs.
expected/: stable diagnostic projections and complete files after valid repairs.

The existing suites compare output with expected/prose-link-checks.txt using
unified diffs. Model-file diffs compare rejected edits against fixture inputs;
valid repairs are matched against checked-in expected files.
