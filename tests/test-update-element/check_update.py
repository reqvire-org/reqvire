"""Focused-editing acceptance scenarios through the real stdio MCP server.

The initial red run used a discoverable no-op facade. Assertions observe files,
accepted reads/revisions and normal protocol evidence; successful dispatch alone
does not prove that an edit occurred.
"""
import argparse
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import traceback

SUITE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('transports', SUITE.parent / 'test-mcp-server/check_transports.py')
transport = importlib.util.module_from_spec(spec)
spec.loader.exec_module(transport)
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('binary')
parser.add_argument('destination')
parser.add_argument('--only', action='append', default=[], help='Run only this exact scenario; repeatable.')
arguments = parser.parse_args()
binary, destination = arguments.binary, arguments.destination
binary = str(Path(binary).resolve())
output = Path(destination) / 'output/focused'
output.mkdir(parents=True, exist_ok=True)
TOOL = 'reqvire.update_element'
CASES = []


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def element(name, kind, text, sections='', metadata=''):
    return f'### {name}\n\n{text}\n\n#### Metadata\n  * type: {kind}\n{metadata}{sections}\n---\n'


def relations(*entries):
    return '\n#### Relations\n' + ''.join(f'  * {kind}: [{name}]({target})\n' for kind, name, target in entries)


ONTOLOGY = '''```turtle
@prefix item: <https://example.org/items#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
<https://example.org/items> a owl:Ontology .
item:Item a owl:Class ; rdfs:comment "Original ontology text." .
item:name a owl:DatatypeProperty ; rdfs:domain item:Item .
```'''
SHAPES = '''```turtle
@prefix item: <https://example.org/items#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
item:ItemShape a sh:NodeShape ; sh:targetClass item:Item ;
  sh:property [ sh:path item:name ; sh:minCount 0 ] .
```'''
QUERY = '''```sparql
PREFIX item: <https://example.org/items#>
SELECT ?item WHERE { ?item a item:Item ; item:name ?name }
```'''
PARENT_TEXT = 'The system SHALL preserve café decisions. Repeat token. Repeat token.'
DETAILS_FENCE = '```markdown\n#### Metadata\n### Literal example\n```\n'
DETAILS = 'Original clarification.\n\n' + DETAILS_FENCE + '\n##### Nested note\nNested prose.'
FILES = {
    'Capabilities.md': '# Elements\n\n' + element('Product', 'capability', 'Primary ability.') + element('Independent Product', 'capability', 'Independent ability.') + element('Subsystem', 'capability', 'Child ability.', relations(('derivedFrom', 'Product', '#product'))),
    'Requirements.md': '# Elements\n\n' + element('Parent', 'requirement', PARENT_TEXT + '\n\n#### Details\n' + DETAILS + '\n\n#### Addendum\nExisting decision.', relations(('specify', 'Product', 'Capabilities.md#product')), '  * status: approved\n  * owner: Model Team\n') + element('Owner', 'requirement', 'Own shared behavior.', relations(('specify', 'Product', 'Capabilities.md#product'), ('definedBy', 'Shared Contract', 'Contracts.md#shared-contract'))),
    'Children.md': '# Elements\n\n' + element('Child', 'requirement', 'Retained child.', relations(('derivedFrom', 'Parent', 'Requirements.md#parent'))),
    'Contracts.md': '# Elements\n\n' + element('Shared Contract', 'specification', 'Shared behavior.'),
    'Consumers.md': '# Elements\n\n' + element('Reader', 'requirement', 'Read shared behavior.', '\n#### Concept References\n  * [Topic](Concepts.md#topic)\n\n#### Contract References\n  * [Shared Contract](Contracts.md#shared-contract)\n' + relations(('specify', 'Independent Product', 'Capabilities.md#independent-product'))) + element('Implementer', 'requirement', 'Implement shared behavior.', '\n#### Contract Bindings\n  * [Shared Contract](Contracts.md#shared-contract)\n' + relations(('specify', 'Independent Product', 'Capabilities.md#independent-product'))),
    'Concepts.md': '# Elements\n\n' + element('Vocabulary', 'concept-scheme', 'Curated words.', metadata='  * concept_base: https://example.org/vocabulary\n  * concept_prefix: vocab\n') + element('Topic', 'concept', 'Topic definition.\n\n#### Labels\n  * altLabel: A topic\n\n#### Scope Note\nExisting scope.\n\n#### Examples\n  * Existing example.\n\n#### Mappings\n  * exactMatch: <https://external.example/Topic>\n', relations(('derivedFrom', 'Vocabulary', '#vocabulary'))),
    'Semantic.md': '# Elements\n\n' + element('Item Ontology', 'ontology', 'Item vocabulary.\n\n#### Ontology\n' + ONTOLOGY, metadata='  * ontology_base: https://example.org/items\n  * ontology_prefix: item\n') + element('Item Shape', 'semantic-contract', 'Item constraints.\n\n#### Shapes\n' + SHAPES, relations(('constrain', 'Parent', 'Requirements.md#parent'), ('use', 'Item Ontology', '#item-ontology'))) + element('Item Lookup', 'semantic-query', 'Item query.\n\n#### Query\n' + QUERY, relations(('use', 'Item Ontology', '#item-ontology'))),
    'Standalone.md': '# Element\n\n## Metadata\n  * type: specification\n\n## Relations\n  * define: [Parent](Requirements.md#parent)\n\n## Standalone Contract\n\nStandalone definition.\n\n#### Details\nStandalone clarification.\n',
    'evidence.txt': 'Unrelated stable evidence.\n',
}
FILES['Semantic.md'] += element('Construct Lookup', 'semantic-query', 'Materialized result.\n\n#### Query\n```sparql\nPREFIX item: <https://example.org/items#>\nCONSTRUCT { ?s item:name ?name } WHERE { ?s item:name ?name }\n```\n\n#### Produces\n  * property: item:name\n', relations(('use', 'Item Ontology', '#item-ontology')))
FILES['Semantic.md'] = FILES['Semantic.md'].replace('#### Ontology\n', '#### External Ontology\n  * prefix: ext\n  * namespace: https://example.org/external#\n  * resource: https://example.org/external\n  * source: external.ttl\n\n#### Ontology\n', 1)
FILES['external.ttl'] = '@prefix ext: <https://example.org/external#> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\n<https://example.org/external> a owl:Ontology .\next:ExternalItem a owl:Class .\n'



def case(name, *, readonly=False, crlf=False, restart=False, prepare=None):
    def register(run):
        CASES.append((name, run, readonly, crlf, restart, prepare))
        return run
    return register


def git(root, *args):
    return transport.git(root, *args)


def validate(root):
    result = subprocess.run([binary, '--workspace', str(root), 'validate', '--json'], capture_output=True, text=True, timeout=30)
    require(result.returncode == 0, 'Fixture/reload validation failed: ' + result.stdout + result.stderr)


def snapshot(root):
    return {str(p.relative_to(root)): p.read_bytes() for p in root.rglob('*') if p.is_file() and '.git' not in p.relative_to(root).parts}


def read(server, name):
    return transport.call(server, 'reqvire.read_element', name=name)


def state(server):
    status = transport.call(server, 'reqvire.workspace_status')
    context = status.get('context', {'branch': None, 'model_revision': status['model']['fingerprint'],
                                     'pending_changes': [], 'head': status['git']['head']})
    return {key: context[key] for key in ('branch', 'model_revision', 'pending_changes', 'head')}


def declaration(value):
    return {key: value.get(key) for key in ('identifier', 'name', 'element_type', 'metadata', 'relations',
        'contract_bindings', 'contract_references', 'concept_references', 'external_ontologies')}


def inventory(server):
    files = transport.call(server, 'reqvire.search')['files']
    return {item['name']: {**declaration(item), 'content': item['content']}
            for file in files.values() for item in file['elements']}


def request(server, *, element_name='Parent', target='Element Text', operation='append', dry_run=False, **fields):
    result = dict(element_name=element_name, target=target, operation=operation, dry_run=dry_run)
    if not dry_run and 'expected_model_revision' not in fields:
        result['expected_model_revision'] = state(server)['model_revision']
    return {**result, **fields}


def edit(server, root, expected, *, file='Requirements.md', **options):
    before, accepted = snapshot(root), state(server)
    old = read(server, options.get('element_name', 'Parent'))
    args = request(server, dry_run=True, **options)
    preview = transport.call(server, TOOL, **args)
    require(snapshot(root) == before and state(server) == accepted, 'Preview changed files or accepted state')
    args.update(dry_run=False, expected_model_revision=accepted['model_revision'])
    result = transport.call(server, TOOL, **args)
    after, current = snapshot(root), state(server)
    # Check the physical effect before trusting success-shaped protocol evidence.
    require(after != before, 'Tool reported success without changing source bytes')
    TOUCHED.add(old['name'])
    require({p for p in before.keys() | after.keys() if before.get(p) != after.get(p)} == {file}, 'Edit changed an unrelated file')
    fresh = read(server, old['name'])
    require(expected in fresh['content'] and expected in after[file].decode().replace('\r\n', '\n'), 'Persisted/accepted content differs from the intended edit')
    require(declaration(fresh) == declaration(old), 'Edit changed identity or protected declarations')
    for evidence in (preview, result):
        require(evidence.get('operation') == 'update' and evidence.get('target') == args['target']
                and evidence.get('edit_operation') == args['operation'], 'Missing selected-edit evidence')
        require(evidence.get('base_model_revision') == accepted['model_revision'], 'Missing base revision')
        require(evidence.get('diffs'), 'Effective edit returned an empty diff')
    require(preview['diffs'] == result['diffs'], 'Apply differs from its preview')
    require(current['model_revision'] != accepted['model_revision'] and file in current['pending_changes'], 'Effective edit did not update revision/pending paths')
    require(current['head'] == accepted['head'], 'Edit committed implicitly')
    return fresh['content']


def reject_many(server, root, rows, *, schema_error=False):
    # Every rejection must retain this same baseline; rereading it before each
    # request would add calls without strengthening the preservation check.
    before, accepted = snapshot(root), state(server)
    missing_errors = []
    for label, options in rows:
        options = dict(options)
        remove = options.pop('_remove', ())
        if not options.get('dry_run', False):
            options.setdefault('expected_model_revision', accepted['model_revision'])
        args = request(server, **options)
        for key in remove:
            args.pop(key, None)
        wire = server.raw('tools/call', {'name': TOOL, 'arguments': args})
        require(snapshot(root) == before and state(server) == accepted, f'{label}: rejection changed files or accepted state')
        if schema_error:
            correct_error = wire.get('error', {}).get('code') == -32602
        else:
            require('error' not in wire, f'{label}: semantic rejection used a protocol error')
            require('Unknown Reqvire tool' not in json.dumps(wire), f'{label}: request did not reach the update facade')
            correct_error = wire['result'].get('isError')
        if not correct_error:
            missing_errors.append(label)
    require(not missing_errors, 'Missing rejection: ' + ', '.join(missing_errors))


@case('mcp-contract')
def discovery(server, root):
    tool = next(t for t in server.rpc('tools/list', {})['tools'] if t['name'] == TOOL)
    props = tool['inputSchema']['properties']
    require(props['target']['type'] == 'string', 'Target is not a string')
    require(set(props['operation']['enum']) == {'append', 'replace', 'replace_text', 'create', 'delete'}, 'Wrong operations')
    require({'element_name', 'target', 'operation'} <= set(tool['inputSchema']['required']), 'Missing required selectors')
    require({'content', 'old_text', 'new_text', 'expected_model_revision', 'dry_run', 'branch'} <= props.keys() and 'worktree_id' not in props, 'Missing edit/context arguments')
    require(tool['annotations']['readOnlyHint'] is False, 'Update advertised as read-only')
    contract = transport.call(server, 'reqvire.tool_contract')
    require(next(t for t in contract['tools'] if t['name'] == TOOL) == tool, 'Contract/discovery schema mismatch')


@case('mutation-permission', readonly=True)
def permission(server, root):
    require(TOOL not in [t['name'] for t in server.rpc('tools/list', {})['tools']], 'Read-only startup exposes update')
    reject_many(server, root, [(str(preview), dict(content='Forbidden.', dry_run=preview))
                             for preview in (True, False)], schema_error=True)


@case('text-and-subsection-lifecycle', crlf=True)
def prose(server, root):
    content = edit(server, root, PARENT_TEXT + '\n\nRésumé — κείμενο.',
                   element_name='Requirements.md#parent', branch=transport.call(server, 'reqvire.workspace_status')['context']['branch'], content='Résumé — κείμενο.')
    original_details = DETAILS.replace(DETAILS_FENCE, DETAILS_FENCE.replace('\n', '\r\n'))
    require(original_details in content and 'Existing decision.' in content,
            'Append damaged neighbouring subsections or literal fenced bytes')
    text = (root / 'Requirements.md').read_text()
    require(text.index('Résumé') < text.index('#### Details'), 'Append crossed its target boundary')
    edit(server, root, 'naïve decisions', operation='replace_text', old_text='café decisions', new_text='naïve decisions')
    literal = 'New context.\n\n```markdown\n#### Metadata\n### Literal example\n```\n\n##### Nested note\nNested prose.'
    edit(server, root, literal, target='Details', operation='replace', content=literal)
    edit(server, root, literal + '\n\nAdditional context.', target='Details', content='Additional context.')
    edit(server, root, '#### Decision\nNew decision.', target='Decision', operation='create', content='New decision.')
    edit(server, root, 'New decision.\n\nReason recorded.', target='Decision', content='Reason recorded.')
    content = edit(server, root, literal, target='Decision', operation='delete')
    require('#### Decision' not in content and 'New decision.' not in content, 'Delete left heading or contents')
    content = edit(server, root, literal, operation='replace', content='')
    require('naïve decisions' not in content and 'Résumé' not in content, 'Empty replace did not clear opening text')
    edit(server, root, 'Restored opening text.', content='Restored opening text.')
    # The whitespace append is erased by canonical formatting, after the core
    # edit path; its final change evidence must also describe a no-op.
    for operation, content in [('append', ''), ('replace', 'Restored opening text.'), ('append', '\n')]:
        before, accepted = snapshot(root), state(server)
        result = transport.call(server, TOOL, **request(server, operation=operation, content=content))
        require(not result.get('diffs') and not result.get('changed_files'), 'No-op claims changes')
        require(snapshot(root) == before and state(server) == accepted, 'No-op changed state')


@case('single-element-document')
def single(server, root):
    edit(server, root, 'Standalone updated.', element_name='Standalone Contract', file='Standalone.md',
         operation='replace_text', old_text='Standalone definition.', new_text='Standalone updated.')
    edit(server, root, 'Standalone clarification.\n\nAdditional context.', element_name='Standalone Contract',
         file='Standalone.md', target='Details', content='Additional context.')
    source = (root / 'Standalone.md').read_text()
    require(source.startswith('# Element\n') and '## Standalone Contract\n' in source, 'Document form or named body changed')


@case('parents-and-consumers')
def dependents(server, root):
    for name, file in [('Product', 'Capabilities.md'), ('Shared Contract', 'Contracts.md'), ('Topic', 'Concepts.md')]:
        edit(server, root, 'Retained dependents.', element_name=name, file=file, content='Retained dependents.')
    edit(server, root, 'Existing scope.\n\nAdditional scope.', element_name='Topic', file='Concepts.md', target='Scope Note', content='Additional scope.')
    edit(server, root, 'Revised example.', element_name='Topic', file='Concepts.md', target='Examples',
         operation='replace_text', old_text='Existing example.', new_text='Revised example.')
    source = (root / 'Concepts.md').read_text()
    require('altLabel: A topic' in source and 'exactMatch: <https://external.example/Topic>' in source, 'Protected labels/mappings changed')


@case('semantic-payloads', restart=True)
def payloads(server, root):
    # One representative edit per semantic validator, without an operation cross-product.
    for name, target, fields, expected, tool in [
        ('Item Ontology', 'Ontology', dict(operation='replace', content=ONTOLOGY.replace('Original ontology text.', 'Revised ontology text.')), 'Revised ontology text.', 'reqvire.semantic.ontologies'),
        ('Item Shape', 'Shapes', dict(operation='replace_text', old_text='sh:minCount 0', new_text='sh:minCount 1'), 'sh:minCount 1', 'reqvire.semantic.shapes'),
        ('Item Lookup', 'Query', dict(operation='replace_text', old_text='SELECT ?item', new_text='SELECT DISTINCT ?item'), 'SELECT DISTINCT ?item', 'reqvire.semantic.queries'),
    ]:
        edit(server, root, expected, element_name=name, target=target, file='Semantic.md', **fields)
        options = {'include_content': True} if target == 'Query' else {}
        require(expected in json.dumps(transport.call(server, tool, **options)), 'Semantic read retained stale content')


@case('target-and-operation-rules')
def targets(server, root):
    # Each row exercises a distinct selection or operation-argument rule.
    reject_many(server, root, [
        ('missing element', dict(element_name='Missing', content='No implicit element.')),
        ('missing subsection', dict(target='Missing', content='No implicit subsection.', dry_run=True)),
        ('missing delete', dict(target='Missing', operation='delete')),
        ('existing create', dict(target='Details', operation='create', content='Duplicate.')),
        ('case-sensitive target', dict(target='details', content='Typo.')),
        ('no trimming', dict(target=' Details ', content='Typo.')),
        ('ambiguous match', dict(operation='replace_text', old_text='Repeat token.', new_text='New token.')),
        ('match outside target', dict(operation='replace_text', old_text='Original clarification.', new_text='Wrong region.')),
        ('empty match', dict(operation='replace_text', old_text='', new_text='New.')),
        ('missing content', dict()),
        ('missing replacement', dict(operation='replace_text', old_text='café')),
        ('mixed arguments', dict(content='Text.', old_text='Old', new_text='New')),
        ('unexpected delete content', dict(target='Addendum', operation='delete', content='Unexpected.')),
        ('unexpected edit content', dict(operation='replace_text', old_text='café', new_text='cafe', content='Unexpected.')),
        ('delete opening text', dict(operation='delete')),
        ('create opening text', dict(operation='create', content='Wrong target.')),
        ('concept Details', dict(element_name='Topic', target='Details', operation='create', content='Forbidden.')),
    ])


@case('protected-subsections')
def protected(server, root):
    # Valid same-content payloads prove target protection, independent of syntax errors.
    declarations = [
        ('Metadata', 'Parent', '  * type: requirement\n  * status: approved\n  * owner: Model Team'),
        ('Relations', 'Parent', '  * specify: [Product](Capabilities.md#product)'),
        ('Contract Bindings', 'Implementer', '  * [Shared Contract](Contracts.md#shared-contract)'),
        ('Contract References', 'Reader', '  * [Shared Contract](Contracts.md#shared-contract)'),
        ('Concept References', 'Reader', '  * [Topic](Concepts.md#topic)'),
        ('Labels', 'Topic', '  * altLabel: A topic'),
        ('Mappings', 'Topic', '  * exactMatch: <https://external.example/Topic>'),
        ('External Ontology', 'Item Ontology', '  * prefix: ext\n  * namespace: https://example.org/external#\n  * resource: https://example.org/external\n  * source: external.ttl'),
        ('Produces', 'Construct Lookup', '  * property: item:name'),
    ]
    rows = [(target, dict(element_name=name, target=target, operation='replace', content=content))
            for target, name, content in declarations]
    rows += [
        ('protected absent Properties', dict(target='Properties', operation='create', content='  * note: Stable.')),
        ('ontology append', dict(element_name='Item Ontology', target='Ontology', content='', dry_run=True)),
        ('required Shapes deletion', dict(element_name='Item Shape', target='Shapes', operation='delete')),
        ('Query creation', dict(element_name='Item Lookup', target='Query', operation='create', content=QUERY)),
    ]
    reject_many(server, root, rows)


@case('boundary-and-semantic-validation')
def validation(server, root):
    reject_many(server, root, [
        ('new subsection', dict(content='Text.\n\n#### Injected\nUnexpected section.', dry_run=True)),
        ('new element', dict(content='Text.\n\n### Injected Element\nUnexpected element.')),
        ('separator', dict(content='Text.\n\n---\n\n### Injected Element\nUnexpected element.')),
        ('unclosed fence', dict(content='```markdown\nFence hides unchanged neighbours.')),
        ('unclosed HTML', dict(target='Details', content='<details>\nHTML hides unchanged neighbours.')),
        ('prose model link', dict(content='See [Child](Children.md#child).', dry_run=True)),
        ('invalid Turtle', dict(element_name='Item Ontology', target='Ontology', operation='replace', content='```turtle\nnot valid RDF {{{\n```')),
        ('unreachable SHACL term', dict(element_name='Item Shape', target='Shapes', operation='replace', content=SHAPES.replace('item:name', 'item:UndeclaredProperty'))),
        ('invalid SPARQL', dict(element_name='Item Lookup', target='Query', operation='replace', content='```sparql\nSELECT broken syntax\n```', dry_run=True)),
    ])


def prepare_pending_work(server, root):
    initial = state(server)['model_revision']
    prior = element('Earlier Work', 'requirement', 'Earlier accepted work.', relations(('specify', 'Product', 'Capabilities.md#product')))
    transport.call(server, 'reqvire.add_element', file='Earlier.md', content=prior)
    return dict(initial_revision=initial, earlier=(root / 'Earlier.md').read_bytes())


@case('revision-and-pending-state', restart=True, prepare=prepare_pending_work)
def revisions(server, root, initial_revision, earlier):
    reject_many(server, root, [
        ('missing application revision', dict(content='Needs precondition.', _remove=('expected_model_revision',))),
        ('stale application revision', dict(content='Stale.', expected_model_revision=initial_revision)),
        ('stale preview revision', dict(content='Stale.', expected_model_revision=initial_revision, dry_run=True)),
    ])
    stale_retry = request(server, content='Accepted once.')
    edit(server, root, 'Accepted once.', content='Accepted once.')
    reject_many(server, root, [('retry cannot append twice', stale_retry)])
    require(read(server, 'Parent')['content'].count('Accepted once.') == 1, 'Retry duplicated text')
    edit(server, root, 'Valid after rejection.', content='Valid after rejection.')
    require((root / 'Earlier.md').read_bytes() == earlier and 'Earlier.md' in state(server)['pending_changes'], 'Earlier pending work was lost')


records = []
selected = [entry for entry in CASES if not arguments.only or entry[0] in arguments.only]
require(not (set(arguments.only) - {entry[0] for entry in CASES}), 'Unknown --only scenario')
for name, run, readonly, crlf, restart, prepare in selected:
    folder = output / name
    root = folder / 'workspace'
    root.mkdir(parents=True)
    TOUCHED = set()
    errors = []
    try:
        for file, content in FILES.items():
            (root / file).write_bytes((content.replace('\n', '\r\n') if crlf and file.endswith('.md') else content).encode())
        git(root, 'init', '-qb', 'main')
        git(root, 'config', 'user.name', 'Focused Edit Test')
        git(root, 'config', 'user.email', 'focused@example.invalid')
        git(root, 'add', '.')
        git(root, '-c', 'commit.gpgSign=false', 'commit', '-qm', 'Focused edit fixture')
        validate(root)
        head, index = git(root, 'rev-parse', 'HEAD'), git(root, 'write-tree')
        with transport.Stdio(binary, root, folder / 'wire', *([] if readonly else ['--enable-mutations'])) as server:
            prepared = prepare(server, root) if prepare else {}
            baseline = inventory(server)
            try:
                run(server, root, **prepared)
            except AssertionError as error:
                errors.append(str(error))
                (folder / 'failure.txt').write_text(traceback.format_exc())
            accepted = inventory(server)
            require(set(accepted) == set(baseline), 'Unexpected added/deleted model element')
            for key, value in baseline.items():
                require(declaration(accepted[key]) == declaration(value), f'{key}: identity/declarations changed')
                if key not in TOUCHED:
                    require(accepted[key]['content'] == value['content'], f'{key}: untargeted content changed')
            accepted_revision = state(server)['model_revision']
        require(git(root, 'rev-parse', 'HEAD') == head and git(root, 'write-tree') == index, 'Scenario staged/committed implicitly')
        validate(root)
        if restart and not errors:
            with transport.Stdio(binary, root, folder / 'restart') as fresh:
                require(inventory(fresh) == accepted and state(fresh)['model_revision'] == accepted_revision, 'Restart differs from accepted content/declarations/revision')
    except Exception as error:
        errors.append(str(error))
        (folder / 'failure.txt').write_text(traceback.format_exc())
    record = {'case': name, 'passed': not errors}
    if errors:
        record['error'] = '; '.join(errors)
        print(name + ': ' + record['error'], file=sys.stderr, flush=True)
    records.append(record)
    print(('FAIL ' if errors else 'PASS ') + name, flush=True)

(output / 'results.json').write_text(json.dumps(records, indent=2) + '\n')
print(f'Focused editing: {sum(r["passed"] for r in records)} passed, {sum(not r["passed"] for r in records)} failed', file=sys.stderr)
raise SystemExit(any(not r['passed'] for r in records))
