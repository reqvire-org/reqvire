"""Worktree routing and accepted-only commits through real standalone/embedded MCP."""
import importlib.util
import json
import os
import shutil
import pathlib
import subprocess
import sys
import tempfile
import traceback
import time
import urllib.error

binary, destination = sys.argv[1:]
suite = pathlib.Path(__file__).resolve().parent
helpers_spec = importlib.util.spec_from_file_location('cache_checks', suite.parent / 'test-cache-integration/check_correctness.py')
helpers = importlib.util.module_from_spec(helpers_spec)
helpers_spec.loader.exec_module(helpers)
output = pathlib.Path(destination) / 'output'
output.mkdir(exist_ok=True)
failures = []


def check(name, ok, detail=''):
    print(f"{'PASS' if ok else 'FAIL'} {name}", flush=True)
    if not ok:
        failures.append(name)
        print(f'{name}: {detail}', file=sys.stderr)


def git(root, *args):
    return subprocess.check_output(['git', *args], cwd=root, text=True).strip()


def selector_error(result, tool, *diagnostics):
    error = result.get('structuredContent', {}).get('error', {})
    return (result.get('isError') is True and error.get('tool') == tool
            and all(text in error.get('message', '') for text in diagnostics)
            and '_meta' not in result and 'context' not in result.get('structuredContent', {}))


for mode in ('mcp', 'serve'):
    base = [binary, mode, *(['--enable-mcp'] if mode == 'serve' else [])]
    help_result = subprocess.run([*base, '--help'], capture_output=True, text=True)
    check(f'{mode}/github-help', '--enable-github' in help_result.stdout and '--github-remote' in help_result.stdout, help_result.stdout)
    for suffix, args in [('github-requires-mutations', ['--enable-github']),
                         ('remote-requires-github', ['--enable-mutations', '--github-remote', 'upstream'])]:
        result = subprocess.run([*base, *args], capture_output=True, text=True)
        check(f'{mode}/{suffix}', result.returncode != 0 and 'required arguments' in result.stderr,
              result.stderr)

# Fail at the missing contract, rather than attempting to launch an unsupported server.
if failures:
    raise SystemExit(1)

for embedded in (False, True):
    mode = 'read-only-mcp' if embedded else 'plain'
    server = None
    try:
        with tempfile.TemporaryDirectory(prefix='reqvire-read-worktrees-') as temp:
            root = pathlib.Path(temp) / 'repo'
            second = pathlib.Path(temp) / 'second'
            invalid = pathlib.Path(temp) / 'invalid'
            root.mkdir()
            git(root, 'init', '-qb', 'main')
            git(root, 'config', 'user.email', 'test@example.invalid')
            git(root, 'config', 'user.name', 'Read Test')
            model = (suite.parent / 'test-cache-integration/fixtures/model.md.txt').read_text()
            (root / 'Model.md').write_text(model)
            (root / 'shared.txt').write_text('original bytes')
            git(root, 'add', '.')
            git(root, 'commit', '-qm', 'baseline')
            git(root, 'branch', 'unopened')
            git(root, 'worktree', 'add', '-qb', 'second', str(second))
            git(root, 'worktree', 'add', '-qb', 'invalid', str(invalid))
            (root / 'Model.md').write_text(model.replace('Cache Subject', 'Dirty Subject'))
            (second / 'Model.md').write_text(model.replace('Cache Subject', 'Other Subject'))
            (second / 'shared.txt').write_text('second bytes')
            (invalid / 'Model.md').write_text(model.replace('capability', 'invalid-type'))
            before = [(git(p, 'rev-parse', 'HEAD'), git(p, 'status', '--porcelain=v1')) for p in (root, second, invalid)]
            server = helpers.Server(binary, root, output, 'serve', mode, embedded_mcp=embedded)
            try:
                for _ in range(200):
                    if server.process.poll() is not None:
                        raise RuntimeError(f'{mode} exited: {(server.output / "server.log").read_text()}')
                    try:
                        status, body = server.http('/api/worktrees')
                        if status == 200:
                            break
                    except (urllib.error.URLError, TimeoutError, ConnectionError):
                        pass
                    time.sleep(0.05)
                else:
                    raise RuntimeError(f'{mode} inventory did not become available')
                inventory = json.loads(body)
                rows = {r['branch']: r for r in inventory['worktrees']}
                check(f'{mode}/read-only-inventory', set(rows) == {'main', 'second', 'invalid', 'unopened'}
                      and all(r['owned'] is False for r in rows.values())
                      and all(rows[b]['state'] == 'unloaded' for b in ('second', 'invalid', 'unopened')))
                counts = server.counts()
                server.http('/api/worktrees')
                check(f'{mode}/inventory-does-not-load', server.counts() == counts
                      and len(git(root, 'worktree', 'list', '--porcelain').split('worktree ')) - 1 == 3)
                for branch, name, asset in [('main', 'Dirty Subject', 'original bytes'), ('second', 'Other Subject', 'second bytes')]:
                    ident = rows[branch]['worktree_id']
                    status, body = server.http('/api/project-store?worktree_id=' + ident)
                    store = json.loads(body)['store']
                    check(f'{mode}/{branch}-snapshot', status == 200 and store['project']['branch'] == branch
                          and any(e['name'] == name for e in store['elements']))
                    status, data = server.http('/shared.txt?worktree_id=' + ident)
                    check(f'{mode}/{branch}-asset', status == 200 and data == asset)
                    _, seed = server.http('/assets/project-store.js?worktree_id=' + ident)
                    check(f'{mode}/{branch}-no-live-refresh', ident in seed and 'reqvireLiveRefresh' not in seed)
                status, shell = server.http('/')
                check(f'{mode}/picker-enabled', status == 200 and 'window.reqvireWorktreeRouting = true' in shell)
                for selector in ('unknown', rows['invalid']['worktree_id']):
                    status, _ = server.http('/api/project-store?worktree_id=' + selector)
                    check(f'{mode}/unavailable-' + ('unknown' if selector == 'unknown' else 'invalid'), status == 503)
                if embedded:
                    server.__enter__()
                    definitions = server.rpc('tools/list', {})['tools']
                    check(f'{mode}/mutation-tools-hidden', not any(d['name'] in ('reqvire.add_element', 'reqvire.worktree.create', 'reqvire.git.commit') for d in definitions))
                    try:
                        rejected = server.raw_tool('reqvire.add_element', file='Model.md', content='invalid').get('isError') is True
                    except RuntimeError:
                        response = json.loads((server.output / f'response-{server.request_id:03}.json').read_text())['response']
                        rejected = response.get('error', {}).get('code') == -32602
                    check(f'{mode}/mutation-rejected', rejected)
                run = subprocess.run(['node', str(suite / 'browser-check.mjs'), server.base, rows['main']['worktree_id'], rows['second']['worktree_id'], str(output / f'{mode}-browser-profile')], capture_output=True, text=True, timeout=100)
                check(f'{mode}/browser-selection', run.returncode == 0, run.stdout + run.stderr)
                check(f'{mode}/git-unchanged', before == [(git(p, 'rev-parse', 'HEAD'), git(p, 'status', '--porcelain=v1')) for p in (root, second, invalid)]
                      and not [p for p in (root / '.git').glob('reqvire-mcp*') if p.name != 'reqvire-mcp-administration.lock'])
                selected = rows['second']['worktree_id']
                counts = server.counts()
                status, _ = server.http('/api/worktrees/load', {'worktree_id': selected})
                check(f'{mode}/unchanged-load-reuses-cache', status == 200 and server.counts()['builds'] == counts['builds'])
                # Equal-length edits with restored timestamps must still invalidate.
                path = second / 'Model.md'
                times = path.stat()
                path.write_text(model.replace('Cache Subject', 'Fresh Subject'))
                os.utime(path, ns=(times.st_atime_ns, times.st_mtime_ns))
                status, _ = server.http('/api/worktrees/load', {'worktree_id': selected})
                _, body = server.http('/api/project-store?worktree_id=' + selected)
                check(f'{mode}/read-only-refresh-on-demand', status == 200 and 'Fresh Subject' in body
                      and server.counts()['builds'] > counts['builds'])
                path.write_text(model.replace('capability', 'invalid-type'))
                status, _ = server.http('/api/worktrees/load', {'worktree_id': selected})
                path.write_text(model.replace('Cache Subject', 'Fresh Subject'))
                recovered, _ = server.http('/api/worktrees/load', {'worktree_id': selected})
                check(f'{mode}/invalid-refresh-recovers', status == 503 and recovered == 200)
                status, loaded = server.http('/api/worktrees/load', {'worktree_id': rows['unopened']['worktree_id']})
                prepared = pathlib.Path(json.loads(loaded)['workspace_root'])
                check(f'{mode}/branch-worktree-created-on-demand', status == 200 and prepared != root
                      and git(prepared, 'branch', '--show-current') == 'unopened'
                      and 'Cache Subject' in (prepared / 'Model.md').read_text())
            finally:
                server.__exit__()
                server = None
            check(f'{mode}/prepared-worktree-persists', prepared.is_dir())
    except Exception:
        check(f'{mode}/lifecycle', False, traceback.format_exc())

for mode in ('mcp', 'serve'):
    try:
        with tempfile.TemporaryDirectory(prefix='reqvire-worktrees-e2e-') as temp:
            root = pathlib.Path(temp) / 'repo'
            root.mkdir()
            git(root, 'init', '-qb', 'main')
            git(root, 'config', 'user.email', 'test@example.invalid')
            git(root, 'config', 'user.name', 'Worktree Test')
            fixture = suite.parent / 'test-cache-integration'
            original = (fixture / 'fixtures/model.md.txt').read_text()
            (root / 'Model.md').write_text(original)
            git(root, 'add', '.')
            git(root, 'commit', '-qm', 'baseline')
            initial = git(root, 'rev-parse', 'HEAD')
            with helpers.Server(binary, root, output, mode, mode, mutations=True) as server:
                inventory = server.tool('reqvire.worktree.list')['worktrees']
                origin = next(row for row in inventory if row.get('original'))['worktree_id']
                created = server.tool('reqvire.worktree.create', branch='feature-a', base_ref='main')
                child = created['worktree_id']
                check(f'{mode}/create-isolated-context', child != origin and created['base_commit'] == initial
                      and git(root, 'branch', '--show-current') == 'main')
                same = server.tool('reqvire.worktree.open', branch='feature-a')
                check(f'{mode}/open-idempotent', same['worktree_id'] == child)
                ambiguous = server.raw_tool('reqvire.search')
                check(f'{mode}/ambiguous-selector-rejected', selector_error(
                    ambiguous, 'reqvire.search', 'worktree_id is required', origin, child), ambiguous)
                unknown = server.raw_tool('reqvire.search', worktree_id='unknown-context')
                check(f'{mode}/unknown-selector-tool-error', selector_error(
                    unknown, 'reqvire.search', 'Unknown or removed worktree_id: unknown-context'), unknown)
                try:
                    server.raw_tool('reqvire.search', worktree_id=42)
                    invalid = {}
                except RuntimeError:
                    invalid = json.loads((server.output / f'response-{server.request_id:03}.json').read_text())['response']
                check(f'{mode}/invalid-selector-protocol-error', invalid.get('error', {}).get('code') == -32602
                      and 'result' not in invalid, invalid)
                content = (fixture / 'fixtures/other.md.txt').read_text().split('# Elements\n\n', 1)[1]
                server.tool('reqvire.add_element', worktree_id=child, file='Model.md', content=content)
                a = server.tool('reqvire.search', worktree_id=origin)
                b = server.tool('reqvire.search', worktree_id=child)
                check(f'{mode}/model-isolation', 'Other Subject' not in str(a) and 'Other Subject' in str(b)
                      and (root / 'Model.md').read_text() == original)
                semantic = server.tool('reqvire.semantic.sparql', worktree_id=child,
                    query='ASK { ?s <https://www.reqvire.org/ontology#elementName> "Other Subject" }')
                check(f'{mode}/semantic-isolation', semantic['boolean'] is True)
                pending = server.raw_tool('reqvire.worktree.remove', worktree_id=child)
                check(f'{mode}/dirty-removal-rejected', pending.get('isError') is True)
                child_root = pathlib.Path(created['workspace_root'])
                (child_root / 'unrelated.txt').write_text('external staged work')
                git(child_root, 'add', 'unrelated.txt')
                commit = server.tool('reqvire.git.commit', worktree_id=child, message='Accepted model\n\nExplicit commit.')
                expected = (fixture / 'expected/runtime-after-write.md.txt').read_text()
                actual = subprocess.check_output(['git', 'show', 'HEAD:Model.md'], cwd=child_root, text=True)
                check(f'{mode}/accepted-only-commit', actual == expected
                      and git(child_root, 'diff-tree', '--no-commit-id', '--name-only', '-r', 'HEAD') == 'Model.md'
                      and git(child_root, 'diff', '--cached', '--name-only') == 'unrelated.txt'
                      and commit['commit'] == git(child_root, 'rev-parse', 'HEAD')
                      and git(root, 'rev-parse', 'HEAD') == initial)
                noop = server.tool('reqvire.git.commit', worktree_id=child, message='Nothing pending')
                check(f'{mode}/commit-noop', noop['outcome'] == 'no_op')
                git(child_root, 'reset', '-q', 'HEAD', '--', 'unrelated.txt')
                (child_root / 'unrelated.txt').unlink()
                protected = server.raw_tool('reqvire.worktree.remove', worktree_id=origin)
                check(f'{mode}/original-protected', protected.get('isError') is True)
                if mode == 'serve':
                    dirty_root = pathlib.Path(temp) / 'dirty'
                    git(root, 'worktree', 'add', '-qb', 'dirty', str(dirty_root))
                    git(root, 'branch', 'unopened')
                    _, listing = server.http('/api/worktrees')
                    rows = {r['branch']: r for r in json.loads(listing)['worktrees']}
                    check('serve/branches-listed-without-loading', rows['dirty']['state'] == rows['unopened']['state'] == 'unloaded')
                    for dirty_kind in ('unstaged', 'staged', 'untracked'):
                        path = dirty_root / ('Untracked.md' if dirty_kind == 'untracked' else 'Model.md')
                        path.write_text(original.replace('capability', 'invalid-type'))
                        if dirty_kind == 'staged':
                            git(dirty_root, 'add', '.')
                        before = git(dirty_root, 'status', '--porcelain=v1')
                        counts = server.counts()
                        status, failure = server.http('/api/worktrees/load', {'worktree_id': rows['dirty']['worktree_id']})
                        check('serve/reject-dirty-' + dirty_kind, status == 503 and 'clean' in failure
                              and git(dirty_root, 'status', '--porcelain=v1') == before
                              and server.counts()['builds'] == counts['builds'])
                        git(dirty_root, 'reset', '--hard', 'HEAD')
                        if dirty_kind == 'untracked':
                            path.unlink()
                    status, _ = server.http('/api/worktrees/load', {'worktree_id': rows['dirty']['worktree_id']})
                    check('serve/load-after-clean-admission', status == 200)
                    status, loaded = server.http('/api/worktrees/load', {'worktree_id': rows['unopened']['worktree_id']})
                    check('serve/create-selected-worktree', status == 200
                          and git(pathlib.Path(json.loads(loaded)['workspace_root']), 'branch', '--show-current') == 'unopened')
                    counts = server.counts()
                    (child_root / 'Model.md').write_text('external edit must not replace accepted state')
                    status, _ = server.http('/api/worktrees/load', {'worktree_id': child})
                    _, retained = server.http('/api/project-store?worktree_id=' + child)
                    check('serve/owned-selection-reuses-accepted-cache', status == 200 and 'Other Subject' in retained and server.counts() == counts)
                    (child_root / 'Model.md').write_text(expected)
                    status, a = server.http('/api/project-store?worktree_id=' + origin)
                    status_b, b = server.http('/api/project-store?worktree_id=' + child)
                    check('serve/runtime-isolation', status == status_b == 200
                          and 'Other Subject' not in a and 'Other Subject' in b)
                    browser_dirty = pathlib.Path(temp) / 'browser-dirty'
                    git(root, 'worktree', 'add', '-qb', 'browser-dirty', str(browser_dirty))
                    (browser_dirty / 'untracked.txt').write_text('preserve me')
                    _, listing = server.http('/api/worktrees')
                    browser_dirty_id = next(r['worktree_id'] for r in json.loads(listing)['worktrees'] if r['branch'] == 'browser-dirty')
                    browser_run = subprocess.run(['node', str(suite / 'browser-check.mjs'), server.base, origin, child, str(output / 'serve-browser-profile'), browser_dirty_id], capture_output=True, text=True, timeout=100)
                    check('serve/browser-worktree-selection', browser_run.returncode == 0, browser_run.stdout + browser_run.stderr)
                    status, _ = server.http('/api/project-store?worktree_id=unknown')
                    check('serve/unknown-runtime-rejected', status in (404, 410, 503))
                server.tool('reqvire.worktree.remove', worktree_id=child)
                removed = server.raw_tool('reqvire.search', worktree_id=child)
                remaining = server.raw_tool('reqvire.search', worktree_id=origin)
                check(f'{mode}/remove-keeps-branch', not child_root.exists()
                      and git(root, 'rev-parse', 'feature-a') == commit['commit']
                      and selector_error(removed, 'reqvire.search', child)
                      and remaining.get('_meta', {}).get('reqvire/context', {}).get('worktree_id') == origin)
    except Exception:
        check(f'{mode}/lifecycle', False, traceback.format_exc())
raise SystemExit(bool(failures))
