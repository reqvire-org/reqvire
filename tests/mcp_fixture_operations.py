#!/usr/bin/env python3
"""Run existing operation fixtures through typed MCP, never removed CLI handlers.

Each request starts from an isolated clean Git snapshot of the fixture's current
files. Publish the successful candidate back to the fixture without changing its
Git history/index. Positional fixture arguments and text rendering preserve the
existing core-operation goldens; the interface-boundary suite checks MCP schemas,
envelopes, permissions, accepted revisions, and rejected CLI invocations directly.
"""
import argparse
import contextlib
import io
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

SOURCE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("revision_e2e", SOURCE / "test-model-revision-hashing/check_revision.py")
helpers = importlib.util.module_from_spec(spec)
spec.loader.exec_module(helpers)


def file_diffs(diffs):
    for diff in diffs:
        print(f"📄 {diff['file_path']}")
        for line in diff["lines"]:
            if line["color"] == "separator":
                print()
                continue
            value = line["prefix"] + (" " + line["content"] if line["content"] else "")
            color = {"green": "32", "red": "31", "context": "37"}.get(line["color"])
            print("  " + (f"\x1b[{color}m{value}\x1b[0m" if color else value))
        print("\n\n")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("operation")
    parser.add_argument("values", nargs="*")
    parser.add_argument("--workspace", default=os.getcwd())
    for flag in ("dry-run", "fix", "fixable", "auditable", "with-full-relations", "squash", "json", "override"):
        parser.add_argument("--" + flag, action="store_true")
    parser.add_argument("--core-workspace", action="store_true")
    parser.add_argument("--content")
    parser.add_argument("--output")
    args = parser.parse_intermixed_args()
    operations = {
        "add": ("add_element", ["file"]), "rm": ("remove_element", ["element_name"]),
        "mv": ("move_element", ["element_name", "file"]),
        "rename": ("rename_element", ["element_name", "new_name"]),
        "merge": ("merge_elements", ["target"]),
        "mv-file": ("move_file", ["source_file", "target_file"]),
        "mv-folder": ("move_folder", ["source_folder", "target_folder"]),
        "link": ("link", ["source", "relation_type", "target"]),
        "unlink": ("unlink", ["source", "target"]),
        "relink": ("relink", ["source", "relation_type", "from_target", "to_target"]),
        "mv-asset": ("move_asset", ["old_path", "new_path"]),
        "rm-asset": ("remove_asset", ["file_path"]),
        "format": ("format", []), "lint": ("lint", []),
    }
    name, keys = operations[args.operation]
    if len(args.values) < len(keys) or (args.operation != "merge" and len(args.values) != len(keys)):
        parser.error(f"{args.operation} requires {', '.join(keys)}")
    arguments = dict(zip(keys, args.values))
    if args.operation == "merge":
        arguments["sources"] = args.values[1:]
    if args.operation == "add":
        arguments.update(content=args.content if args.content is not None else sys.stdin.read(),
                         override_existing=args.override)
    if args.operation in ("format", "lint"):
        arguments["fix"] = args.fix
        if args.operation == "format":
            arguments["with_full_relations"] = args.with_full_relations
        else:
            arguments.update(fixable=args.fixable, auditable=args.auditable)
    else:
        arguments["dry_run"] = args.dry_run
    if args.operation == "mv-file":
        arguments["squash"] = args.squash
    root = Path(args.workspace).resolve()
    binary = os.environ["REQVIRE_BIN"]
    if args.core_workspace:
        result = subprocess.run([os.environ["CORE_OPERATION_BIN"]], cwd=root, text=True,
                                input=json.dumps({"name": "reqvire." + name, "arguments": arguments}),
                                capture_output=True, check=False)
        data = json.loads(result.stdout)
        if result.returncode:
            print(data["error"]["message"], file=sys.stderr)
            return 1
        render(args, data, root, binary)
        return 0
    with tempfile.TemporaryDirectory(prefix="reqvire-mcp-fixture-") as temporary:
        temporary = Path(temporary)
        snapshot, output = temporary / "model", temporary / "output"
        output.mkdir()
        shutil.copytree(root, snapshot, ignore=shutil.ignore_patterns(".git", "output", "node_modules"))
        subprocess.run(["git", "init", "-q", str(snapshot)], check=True)
        subprocess.run(["git", "-C", str(snapshot), "config", "user.name", "Fixture"], check=True)
        subprocess.run(["git", "-C", str(snapshot), "config", "user.email", "fixture@example.org"], check=True)
        subprocess.run(["git", "-C", str(snapshot), "add", "."], check=True)
        subprocess.run(["git", "-C", str(snapshot), "-c", "commit.gpgSign=false",
                        "commit", "--allow-empty", "-qm", "fixture"], check=True)
        before = {p.relative_to(snapshot): p.read_bytes() for p in snapshot.rglob("*")
                  if p.is_file() and ".git" not in p.relative_to(snapshot).parts}
        mutations = args.operation not in ("format", "lint") or args.fix
        try:
            with helpers.McpServer(binary, snapshot, output, "server",
                                   *(["--enable-mutations"] if mutations else [])) as server:
                result = server.rpc("tools/call", {"name": "reqvire." + name, "arguments": arguments})
                if result.get("isError"):
                    error = result.get("structuredContent", {}).get("error", {})
                    if args.json and error.get("code") == "validation_failed":
                        print(json.dumps({"errors": error.get("related_errors") or [error["message"]],
                                          "diagnostics": error.get("diagnostics") or []}))
                    else:
                        print("error: " + (error.get("message") or str(result)), file=sys.stderr)
                        for message in error.get("related_errors") or []:
                            print(message, file=sys.stderr)
                    return 1
                data = result["structuredContent"]
                data.pop("context", None)
        except RuntimeError as error:
            log = (output / "server/server.log").read_text()
            validation = subprocess.run([binary, "--workspace", str(snapshot), "validate", "--json"], text=True, capture_output=True)
            if validation.returncode and validation.stdout.strip():
                if args.json:
                    print(validation.stdout, end="")
                else:
                    payload = json.loads(validation.stdout)
                    print("error: MCP startup validation failed", file=sys.stderr)
                    for message in payload["errors"]:
                        print("    " + message, file=sys.stderr)
                    for diagnostic in payload.get("diagnostics", []):
                        print(json.dumps(diagnostic), file=sys.stderr)
            else:
                print(log or str(error), file=sys.stderr)
            return 1
        after = {p.relative_to(snapshot): p.read_bytes() for p in snapshot.rglob("*")
                 if p.is_file() and ".git" not in p.relative_to(snapshot).parts}
        if mutations and not args.dry_run:
            for path in before.keys() | after.keys():
                if before.get(path) == after.get(path):
                    continue
                dest = root / path
                if path not in after:
                    dest.unlink()
                else:
                    dest.parent.mkdir(parents=True, exist_ok=True)
                    dest.write_bytes(after[path])
                    shutil.copymode(snapshot / path, dest)
            # Match MCP's removal of empty source directories after folder moves.
            if args.operation == "mv-folder":
                assert not (snapshot / arguments["source_folder"]).exists(), "MCP left its source folder behind"
                source_folder = root / arguments["source_folder"]
                if source_folder.exists():
                    for directory in sorted((p for p in source_folder.rglob("*") if p.is_dir()), key=lambda p: len(p.parts), reverse=True):
                        directory.rmdir()
                    source_folder.rmdir()
        render(args, data, root, binary)
    return 0


def render(args, data, root, binary):
    if args.operation == "lint":
        return _render(args, data, root, binary)
    output = io.StringIO()
    with contextlib.redirect_stdout(output):
        _render(args, data, root, binary)
    text = output.getvalue()
    evidence = Path(os.environ["TEST_DIR"]) / "output"
    evidence.mkdir(exist_ok=True)
    (evidence / f"typed-{args.operation}.json").write_text(json.dumps(data, indent=2, sort_keys=True) + "\n")
    (evidence / f"typed-{args.operation}.txt").write_text(text.rstrip() + "\n")
    print(text, end="")


def _render(args, data, root, binary):
    if args.json:
        text = json.dumps(data, indent=2, sort_keys=True) + "\n"
        if args.output:
            Path(args.output).write_text(text)
            print(f"✅ Output saved to {args.output}")
        else:
            print(text, end="")
    elif args.operation == "format":
        if not data["diffs"]:
            print("✅ No formatting changes needed." if not args.fix else "✅ No files needed formatting.")
        else:
            print(f"Formatted {data['files_changed']} file(s):\n" if args.fix else f"Found {len(data['diffs'])} file(s) with formatting changes:\n")
            file_diffs(data["diffs"])
            if not args.fix:
                print("Apply with MCP reqvire.format using fix: true and mutation enablement.")
    elif args.operation == "lint":
        # The existing lint suite compares the CLI inspection report after
        # typed MCP repair; repair evidence is checked in the boundary suite.
        print(f"✅ Fixed {data['applied_fixes']['relations_removed']} redundant verify relation(s)\n")
        print(f"Formatted {len(data['applied_fixes']['changed_files'])} file(s) with removed relations.\n")
        subprocess.run([binary, "--workspace", str(root), "lint"], check=True)
    else:
        verb = {"add": "Added", "remove": "Removed", "move": "Moved", "rename": "Renamed", "update": "Updated", "merge": "Merged"}[data["operation"]]
        print(f"{verb} element: {data['element_name']} ({data['element_id']})")
        print("Dry run - no files modified\n" if args.dry_run else "")
        if data["diffs"]:
            file_diffs(data["diffs"])
        else:
            print("No file changes.")


if __name__ == "__main__":
    sys.exit(main())
