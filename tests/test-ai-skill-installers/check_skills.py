import difflib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
from urllib.parse import unquote, urlsplit

repo, work = map(Path, sys.argv[1:])
suite = repo / 'tests/test-ai-skill-installers'
source = repo / 'skills'
env = {**os.environ, 'DISABLE_TELEMETRY': '1'}
checks = []


def match(name, expected, actual):
    (work / name).write_text(actual)
    diff = ''.join(difflib.unified_diff(expected.splitlines(True), actual.splitlines(True), fromfile='expected/' + name, tofile='actual/' + name))
    if diff:
        raise AssertionError(diff)


def fixture(name, actual):
    match(name, (suite / 'expected' / name).read_text(), actual)


def manifest(directory):
    return ''.join(str(p.relative_to(directory)) + '\n' for p in sorted(directory.rglob('*')) if p.is_file())


def skills(label, cwd, *args):
    result = subprocess.run(['npx', '--yes', 'skills@1.7.2', 'add', str(source), *args], cwd=cwd, env=env, capture_output=True, text=True)
    (work / (label + '.stdout')).write_text(result.stdout)
    (work / (label + '.stderr')).write_text(result.stderr)
    assert result.returncode == 0, result.stderr or result.stdout
    return result.stdout


fixture('manifest.txt', manifest(source))
for package in sorted(source.iterdir()):
    text = (package / 'SKILL.md').read_text()
    assert re.search(r'^name: ' + re.escape(package.name) + r'$', text, re.M), package
    assert re.search(r'^description: \S', text, re.M), package
    for file in package.rglob('*.md'):
        body = file.read_text()
        body = re.sub(r'^\s*(`{3,}|~{3,})[^\n]*\n.*?^\s*\1\s*$', '', body, flags=re.M | re.S)
        for reference in re.findall(r'`(references/[^`]+\.md)`', body):
            assert (file.parent / reference).is_file(), (file, reference)
        prose = re.sub(r'`+[^`]*`+', '', body)
        for target in re.findall(r'\[[^\]]+\]\(([^)]+)\)', prose):
            path = target.split('#', 1)[0]
            if not path or urlsplit(path).scheme or path.startswith('/') or path in {'file.md', 'path.md'}:
                continue
            assert (file.parent / unquote(path)).is_file(), (file, target)
        assert not re.search(r'(?:/reqvire:|`reqvire:(?:syseng|audit)|add --override|lint --fix(?:\s|\)|:|`))', file.read_text()), file
        assert not re.search(r'(?:reqvire\s+|`)(?:model|containment|resources|traces)\s+[^`\n]*(?:--json|--mmd)', file.read_text()), file

listed = skills('discovery', work, '--list')
listed = re.sub(r'\x1b\[[0-?]*[ -/]*[@-~]', '', listed)
names = sorted(re.findall(r'^│\s+(reqvire-[a-z-]+)\s*$', listed, re.M))
fixture('names.txt', ''.join(name + '\n' for name in names))
checks.append('PASS canonical package manifests, links, and CLI discovery')

main_source = 'https://github.com/reqvire-org/reqvire/tree/main/skills'
for file in [repo / 'README.md', repo / 'doc/SKILLS.md', repo / 'website/src/pages/CodingAssistants.tsx']:
    text = file.read_text()
    assert 'npx skills add ' + main_source in text, file
    assert '--agent codex claude-code' in text and '--global' in text and '--skill reqvire-syseng' in text, file
    assert 'deprecated' in text and '/plugin' in text, file
    assert '/plugin install' not in text and not re.search(r'curl.*install-(?:codex|claude)-skill', text), file
for obsolete in ['codex-skills', 'claude-plugins', '.claude-plugin', 'scripts/install-codex-skill.sh', 'scripts/install-claude-skill.sh']:
    assert not (repo / obsolete).exists(), obsolete
checks.append('PASS main-branch installation and marketplace migration guidance')

installation = work / 'installed'
installation.mkdir()
installed = json.loads(skills('installation', installation, '--agent', 'codex', 'claude-code', '--skill', '*', '--copy', '--yes', '--json'))
fixture('names.txt', ''.join(name + '\n' for name in sorted(item['name'] for item in installed)))
for item in installed:
    assert item['status'] == 'installed' and item['scope'] == 'project' and item['mode'] == 'copy', item
    assert sorted(item['agents']) == ['Claude Code', 'Codex'], item
for agent in ['.agents', '.claude']:
    target = installation / agent / 'skills'
    fixture('manifest.txt', manifest(target))
    for file in source.rglob('*'):
        if file.is_file():
            actual = target / file.relative_to(source)
            match(agent.removeprefix('.') + '-' + str(file.relative_to(source)).replace('/', '-'), file.read_text(), actual.read_text())

skill = (installation / '.agents/skills/reqvire-syseng/SKILL.md').read_text()
reference = re.search(r'^traces .*$', skill, re.M).group()
assert '--json' not in reference and '--output' in reference, reference
flags = re.search(r'^\*\*Common flags:\*\*.*?(?=\n\n)', skill, re.M | re.S).group()
assert 'JSON-only' in flags and all('`' + command + '`' in flags for command in ['model', 'containment', 'resources', 'traces'])
binary = os.environ['REQVIRE_BIN']
stdout = subprocess.check_output([binary, 'traces'], cwd=work, text=True)
report = json.loads(stdout)
assert any(v['name'] == 'Skill Reference Test' for file in report['files'].values() for v in file.get('verifications', [])), report
subprocess.run([binary, 'traces', '--output', 'traces-file.json'], cwd=work, check=True, capture_output=True)
match('traces.json', json.dumps(report, sort_keys=True, indent=2) + '\n', json.dumps(json.loads((work / 'traces-file.json').read_text()), sort_keys=True, indent=2) + '\n')
checks.append('PASS complete Codex and Claude Code installs and executable report guidance')

selection = work / 'selected'
selection.mkdir()
selected = json.loads(skills('selection', selection, '--agent', 'codex', '--skill', 'reqvire-syseng', '--copy', '--yes', '--json'))
assert [item['name'] for item in selected] == ['reqvire-syseng'], selected
assert sorted(p.name for p in (selection / '.agents/skills').iterdir()) == ['reqvire-syseng']
match('selected-manifest.txt', manifest(source / 'reqvire-syseng'), manifest(selection / '.agents/skills/reqvire-syseng'))
assert not (selection / '.claude').exists()
checks.append('PASS selection installs only the requested skill and agent')
fixture('checks.txt', '\n'.join(checks) + '\n')
print('\n'.join(checks))
