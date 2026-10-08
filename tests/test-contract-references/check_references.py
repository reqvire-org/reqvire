"""Contract-reference acceptance cases, isolated within disposable Git worktrees."""
import difflib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

FIXTURE = Path(__file__).parent
OUTPUT = Path(os.environ['TEST_DIR']) / 'output'
BINARY = os.environ['REQVIRE_BIN']
MODEL = 'specifications/Model.md'
TARGET = MODEL + '#error-response-specification'
REFERENCE = '#### Contract References\n  * [Error Response Specification](#error-response-specification)\n\n'
BASE = (FIXTURE / MODEL).read_text()
checks = []
failures = []


def equal(actual, expected, message):
    if actual != expected:
        before = json.dumps(expected, indent=2, sort_keys=True).splitlines(True)
        after = json.dumps(actual, indent=2, sort_keys=True).splitlines(True)
        raise AssertionError(message + '\n' + ''.join(difflib.unified_diff(before, after, 'expected', 'actual')))


def require(value, message):
    if not value:
        raise AssertionError(message)


def records(value):
    if isinstance(value, dict):
        yield value
        for item in value.values():
            yield from records(item)
    elif isinstance(value, list):
        for item in value:
            yield from records(item)


def row(value, name):
    found = [item for item in records(value) if item.get('name') == name]
    require(found, f'Missing element {name}')
    return found[0]


class Workspace:
    def __init__(self, root):
        self.root = root
        for directory in ['specifications', 'evidence']:
            shutil.copytree(FIXTURE / directory, root / directory)
        (root / '.reqvireignore').write_text('site/\n')
        self.git('init', '-q')
        self.git('config', 'user.name', 'Contract reference tests')
        self.git('config', 'user.email', 'tests@example.com')
        self.git('remote', 'add', 'origin', 'https://example.com/model.git')
        self.commit()

    def git(self, *args):
        return subprocess.run(['git', *args], cwd=self.root, check=True, capture_output=True)

    def commit(self):
        self.git('add', '.')
        self.git('commit', '-qm', 'Fixture')

    def run(self, *args, ok=True, input=None):
        result = subprocess.run([BINARY, *args], cwd=self.root, text=True, input=input, capture_output=True, timeout=40)
        if ok:
            require(result.returncode == 0, f'{args} failed: {result.stdout}\n{result.stderr}')
        else:
            require(result.returncode != 0, f'{args} unexpectedly succeeded')
        return result.stdout + result.stderr if not ok else result.stdout

    def data(self, *args):
        return json.loads(self.run(*args, *([] if args[0] in ['model', 'containment'] else ['--json'])))

    def write(self, text):
        (self.root / MODEL).write_text(text)

    def replace(self, old, new):
        text = (self.root / MODEL).read_text()
        require(old in text, f'Fixture text not found: {old}')
        self.write(text.replace(old, new, 1))

    def refs(self, name='Documentation'):
        return row(self.data('model'), name).get('contract_references', [])

    def coverage(self, name='Contract Owner'):
        report = self.data('coverage')
        return next(item for section in ['covered_requirements', 'uncovered_requirements'] for items in report[section]['files'].values() for item in items if item['name'] == name)

    def snapshot(self):
        return {str(p.relative_to(self.root)): p.read_text() for p in self.root.rglob('*.md') if '.git' not in p.parts}


def check(name, fn):
    try:
        with tempfile.TemporaryDirectory(prefix='contract-reference-') as folder:
            fn(Workspace(Path(folder)))
        checks.append('PASS ' + name)
        print(checks[-1], flush=True)
    except Exception as error:
        checks.append('FAIL ' + name)
        failures.append(f'{name}: {error}')
        print(checks[-1] + ': ' + str(error), flush=True)


def parsed(w):
    w.run('validate')
    equal(w.refs(), [TARGET], 'Reference not retained as structured data')
    equal(row(w.data('model'), 'Documentation')['contract_bindings'], [], 'Reference converted to binding')


check('parse-reference', parsed)
for kind in ['source', 'constraint', 'behavior', 'specification', 'state', 'input-output']:
    def target_type(w, kind=kind):
        w.replace('  * type: specification', '  * type: ' + kind)
        parsed(w)
    check('target-type-' + kind, target_type)

for label, target in [('missing', '#missing'), ('file', '../evidence/endpoint.txt'),
                      ('url', 'https://example.com/spec#schema'), ('requirement', '#contract-owner'),
                      ('capability', '#contract-provider'), ('verification', '#documentation-test')]:
    def invalid(w, target=target):
        w.replace(REFERENCE, REFERENCE.replace('#error-response-specification', target))
        message = w.run('validate', ok=False)
        require('reference' in message.lower(), 'Diagnostic does not identify reference error')
    check('reject-target-' + label, invalid)


def source_invalid(w):
    w.replace('### Documentation\n\nThe system SHALL document error responses.\n\n#### Metadata\n  * type: requirement',
              '### Documentation\n\nThe system SHALL document error responses.\n\n#### Metadata\n  * type: capability')
    require('contract_reference' in w.run('validate', ok=False).lower() or 'Contract References' in w.run('validate', ok=False), 'Missing source diagnostic')


check('reject-source', source_invalid)
check('reject-unowned', lambda w: (w.replace('  * definedBy: [Error Response Specification](#error-response-specification)\n', ''), w.run('validate', ok=False)))
check('reject-duplicate', lambda w: (w.replace(REFERENCE, REFERENCE.rstrip() + '\n  * [Duplicate](#error-response-specification)\n\n'), w.run('validate', ok=False)))
check('reject-binding-conflict', lambda w: (w.replace(REFERENCE, REFERENCE + REFERENCE.replace('Contract References', 'Contract Bindings')), w.run('validate', ok=False)))
check('reject-mixed-distinct-targets', lambda w: (w.replace(REFERENCE, REFERENCE + '#### Contract Bindings\n  * [Other Specification](#other-specification)\n\n'), w.run('validate', ok=False)))


def coverage_isolation(w):
    parsed(w)
    owner = w.coverage()
    equal({key: owner[key] for key in ['coverage_source', 'is_terminal', 'evidence', 'contributing_requirements', 'blocking_requirements']},
          {'coverage_source': 'uncovered', 'is_terminal': True, 'evidence': [], 'contributing_requirements': [], 'blocking_requirements': [MODEL + '#contract-owner']}, 'Reference affected owner coverage')
    equal(owner['aggregate_terminal_requirements'], 1, 'Reference does not add implementation terminals')
    equal(owner['aggregate_covered_terminal_requirements'], 0, 'Reference evidence does not cover owner progress')
    w.replace('The system SHALL implement endpoint errors.\n', 'The system SHALL implement endpoint errors.\n\n#### Contract Bindings\n  * [Error Response Specification](#error-response-specification)\n')
    equal(w.coverage()['coverage_source'], 'contract_consumer_rollup', 'Real binding lost implementation contribution')
    equal(w.coverage()['contributing_requirements'], [MODEL + '#endpoint-implementation'], 'Documentation contributed to fulfillment')


check('coverage-isolation', coverage_isolation)


def adapter_output_obligation(w):
    w.write(BASE + """
### Trace Adapter

When traces are requested, the system SHALL expose the shared core result.

#### Metadata
  * type: requirement

#### Contract References
  * [Error Response Specification](#error-response-specification)

#### Relations
  * specify: [Contract Consumer](#contract-consumer)
---

### Adapter Output

When a result is available, the system SHALL present the output contract.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Other Specification](#other-specification)

#### Relations
  * derivedFrom: [Trace Adapter](#trace-adapter)
  * satisfiedBy: [endpoint.txt](../evidence/endpoint.txt)
---
""")
    w.run('validate')
    core = w.coverage()
    equal(core['coverage_source'], 'uncovered', 'Adapter presentation falsely covers the referenced core')
    equal(core['is_terminal'], True, 'Reference changes core terminal classification')
    equal(core['evidence'], [], 'Adapter evidence leaks into referenced core coverage')
    equal(core['contributing_requirements'], [], 'Adapter creates a core fulfillment dependency')
    adapter = w.coverage('Trace Adapter')
    equal(adapter['coverage_source'], 'requirement_rollup', 'Adapter loses child presentation coverage')
    equal(adapter['contributing_requirements'], [MODEL + '#adapter-output'], 'Wrong adapter contribution')
    output_owner = w.coverage('Other Contract Owner')
    equal(output_owner['coverage_source'], 'contract_consumer_rollup', 'Output obligation lost its binding')
    equal(output_owner['contributing_requirements'], [MODEL + '#adapter-output'], 'Wrong output contract consumer')


check('coverage-adapter-reference-output-binding', adapter_output_obligation)


def collection(w):
    result = w.data('collect', 'Documentation')
    contract = row(result, 'Error Response Specification')
    equal(contract['source_type'], 'contract_reference_element', 'Collection loses reference provenance')
    equal(contract['content'], 'Error responses contain a code and message.', 'Referenced content absent')


check('collect-reference', collection)


def outputs(w):
    for command in ['search', 'containment']:
        item = row(w.data(command), 'Documentation')
        require(item.get('contract_references'), f'{command} drops references')
    w.run('export', '--output', 'site')
    source = (w.root / 'site/assets/project-store.js').read_text()
    store = json.loads(source.removeprefix('window.reqvireProjectStore = ').strip().removesuffix(';'))
    equal([(r['source_id'], r['target']) for r in store['contract_references']], [(MODEL + '#documentation', TARGET)], 'Project Store drops reference endpoints')
    require(any(edge.get('kind') == 'contract_references' and edge['source'] == MODEL + '#documentation' and edge['target'] == TARGET for edge in store['knowledge_graph']['edges']), 'Knowledge graph drops reference edge')
    equal(store['contract_bindings'], [], 'Reference became a Project Store binding')


check('public-projections', outputs)


def rdf(w):
    output = w.run('semantic', 'export', '--layer', 'model')
    require('referencesContract' in output and 'contractReferencedBy' in output and 'contractReferencesTargetIdentifier' in output, 'Missing RDF reference facts')
    require('bindsContract' not in output, 'Reference exported as a binding')


check('semantic-projection', rdf)


def impact(w):
    w.replace('Error responses contain a code and message.', 'Error responses contain a code, message, and details.')
    result = w.data('change-impact')
    text = json.dumps(result)
    require('contract_references' in text and 'Documentation Test' in text and 'evidence/documentation.txt' in text, 'Missing reference impact/downstream context')


check('contract-impact', impact)


def reverse_impact(w):
    parsed(w)
    w.replace('The system SHALL document error responses.', 'The system SHALL document all error responses.')
    result = w.data('change-impact')
    names = {r.get('name') for r in records(result)}
    require('Contract Owner' not in names and 'Error Response Specification' not in names, 'Impact flowed backward through reference')


check('impact-direction', reverse_impact)


def same_hierarchy(w):
    w.replace('  * specify: [Contract Consumer](#contract-consumer)', '  * derivedFrom: [Contract Owner](#contract-owner)')
    parsed(w)


check('same-hierarchy', same_hierarchy)


def link_unlink(w):
    w.replace(REFERENCE, '')
    before = w.snapshot()
    dry = w.run('link', 'Documentation', 'referenceContract', 'Error Response Specification', '--dry-run')
    require('Contract References' in dry, 'Dry run lacks reference edit')
    equal(w.snapshot(), before, 'Dry run wrote files')
    w.run('link', 'Documentation', 'referenceContract', 'Error Response Specification')
    equal(w.refs(), [TARGET], 'Link did not create reference')
    actual = (w.root / MODEL).read_text()
    expected = (FIXTURE / 'expected/linked.md').read_text()
    require(actual == expected, ''.join(difflib.unified_diff(expected.splitlines(True), actual.splitlines(True), 'expected/linked.md', MODEL)))
    w.run('unlink', 'Documentation', 'Error Response Specification')
    equal(w.refs(), [], 'Unlink retained reference')
    actual = (w.root / MODEL).read_text()
    expected = (FIXTURE / 'expected/unlinked.md').read_text()
    require(actual == expected, ''.join(difflib.unified_diff(expected.splitlines(True), actual.splitlines(True), 'expected/unlinked.md', MODEL)))


check('link-unlink-dry-run', link_unlink)


def invalid_mutation(w):
    before = w.snapshot()
    w.run('link', 'Documentation', 'referenceContract', 'Contract Owner', ok=False)
    equal(w.snapshot(), before, 'Invalid reference mutation changed files')
    # A second, valid mutation establishes that rejection was reference-aware.
    w.run('link', 'Endpoint Implementation', 'referenceContract', 'Other Specification')
    equal(w.refs('Endpoint Implementation'), [MODEL + '#other-specification'], 'Valid reference command failed')


check('atomic-invalid-link', invalid_mutation)


def mixed_mutations(w):
    before = w.snapshot()
    message = w.run('link', 'Documentation', 'bindContract', MODEL + '#other-specification', ok=False)
    require('Contract References' in message and 'Contract Bindings' in message, 'Binding rejection must identify both sections')
    equal(w.snapshot(), before, 'Rejected binding wrote files')
    w.run('link', 'Endpoint Implementation', 'bindContract', MODEL + '#other-specification')
    before = w.snapshot()
    message = w.run('link', 'Endpoint Implementation', 'referenceContract', 'Error Response Specification', ok=False)
    require('Contract References' in message and 'Contract Bindings' in message, 'Reference rejection must identify both sections')
    equal(w.snapshot(), before, 'Rejected reference wrote files')


check('atomic-mixed-sections', mixed_mutations)



def rename_move(w):
    w.run('rename', 'Error Response Specification', 'Error Schema')
    equal(w.refs(), [MODEL + '#error-schema'], 'Rename failed to update reference')
    w.run('mv', 'Error Schema', 'specifications/Contracts.md')
    equal(w.refs(), ['specifications/Contracts.md#error-schema'], 'Move failed to update reference')
    w.run('mv', 'Documentation', 'docs/Requirements.md')
    equal(w.refs(), ['specifications/Contracts.md#error-schema'], 'Consumer move lost reference')
    w.run('mv-file', 'specifications/Contracts.md', 'specifications/Shared.md')
    equal(w.refs(), ['specifications/Shared.md#error-schema'], 'File move lost reference')
    w.run('mv-folder', 'specifications', 'model')
    equal(w.refs(), ['model/Shared.md#error-schema'], 'Folder move lost reference')
    w.run('validate')


check('rename-move-preservation', rename_move)


def remove(w):
    parsed(w)
    w.run('rm', 'Error Response Specification')
    equal(w.refs(), [], 'Removal left incoming reference')
    w.run('validate')


check('remove-target', remove)


def format_references(w):
    w.replace(REFERENCE, REFERENCE.replace('[Error Response Specification](#error-response-specification)', '[Wrong label](./Model.md#error-response-specification)'))
    w.run('format', '--fix')
    equal(w.refs(), [TARGET], 'Formatting dropped reference')
    source = (w.root / MODEL).read_text()
    require(REFERENCE in source, 'Formatting did not canonicalize reference label/path')
    before = w.snapshot()
    w.run('format', '--fix')
    equal(w.snapshot(), before, 'Formatting is not idempotent')


check('format-reference', format_references)


def single_contract(w):
    start = BASE.index('### Error Response Specification\n')
    end = BASE.index('### Other Contract Owner\n')
    w.write((BASE[:start] + BASE[end:]).replace('[Error Response Specification](#error-response-specification)', '[Error Response Specification](Contract.md#error-response-specification)'))
    (w.root / 'specifications/Contract.md').write_text('# Element\n\n## Metadata\n  * type: specification\n\n## Relations\n  * define: [Contract Owner](Model.md#contract-owner)\n\n## Error Response Specification\n\nError responses contain a code and message.\n')
    w.run('validate')
    equal(w.refs(), ['specifications/Contract.md#error-response-specification'], 'Single-element target lost')


check('single-element-contract', single_contract)


def override_and_merge(w):
    fresh = "### Additional Documentation\n\nThe system SHALL describe the other contract.\n\n#### Metadata\n  * type: requirement\n\n#### Contract References\n  * [Other Specification](#other-specification)\n\n#### Relations\n  * specify: [Contract Consumer](#contract-consumer)\n"
    w.run('add', MODEL, input=fresh)
    equal(w.refs('Additional Documentation'), [MODEL + '#other-specification'], 'Create dropped reference')
    before = w.snapshot()
    bad = fresh.replace('#### Relations', '#### Contract Bindings\n  * [Error Response Specification](#error-response-specification)\n\n#### Relations')
    w.run('add', MODEL, '--override', input=bad, ok=False)
    equal(w.snapshot(), before, 'Invalid override wrote files')
    w.run('merge', 'Documentation', 'Additional Documentation')
    equal(w.refs(), [TARGET, MODEL + '#other-specification'], 'Merge lost reference')
    w.run('link', 'Endpoint Implementation', 'bindContract', MODEL + '#other-specification')
    before = w.snapshot()
    w.run('merge', 'Documentation', 'Endpoint Implementation', ok=False)
    equal(w.snapshot(), before, 'Mixed-section merge wrote files')


check('create-override-merge', override_and_merge)


def pure_relocation(w):
    parsed(w)
    w.run('mv', 'Error Response Specification', 'specifications/Shared.md')
    report = w.data('change-impact')
    equal(w.refs(), ['specifications/Shared.md#error-response-specification'], 'Relocation lost reference')
    names = [r.get('name') for r in records(report.get('changed', []))]
    require('Documentation' not in names, 'Pure relocation propagated content impact')


check('reference-relocation', pure_relocation)

def changed_references(w):
    w.run('link', 'Documentation', 'referenceContract', 'Other Specification')
    result = row(w.data('change-impact')['changed'], 'Documentation')
    equal(result['changed_contract_references'], [MODEL + '#other-specification'], 'Reference addition absent from diff')
    w.commit()
    w.run('unlink', 'Documentation', 'Other Specification')
    result = row(w.data('change-impact')['changed'], 'Documentation')
    equal(result['changed_contract_references'], [MODEL + '#other-specification'], 'Reference removal absent from diff')


check('reference-membership-impact', changed_references)


def collect_once(w):
    # An ancestor owns the referenced target: collect must preserve its source citation once.
    w.replace('  * specify: [Contract Consumer](#contract-consumer)', '  * derivedFrom: [Contract Owner](#contract-owner)')
    result = w.data('collect', 'Documentation')
    found = [item for item in result['items'] if item['identifier'] == TARGET]
    equal(len(found), 1, 'Owned and referenced contract duplicated')
    equal(found[0]['file_path'], MODEL, 'Contract citation missing')
    equal(result['metadata']['total_items'], len(result['items']), 'Collection count mismatch')


check('collection-deduplication', collect_once)


def descendant_impact(w):
    w.run('add', MODEL, input="### Detailed Documentation\n\nThe system SHALL describe errors in detail.\n\n#### Metadata\n  * type: requirement\n\n#### Relations\n  * derivedFrom: [Documentation](#documentation)\n  * satisfiedBy: [documentation.txt](../evidence/documentation.txt)\n")
    w.commit()
    w.replace('Error responses contain a code and message.', 'Error responses contain a code, message, and details.')
    report = w.data('change-impact')
    require('Detailed Documentation' in json.dumps(report), 'Contract impact lost descendant')


check('reference-descendant-impact', descendant_impact)


def valid_override(w):
    body = "### Documentation\n\nThe system SHALL document error responses.\n\n#### Metadata\n  * type: requirement\n\n#### Contract References\n  * [Other Specification](#other-specification)\n\n#### Relations\n  * specify: [Contract Consumer](#contract-consumer)\n"
    w.run('add', MODEL, '--override', input=body)
    equal(w.refs(), [MODEL + '#other-specification'], 'Valid override lost reference')


check('valid-reference-override', valid_override)
def relation_conflict(w):
    w.replace('  * satisfiedBy: [documentation.txt]', '  * definedBy: [Error Response Specification](#error-response-specification)\n  * satisfiedBy: [documentation.txt]')
    error = w.run('validate', ok=False)
    require('Contract References' in error and 'Relations' in error, 'Missing duplicate relation diagnostic')


check('reject-relation-reference-conflict', relation_conflict)


def block(w, name):
    text = (w.root / MODEL).read_text()
    start = text.index('### ' + name + '\n')
    end = text.index('\n---', start)
    return text[start:end]


def with_dependency(body, section, contract):
    return body.replace('#### Relations', f'#### {section}\n  * [{contract}](#{contract.lower().replace(" ", "-")})\n\n#### Relations')


def dependency(w, source, contract, section='Contract References'):
    old = block(w, source)
    w.replace(old, with_dependency(old, section, contract))


def circular_error(message):
    require('circular' in message.lower() and 'Contract References' in message, 'Missing reference-cycle diagnostic: ' + message)
    require('#' in message and 'contract' in message.lower(), 'Cycle diagnostic lacks participating identifiers')


def rejected_cycle(w):
    first = w.run('validate', ok=False)
    circular_error(first)
    equal(w.run('validate', ok=False), first, 'Cycle diagnostic is nondeterministic')
    circular_error(w.run('coverage', '--json', ok=False))
    circular_error(w.run('change-impact', '--json', ok=False))


def reciprocal(w):
    dependency(w, 'Contract Owner', 'Other Specification')
    dependency(w, 'Other Contract Owner', 'Error Response Specification')
    rejected_cycle(w)


check('reject-reference-cycle', reciprocal)
check('reject-reference-self-cycle', lambda w: (dependency(w, 'Contract Owner', 'Error Response Specification'), rejected_cycle(w)))


def own_documentation_contract(w):
    w.replace('  * satisfiedBy: [documentation.txt]', '  * definedBy: [Documentation Specification](#documentation-specification)\n  * satisfiedBy: [documentation.txt]')
    text = (w.root / MODEL).read_text()
    w.write(text + '\n### Documentation Specification\n\nDocumentation contract.\n\n#### Metadata\n  * type: specification\n---\n')


def longer_cycle(w):
    own_documentation_contract(w)
    dependency(w, 'Contract Owner', 'Other Specification')
    dependency(w, 'Other Contract Owner', 'Documentation Specification')
    rejected_cycle(w)


check('reject-long-reference-cycle', longer_cycle)


def mixed_cycle(w):
    dependency(w, 'Contract Owner', 'Other Specification')
    dependency(w, 'Other Contract Owner', 'Error Response Specification', 'Contract Bindings')
    rejected_cycle(w)


check('reject-reference-binding-cycle', mixed_cycle)


def hierarchy_cycle(w):
    dependency(w, 'Contract Owner', 'Other Specification')
    old = block(w, 'Other Contract Owner')
    w.replace(old, old.replace('specify: [Contract Provider](#contract-provider)', 'derivedFrom: [Contract Owner](#contract-owner)'))
    rejected_cycle(w)


check('reject-reference-hierarchy-cycle', hierarchy_cycle)


def atomic_cycle(w, command):
    dependency(w, 'Contract Owner', 'Other Specification')
    w.run('validate')
    before = w.snapshot()
    if command in ['link', 'dry-run']:
        message = w.run('link', 'Other Contract Owner', 'referenceContract', 'Error Response Specification',
                        *(['--dry-run'] if command == 'dry-run' else []), ok=False)
    elif command == 'binding-link':
        message = w.run('link', 'Other Contract Owner', 'bindContract', TARGET, ok=False)
    elif command == 'override':
        body = with_dependency(block(w, 'Other Contract Owner'), 'Contract References', 'Error Response Specification')
        message = w.run('add', MODEL, '--override', input=body, ok=False)
    elif command == 'hierarchy-edit':
        body = block(w, 'Other Contract Owner').replace('specify: [Contract Provider](#contract-provider)', 'derivedFrom: [Contract Owner](#contract-owner)')
        message = w.run('add', MODEL, '--override', input=body, ok=False)
    else:
        message = w.run('merge', 'Contract Owner', 'Other Contract Owner', ok=False)
    circular_error(message)
    equal(w.snapshot(), before, f'{command} persisted an invalid cycle')
    w.run('validate')


for command in ['link', 'dry-run', 'binding-link', 'override', 'hierarchy-edit', 'merge']:
    check('atomic-reference-cycle-' + command, lambda w, command=command: atomic_cycle(w, command))


def relink_cycle(w):
    own_documentation_contract(w)
    dependency(w, 'Other Contract Owner', 'Documentation Specification')
    w.run('validate')
    before = w.snapshot()
    message = w.run('relink', 'Documentation', 'referenceContract', 'Error Response Specification', 'Other Specification', ok=False)
    circular_error(message)
    equal(w.snapshot(), before, 'Relink persisted a cycle')
    w.run('validate')


check('atomic-reference-cycle-relink', relink_cycle)


def create_cycle(w):
    body = '### New Parent\n\nThe system SHALL coordinate contracts.\n\n#### Metadata\n  * type: requirement\n\n#### Relations\n  * specify: [Contract Provider](#contract-provider)\n  * derive: [Contract Owner](#contract-owner)\n'
    before = w.snapshot()
    message = w.run('add', MODEL, input=with_dependency(body, 'Contract References', 'Error Response Specification'), ok=False)
    circular_error(message)
    equal(w.snapshot(), before, 'Create persisted a cycle')
    w.run('add', MODEL, input=body)
    w.run('validate')


check('atomic-reference-cycle-create', create_cycle)


def relink_reference(w):
    before = w.snapshot()
    w.run('relink', 'Documentation', 'referenceContract', 'Error Response Specification', 'Other Specification', '--dry-run')
    equal(w.snapshot(), before, 'Dry-run relink wrote source')
    w.run('relink', 'Documentation', 'referenceContract', 'Error Response Specification', 'Other Specification')
    equal(w.refs(), [MODEL + '#other-specification'], 'Relink replaced wrong reference')
    before = w.snapshot()
    w.run('relink', 'Documentation', 'referenceContract', 'Other Specification', 'Documentation', ok=False)
    equal(w.snapshot(), before, 'Invalid-target relink wrote source')


check('reference-relink-success-dry-run-invalid', relink_reference)


def invalid_mutations(w):
    before = w.snapshot()
    w.run('link', 'Contract Provider', 'referenceContract', 'Error Response Specification', ok=False)
    equal(w.snapshot(), before, 'Non-requirement reference link wrote source')
    for label, body in [
        ('source type', block(w, 'Documentation').replace('type: requirement', 'type: capability')),
        ('target type', block(w, 'Error Response Specification').replace('type: specification', 'type: capability')),
        ('target ownership', block(w, 'Contract Owner').replace('  * definedBy: [Error Response Specification](#error-response-specification)\n', '')),
    ]:
        try:
            w.run('add', MODEL, '--override', input=body, ok=False)
        except AssertionError as error:
            raise AssertionError(label + ': ' + str(error)) from error
        equal(w.snapshot(), before, label + ' edit wrote invalid source')
    w.run('validate')


check('atomic-invalid-source-target-ownership-edits', invalid_mutations)


def override_referenced_contract(w):
    body = block(w, 'Error Response Specification').replace('code and message.', 'code, message, and details.')
    before = w.snapshot()
    w.run('add', MODEL, '--override', '--dry-run', input=body)
    equal(w.snapshot(), before, 'Contract override dry-run wrote source')
    w.run('add', MODEL, '--override', input=body)
    equal(w.refs(), [TARGET], 'Overriding target removed incoming reference')
    w.run('validate')
    require('details' in row(w.data('collect', 'Documentation'), 'Error Response Specification')['content'], 'Override lost target ownership/content')


check('override-referenced-contract-retains-consumers', override_referenced_contract)


def search_names(w, *filters):
    result = w.data('search', *filters)
    return sorted(item['name'] for file in result['files'].values() for item in file['elements'])


def reference_search(w):
    equal(search_names(w, '--has-contract-references'), ['Documentation'], 'Reference presence filter')
    equal(search_names(w, '--filter-contract-references', '*#error-response-*'), ['Documentation'], 'Reference target glob')
    equal(search_names(w, '--filter-contract-references', TARGET, '--short'), ['Documentation'], 'Exact target/short filter')
    text = w.run('search', '--has-contract-references', '--short')
    require('Documentation' in text and 'Endpoint Implementation' not in text, 'Text filter differs from JSON')


check('reference-search-presence-target', reference_search)
check('reference-search-composition', lambda w: equal(search_names(w, '--has-contract-references', '--have-relations', 'satisfiedBy', '--filter-name', '^Documentation$'), ['Documentation'], 'Conjunctive reference filter'))
check('reference-search-empty', lambda w: (equal(search_names(w, '--filter-contract-references', '*#absent'), [], 'Unmatched target filter'), equal(search_names(w, '--has-contract-references', '--filter-name', '^Contract Owner$'), [], 'Unmatched name filter')))


def separate_filters(w):
    w.run('link', 'Endpoint Implementation', 'bindContract', TARGET)
    equal(search_names(w, '--has-contract-references'), ['Documentation'], 'Bindings counted as references')
    equal(search_names(w, '--has-contract-bindings'), ['Endpoint Implementation'], 'References counted as bindings')
    equal(search_names(w, '--has-contract-references', '--has-contract-bindings'), [], 'Disjoint dependency filters')


check('reference-search-binding-separation', separate_filters)
check('reference-search-invalid-glob', lambda w: require('glob' in w.run('search', '--filter-contract-references', '[', ok=False).lower(), 'Invalid glob diagnostic'))


def invalid_relation_filters(w):
    for flag in ['--have-relations', '--not-have-relations']:
        for keyword in ['referenceContract', 'bindContract']:
            message = w.run('search', flag, keyword, ok=False)
            require('Invalid relation type' in message and 'Valid types' in message, 'Invalid relation keyword diagnostic')


check('reject-dependency-keyword-relation-filter', invalid_relation_filters)


def expected_files(w, scenario):
    expected_root = FIXTURE / 'expected' / scenario
    expected = {str(path.relative_to(expected_root)): path.read_text()
                for path in expected_root.rglob('*.md')}
    require(expected, f'Missing expected files for {scenario}')
    actual = w.snapshot()
    equal(sorted(actual), sorted(expected), f'{scenario}: unexpected source file set')
    for name, content in expected.items():
        require(actual[name] == content, ''.join(difflib.unified_diff(
            content.splitlines(True), actual[name].splitlines(True),
            f'expected/{scenario}/{name}', name)))


def owner_type_override(w, dry_run):
    w.run('validate')
    before = w.snapshot()
    body = block(w, 'Contract Owner').replace('type: requirement', 'type: capability')
    body = body.replace('specify: [Contract Provider]', 'derivedFrom: [Contract Provider]')
    message = w.run('add', MODEL, '--override', *(['--dry-run'] if dry_run else []),
                    input=body, ok=False)
    require('Contract Owner' in message and 'Error Response Specification' in message,
            'Owner-type rejection must identify the owner and affected contract')
    equal(w.snapshot(), before, 'Owner-type override wrote an invalid model')
    w.run('validate')


for dry_run in [False, True]:
    check('reject-reference-owner-type-' + ('dry-run' if dry_run else 'apply'),
          lambda w, dry_run=dry_run: owner_type_override(w, dry_run))


def cross_file_override(w, dry_run):
    w.run('validate')
    before = w.snapshot()
    result = json.loads(w.run('add', 'specifications/Other.md', '--override', '--json',
                             *(['--dry-run'] if dry_run else []),
                             input=block(w, 'Error Response Specification')))
    if dry_run:
        equal(w.snapshot(), before, 'Cross-file override dry-run wrote source')
        w.run('validate')
        additions = [line['content'].removeprefix('+').strip() for diff in result['diffs']
                     if diff['file_path'] == MODEL for line in diff['lines']
                     if line['color'] == 'green']
        for line in [
            '* [Error Response Specification](Other.md#error-response-specification)',
            '* definedBy: [Error Response Specification](Other.md#error-response-specification)',
        ]:
            require(line in additions, f'Cross-file override preview omits dependency rewrite: {line}')
    else:
        equal(w.refs(), ['specifications/Other.md#error-response-specification'],
              'Cross-file override dropped the incoming reference')
        w.run('validate')
        expected_files(w, 'cross-file-override')


for dry_run in [False, True]:
    check('cross-file-reference-override-' + ('dry-run' if dry_run else 'apply'),
          lambda w, dry_run=dry_run: cross_file_override(w, dry_run))


def same_owner_contract_merge(w, dry_run):
    # Both contracts have exactly one owner before the merge; one consumer uses both.
    w.replace('  * definedBy: [Other Specification](#other-specification)\n', '')
    w.replace('  * definedBy: [Error Response Specification](#error-response-specification)\n',
              '  * definedBy: [Error Response Specification](#error-response-specification)\n'
              '  * definedBy: [Other Specification](#other-specification)\n')
    w.replace(REFERENCE, REFERENCE.rstrip() + '\n  * [Other Specification](#other-specification)\n\n')
    w.run('validate')
    before = w.snapshot()
    result = json.loads(w.run('merge', 'Error Response Specification', 'Other Specification',
                             '--json', *(['--dry-run'] if dry_run else [])))
    if dry_run:
        equal(w.snapshot(), before, 'Contract merge dry-run wrote source')
        w.run('validate')
        removed = [line['content'].removeprefix('-').strip() for diff in result['diffs']
                   for line in diff['lines'] if line['color'] == 'red']
        require('### Other Specification' in removed, 'Merge preview omits removal of source contract')
    else:
        equal(w.refs(), [TARGET], 'Merge did not consolidate references to the surviving contract')
        w.run('validate')
        expected_files(w, 'merged-contracts')


for dry_run in [False, True]:
    check('merge-referenced-contracts-' + ('dry-run' if dry_run else 'apply'),
          lambda w, dry_run=dry_run: same_owner_contract_merge(w, dry_run))


def normalized_reference(w, href):
    w.replace(REFERENCE, REFERENCE.replace('#error-response-specification', href))
    parsed(w)


for label, href in [('fragment', '#Error-Response-Specification'),
                    ('file-qualified', 'Model.md#Error-Response-Specification')]:
    check('normalize-authored-reference-' + label,
          lambda w, href=href: normalized_reference(w, href))


def duplicate_normalized_reference(w):
    w.replace(REFERENCE, REFERENCE.rstrip() + '\n  * [Same contract](#Error-Response-Specification)\n\n')
    before = w.snapshot()
    message = w.run('validate', ok=False)
    require('duplicate' in message.lower() and 'reference' in message.lower(),
            'Equivalent spellings must fail as duplicate references, not as missing targets:\n' + message)
    equal(w.snapshot(), before, 'Validation changed authored duplicate references')


check('reject-normalized-reference-duplicate', duplicate_normalized_reference)


def normalized_reference_command(w, href):
    w.replace(REFERENCE, '')
    w.run('validate')
    before = w.snapshot()
    w.run('link', 'Documentation', 'referenceContract', href, '--dry-run')
    equal(w.snapshot(), before, 'Normalized reference dry-run wrote source')
    w.run('link', 'Documentation', 'referenceContract', href)
    parsed(w)
    actual = (w.root / MODEL).read_text()
    expected = (FIXTURE / 'expected/linked.md').read_text()
    require(actual == expected, ''.join(difflib.unified_diff(
        expected.splitlines(True), actual.splitlines(True), 'expected/linked.md', MODEL)))


for label, href in [('fragment', '#Error-Response-Specification'),
                    ('file-qualified', 'Model.md#Error-Response-Specification')]:
    check('normalize-reference-command-' + label,
          lambda w, href=href: normalized_reference_command(w, href))


def convergent_reference_impact(w):
    # Documentation is reachable via both ownership/derive and an explicit reference.
    w.replace('  * specify: [Contract Consumer](#contract-consumer)',
              '  * derivedFrom: [Contract Owner](#contract-owner)')
    w.run('validate')
    w.commit()
    w.replace('Error responses contain a code and message.',
              'Error responses contain a code, message, and details.')
    result = w.data('change-impact')
    equal([item['name'] for item in result['changed']], ['Error Response Specification'],
          'Fixture must change only the contract content')
    equal([item['target_text'] for item in result['invalidated_verifications']], ['Documentation Test'],
          'Convergent impact must invalidate each verification once')
    tree = result['changed'][0]['change_impact_tree']
    require(any(item.get('identifier', '').endswith('/evidence/documentation.txt')
                for item in records(tree)), 'Convergent impact lost downstream implementation evidence')
    edges = [item['contract_references'] for item in records(tree) if 'contract_references' in item]
    equal([edge['name'] for edge in edges], ['Documentation'],
          'Hierarchy traversal hid the explicit reference impact edge')
    require(edges[0]['identifier'].endswith('/' + MODEL + '#documentation'),
            'Reference edge points to the wrong consumer')


check('convergent-impact-preserves-reference-edge', convergent_reference_impact)

OUTPUT.mkdir(exist_ok=True)
(OUTPUT / 'checks.txt').write_text('\n'.join(checks) + '\n')
(OUTPUT / 'failures.txt').write_text('\n\n'.join(failures) + '\n')
print(f'{len(checks) - len(failures)} passed; {len(failures)} failed')
raise SystemExit(bool(failures))
