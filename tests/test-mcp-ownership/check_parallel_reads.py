#!/usr/bin/env python3
"""Real fixed-root worker regressions, including out-of-order pipe responses."""
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import threading
import time
import traceback

parser = argparse.ArgumentParser()
parser.add_argument('--binary', required=True)
args = parser.parse_args()
binary = str(Path(args.binary).resolve())
fixtures = Path(__file__).resolve().parents[1] / 'test-cache-integration/fixtures'
model = (fixtures / 'model.md.txt').read_text()
other = (fixtures / 'other.md.txt').read_text().split('# Elements\n\n', 1)[1]
real_git = shutil.which('git')


def scenario(commits):
    with tempfile.TemporaryDirectory(prefix='reqvire-parallel-worker-') as tmp:
        base = Path(tmp)
        root = base / 'repo'
        root.mkdir()
        def git(*argv):
            return subprocess.check_output([real_git, *argv], cwd=root, stderr=subprocess.PIPE).decode().strip()
        git('init', '-qb', 'main')
        git('config', 'user.name', 'Worker Test')
        git('config', 'user.email', 'worker@example.invalid')
        (root / 'Model.md').write_text(model)
        git('add', '.')
        git('commit', '-qm', 'baseline')
        shim = base / 'git'
        shim.write_text('#!/usr/bin/env python3\n' + f'''
import os,sys,time
from pathlib import Path
base=Path({str(base)!r})
if sys.argv[1:]==['status','--porcelain'] and (base/'armed').exists():
    (base/('entered-'+str(os.getpid()))).write_text('ready')
    deadline=time.monotonic()+15
    while not (base/'release').exists():
        if time.monotonic()>deadline: sys.exit(99)
        time.sleep(0.005)
os.execv({real_git!r},[{real_git!r},*sys.argv[1:]])
''')
        shim.chmod(0o755)
        env = dict(os.environ, PATH=str(base)+os.pathsep+os.environ['PATH'])
        process = subprocess.Popen([binary, '__mcp-worktree-worker'], cwd=root, env=env,
                                   stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        condition = threading.Condition()
        received = {}
        errors = []
        def read():
            try:
                for line in process.stdout:
                    value = json.loads(line)
                    with condition:
                        received[value.get('request_id', 'ready')] = value
                        condition.notify_all()
            except Exception as error:
                errors.append(error)
            finally:
                with condition:
                    condition.notify_all()
        reader = threading.Thread(target=read, daemon=True)
        reader.start()
        def send(request_id=None, **value):
            if request_id is not None:
                value['request_id'] = request_id
            process.stdin.write(json.dumps(value)+'\n')
            process.stdin.flush()
        def receive(request_id, seconds=5):
            deadline = time.monotonic()+seconds
            with condition:
                while request_id not in received:
                    remaining = deadline-time.monotonic()
                    assert remaining > 0 and process.poll() is None and not errors, (request_id, list(received), errors)
                    condition.wait(min(remaining, 0.1))
                return received.pop(request_id)
        def tool(request_id, tool_name, **arguments):
            send(request_id, operation='rpc', method='tools/call', params=dict(name=tool_name, arguments=arguments))
        def ready_count(count):
            deadline = time.monotonic()+3
            while len(list(base.glob('entered-*'))) < count and time.monotonic()<deadline:
                time.sleep(0.01)
            assert len(list(base.glob('entered-*'))) >= count, 'reads serialized before their shared barrier'
        try:
            send(worktree_id='parallel', read_only=False, enable_commits=commits,
                 with_size_estimates=False, explorer=True, read_parallelism=6)
            initial = receive('ready')
            assert initial['status']['available'], initial
            before = initial['status']
            (base / 'armed').touch()
            for request_id in range(1, 6):
                tool(request_id, 'reqvire.workspace_status')
            ready_count(5)
            tool(6, 'reqvire.search')
            fast = receive(6)
            assert 'Cache Subject' in json.dumps(fast['result']) and not fast['result'].get('isError'), fast
            tool(7, 'reqvire.workspace_status')
            ready_count(6)
            # Only the six reads already at the barrier should wait. Explorer
            # publication also observes Git status and must remain independent.
            (base / 'armed').unlink()
            tool(8, 'reqvire.search')
            overloaded = receive(8)
            assert overloaded['rpc_error']['code'] == -32000, overloaded
            assert overloaded['rpc_error']['data']['retryable'], overloaded
            tool(9, 'reqvire.add_element', file='Model.md', content=other, dry_run=True)
            preview = receive(9)
            assert not preview['result'].get('isError') and preview['status']['model_revision'] == before['model_revision'], preview
            tool(10, 'reqvire.add_element', file='Model.md', content=model.split('# Elements\n\n',1)[1])
            rejected = receive(10)
            assert rejected['result']['isError'] and rejected['status']['model_revision'] == before['model_revision'], rejected
            tool(11, 'reqvire.add_element', file='Model.md', content=other)
            written = receive(11)
            assert not written['result'].get('isError'), written
            assert written['status']['model_revision'] != before['model_revision'], written
            (base / 'release').touch()
            for request_id in [7, 5, 1, 4, 2, 3]:
                old = receive(request_id)
                assert old['result']['structuredContent']['model']['fingerprint'] == before['model_revision'], old
                assert old['read_status']['model_revision'] == before['model_revision'], old
                assert old['read_status']['head'] == before['head'], old
                assert old['read_status']['pending_changes'] == before['pending_changes'], old
                assert old['read_status']['writes_available'], old
                assert old['status']['model_revision'] == written['status']['model_revision'], old
                assert old['sequence'] > written['sequence'], old
            (root / 'Model.md').write_text('external invalid bytes')
            tools = [('reqvire.workspace_status', {}), ('reqvire.search', {}),
                     ('reqvire.read_element', {'name':'Other Subject'}), ('reqvire.coverage', {}),
                     ('reqvire.lint', {}), ('reqvire.traces', {}),
                     ('reqvire.semantic.sparql', {'query':'ASK { ?s ?p ?o }'})]
            for n, (name, arguments) in enumerate(tools):
                ids = list(range(100+n*5, 105+n*5))
                for request_id in ids:
                    tool(request_id, name, **arguments)
                results = [receive(request_id) for request_id in reversed(ids)]
                assert all(not r['result'].get('isError') for r in results), results
                assert all(r['read_status']['model_revision'] == written['status']['model_revision'] for r in results), results
                assert all(r['result'] == results[0]['result'] for r in results), results
            # Distinct filters must retain their own replies even when the pipe
            # completes requests out of order.
            for request_id, label in [(501, 'Cache Subject'), (502, 'Other Subject')]:
                tool(request_id, 'reqvire.search', filter_name=label)
            for request_id, label in [(502, 'Other Subject'), (501, 'Cache Subject')]:
                result = receive(request_id)['result']
                assert not result.get('isError'), result
                names = [element['name'] for file in result['structuredContent']['files'].values()
                         for element in file['elements']]
                assert names == [label], (request_id, names)
        finally:
            (base / 'release').touch()
            process.stdin.close()
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait(timeout=5)
            reader.join(timeout=2)
            stderr = process.stderr.read()
            if process.returncode:
                print(stderr)

failed = False
for commits in (False, True):
    label = 'commits-enabled' if commits else 'commits-disabled'
    try:
        scenario(commits)
        print(f'PASS parallel-worker/{label}')
    except Exception:
        failed = True
        print(f'FAIL parallel-worker/{label}')
        traceback.print_exc()
raise SystemExit(failed)
