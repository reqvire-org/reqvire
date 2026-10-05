"""Offline publication regressions through the actual HTTP MCP transports."""
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import traceback

binary, destination = sys.argv[1:]
suite = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('cache_checks', suite.parent / 'test-cache-integration/check_correctness.py')
helpers = importlib.util.module_from_spec(spec)
spec.loader.exec_module(helpers)
output = Path(destination) / 'output'
output.mkdir(exist_ok=True)
failures = []
def check(name, ok, detail=''):
    print(f"{'PASS' if ok else 'FAIL'} {name}", flush=True)
    if not ok:
        failures.append(name)
        print(detail, file=sys.stderr)
def git(root, *args):
    return subprocess.check_output(['git', *args], cwd=root, text=True).strip()

for mode in ('mcp', 'serve'):
    try:
        with tempfile.TemporaryDirectory(prefix='reqvire-publication-') as temp:
            directory = Path(temp)
            root = directory / 'repo'
            root.mkdir()
            git(root, 'init', '-qb', 'main')
            git(root, 'config', 'user.name', 'Publication Test')
            git(root, 'config', 'user.email', 'test@example.invalid')
            fixture = suite.parent / 'test-cache-integration'
            (root / 'Model.md').write_text((fixture / 'fixtures/model.md.txt').read_text())
            git(root, 'add', '.')
            git(root, 'commit', '-qm', 'baseline')
            remote = directory / 'bare.git'
            git(root, 'init', '--bare', str(remote))
            git(root, 'remote', 'add', 'origin', 'https://github.com/test/project.git')
            state = directory / 'state'
            state.mkdir()
            (state / 'scenario.json').write_text('{}')
            bins = directory / 'bin'
            bins.mkdir()
            shutil.copy2(suite / 'fixtures/gh-double.py', bins / 'gh')
            (bins / 'gh').chmod(0o755)
            # The real git binary handles all commands. Network transports are redirected
            # to one local bare fixture; any other network URL is rejected by the wrapper.
            real_git = shutil.which('git')
            (bins / 'git').write_text('#!/usr/bin/python3\n' +
                'import os, sys\nargs = sys.argv[1:]\n' +
                f'args = [{str(remote)!r} if a == "https://github.com/test/project.git" and ("push" in args or "ls-remote" in args) else a for a in args]\n' +
                'if ("push" in args or "ls-remote" in args) and any(a.startswith(("https://", "ssh://", "git@")) for a in args): sys.exit(99)\n' +
                f'os.execv({real_git!r}, [{real_git!r}, *args])\n')
            (bins / 'git').chmod(0o755)
            env = {'PATH': str(bins) + os.pathsep + os.environ['PATH'], 'REQVIRE_GH_TEST_STATE': str(state)}
            with helpers.Server(binary, root, output, mode, mode, mutations=True,
                                extra_args=['--enable-github'], environment=env) as server:
                tools = server.rpc('tools/list', {})['tools']
                publication = [tool for tool in tools if tool['name'] in ('reqvire.git.push', 'reqvire.github.pr.create', 'reqvire.github.pr.comment')]
                check(f'{mode}/discovery', len(publication) == 3 and all(t['annotations']['openWorldHint'] for t in publication))
                initial = git(root, 'rev-parse', 'HEAD')
                pushed = server.tool('reqvire.git.push')
                check(f'{mode}/push-exact-head', pushed['commit'] == git(remote, 'rev-parse', 'refs/heads/main') == initial)
                check(f'{mode}/push-noop', server.tool('reqvire.git.push')['outcome'] == 'no_op')
                created = server.tool('reqvire.worktree.create', branch='feature', base_ref='main')
                context = created['worktree_id']
                content = (fixture / 'fixtures/other.md.txt').read_text().split('# Elements\n\n', 1)[1]
                server.tool('reqvire.add_element', worktree_id=context, file='Model.md', content=content)
                check(f'{mode}/pending-push-rejected', server.raw_tool('reqvire.git.push', worktree_id=context).get('isError') is True)
                committed = server.tool('reqvire.git.commit', worktree_id=context, message='Accepted feature')
                check(f'{mode}/unpushed-pr-rejected', server.raw_tool('reqvire.github.pr.create', worktree_id=context,
                      base='main', title='Feature', body='Review').get('isError') is True)
                server.tool('reqvire.git.push', worktree_id=context)
                body = '--flag\n\nQuotes " and `code` $(literal)\n'
                (state / 'scenario.json').write_text(json.dumps({'lost_create': True}))
                pr = server.tool('reqvire.github.pr.create', worktree_id=context, base='main', title='Feature', body=body, draft=True)
                again = server.tool('reqvire.github.pr.create', worktree_id=context, base='main', title='Changed ignored', body='ignored')
                calls = [json.loads(line) for line in (state / 'calls.jsonl').read_text().splitlines()]
                creates = [call for call in calls if call['args'][:2] == ['pr', 'create']]
                check(f'{mode}/pr-reconcile-no-duplicate', pr['pr_number'] == 7 and again['outcome'] == 'no_op'
                      and len(creates) == 1 and creates[0]['body'] == body)
                (state / 'scenario.json').write_text('{}')
                comment = server.tool('reqvire.github.pr.comment', worktree_id=context, pr_number=22, body=body)
                check(f'{mode}/comment-exact-body', comment['comment_id'] == 123
                      and json.loads((state / 'comments.jsonl').read_text())['body'] == body)
                (state / 'scenario.json').write_text(json.dumps({'lost_comment': True}))
                unknown = server.raw_tool('reqvire.github.pr.comment', worktree_id=context, pr_number=22, body=body)
                check(f'{mode}/ambiguous-comment-no-retry', unknown.get('isError') is True
                      and unknown['structuredContent']['outcome'] == 'unknown'
                      and len((state / 'comments.jsonl').read_text().splitlines()) == 2)
                (state / 'scenario.json').write_text(json.dumps({'fail': 'pr comment'}))
                denied = server.raw_tool('reqvire.github.pr.comment', worktree_id=context, pr_number=22, body='Denied')
                check(f'{mode}/denial-preserves-state', denied.get('isError') is True
                      and 'secret-token' not in json.dumps(denied)
                      and git(Path(created['workspace_root']), 'rev-parse', 'HEAD') == committed['commit'])
            (state / 'scenario.json').write_text(json.dumps({'fail': 'auth status'}))
            with helpers.Server(binary, root, output, mode, mode + '-disabled', mutations=True,
                                extra_args=['--enable-github'], environment=env) as server:
                names = [tool['name'] for tool in server.rpc('tools/list', {})['tools']]
                status = server.tool('reqvire.workspace_status')
                check(f'{mode}/failed-check-disables-only-publication', 'reqvire.git.push' not in names
                      and 'reqvire.git.commit' in names and 'reqvire.worktree.create' in names
                      and status['github']['available'] is False)
                try:
                    server.raw_tool('reqvire.git.push')
                    rejected = False
                except RuntimeError as failure:
                    rejected = '-32602' in str(failure)
                check(f'{mode}/disabled-direct-call-rejected', rejected)
    except Exception:
        check(f'{mode}/publication', False, traceback.format_exc())
raise SystemExit(bool(failures))
