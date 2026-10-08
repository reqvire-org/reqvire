"""Check taxonomy validation and mutation rejection against actual CLI files."""
import pathlib
import subprocess
import sys

binary, root = sys.argv[1], pathlib.Path(sys.argv[2])
path = root / 'specifications/Projection.md'
original = path.read_bytes()
source = original.decode()
parent_link = 'narrower: [Projection Child](#projection-child)'
failures = []


def files():
    return {str(p.relative_to(root)): p.read_bytes() for p in (root / 'specifications').glob('*.md')}


def run(*args):
    return subprocess.run([binary, *args], cwd=root, text=True, capture_output=True, timeout=15)


def reject(label, args):
    before = files()
    result = run(*args)
    output = result.stdout + result.stderr
    problems = []
    if result.returncode == 0:
        problems.append('accepted invalid taxonomy')
    if 'Concept taxonomy cycle' not in output or not all(name in output for name in ('projection-child', 'projection-parent')):
        problems.append('missing taxonomy path diagnostic')
    if files() != before:
        problems.append('rejected operation changed source files')
    if problems:
        failures.append(label)
        print(f'FAIL {label}: {", ".join(problems)}\n{output}', file=sys.stderr)
    else:
        print(f'PASS {label}')
    # Keep later cases independent even when an unfixed command persisted a cycle.
    for relative, content in before.items():
        (root / relative).write_bytes(content)



try:
    for suffix, replacement in [
        ('broader', 'broader: [Projection Child](#projection-child)'),
        ('mixed', 'broader: [Projection Peer](#projection-peer)'),
    ]:
        invalid = source.replace(parent_link, replacement)
        if suffix == 'mixed':
            invalid = invalid.replace('related: [Projection Peer](#projection-peer)', 'narrower: [Projection Peer](#projection-peer)')
        path.write_text(invalid)
        reject('validate-' + suffix, ['validate'])
        reject('export-' + suffix, ['semantic', 'export', '--layer', 'concepts'])
    path.write_bytes(original)
    assert run('validate').returncode == 0, 'removing cycle edge did not restore validation'
    print('PASS repaired-taxonomy')
    replacement = '''### Projection Parent
Parent with an invalid closing edge.
#### Metadata
  * type: concept
#### Relations
  * derivedFrom: [Projection Alpha Scheme](#projection-alpha-scheme)
  * broader: [Projection Child](#projection-child)
'''
    for dry_run in [False, True]:
        flags = ['--dry-run'] if dry_run else []
        label = 'dry-run' if dry_run else 'persist'
        reject('link-' + label, ['link', 'Projection Parent', 'broader', 'Projection Child', *flags])
        reject('replace-' + label, ['add', 'specifications/Projection.md', '--override', '--content', replacement, *flags])
finally:
    path.write_bytes(original)

if failures:
    raise SystemExit("FAILED taxonomy checks: " + ", ".join(failures))
