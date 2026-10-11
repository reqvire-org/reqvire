"""Contract-driven E2E checks shared by parsing, CRUD, formatting and MCP suites.

Checks use authored fixtures and expected-output diffs to verify rejection,
repair guidance, atomic persistence and successful retries.
"""
import argparse
from contextlib import contextmanager
import json
from pathlib import Path
import shutil
import socket
import subprocess
import sys
import time
import urllib.error
import urllib.request


CODE = 'model-element-link-in-prose'
FIXTURES = Path(__file__).with_name('prose-link-fixtures')
TARGET = 'model/Targets.md#target-specification'
LINK = '[delivery contract](Targets.md#target-specification)'


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def fixture(name):
    return (FIXTURES / name).read_text()


class Checks:
    def __init__(self, binary, directory, mode):
        self.binary = str(Path(binary).resolve())
        self.artifacts = Path(directory).resolve() / 'output' / f'prose-links-{mode}'
        self.root = self.artifacts / 'workspace'
        self.artifacts.mkdir(parents=True, exist_ok=True)
        self.results = []
        self.calls = 0

    def reset(self, source=None):
        if self.root.exists():
            shutil.rmtree(self.root)
        shutil.copytree(FIXTURES / 'base', self.root)
        if source:
            shutil.copyfile(FIXTURES / 'sources' / f'{source}.md.fixture', self.root / 'model/Source.md')
        # Expected rejection state is assembled only from checked-in inputs,
        # before Reqvire runs, and compared with recursive unified file diffs.
        expected = self.artifacts / 'expected'
        if expected.exists():
            shutil.rmtree(expected)
        shutil.copytree(self.root, expected)
        for args in [('init', '-q'), ('config', 'user.email', 'test@example.com'), ('config', 'user.name', 'Test User'), ('add', '.'), ('-c', 'commit.gpgSign=false', 'commit', '-qm', 'Initial prose-link fixture')]:
            subprocess.run(['git', '-C', str(self.root), *args], check=True, capture_output=True)

    def run(self, *args, content=None):
        self.calls += 1
        result = subprocess.run([self.binary, '--workspace', str(self.root), *args], cwd=self.root,
                                input=content, text=True, capture_output=True, timeout=30)
        stem = self.artifacts / f'{self.calls:03d}-{args[0]}'
        stem.with_suffix('.stdout').write_text(result.stdout)
        stem.with_suffix('.stderr').write_text(result.stderr)
        return result

    def valid(self):
        result = self.run('validate', '--json')
        require(result.returncode == 0, f'fixture control failed: {result.stdout}{result.stderr}')

    def snapshot(self):
        return self.artifacts / 'expected'

    def unchanged(self, expected):
        diff = subprocess.run(['diff', '-ru', str(expected / 'model'), str(self.root / 'model')], text=True, capture_output=True)
        require(diff.returncode == 0, f'rejected operation changed authored files:\n{diff.stdout}{diff.stderr}')

    def rejected(self, result):
        require(result.returncode != 0, 'missing enforcement: command accepted a prose link to a registered element')
        require(CODE in result.stdout + result.stderr, f'rejection omitted {CODE}: {result.stdout}{result.stderr}')

    def repaired_file_matches(self, expected):
        diff = subprocess.run(['diff', '-u', str(FIXTURES / 'expected' / expected), str(self.root / 'model/Source.md')], text=True, capture_output=True)
        require(diff.returncode == 0, f'explicit author repair differs from expected file:\n{diff.stdout}{diff.stderr}')

    def diagnostic_matches(self, value, kind):
        projected = []
        for item in diagnostics(value):
            projected.append({
                'code': item.get('code'),
                'context': {key: item.get('context', {}).get(key) for key in ('file', 'line', 'element_id', 'element_type')},
                'target': {key: item.get('target', {}).get(key) for key in ('element_id', 'element_type')},
                'guidance_present': bool(item.get('guidance')),
            })
        projected.sort(key=lambda item: (item['context']['line'] or 0, item['target']['element_id'] or ''))
        actual = self.artifacts / f'diagnostic-{kind}.json'
        actual.write_text(json.dumps(projected, indent=2, sort_keys=True) + '\n')
        diff = subprocess.run(['diff', '-u', str(FIXTURES / 'expected' / f'diagnostic-{kind}.json'), str(actual)], text=True, capture_output=True)
        require(diff.returncode == 0, f'structured diagnostic differs from expected fixture:\n{diff.stdout}{diff.stderr}')

    def case(self, name, check):
        try:
            check()
        except (AssertionError, KeyError, ValueError, subprocess.SubprocessError, OSError) as error:
            self.results.append({'name': name, 'passed': False, 'failure': str(error)})
        else:
            self.results.append({'name': name, 'passed': True})

    def finish(self):
        (self.artifacts / 'checks.json').write_text(json.dumps(self.results, indent=2) + '\n')
        failures = [item for item in self.results if not item['passed']]
        for item in self.results:
            print(f'{"PASS" if item["passed"] else "FAIL"} {item["name"]}')
            if not item['passed']:
                print(f'FAIL {item["name"]}: {item["failure"]}', file=sys.stderr)
        if failures:
            print(f'{len(failures)} failed, {len(self.results) - len(failures)} passed; details: {self.artifacts}', file=sys.stderr)
        return bool(failures)


def diagnostics(value):
    """Locate diagnostic records without prescribing a new enclosing envelope."""
    if isinstance(value, dict):
        if value.get('code') == CODE:
            yield value
        else:
            for child in value.values():
                yield from diagnostics(child)
    elif isinstance(value, list):
        for child in value:
            yield from diagnostics(child)


def guidance(diag, kind='requirement', source='model/Source.md#prose-source', concept=False):
    context = diag['context']
    require(context['element_id'] == source and context['element_type'] == kind, 'incorrect source identity/type')
    require(context['file'] == source.split('#')[0] and isinstance(context.get('line'), int) and context['line'] > 0, 'missing source occurrence')
    target = 'model/Targets.md#target-concept' if concept else TARGET
    require(diag['target']['element_id'] == target and diag['target']['element_type'] == ('concept' if concept else 'specification'), 'incorrect resolved target')
    entries = diag.get('guidance')
    require(isinstance(entries, list) and entries and all(entry.get('message') for entry in entries), 'missing actionable structured guidance')
    messages = '\n'.join(entry['message'] for entry in entries).lower()
    require(any(word in entries[0]['message'].lower() for word in ('remove', 'replace')), 'first repair must remove the hyperlink')
    require(any(word in messages for word in ('plain', 'readable', 'domain wording')), 'missing readable prose repair')
    rendered = json.dumps(diag)
    label = 'delivery term' if concept else 'delivery contract'
    require(label in rendered and target.removeprefix('model/') in rendered, 'missing authored link text/destination')
    if concept:
        require(any('#### Concept References' in entry.get('example', '') and '[Target Concept](Targets.md#target-concept)' in entry.get('example', '') for entry in entries), 'missing actual Concept References example')
    elif kind == 'requirement':
        require('model/Targets.md#contract-owner' in rendered, 'missing target owner')
        for section in ('Contract References', 'Contract Bindings'):
            require(any(f'#### {section}' in entry.get('example', '') and '[Target Specification](Targets.md#target-specification)' in entry.get('example', '') for entry in entries), f'missing actual {section} alternative')
        require('review' in messages and 'implementation' in messages and any(word in messages for word in ('both', 'one section', 'one-section')), 'missing dependency choice and one-section guidance')
    else:
        require('requirement' in messages, 'dependency repair must identify the responsible requirement')
        require(all('requirement' in entry['message'].lower() for entry in entries if any(f'#### {section}' in entry.get('example', '') for section in ('Contract References', 'Contract Bindings'))), 'invalid dependency edit proposed on non-requirement source')


def read_checks(checks):
    def allowed():
        checks.reset(source='allowed-authoring')
        checks.valid()
        checks.unchanged(checks.snapshot())
    checks.case('allowed-authoring', allowed)

    def authored_links():
        checks.reset(source='authored-links')
        result = checks.run('validate', '--json')
        checks.unchanged(checks.snapshot())
        report = json.loads(result.stdout)
        checks.diagnostic_matches(report, 'authored-links')
        checks.rejected(result)
        found = list(diagnostics(report))
        for diag in found:
            guidance(diag, concept=diag['target']['element_type'] == 'concept')
        text_result = checks.run('validate')
        checks.rejected(text_result)
        for diag in found:
            require(diag['context']['element_id'] in text_result.stdout + text_result.stderr and diag['target']['element_id'] in text_result.stdout + text_result.stderr, 'text renderer dropped source/target context')
            require(any(entry['message'] in text_result.stdout + text_result.stderr for entry in diag['guidance']), 'text renderer dropped repair guidance')
    checks.case('disk-links-and-guidance', authored_links)

    def single_element():
        checks.reset(source='single-element-file-link')
        result = checks.run('validate', '--json')
        checks.unchanged(checks.snapshot())
        checks.diagnostic_matches(json.loads(result.stdout), 'single-element-file-link')
        checks.rejected(result)
    checks.case('disk-single-element-file-link', single_element)

    def capability():
        checks.reset(source='guidance-capability')
        result = checks.run('validate', '--json')
        checks.diagnostic_matches(json.loads(result.stdout), 'capability')
        checks.rejected(result)
        guidance(next(diagnostics(json.loads(result.stdout))), kind='capability')
    checks.case('disk-capability-guidance', capability)

    def report_gate():
        checks.reset()
        require(checks.run('model').returncode == 0, 'valid report control failed')
        checks.reset(source='guidance-requirement')
        output = checks.artifacts / 'rejected-report.json'
        output.unlink(missing_ok=True)
        result = checks.run('model', '--output', str(output))
        checks.unchanged(checks.snapshot())
        checks.rejected(result)
        require(not output.exists(), 'invalid read published an authoritative report')
    checks.case('model-read-gate', report_gate)


def crud_checks(checks):
    def edit(override=False, dry=False):
        checks.reset()
        checks.valid()
        action = 'override' if override else 'add'
        name = 'Prose Source' if override else 'Added Prose Source'
        flags = (['--override'] if override else []) + (['--dry-run'] if dry else [])
        result = checks.run('add', 'model/Source.md', *flags, '--json', content=fixture(f'payloads/{action}-invalid.md.fixture'))
        checks.unchanged(checks.snapshot())
        checks.rejected(result)
        found = list(diagnostics(json.loads(result.stdout)))
        require(len(found) == 1, 'CRUD omitted structured diagnostic')
        guidance(found[0], source=f'model/Source.md#{name.lower().replace(" ", "-")}')
        result = checks.run('add', 'model/Source.md', *(['--override'] if override else []), content=fixture(f'payloads/{action}-repair.md.fixture'))
        require(result.returncode == 0, 'valid repair failed after rejected candidate')
        checks.repaired_file_matches(f'{action}-repair.md')
        checks.valid()
    checks.case('crud-create-atomic-rejection', edit)
    checks.case('crud-override-dry-run-rejection', lambda: edit(override=True, dry=True))

    def untouched_prose():
        checks.reset(source='future-create')
        checks.valid()
        result = checks.run('add', 'model/Targets.md', content=fixture('payloads/future-delivery.md.fixture'))
        checks.unchanged(checks.snapshot())
        checks.rejected(result)
    checks.case('crud-untouched-prose-candidate', untouched_prose)


def format_checks(checks):
    def formatting():
        checks.reset(source='repair-specification')
        require(checks.run('format', '--fix').returncode == 0, 'valid format control failed')
        checks.reset(source='guidance-specification')
        for flags in ([], ['--fix']):
            result = checks.run('format', *flags, '--json')
            checks.unchanged(checks.snapshot())
            checks.rejected(result)
            found = list(diagnostics(json.loads(result.stdout)))
            require(len(found) == 1, 'format omitted shared diagnostic')
            guidance(found[0], kind='specification')
        shutil.copyfile(FIXTURES / 'sources/repair-specification.md.fixture', checks.root / 'model/Source.md')
        require(checks.run('format', '--fix').returncode == 0, 'explicit prose repair failed')
        checks.repaired_file_matches('format-specification-repair.md')
        checks.valid()
    checks.case('format-preserves-author-decisions', formatting)


@contextmanager
def mcp_server(checks):
    with socket.socket() as sock:
        sock.bind(('127.0.0.1', 0))
        port = sock.getsockname()[1]
    stderr = (checks.artifacts / 'mcp.stderr').open('w')
    stdout = (checks.artifacts / 'mcp.stdout').open('w')
    process = subprocess.Popen([checks.binary, '--workspace', str(checks.root), 'mcp', '--host', '127.0.0.1', '--port', str(port), '--enable-mutations'], cwd=checks.root, stdout=stdout, stderr=stderr)
    url = f'http://127.0.0.1:{port}/mcp'
    sequence = 0

    def call(method, params):
        nonlocal sequence
        sequence += 1
        request = urllib.request.Request(url, data=json.dumps({'jsonrpc': '2.0', 'id': sequence, 'method': method, 'params': params}).encode(), headers={'Content-Type': 'application/json', 'Accept': 'application/json, text/event-stream', 'Mcp-Protocol-Version': '2025-11-25'})
        with urllib.request.urlopen(request, timeout=10) as response:
            value = json.load(response)
        (checks.artifacts / f'mcp-{sequence:03d}.json').write_text(json.dumps(value, indent=2) + '\n')
        return value

    try:
        deadline = time.monotonic() + 5
        while True:
            require(process.poll() is None, f'MCP startup failed: {(checks.artifacts / "mcp.stderr").read_text()}')
            try:
                initialized = call('initialize', {'protocolVersion': '2025-11-25', 'capabilities': {}, 'clientInfo': {'name': 'prose-links-e2e', 'version': '1'}})
                require(initialized['result']['protocolVersion'] == '2025-11-25', 'MCP initialize control failed')
                break
            except urllib.error.URLError:
                if time.monotonic() >= deadline:
                    raise AssertionError('MCP startup timed out')
                time.sleep(0.05)
        yield lambda name, arguments: call('tools/call', {'name': name, 'arguments': arguments})
    finally:
        if process.poll() is None:
            process.terminate()
        try:
            process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            process.kill()
            process.wait(timeout=5)
        stdout.close()
        stderr.close()


def mcp_checks(checks):
    def override():
        checks.reset()
        checks.valid()
        with mcp_server(checks) as call:
            before = call('reqvire.workspace_status', {})['result']['structuredContent']['model']
            response = call('reqvire.add_element', {'file': 'model/Source.md', 'content': fixture('payloads/override-invalid.md.fixture'), 'override_existing': True, 'dry_run': False})
            require('error' not in response, f'MCP request control failed: {response}')
            checks.unchanged(checks.snapshot())
            require(response.get('result', {}).get('isError') is True, 'missing enforcement: MCP accepted invalid candidate')
            found = list(diagnostics(response))
            require(len(found) == 1, 'MCP omitted structured diagnostic')
            guidance(found[0])
            after = call('reqvire.workspace_status', {})['result']['structuredContent']['model']
            require(before == after, 'rejected candidate changed accepted graph')
            source = call('reqvire.read_element', {'name': 'Prose Source'})['result']['structuredContent']
            require(LINK not in source['content'], 'rejected content leaked into accepted model')
            repaired = call('reqvire.add_element', {'file': 'model/Source.md', 'content': fixture('payloads/override-repair.md.fixture'), 'override_existing': True, 'dry_run': False})
            require('error' not in repaired and not repaired.get('result', {}).get('isError'), 'valid repair failed after rejected candidate')
            checks.repaired_file_matches('override-repair.md')
        checks.valid()
    checks.case('mcp-override-atomic-rejection', override)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('mode', choices=('read', 'crud', 'format', 'mcp'))
    parser.add_argument('binary')
    parser.add_argument('directory')
    args = parser.parse_args()
    checks = Checks(args.binary, args.directory, args.mode)
    {'read': read_checks, 'crud': crud_checks, 'format': format_checks, 'mcp': mcp_checks}[args.mode](checks)
    raise SystemExit(checks.finish())


if __name__ == '__main__':
    main()
