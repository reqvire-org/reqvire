"""Atomic override regressions through the production stdio MCP boundary."""
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile

SUITE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('transports', SUITE.parent / 'test-mcp-server/check_transports.py')
transport = importlib.util.module_from_spec(spec)
spec.loader.exec_module(transport)
binary, destination = sys.argv[1:]
binary = str(Path(binary).resolve())
output = Path(destination) / 'output/atomic'
output.mkdir(parents=True, exist_ok=True)


def element(name, kind, body='Original definition.', metadata='', sections=''):
    return f'### {name}\n\n{body}\n\n#### Metadata\n  * type: {kind}\n{metadata}{sections}'


def relations(*entries):
    return '\n#### Relations\n' + ''.join(f'  * {kind}: [{name}]({target})\n' for kind, name, target in entries)


PRODUCT = element('Product', 'capability')
PARENT = element('Parent', 'requirement', metadata='  * status: approved\n', sections=relations(('specify', 'Product', 'Capabilities.md#product')))
CONTRACT = element('Shared Contract', 'specification')
TOPIC = element('Topic', 'concept', sections=relations(('derivedFrom', 'Vocabulary', '#vocabulary')))
PLAN = element('Verification Plan', 'verification-objective')
FILES = {
    'Capabilities.md': [PRODUCT, element('Subsystem', 'capability', sections=relations(('derivedFrom', 'Product', '#product'))), element('Independent Product', 'capability')],
    'Requirements.md': [PARENT, element('Owner', 'requirement', sections=relations(('specify', 'Product', 'Capabilities.md#product'), ('definedBy', 'Shared Contract', 'Contracts.md#shared-contract')))],
    'Children.md': [element('Child', 'requirement', sections=relations(('derivedFrom', 'Parent', 'Requirements.md#parent')))],
    'Contracts.md': [CONTRACT],
    'Consumers.md': [element('Reader', 'requirement', sections='\n#### Concept References\n  * [Topic](Concepts.md#topic)\n\n#### Contract References\n  * [Shared Contract](Contracts.md#shared-contract)\n' + relations(('specify', 'Independent Product', 'Capabilities.md#independent-product'))), element('Implementer', 'requirement', sections='\n#### Contract Bindings\n  * [Shared Contract](Contracts.md#shared-contract)\n' + relations(('specify', 'Independent Product', 'Capabilities.md#independent-product')))],
    'Concepts.md': [element('Vocabulary', 'concept-scheme', metadata='  * concept_base: https://example.test/override-vocabulary\n  * concept_prefix: vocab\n'), TOPIC],
    'Verifications.md': [PLAN, element('Child Verification', 'test-verification', sections=relations(('derivedFrom', 'Verification Plan', '#verification-plan'), ('satisfiedBy', 'evidence.txt', 'evidence.txt'), ('verify', 'Child', 'Children.md#child')))],
}


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def snapshot(root):
    return {str(p.relative_to(root)): p.read_bytes() for p in root.rglob('*')
            if p.is_file() and '.git' not in p.relative_to(root).parts}


def state(server):
    value = transport.call(server, 'reqvire.workspace_status')['context']
    return {key: value[key] for key in ('model_revision', 'pending_changes', 'head')}


def read(server, name):
    return transport.call(server, 'reqvire.read_element', name=name)


def override(server, name, content, file, dry_run=False):
    value = transport.call(server, 'reqvire.add_element', file=file, content=content,
                           override_existing=True, dry_run=dry_run)
    require(value['operation'] == 'update' and value['element_name'] == name, f'Wrong update evidence: {value}')
    return value


def reject(server, root, tool, **arguments):
    before, accepted = snapshot(root), state(server)
    result = server.rpc('tools/call', {'name': tool, 'arguments': arguments})
    require(result.get('isError'), f'Invalid candidate accepted: {result}')
    require(snapshot(root) == before, 'Rejected candidate changed source files')
    require(state(server) == accepted, 'Rejected candidate changed accepted revision or pending work')
    return result


def validate(root):
    result = subprocess.run([binary, '--workspace', str(root), 'validate', '--json'], capture_output=True, text=True, timeout=20)
    require(result.returncode == 0, result.stdout + result.stderr)


def changed_files(before, after):
    return {name for name in before.keys() | after.keys() if before.get(name) != after.get(name)}


def same_file(server, root):
    for name, file, source in [('Product', 'Capabilities.md', PRODUCT), ('Parent', 'Requirements.md', PARENT)]:
        before, accepted = snapshot(root), state(server)
        updated = source.replace('Original definition.', 'Updated definition.')
        preview = override(server, name, updated, file, True)
        require(snapshot(root) == before and state(server) == accepted, 'Preview changed source or accepted state')
        applied = override(server, name, updated, file)
        require(preview['diffs'] == applied['diffs'], 'Apply differs from validated preview')
        require(changed_files(before, snapshot(root)) == {file}, 'Prose override rewrote incoming declarations')
        require('Updated definition.' in read(server, name)['content'], 'Updated definition is not visible')
        require(read(server, 'Child')['identifier'] == 'Children.md#child', 'Child identity changed')
        validate(root)
    search = transport.call(server, 'reqvire.search', filter_name='^Child$', filter_status='approved')
    names = {item['name'] for file in search['files'].values() for item in file['elements']}
    require(names == {'Child'}, 'Child lost inherited governance')
    override(server, 'Parent', PARENT.replace('  * status: approved\n', ''), 'Requirements.md')
    require('status' not in read(server, 'Parent')['metadata'], 'Omitted authored metadata was merged back into the replacement')
    search = transport.call(server, 'reqvire.search', filter_name='^Child$')
    children = [item for file in search['files'].values() for item in file['elements']]
    require(len(children) == 1 and children[0]['governance_metadata']['status']['source'] == 'default', 'Child retained stale inherited governance')
    deletion = reject(server, root, 'reqvire.remove_element', element_name='Parent')
    require('orphan' in json.dumps(deletion).lower() and 'Child' in json.dumps(deletion), 'Actual deletion lost its orphan protection')


def incoming_consumers(server, root):
    for name, file, source, consumers in [
        ('Shared Contract', 'Contracts.md', CONTRACT, ['Owner', 'Reader', 'Implementer']),
        ('Topic', 'Concepts.md', TOPIC, ['Reader']),
        ('Verification Plan', 'Verifications.md', PLAN, ['Child Verification']),
    ]:
        before = {name: read(server, name) for name in consumers}
        sources = snapshot(root)
        override(server, name, source.replace('Original definition.', 'Updated definition.'), file)
        require(changed_files(sources, snapshot(root)) == {file}, 'Incoming consumer document was rewritten')
        for consumer in consumers:
            for key in ('relations', 'contract_references', 'contract_bindings', 'concept_references'):
                require(read(server, consumer).get(key) == before[consumer].get(key), f'{consumer} lost {key}')
        validate(root)


def outgoing_replacement(server, root):
    # Mandatory has no independently authored parent; omission must be rejected.
    for dry_run in (True, False):
        reject(server, root, 'reqvire.add_element', file='Requirements.md', content=PARENT,
               override_existing=True, dry_run=dry_run)
    # Optional retains its own capability parent when Parent omits its derive.
    updated = PARENT + '  * derive: [Mandatory](Owned.md#mandatory)\n'
    override(server, 'Parent', updated, 'Requirements.md')
    require('Owned.md#optional' not in json.dumps(read(server, 'Parent')['relations']), 'Omitted outgoing edge was resurrected')
    require('Requirements.md#parent' not in json.dumps(read(server, 'Optional')['relations']), 'Stale generated inverse survived')
    require('Requirements.md#parent' in json.dumps(read(server, 'Mandatory')['relations']), 'Retained outgoing edge was lost')
    validate(root)


def invalid_replacements(server, root):
    candidates = [
        ('Product', 'Capabilities.md', PRODUCT.replace('type: capability', 'type: specification')),
        ('Parent', 'Requirements.md', element('Parent', 'requirement')),
        ('Parent', 'Requirements.md', PARENT + '  * derivedFrom: [Child](Children.md#child)\n'),
        ('Parent', 'Requirements.md', PARENT + '  * specify: [Independent Product](Capabilities.md#independent-product)\n'),
        ('Shared Contract', 'Contracts.md', CONTRACT.replace('type: specification', 'type: capability')),
        ('Owner', 'Requirements.md', element('Owner', 'requirement', sections=relations(('specify', 'Product', 'Capabilities.md#product')))),
        ('Topic', 'Concepts.md', TOPIC.replace('type: concept', 'type: capability')),
        ('Vocabulary', 'Concepts.md', element('Vocabulary', 'concept-scheme')),
        ('Verification Plan', 'Verifications.md', PLAN.replace('type: verification-objective', 'type: capability')),
        ('Parent', 'Requirements.md', PARENT.replace('Original definition.', 'See [Child](Children.md#child).')),
    ]
    for name, file, content in candidates:
        for dry_run in (True, False):
            reject(server, root, 'reqvire.add_element', file=file, content=content,
                   override_existing=True, dry_run=dry_run)
        require(read(server, name)['name'] == name, 'Rejection damaged subsequent reads')
    override(server, 'Parent', PARENT.replace('Original definition.', 'Valid after rejection.'), 'Requirements.md')
    validate(root)


def cross_file(server, root):
    for name, source, file in [('Parent', PARENT, 'Requirements.md'), ('Shared Contract', CONTRACT, 'Contracts.md'), ('Topic', TOPIC, 'Concepts.md')]:
        destination = 'relocated/' + file
        # The complete payload is authored relative to its requested destination.
        content = source.replace('Capabilities.md#', '../Capabilities.md#').replace('](#vocabulary)', '](../Concepts.md#vocabulary)')
        before = snapshot(root)
        preview = override(server, name, content, destination, True)
        require(snapshot(root) == before, 'Cross-file preview wrote source')
        applied = override(server, name, content, destination)
        require(preview['diffs'] == applied['diffs'], 'Cross-file preview differs from apply')
        require(read(server, name)['identifier'] == destination + '#' + {'Parent': 'parent', 'Shared Contract': 'shared-contract', 'Topic': 'topic'}[name], 'Wrong relocated identity')
        validate(root)
    require('relocated/Requirements.md#parent' in (root / 'Children.md').read_text(), 'Child reference did not relocate')
    consumers = (root / 'Consumers.md').read_text()
    require(consumers.count('relocated/Contracts.md#shared-contract') == 2, 'Contract consumers did not both relocate')
    require('relocated/Concepts.md#topic' in consumers, 'Concept reference did not relocate')


cases = [('same-file-parent-preview-apply', same_file), ('incoming-consumers', incoming_consumers),
         ('outgoing-definition-replacement', outgoing_replacement),
         ('invalid-replacements-atomic', invalid_replacements), ('cross-file-incoming-rewrites', cross_file)]
for name, run in cases:
    with tempfile.TemporaryDirectory(prefix='reqvire-atomic-override-') as temp:
        root = Path(temp) / 'model'
        root.mkdir()
        files = dict(FILES)
        if run is outgoing_replacement:
            files['Requirements.md'] = [PARENT + '  * derive: [Mandatory](Owned.md#mandatory)\n  * derive: [Optional](Owned.md#optional)\n', *FILES['Requirements.md'][1:]]
            files['Owned.md'] = [element('Mandatory', 'requirement'), element('Optional', 'requirement', sections=relations(('specify', 'Product', 'Capabilities.md#product')))]
        for file, elements in files.items():
            (root / file).write_text('# Elements\n\n' + '\n---\n\n'.join(elements) + '\n---\n')
        (root / 'evidence.txt').write_text('Stable unrelated evidence.\n')
        transport.git(root, 'init', '-qb', 'main')
        transport.git(root, 'add', '.')
        transport.git(root, '-c', 'user.name=Override Test', '-c', 'user.email=override@example.invalid', '-c', 'commit.gpgSign=false', 'commit', '-qm', 'fixture')
        validate(root)
        head, index = transport.git(root, 'rev-parse', 'HEAD'), transport.git(root, 'write-tree')
        with transport.Stdio(binary, root, output / name, '--enable-mutations') as server:
            run(server, root)
        require(transport.git(root, 'rev-parse', 'HEAD') == head and transport.git(root, 'write-tree') == index, 'Override committed or staged implicitly')
        require((root / 'evidence.txt').read_text() == 'Stable unrelated evidence.\n', 'Evidence was changed')
        print('PASS ' + name, flush=True)
