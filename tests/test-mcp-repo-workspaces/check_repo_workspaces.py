"""Repo-manifest acceptance through real CLI, HTTP MCP and embedded Explorer.

All expected outcomes are success. Cases accumulate failures; missing production
support is neither skipped nor converted into an expected exception. Fixture
repositories, responses, server logs and diffs stay under the runner's TEST_DIR.
"""
import difflib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import traceback
import urllib.parse
import xml.etree.ElementTree as ET


BINARY, DESTINATION = sys.argv[1:]
SUITE = Path(__file__).resolve().parent
ROOT = Path(DESTINATION) / 'repo-cases'
OUTPUT = Path(DESTINATION) / 'output/repo-cases'
ROOT.mkdir()
OUTPUT.mkdir()
spec = importlib.util.spec_from_file_location('cache_checks', SUITE.parent / 'test-cache-integration/check_correctness.py')
helpers = importlib.util.module_from_spec(spec)
spec.loader.exec_module(helpers)
REPO_SOURCE = Path(os.environ.get('REQVIRE_TEST_REPO_SOURCE', '/usr/share/git-repo'))
REPO = shutil.which('repo')
ENV = {**os.environ, 'GIT_ALLOW_PROTOCOL': 'file', 'GIT_TERMINAL_PROMPT': '0',
       'GIT_CONFIG_COUNT': '2', 'GIT_CONFIG_KEY_0': 'user.name',
       'GIT_CONFIG_VALUE_0': 'Repo Acceptance', 'GIT_CONFIG_KEY_1': 'user.email',
       'GIT_CONFIG_VALUE_1': 'repo-test@example.invalid'}
EXPECTED_NAMES = ['Central Capability', 'Central Consumer', 'Component Capability',
                  'Component Requirement', 'Component Specification']


def command(root, *args, env=None, timeout=60, check=True):
    result = subprocess.run(args, cwd=root, env=env or ENV, text=True,
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=timeout)
    if check and result.returncode:
        raise AssertionError(f'{args!r} failed ({result.returncode}):\n{result.stdout}\n{result.stderr}')
    return result


def git(root, *args):
    return command(root, 'git', *args).stdout.strip()


def commit(root, message):
    git(root, 'add', '.')
    git(root, '-c', 'commit.gpgSign=false', 'commit', '-qm', message)
    return git(root, 'rev-parse', 'HEAD')


def golden(actual, expected):
    path = SUITE / 'expected' / expected
    received = actual.read_text() if actual.exists() else ''
    wanted = path.read_text()
    diff = ''.join(difflib.unified_diff(wanted.splitlines(True), received.splitlines(True),
                                      fromfile=str(path), tofile=str(actual)))
    assert received == wanted, diff


def names(server, branch='main'):
    result = server.tool('reqvire.search', branch=branch)
    return sorted(e['name'] for f in result['files'].values() for e in f['elements'])


class Fixture:
    def __init__(self, name, cross_reference=False):
        self.base = ROOT / name
        self.base.mkdir()
        self.output = OUTPUT / name
        self.output.mkdir()
        self.remotes = self.base / 'remotes'
        self.remotes.mkdir()
        component_seed = self.base / 'component-seed'
        component_seed.mkdir()
        git(component_seed, 'init', '-qb', 'main')
        source = (SUITE / 'fixtures/component.md.txt').read_text()
        (component_seed / 'Model.md').write_text(source)
        self.alpha = commit(component_seed, 'Component alpha')
        (component_seed / 'Model.md').write_text(source.replace('behavior is alpha.', 'behavior is omega.'))
        self.omega = commit(component_seed, 'Component omega')
        git(self.base, 'clone', '-q', '--bare', str(component_seed), str(self.remotes / 'component'))
        self.central = self.base / 'central'
        self.central.mkdir()
        git(self.central, 'init', '-qb', 'main')
        text = (SUITE / 'fixtures/central.md.txt').read_text()
        if not cross_reference:
            text = text.replace('#### Contract References\n  * [Component Specification](repositories/component/Model.md#component-specification)\n\n', '')
        (self.central / 'Model.md').write_text(text)
        # Central Git excludes components; declared component Git ownership must
        # override that parent boundary, without admitting undeclared projects.
        (self.central / '.gitignore').write_text('/repositories\n/.repo\n')
        (self.central / '.reqvireignore').write_text('**/.repo/**\n')
        self.write_manifest(self.alpha)
        self.initial_central = commit(self.central, 'Central alpha selection')
        git(self.base, 'clone', '-q', '--bare', str(self.central), str(self.remotes / 'central'))
        git(self.central, 'remote', 'add', 'origin', str(self.remotes / 'central'))
        # Keep the locally available second revision reachable through Central.
        self.write_manifest(self.omega)
        self.review = commit(self.central, 'Central omega selection')
        git(self.central, 'branch', 'review')
        git(self.central, 'reset', '--hard', self.initial_central)
        runtime_sha = git(REPO_SOURCE, 'rev-parse', 'HEAD')
        with (self.output / 'repo-setup.log').open('w') as log:
            for args in (
                [REPO, 'init', '-q', '-u', (self.remotes / 'central').as_uri(), '-b', 'main',
                 '--repo-url', REPO_SOURCE.as_uri(), '--repo-rev', runtime_sha,
                 '--no-repo-verify', '--no-clone-bundle'],
                [REPO, 'sync', '-q', '--no-clone-bundle', '--no-tags'],
            ):
                result = command(self.central, *args, check=False)
                log.write(result.stdout + result.stderr)
                assert result.returncode == 0, f'Real Repo setup failed; see {log.name}'
        self.component = self.central / 'repositories/component'
        assert git(self.component, 'rev-parse', 'HEAD') == self.alpha, 'Repo did not honor the pinned revision'
        assert git(self.component, 'cat-file', '-t', self.omega) == 'commit', 'Review revision must be available offline'
        assert not git(self.central, 'status', '--porcelain'), 'Fixture Central is dirty'
        assert not git(self.component, 'status', '--porcelain'), 'Fixture component is dirty'
        (self.output / 'provenance.json').write_text(json.dumps({
            'repo_runtime_commit': runtime_sha, 'central': self.initial_central,
            'alpha': self.alpha, 'omega': self.omega, 'review': self.review}, indent=2))

    def write_manifest(self, sha, fetch=None):
        fetch = fetch or self.remotes.as_uri() + '/'
        (self.central / 'default.xml').write_text(
            '<manifest>\n'
            f'  <remote name="local" fetch="{fetch}"/>\n'
            '  <default remote="local" revision="main"/>\n'
            f'  <project name="component" path="repositories/component" revision="{sha}"/>\n'
            '</manifest>\n')

    def server(self, mode='mcp', commits=False, extra_args=(), environment=None):
        return helpers.Server(BINARY, self.central, self.output, mode, 'server',
                              mutations=True, commits=commits, extra_args=extra_args,
                              environment={**ENV, **(environment or {})})

    def git_state(self):
        return [(git(p, 'rev-parse', 'HEAD'), git(p, 'ls-files', '--stage'))
                for p in (self.central, self.component)]

    def files(self):
        paths = [self.central / 'Model.md', self.central / 'default.xml',
                 self.component / 'Model.md', self.component / 'Added.md']
        return {str(p): p.read_bytes() if p.exists() else None for p in paths}

    def add(self, server):
        return server.tool('reqvire.add_element', branch='main', file='repositories/component/Added.md',
                           content=(SUITE / 'fixtures/addition.md.txt').read_text())

    def saved(self, server, result):
        golden(self.component / 'Added.md', 'added.md.txt')
        component = git(self.component, 'rev-parse', 'HEAD')
        central = git(self.central, 'rev-parse', 'HEAD')
        assert component != self.alpha, 'The owning component received no commit'
        assert git(self.component, 'show', 'HEAD:Added.md') + '\n' == (SUITE / 'expected/added.md.txt').read_text()
        assert git(self.component, 'rev-parse', 'HEAD^') == self.alpha, 'Component commit has the wrong base'
        assert not git(self.central, 'ls-files', 'repositories'), 'Central tracked component files'
        manifest = ET.fromstring(git(self.central, 'show', 'HEAD:default.xml'))
        assert manifest.find('project').get('revision') == component, 'Central did not pin the committed component'
        assert central != self.initial_central and result.get('commit', result.get('context', {}).get('head')) == central
        assert not git(self.component, 'status', '--porcelain'), 'Committed component remains dirty'
        assert server.tool('reqvire.workspace_status', branch='main')['context']['pending_changes'] == []
        assert 'Added Capability' in names(server), 'Commit replaced the accepted model'
        return component, central


def complete_snapshot(mode):
    f = Fixture(mode + '-snapshot', cross_reference=True)
    cli = command(f.central, BINARY, 'search', '--json')
    cli_names = sorted(e['name'] for data in json.loads(cli.stdout)['files'].values() for e in data['elements'])
    assert cli_names == EXPECTED_NAMES, 'CLI did not see the complete Repo model'
    with f.server(mode) as server:
        assert names(server) == cli_names, 'MCP snapshot omitted declared component sources'
        result = server.tool('reqvire.read_element', branch='main', name='Component Specification')
        assert 'repositories/component/Model.md#component-specification' in json.dumps(result)
        status = server.tool('reqvire.workspace_status', branch='main')
        assert 'workspace_mode' in json.dumps(status) and 'repo' in json.dumps(status)


def explicit_manifest():
    f = Fixture('explicit-manifest', cross_reference=True)
    result = command(f.central, BINARY, '--manifest', 'default.xml', 'search', '--json')
    found = sorted(e['name'] for data in json.loads(result.stdout)['files'].values() for e in data['elements'])
    assert found == EXPECTED_NAMES, 'Explicit selection did not reach shared discovery'
    missing = command(f.central, BINARY, '--manifest', 'missing.xml', 'validate', check=False)
    assert missing.returncode != 0 and 'missing.xml' in missing.stderr + missing.stdout
    assert 'unexpected argument' not in missing.stderr, 'The selector was not implemented'


def invalid_manifest():
    f = Fixture('invalid-manifest')
    # Initialized Repo is the selector. No flag parsing can hide this rejection.
    (f.central / 'default.xml').write_text('<manifest>')
    before = f.files(), f.git_state()
    result = command(f.central, BINARY, 'validate', '--json', check=False)
    assert result.returncode != 0, 'Invalid selected Central manifest silently fell back to Git discovery'
    assert 'default.xml' in result.stdout + result.stderr, 'Rejection omitted the selected manifest path'
    assert (f.files(), f.git_state()) == before, 'Inspection changed rejected inputs'


def project_boundaries():
    f = Fixture('project-boundaries')
    manifest = (f.central / 'default.xml').read_text()
    invalid = {
        'duplicate-path': manifest.replace('</manifest>',
            f'  <project name="another" path="repositories/component" revision="{f.alpha}"/>\n</manifest>'),
        'escaping-path': manifest.replace('path="repositories/component"', 'path="../outside"'),
        'unavailable-commit': manifest.replace(f.alpha, '0' * 40),
        'missing-project': manifest.replace('path="repositories/component"', 'path="repositories/missing"'),
    }
    errors = []
    for label, text in invalid.items():
        (f.central / 'default.xml').write_text(text)
        before = f.files(), f.git_state()
        result = command(f.central, BINARY, 'validate', '--json', check=False)
        diagnostic = result.stdout + result.stderr
        (f.output / (label + '.log')).write_text(diagnostic)
        if result.returncode == 0 or 'default.xml' not in diagnostic:
            errors.append(label + ': invalid selected project was accepted or lacks its manifest diagnostic')
        assert (f.files(), f.git_state()) == before, 'Rejected selection changed files or Git state'
    assert not errors, '\n'.join(errors)


def declared_sources_only():
    f = Fixture('declared-only')
    extra = f.central / 'repositories/undeclared'
    extra.mkdir()
    git(extra, 'init', '-qb', 'main')
    (extra / 'Model.md').write_text('# Elements\n\n### Undeclared Capability\n\nOutside the selected manifest.\n\n#### Metadata\n  * type: capability\n')
    commit(extra, 'Undeclared source')
    result = command(f.central, BINARY, 'search', '--json')
    found = sorted(e['name'] for data in json.loads(result.stdout)['files'].values() for e in data['elements'])
    assert found == EXPECTED_NAMES, f'Manifest-defined discovery scanned undeclared sources: {found}'


def branch_revisions():
    f = Fixture('branch-revisions')
    original = f.files(), f.git_state()
    with f.server() as server:
        child = server.tool('reqvire.worktree.create', branch='topic', from_branch='main', base_ref='review')
        target = Path(child['workspace_root'])
        assert child['base_commit'] == f.review, 'base_ref did not select Central'
        project = target / 'repositories/component'
        assert project.is_dir(), 'Central context did not prepare its declared project'
        assert git(project, 'rev-parse', 'HEAD') == f.omega
        assert (target / '.repo').resolve() != (f.central / '.repo').resolve(), 'Repo metadata was shared'
        assert 'omega' in json.dumps(server.tool('reqvire.read_element', branch='topic', name='Component Specification'))
        assert 'alpha' in json.dumps(server.tool('reqvire.read_element', branch='main', name='Component Specification'))
        assert (f.files(), f.git_state()) == original, 'Branch creation altered the original project checkout'
        assert 'topic' not in git(f.component, 'branch', '--format=%(refname:short)').splitlines(), 'Invented component branch'
        server.tool('reqvire.worktree.remove', branch='topic')
        reopened = server.tool('reqvire.worktree.open', branch='topic')
        restored = Path(reopened['workspace_root']) / 'repositories/component'
        assert git(restored, 'rev-parse', 'HEAD') == f.omega, 'Reopening lost the retained manifest selection'
        assert 'omega' in json.dumps(server.tool('reqvire.read_element', branch='topic', name='Component Specification'))


def cross_project_mutation():
    f = Fixture('cross-project', cross_reference=True)
    before_files, before_git = f.files(), f.git_state()
    with f.server() as server:
        revision = server.revision()
        preview = server.tool('reqvire.rename_element', branch='main', element_name='Component Specification',
                              new_name='Updated Component Specification', dry_run=True)
        assert 'Model.md' in json.dumps(preview) and 'repositories/component/Model.md' in json.dumps(preview)
        assert f.files() == before_files and f.git_state() == before_git and server.revision() == revision
        rejected = server.raw_tool('reqvire.rename_element', branch='main', element_name='Component Specification',
                                   new_name='Central Capability')
        assert rejected.get('isError') is True and f.files() == before_files
        server.tool('reqvire.rename_element', branch='main', element_name='Component Specification',
                    new_name='Updated Component Specification')
        golden(f.central / 'Model.md', 'central-renamed.md.txt')
        golden(f.component / 'Model.md', 'component-renamed.md.txt')
        assert f.git_state() == before_git, 'Default mutation changed an index or HEAD'
        assert 'Updated Component Specification' in names(server) and server.revision() != revision
        assert set(server.tool('reqvire.workspace_status')['context']['pending_changes']) == {
            'Model.md', 'repositories/component/Model.md'}


def component_commit(automatic):
    f = Fixture('automatic-commit' if automatic else 'explicit-commit')
    before = f.git_state()
    with f.server(commits=automatic) as server:
        result = f.add(server)
        golden(f.component / 'Added.md', 'added.md.txt')
        if not automatic:
            assert f.git_state() == before, 'Pending acceptance changed Git state'
            result = server.tool('reqvire.git.commit', branch='main', message='Save combined component addition')
        f.saved(server, result)
        saved = f.git_state()
        server.tool('reqvire.git.commit', branch='main', message='No further changes')
        assert f.git_state() == saved, 'No-op created an extra commit'


def dirty_component():
    f = Fixture('dirty-component')
    (f.component / 'Model.md').write_text((f.component / 'Model.md').read_text().replace('alpha.', 'dirty.'))
    before = f.files(), f.git_state()
    server = f.server()
    admitted = False
    try:
        with server:
            admitted = True
    except RuntimeError:
        log = (server.output / 'server.log').read_text()
        assert 'repositories/component' in log and any(w in log.lower() for w in ('dirty', 'clean', 'modified')), log
    assert not admitted, 'A dirty declared component was admitted by ignoring its repository'
    assert (f.files(), f.git_state()) == before, 'Admission modified the rejected component'


def failed_central_commit():
    f = Fixture('central-commit-failure')
    with f.server() as server:
        f.add(server)
        # A real Git ref transaction rejects only the final Central checkpoint.
        # The component commit must survive for explicit recovery.
        hook = f.central / '.git/hooks/reference-transaction'
        hook.write_text('#!/bin/sh\nif [ "$1" = prepared ]; then exit 1; fi\nexit 0\n')
        hook.chmod(0o755)
        outcome = server.raw_tool('reqvire.git.commit', branch='main', message='Rejected Central checkpoint')
        assert outcome.get('isError') is True, 'Failed Central checkpoint reported success'
        component = git(f.component, 'rev-parse', 'HEAD')
        assert component != f.alpha, 'Completed component commit was lost or never made'
        assert git(f.central, 'rev-parse', 'HEAD') == f.initial_central, 'Rejected Central ref changed'
        golden(f.component / 'Added.md', 'added.md.txt')
        assert git(f.component, 'show', 'HEAD:Added.md') + '\n' == (SUITE / 'expected/added.md.txt').read_text()
        status = server.tool('reqvire.workspace_status', branch='main')
        assert status['context']['pending_changes'], 'Failed checkpoint cleared pending work'
        assert 'Added Capability' in names(server), 'Failure discarded the accepted model'


def explorer():
    f = Fixture('explorer')
    with f.server('serve') as server:
        status, body = server.http('/api/project-store?branch=main')
        assert status == 200
        store = json.loads(body)['store']
        assert sorted(e['name'] for e in store['elements']) == EXPECTED_NAMES, 'Explorer published a Central-only snapshot'
        status, baseline = server.http('/api/project-store/manifest?branch=main')
        assert status == 200
        server.tool('reqvire.add_element', branch='main', file='repositories/component/Added.md',
                    content=(SUITE / 'fixtures/addition.md.txt').read_text(), dry_run=True)
        assert server.http('/api/project-store/manifest?branch=main') == (status, baseline), 'Preview changed publication'
        f.add(server)
        _, body = server.http('/api/project-store?branch=main')
        assert 'Added Capability' in [e['name'] for e in json.loads(body)['store']['elements']]
        assert server.http('/api/project-store/manifest?branch=main')[1] != baseline, 'Accepted component mutation was not published'
        server.tool('reqvire.git.commit', branch='main', message='Save browser fixture')
        server.tool('reqvire.worktree.create', branch='topic', from_branch='main', base_ref='review')
        # Use the shared real-browser lifecycle and the existing picker. No UI or
        # store replacement hides a missing combined backend publication.
        browser = command(f.central, 'bun', str(SUITE / 'browser-repo.mjs'), server.base,
                          str(f.output / 'browser-profile'), timeout=90, check=False)
        (f.output / 'browser.log').write_text(browser.stdout + browser.stderr)
        assert browser.returncode == 0, f'Branch picker failed; see {f.output / "browser.log"}'


def publication_environment(f):
    """Reuse the existing GitHub double; real Git transports stay local."""
    bins = f.base / 'bin'
    bins.mkdir()
    state = f.base / 'github-state'
    state.mkdir()
    (state / 'scenario.json').write_text('{}')
    shutil.copy2(SUITE.parent / 'test-mcp-publication/fixtures/gh-double.py', bins / 'gh')
    (bins / 'gh').chmod(0o755)
    mapping = {'https://github.com/test/project.git': str(f.remotes / 'central'),
               'https://github.com/test/component': str(f.remotes / 'component'),
               'https://github.com/test/component.git': str(f.remotes / 'component')}
    real_git = shutil.which('git')
    wrapper = '''#!/usr/bin/python3
import json, os, pathlib, sys
args = sys.argv[1:]
mapping = json.loads(os.environ['REQVIRE_REPO_TEST_REMOTES'])
network = 'push' in args or 'ls-remote' in args
if network:
    # Real git also accepts a configured remote name instead of an explicit URL.
    import subprocess
    for index, arg in enumerate(args):
        if arg in ('origin', 'local'):
            probe = subprocess.run([REAL_GIT, 'remote', 'get-url', arg], capture_output=True, text=True)
            if probe.returncode == 0 and probe.stdout.strip() in mapping:
                args[index] = probe.stdout.strip()
    urls = [a for a in args if a in mapping]
    if 'push' in args:
        with open(os.environ['REQVIRE_REPO_TEST_PUSHES'], 'a') as log:
            log.write(json.dumps({'cwd': os.getcwd(), 'targets': urls}) + '\\n')
        if pathlib.Path(os.environ['REQVIRE_REPO_TEST_FAIL_COMPONENT']).exists() and any('component' in a for a in urls):
            sys.stderr.write('Injected component remote denial\\n')
            sys.exit(1)
    args = [mapping.get(a, a) for a in args]
    if any(a.startswith(('https://', 'ssh://', 'git@')) for a in args):
        sys.stderr.write('Network forbidden in offline Repo test\\n')
        sys.exit(99)
os.execv(REAL_GIT, [REAL_GIT, *args])
'''.replace('REAL_GIT', repr(real_git))
    (bins / 'git').write_text(wrapper)
    (bins / 'git').chmod(0o755)
    git(f.central, 'remote', 'set-url', 'origin', 'https://github.com/test/project.git')
    git(f.component, 'remote', 'set-url', 'local', 'https://github.com/test/component')
    f.write_manifest(f.alpha, fetch='https://github.com/test/')
    f.initial_central = commit(f.central, 'Register authorized publication remotes')
    return {'PATH': str(bins) + os.pathsep + ENV['PATH'],
            'REQVIRE_GH_TEST_STATE': str(state), 'REQVIRE_REPO_TEST_REMOTES': json.dumps(mapping),
            'REQVIRE_REPO_TEST_PUSHES': str(f.output / 'pushes.jsonl'),
            'REQVIRE_REPO_TEST_FAIL_COMPONENT': str(f.output / 'deny-component')}


def push(fail_component):
    f = Fixture('push-denied' if fail_component else 'push-success')
    env = publication_environment(f)
    remote_before = git(f.remotes / 'central', 'rev-parse', 'refs/heads/main')
    # Start from an already saved, valid combined view. Push ordering must fail
    # independently of a broken Reqvire multi-repository commit implementation.
    (f.component / 'Added.md').write_text((SUITE / 'expected/added.md.txt').read_text())
    component = commit(f.component, 'Saved component dependency')
    f.write_manifest(component, fetch='https://github.com/test/')
    central = commit(f.central, 'Saved Central dependency selection')
    with f.server(extra_args=['--enable-github'], environment=env) as server:
        if fail_component:
            (f.output / 'deny-component').touch()
        outcome = server.raw_tool('reqvire.git.push', branch='main')
        attempts = [json.loads(line) for line in (f.output / 'pushes.jsonl').read_text().splitlines()]
        targets = [t for call in attempts for t in call['targets']]
        assert targets and 'component' in targets[0], 'Central was attempted before its required component commit'
        assert git(f.central, 'rev-parse', 'HEAD') == central and git(f.component, 'rev-parse', 'HEAD') == component
        if fail_component:
            assert outcome.get('isError') is True
            assert git(f.remotes / 'central', 'rev-parse', 'refs/heads/main') == remote_before, 'Central published a missing dependency'
            assert not any('project.git' in t for t in targets), 'Central push attempted after dependency failure'
        else:
            assert not outcome.get('isError'), outcome
            assert targets[-1] == 'https://github.com/test/project.git'
            assert git(f.remotes / 'central', 'rev-parse', 'refs/heads/main') == central
            refs = git(f.remotes / 'component', 'for-each-ref', '--format=%(objectname)').splitlines()
            assert component in refs, 'Manifest component commit has no published ref'


def main():
    if not REPO or not (REPO_SOURCE / 'main.py').is_file() or not (REPO_SOURCE / '.git').exists():
        print('Fixture dependency missing: install repo and set REQVIRE_TEST_REPO_SOURCE to a local git-repo Git checkout.', file=sys.stderr)
        return 2
    cases = [
        ('mcp/complete-declared-snapshot', lambda: complete_snapshot('mcp')),
        ('serve/complete-declared-snapshot', lambda: complete_snapshot('serve')),
        ('cli/explicit-manifest-selection', explicit_manifest),
        ('cli/invalid-selected-manifest-rejected', invalid_manifest),
        ('cli/invalid-project-boundaries-rejected', project_boundaries),
        ('cli/only-declared-project-sources', declared_sources_only),
        ('mcp/central-branch-component-revisions', branch_revisions),
        ('mcp/cross-project-preview-apply', cross_project_mutation),
        ('mcp/explicit-component-first-commit', lambda: component_commit(False)),
        ('mcp/automatic-component-first-commit', lambda: component_commit(True)),
        ('mcp/dirty-component-admission', dirty_component),
        ('mcp/failed-central-checkpoint-retains-component', failed_central_commit),
        ('serve/combined-publication-and-branch-picker', explorer),
        ('mcp/dependency-first-push', lambda: push(False)),
        ('mcp/failed-dependency-push-preserves-central', lambda: push(True)),
    ]
    failures = []
    for name, test in cases:
        try:
            test()
        except Exception:
            failures.append(name)
            detail = traceback.format_exc()
            (OUTPUT / (name.replace('/', '-') + '.failure.txt')).write_text(detail)
            print(detail, file=sys.stderr)
            print('FAIL ' + name, flush=True)
        else:
            print('PASS ' + name, flush=True)
    (OUTPUT / 'summary.json').write_text(json.dumps({'cases': len(cases), 'failures': failures}, indent=2))
    return int(bool(failures))


if __name__ == '__main__':
    raise SystemExit(main())
