"""Repo-manifest acceptance through real CLI, HTTP MCP and embedded Explorer.

All expected outcomes are success. Cases accumulate failures; missing production
support is neither skipped nor converted into an expected exception. Fixture
repositories, responses, server logs and diffs stay under the runner's TEST_DIR.
"""
import difflib
from contextlib import ExitStack, nullcontext
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import traceback
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
REPO_SOURCE = Path(DESTINATION) / 'repo-tool'
REPO = shutil.which('repo')
ENV = {**os.environ, 'GIT_ALLOW_PROTOCOL': 'file', 'GIT_TERMINAL_PROMPT': '0',
       'GIT_CONFIG_COUNT': '2', 'GIT_CONFIG_KEY_0': 'user.name',
       'GIT_CONFIG_VALUE_0': 'Repo Acceptance', 'GIT_CONFIG_KEY_1': 'user.email',
       'GIT_CONFIG_VALUE_1': 'repo-test@example.invalid'}
EXPECTED_NAMES = ['Central Capability', 'Central Consumer', 'Component Capability',
                  'Component Requirement', 'Component Specification']
SERVER_STARTS = []


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
    def __init__(self, name, cross_reference=False, multiple_components=False):
        self.base = ROOT / name
        self.base.mkdir()
        self.output = OUTPUT / name
        self.output.mkdir()
        self.remotes = self.base / 'remotes'
        self.remotes.mkdir()
        component_seed = self.base / 'component-seed'
        component_seed.mkdir()
        git(component_seed, 'init', '-qb', 'main')
        source = (SUITE / ('fixtures/commit-component.md.txt' if multiple_components
                          else 'fixtures/component.md.txt')).read_text()
        (component_seed / 'Model.md').write_text(source)
        self.alpha = commit(component_seed, 'Component alpha')
        (component_seed / 'Model.md').write_text(source.replace('behavior is alpha.', 'behavior is omega.'))
        self.omega = commit(component_seed, 'Component omega')
        git(self.base, 'clone', '-q', '--bare', str(component_seed), str(self.remotes / 'component'))
        self.extra_revisions = {}
        if multiple_components:
            for project, text in {
                'secondary': source.replace('Component', 'Secondary').replace('component-', 'secondary-'),
                'retained': (SUITE / 'fixtures/retained.md.txt').read_text(),
            }.items():
                seed = self.base / (project + '-seed')
                seed.mkdir()
                git(seed, 'init', '-qb', 'main')
                (seed / 'Model.md').write_text(text)
                self.extra_revisions[project] = commit(seed, 'Initial ' + project)
                git(self.base, 'clone', '-q', '--bare', str(seed), str(self.remotes / project))
        self.central = self.base / 'central'
        self.central.mkdir()
        git(self.central, 'init', '-qb', 'main')
        text = (SUITE / ('fixtures/commit-central.md.txt' if multiple_components
                        else 'fixtures/central.md.txt')).read_text()
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
        self.projects = {'component': self.component,
                         **{p: self.central / 'repositories' / p for p in self.extra_revisions}}
        self.initial_projects = {'component': self.alpha, **self.extra_revisions}
        assert git(self.component, 'rev-parse', 'HEAD') == self.alpha, 'Repo did not honor the pinned revision'
        assert git(self.component, 'cat-file', '-t', self.omega) == 'commit', 'Review revision must be available offline'
        assert not git(self.central, 'status', '--porcelain'), 'Fixture Central is dirty'
        assert not git(self.component, 'status', '--porcelain'), 'Fixture component is dirty'
        for project in self.extra_revisions:
            assert git(self.projects[project], 'rev-parse', 'HEAD') == self.extra_revisions[project]
            assert not git(self.projects[project], 'status', '--porcelain'), project + ' fixture is dirty'
        (self.output / 'provenance.json').write_text(json.dumps({
            'repo_runtime_commit': runtime_sha, 'central': self.initial_central,
            'alpha': self.alpha, 'omega': self.omega, 'review': self.review,
            'projects': self.initial_projects}, indent=2))

    def write_manifest(self, sha, fetch=None):
        fetch = fetch or self.remotes.as_uri() + '/'
        projects = {'component': sha, **self.extra_revisions}
        declarations = ''.join(
            f'  <project name="{name}" path="repositories/{name}" revision="{revision}"/>\n'
            for name, revision in projects.items())
        (self.central / 'default.xml').write_text(
            '<manifest>\n'
            f'  <remote name="local" fetch="{fetch}"/>\n'
            '  <default remote="local" revision="main"/>\n'
            + declarations + '</manifest>\n')

    def server(self, mode='mcp', commits=False, extra_args=(), environment=None, native=False):
        SERVER_STARTS.append({'fixture': self.base.name, 'mode': mode, 'automatic_commits': commits})
        return helpers.Server(BINARY, self.central, self.output, mode, 'native-' + mode if native else 'server',
                              mutations=True, commits=commits,
                              extra_args=(*([] if native else ['--manifest', 'default.xml']), *extra_args),
                              environment={**ENV, **(environment or {})})

    def git_state(self):
        return [(git(p, 'rev-parse', 'HEAD'), git(p, 'ls-files', '--stage'))
                for p in (self.central, *self.projects.values())]

    def files(self):
        paths = [self.central / 'Model.md', self.central / 'default.xml',
                 *[root / name for root in self.projects.values() for name in ('Model.md', 'Added.md')]]
        return {str(p): p.read_bytes() if p.exists() else None for p in paths}

    def add(self, server):
        return server.tool('reqvire.add_element', branch='main', file='repositories/component/Added.md',
                           content=(SUITE / 'fixtures/addition.md.txt').read_text())

    def saved_multiple(self, server, result, retained_state):
        central = git(self.central, 'rev-parse', 'HEAD')
        heads = {name: git(root, 'rev-parse', 'HEAD') for name, root in self.projects.items()}
        for name in ('component', 'secondary'):
            root = self.projects[name]
            assert heads[name] != self.initial_projects[name], name + ' received no commit'
            assert git(root, 'rev-parse', 'HEAD^') == self.initial_projects[name], name + ' has the wrong parent'
            expected = 'commit-' + name + '-renamed.md.txt'
            golden(root / 'Model.md', expected)
            assert git(root, 'show', 'HEAD:Model.md') + '\n' == (SUITE / 'expected' / expected).read_text()
            assert not git(root, 'status', '--porcelain'), name + ' remains dirty'
        assert self.retained_state() == retained_state, 'The unchanged component received a commit or was modified'
        golden(self.central / 'Model.md', 'commit-central-renamed.md.txt')
        assert git(self.central, 'show', 'HEAD:Model.md') + '\n' == (SUITE / 'expected/commit-central-renamed.md.txt').read_text()
        assert not git(self.central, 'ls-files', 'repositories'), 'Central tracked component files'
        manifest = ET.fromstring(git(self.central, 'show', 'HEAD:default.xml'))
        pins = {p.get('name'): p.get('revision') for p in manifest.findall('project')}
        assert pins == heads, 'Central did not pin every changed and unchanged component exactly'
        assert central != self.initial_central and result.get('commit', result.get('context', {}).get('head')) == central
        assert git(self.central, 'rev-parse', 'HEAD^') == self.initial_central, 'Save created extra Central commits'
        assert server.tool('reqvire.workspace_status', branch='main')['context']['pending_changes'] == []
        expected_names = sorted(EXPECTED_NAMES + ['Updated Shared Specification', 'Secondary Capability',
                                                 'Secondary Requirement', 'Secondary Specification', 'Retained Capability'])
        assert names(server) == expected_names, 'Commit lost part of the accepted combined model'
        cli = json.loads(command(self.central, BINARY, '--manifest', 'default.xml', 'search', '--json').stdout)
        assert sorted(e['name'] for data in cli['files'].values() for e in data['elements']) == expected_names

    def retained_state(self):
        root = self.projects['retained']
        return (git(root, 'rev-parse', 'HEAD'), git(root, 'ls-files', '--stage'),
                git(root, 'for-each-ref', '--format=%(refname) %(objectname)'),
                (root / 'Model.md').read_bytes(), git(root, 'status', '--porcelain'))


def complete_snapshot(mode, shared):
    f, server = shared(mode + '-snapshot')
    cli = command(f.central, BINARY, 'search', '--json')
    cli_names = sorted(e['name'] for data in json.loads(cli.stdout)['files'].values() for e in data['elements'])
    assert cli_names == EXPECTED_NAMES, 'CLI did not see the complete Repo model'
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
    # Count actual Repo resolver launches, rather than relying on a wall-clock
    # threshold. File evidence must not reload the whole project inventory.
    original = (f.central / 'Model.md').read_text()
    evidence = f.central / 'evidence'
    evidence.mkdir()
    resolver_bin = f.output / 'resolver-bin'
    resolver_bin.mkdir()
    launches = f.output / 'resolver-launches.txt'
    wrapper = resolver_bin / 'python3'
    shutil.copy2(SUITE / 'fixtures/resolver-double.sh', wrapper)
    wrapper.chmod(0o755)
    counts = []
    for count in (1, 64):
        relations = ''.join(f'  * satisfiedBy: [Evidence {i}](evidence/{i}.txt)\n'
                            for i in range(count))
        for i in range(count):
            (evidence / f'{i}.txt').write_text('Evidence\n')
        (f.central / 'Model.md').write_text(original.replace(
            '#### Relations\n', '#### Relations\n' + relations))
        launches.write_text('')
        command(f.central, BINARY, '--manifest', 'default.xml', 'validate',
                env={**ENV, 'PATH': str(resolver_bin) + os.pathsep + ENV['PATH'],
                     'REQVIRE_REPO_TEST_RESOLVER_LOG': str(launches),
                     'REQVIRE_REPO_TEST_PYTHON': sys.executable})
        counts.append(len(launches.read_text().splitlines()))
    (f.output / 'resolver-counts.json').write_text(json.dumps(counts))
    assert counts[0] > 0 and counts[0] == counts[1], (
        f'File references repeated Repo scope resolution: 1 reference={counts[0]}, '
        f'64 references={counts[1]}')
    (f.central / 'Model.md').write_text(original)
    shutil.rmtree(evidence)
    # Only Central has this include. Repo's separate remote-manifest checkout
    # must not supply an older input, and Repo still owns extend semantics.
    manifest = (f.central / 'default.xml').read_text()
    declaration = next(line for line in manifest.splitlines() if '<project ' in line)
    (f.central / 'projects.xml').write_text('<manifest>\n' + declaration + '\n</manifest>\n')
    (f.central / 'default.xml').write_text(manifest.replace(declaration,
        '  <include name="projects.xml"/>\n'
        f'  <extend-project name="component" revision="{f.alpha}"/>'))
    commit(f.central, 'Select Central-owned manifest composition')
    before = f.files(), f.git_state(), (f.central / 'projects.xml').read_bytes()
    composed = json.loads(command(f.central, BINARY, '--manifest', 'default.xml', 'search', '--json').stdout)
    assert sorted(e['name'] for data in composed['files'].values() for e in data['elements']) == EXPECTED_NAMES
    assert (f.files(), f.git_state(), (f.central / 'projects.xml').read_bytes()) == before
    # Existing manually managed checkouts can use origin for the same identity.
    # Relative fetch roots and SSH/scp spelling are resolved without any fetch.
    git(f.component, 'remote', 'rename', 'local', 'origin')
    composed_text = (f.central / 'default.xml').read_text()
    for fetch, checkout_url in [
        ('.', str(f.remotes / 'component')),
        ('ssh://git@github.com/test', 'git@github.com:test/component.git'),
    ]:
        (f.central / 'default.xml').write_text(composed_text.replace(f.remotes.as_uri() + '/', fetch))
        git(f.component, 'remote', 'set-url', 'origin', checkout_url)
        found = json.loads(command(f.central, BINARY, '--manifest', 'default.xml', 'search', '--json').stdout)
        assert sorted(e['name'] for data in found['files'].values() for e in data['elements']) == EXPECTED_NAMES
    missing = command(f.central, BINARY, '--manifest', 'missing.xml', 'validate', check=False)
    assert missing.returncode != 0 and 'missing.xml' in missing.stderr + missing.stdout
    assert 'unexpected argument' not in missing.stderr, 'The selector was not implemented'


def invalid_manifest():
    f = Fixture('invalid-manifest')
    # Initialized Repo is the selector. No flag parsing can hide this rejection.
    (f.central / '.repo/manifests/default.xml').write_text('<manifest>')
    before = f.files(), f.git_state()
    result = command(f.central, BINARY, 'validate', '--json', check=False)
    assert result.returncode != 0, 'Invalid active Repo manifest silently fell back to Central'
    assert 'default.xml' in result.stdout + result.stderr, 'Rejection omitted the selected manifest path'
    assert (f.files(), f.git_state()) == before, 'Inspection changed rejected inputs'


def project_boundaries():
    f = Fixture('project-boundaries')
    manifest = (f.central / 'default.xml').read_text()
    (f.central / 'repositories/escape').symlink_to(f.base / 'component-seed', target_is_directory=True)
    invalid = {
        'duplicate-path': manifest.replace('</manifest>',
            f'  <project name="another" path="repositories/component" revision="{f.alpha}"/>\n</manifest>'),
        'escaping-path': manifest.replace('path="repositories/component"', 'path="../outside"'),
        'unavailable-commit': manifest.replace(f.alpha, '0' * 40),
        'missing-project': manifest.replace('path="repositories/component"', 'path="repositories/missing"'),
        'symlink-escape': manifest.replace('path="repositories/component"', 'path="repositories/escape"'),
        'overlapping-path': manifest.replace('</manifest>',
            f'  <project name="nested" path="repositories/component/nested" revision="{f.alpha}"/>\n</manifest>'),
    }
    errors = []
    for label, text in invalid.items():
        (f.central / 'default.xml').write_text(text)
        before = f.files(), f.git_state()
        result = command(f.central, BINARY, '--manifest', 'default.xml', 'validate', '--json', check=False)
        diagnostic = result.stdout + result.stderr
        (f.output / (label + '.log')).write_text(diagnostic)
        if result.returncode == 0 or 'default.xml' not in diagnostic:
            errors.append(label + ': invalid selected project was accepted or lacks its manifest diagnostic')
        assert (f.files(), f.git_state()) == before, 'Rejected selection changed files or Git state'
    (f.central / 'default.xml').write_text(manifest)
    remote = git(f.component, 'remote', 'get-url', 'local')
    try:
        git(f.component, 'remote', 'set-url', 'local', str(f.base / 'foreign'))
        before = f.files(), f.git_state()
        result = command(f.central, BINARY, 'validate', '--json', check=False)
        diagnostic = result.stdout + result.stderr
        if result.returncode == 0 or 'default.xml' not in diagnostic or 'repositories/component' not in diagnostic:
            errors.append('foreign-checkout: mismatched project identity was accepted or lacks provenance')
        assert (f.files(), f.git_state()) == before
    finally:
        git(f.component, 'remote', 'set-url', 'local', remote)
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


def branch_revisions(shared):
    f, server = shared('explicit-commit')
    original = f.files(), f.git_state()
    child = server.tool('reqvire.branch.create', branch='topic', from_branch='main', base_ref='review')
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
    server.tool('reqvire.branch.close', branch='topic')
    reopened = server.tool('reqvire.branch.open', branch='topic')
    restored = Path(reopened['workspace_root']) / 'repositories/component'
    assert git(restored, 'rev-parse', 'HEAD') == f.omega, 'Reopening lost the retained manifest selection'
    assert 'omega' in json.dumps(server.tool('reqvire.read_element', branch='topic', name='Component Specification'))


def cross_project_mutation(shared):
    f, server = shared('mcp-snapshot')
    before_files, before_git = f.files(), f.git_state()
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
    assert set(server.tool('reqvire.workspace_status', branch='main')['context']['pending_changes']) == {
        'Model.md', 'repositories/component/Model.md'}


def component_commit(automatic, shared):
    if automatic:
        f = Fixture('automatic-commit', multiple_components=True)
        session = f.server(commits=True)
    else:
        f, server = shared('explicit-commit')
        session = nullcontext(server)
    before = f.git_state()
    retained = f.retained_state()
    with session as server:
        # One rename updates Central and references in both changed projects.
        # This is one transaction even when commits happen automatically.
        result = server.tool('reqvire.rename_element', branch='main', element_name='Shared Specification',
                             new_name='Updated Shared Specification')
        if not automatic:
            assert f.git_state() == before, 'Pending acceptance changed Git state'
            result = server.tool('reqvire.git.commit', branch='main', message='Save combined contract rename')
        f.saved_multiple(server, result, retained)

        # Preserve the original new-file save coverage in the same session.
        # Only its owning component changes in this second transaction.
        before_add = f.git_state()
        old_heads = {name: git(root, 'rev-parse', 'HEAD') for name, root in f.projects.items()}
        old_central = git(f.central, 'rev-parse', 'HEAD')
        f.add(server)
        if not automatic:
            assert f.git_state() == before_add, 'Pending addition changed Git state'
            server.tool('reqvire.git.commit', branch='main', message='Save new component file')
        golden(f.component / 'Added.md', 'added.md.txt')
        assert git(f.component, 'show', 'HEAD:Added.md') + '\n' == (SUITE / 'expected/added.md.txt').read_text()
        assert git(f.component, 'rev-parse', 'HEAD^') == old_heads['component']
        for name in ('secondary', 'retained'):
            assert git(f.projects[name], 'rev-parse', 'HEAD') == old_heads[name], name + ' received an unrelated commit'
        assert f.retained_state() == retained, 'Unchanged component was altered by the addition'
        manifest = ET.fromstring(git(f.central, 'show', 'HEAD:default.xml'))
        assert {p.get('name'): p.get('revision') for p in manifest.findall('project')} == {
            name: git(root, 'rev-parse', 'HEAD') for name, root in f.projects.items()}
        assert git(f.central, 'rev-parse', 'HEAD^') == old_central
        assert not git(f.central, 'ls-files', 'repositories'), 'Central tracked the new component file'
        assert server.tool('reqvire.workspace_status', branch='main')['context']['pending_changes'] == []
        assert 'Added Capability' in names(server), 'New component source was lost after save'
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


def explorer(shared):
    f, server = shared('serve-snapshot')
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
    server.tool('reqvire.branch.create', branch='topic', from_branch='main', base_ref='review')
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
    shutil.copy2(SUITE.parent / 'test-mcp-publication/fixtures/gh-double.sh', bins / 'gh')
    (bins / 'gh').chmod(0o755)
    mapping = {'https://github.com/test/project.git': str(f.remotes / 'central'),
               'https://github.com/test/component': str(f.remotes / 'component'),
               'https://github.com/test/component.git': str(f.remotes / 'component')}
    real_git = shutil.which('git')
    shutil.copy2(SUITE / 'fixtures/git-double.sh', bins / 'git')
    (bins / 'git').chmod(0o755)
    git(f.central, 'remote', 'set-url', 'origin', 'https://github.com/test/project.git')
    git(f.component, 'remote', 'set-url', 'local', 'https://github.com/test/component')
    f.write_manifest(f.alpha, fetch='https://github.com/test/')
    f.initial_central = commit(f.central, 'Register authorized publication remotes')
    return {'PATH': str(bins) + os.pathsep + ENV['PATH'],
            'REQVIRE_GH_TEST_STATE': str(state), 'REQVIRE_REPO_TEST_REMOTES': json.dumps(mapping),
            'REQVIRE_REPO_TEST_GIT_BINARY': real_git,
            'REQVIRE_REPO_TEST_PUSHES': str(f.output / 'pushes.jsonl'),
            'REQVIRE_REPO_TEST_FAIL_COMPONENT': str(f.output / 'deny-component')}


def push_fixture():
    f = Fixture('push')
    env = publication_environment(f)
    f.remote_before_push = git(f.remotes / 'central', 'rev-parse', 'refs/heads/main')
    # Start from an already saved, valid combined view. Push ordering must fail
    # independently of a broken Reqvire multi-repository commit implementation.
    (f.component / 'Added.md').write_text((SUITE / 'expected/added.md.txt').read_text())
    f.component_to_push = commit(f.component, 'Saved component dependency')
    f.write_manifest(f.component_to_push, fetch='https://github.com/test/')
    f.central_to_push = commit(f.central, 'Saved Central dependency selection')
    return f, {'extra_args': ['--enable-github'], 'environment': env}


def push(fail_component, shared):
    f, server = shared('push')
    gate = f.output / 'deny-component'
    if fail_component:
        gate.touch()
    else:
        gate.unlink(missing_ok=True)
    log = f.output / 'pushes.jsonl'
    before = len(log.read_text().splitlines()) if log.exists() else 0
    outcome = server.raw_tool('reqvire.git.push', branch='main')
    lines = log.read_text().splitlines() if log.exists() else []
    attempts = [json.loads(line) for line in lines[before:]]
    targets = [t for call in attempts for t in call['targets']]
    assert targets and 'component' in targets[0], 'Central was attempted before its required component commit'
    assert git(f.central, 'rev-parse', 'HEAD') == f.central_to_push
    assert git(f.component, 'rev-parse', 'HEAD') == f.component_to_push
    if fail_component:
        assert outcome.get('isError') is True
        assert git(f.remotes / 'central', 'rev-parse', 'refs/heads/main') == f.remote_before_push, 'Central published a missing dependency'
        assert not any('project.git' in t for t in targets), 'Central push attempted after dependency failure'
    else:
        assert not outcome.get('isError'), outcome
        assert targets[-1] == 'https://github.com/test/project.git'
        assert git(f.remotes / 'central', 'rev-parse', 'refs/heads/main') == f.central_to_push
        refs = git(f.remotes / 'component', 'for-each-ref', '--format=%(objectname)').splitlines()
        assert f.component_to_push in refs, 'Manifest component commit has no published ref'


def main():
    if not REPO or not (REPO_SOURCE / 'main.py').is_file() or not (REPO_SOURCE / '.git').exists():
        print('Temporary Repo tool fixture is missing; run this suite through test.sh.', file=sys.stderr)
        return 2
    # Each group has one fixed startup policy and an intentional sequence:
    # reads before edits; isolated branch checks before editing main; denied
    # publication before its repaired retry. Failure injection/admission and
    # automatic commits keep their own processes. No protocol helper is copied.
    factories = {
        'mcp-snapshot': lambda: (Fixture('mcp-snapshot', cross_reference=True), {}),
        'serve-snapshot': lambda: (Fixture('serve-snapshot'), {'mode': 'serve'}),
        'explicit-commit': lambda: (Fixture('explicit-commit', multiple_components=True), {}),
        'push': push_fixture,
    }
    sessions = {}
    stack = ExitStack()

    def shared(key):
        if key not in sessions:
            try:
                fixture, options = factories[key]()
                sessions[key] = fixture, stack.enter_context(fixture.server(**options))
            except Exception as error:
                # Keep startup failure visible for every affected case without
                # repeatedly constructing the same failed fixture/server.
                sessions[key] = error
        value = sessions[key]
        if isinstance(value, Exception):
            raise RuntimeError('Shared server unavailable: ' + key) from value
        return value

    cases = [
        ('mcp/complete-declared-snapshot', lambda: complete_snapshot('mcp', shared)),
        ('serve/complete-declared-snapshot', lambda: complete_snapshot('serve', shared)),
        ('cli/explicit-manifest-selection', explicit_manifest),
        ('cli/invalid-selected-manifest-rejected', invalid_manifest),
        ('cli/invalid-project-boundaries-rejected', project_boundaries),
        ('cli/only-declared-project-sources', declared_sources_only),
        ('mcp/central-branch-component-revisions', lambda: branch_revisions(shared)),
        ('mcp/cross-project-preview-apply', lambda: cross_project_mutation(shared)),
        ('mcp/explicit-component-first-commit', lambda: component_commit(False, shared)),
        ('mcp/automatic-component-first-commit', lambda: component_commit(True, shared)),
        ('mcp/dirty-component-admission', dirty_component),
        ('mcp/failed-central-checkpoint-retains-component', failed_central_commit),
        ('serve/combined-publication-and-branch-picker', lambda: explorer(shared)),
        ('mcp/failed-dependency-push-preserves-central', lambda: push(True, shared)),
        ('mcp/dependency-first-push', lambda: push(False, shared)),
    ]
    failures = []
    with stack:
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
    (OUTPUT / 'summary.json').write_text(json.dumps({
        'cases': len(cases), 'failures': failures, 'server_starts': len(SERVER_STARTS),
        'servers': SERVER_STARTS}, indent=2))
    return int(bool(failures))


if __name__ == '__main__':
    raise SystemExit(main())
