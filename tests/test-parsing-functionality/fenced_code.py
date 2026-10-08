"""CLI regression evidence for Fenced Code Block Parsing Verification."""
import json
from pathlib import Path
import subprocess
import sys

binary, workspace = sys.argv[1:]
root = Path(workspace)
fixture = root / 'specifications/FencedCode.md'
relative_fixture = fixture.relative_to(root).as_posix()


def run(*arguments, content=None, label='command'):
    result = subprocess.run(
        [binary, *arguments], cwd=root, input=content,
        text=True, capture_output=True,
    )
    if result.returncode:
        raise SystemExit(
            f'FAIL {label}: {" ".join(arguments)} exited {result.returncode}\n'
            f'{result.stdout}{result.stderr}'
        )
    return result.stdout


def elements(path=relative_fixture):
    report = json.loads(run('search', '--json'))
    return {element['name']: element for element in report['files'][path]['elements']}


payload = '''      ### Phantom Element
      #### Metadata
        * type: capability
      #### Relations
        * derivedFrom: [Missing](#missing)
      #### Concept References
        * [Missing](#missing)
      ---
'''


def block(opening, closing='     ```', extra=''):
    return f'{opening}\n{payload}{extra}{closing}\n'


def document(code):
    return f'''# Elements

### Fence Parent

#### Metadata
  * type: requirement

#### Relations
  * derive: [Fence Following](#fence-following)
  * specify: [Fence Example](#fence-example)
---

### Fence Example

Literal code example.

{code}
#### Metadata
  * type: capability
---

### Fence Following

Following element remains addressable.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Fence Parent](#fence-parent)
---
'''


cases = [
    ('bare', block('```markdown', '```')),
    ('unordered-star', block('  * ```')),
    ('unordered-dash', block('  - ```markdown')),
    ('unordered-plus', block('  +   ```markdown')),
    ('ordered-dot', block('  1. ```markdown')),
    ('ordered-parenthesis', block('  2) ```')),
    ('nested-list', '* Parent list item\n' + block('    *   ```markdown', '        ```')),
    ('longer-fence', block('  * ````markdown', '      ````', '      ```\n      ### Still Literal\n')),
    ('longer-closer', block('  * ```markdown', '      ````')),
    ('trailing-text', block('  * ```markdown', '      ```', '      ``` not a closing fence\n      ### Still Literal\n')),
    ('red-three-fences', block('  * ```') + block('  *   ```', '        ```') + block('  *   ```', '        ```')),
]

for label, code in cases:
    source = document(code)
    fixture.write_text(source)
    run('validate', label=label)
    found = elements()
    assert set(found) == {'Fence Parent', 'Fence Example', 'Fence Following'}, (label, found)
    assert found['Fence Example']['type'] == 'capability', (label, found['Fence Example'])
    assert code.strip('\n') in found['Fence Example']['content'], (label, found['Fence Example'])
    assert all(
        not relation['target']['target'].endswith('#missing')
        for relation in found['Fence Example']['relations']
    ), (label, found['Fence Example'])
    assert any(
        relation['relation_type'] == 'derivedFrom'
        and relation['target']['target'].endswith('#fence-parent')
        for relation in found['Fence Following']['relations']
    ), (label, found['Fence Following'])
    expected_source = root / 'output/fenced-source-before.txt'
    expected_source.write_text(source)
    subprocess.run(['diff', '-u', str(expected_source), str(fixture)], check=True)
    print(f'PASS {label}', flush=True)

# Exercise the format command directly for each fence example, with exact code diffs.
for label, code in cases:
    fixture.write_text(document(code))
    run('format', '--fix', label=f'format-{label}')
    run('validate', label=f'formatted-{label}')
    found = elements()
    assert set(found) == {'Fence Parent', 'Fence Example', 'Fence Following'}, (label, found)
    assert found['Fence Example']['type'] == 'capability', (label, found['Fence Example'])
    assert any(
        relation['relation_type'] == 'derivedFrom'
        and relation['target']['target'].endswith('#fence-parent')
        for relation in found['Fence Following']['relations']
    ), (label, found['Fence Following'])
    body = found['Fence Example']['content']
    start = body.index(code.splitlines()[0])
    expected = code.rstrip('\n')
    expected_code = root / 'output/fenced-format-before.txt'
    actual_code = root / 'output/fenced-format-after.txt'
    expected_code.write_text(expected + '\n')
    actual_code.write_text(body[start:start + len(expected)] + '\n')
    subprocess.run(['diff', '-u', str(expected_code), str(actual_code)], check=True)
    print(f'PASS format-{label}', flush=True)

# Single-element mutation parsing must use the same fence boundaries.
fixture.write_text(document(block('```markdown', '```')))
added = f'''### Fence Added

Initial example.

{block('  *   ```markdown', '        ```')}
#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [Fence Example](#fence-example)
'''
for label, content, options in [
    ('add', added, ()),
    ('override', added.replace('Initial example.', 'Updated example.'), ('--override',)),
]:
    run('add', relative_fixture, *options, content=content, label=label)
    found = elements()
    assert len(found) == 4 and found['Fence Added']['type'] == 'capability', (label, found)
    assert block('  *   ```markdown', '        ```').strip('\n') in found['Fence Added']['content']
    assert any(
        relation['relation_type'] == 'derivedFrom'
        and relation['target']['target'].endswith('#fence-example')
        for relation in found['Fence Added']['relations']
    ), (label, found['Fence Added'])
    if label == 'override':
        assert 'Updated example.' in found['Fence Added']['content']
        assert 'Initial example.' not in found['Fence Added']['content']
    print(f'PASS {label}', flush=True)

# Subsection helpers and the query parser must also ignore list-fenced examples.
fixture.write_text(document(block('```markdown', '```')))
semantic_fixture = root / 'specifications/FencedSemantic.md'
example = block('  * ```markdown', '      ```')
query_text = 'SELECT ("""first\n#### Metadata\n### Literal Query Heading\n---\nlast""" AS ?text) WHERE {}\n'
semantic_source = f'''# Elements

### Fence Vocabulary

#### Metadata
  * type: ontology
  * ontology_base: https://example.org/fence
  * ontology_prefix: fence

{example}
#### Ontology
```turtle
@prefix fence: <https://example.org/fence#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
<https://example.org/fence> a owl:Ontology .
fence:Item a owl:Class .
```
---

### Fence Query

Query purpose.

{example}
#### Query
```sparql
{query_text}```

#### Metadata
  * type: semantic-query

#### Relations
  * use: [Fence Vocabulary](#fence-vocabulary)
---
'''
semantic_fixture.write_text(semantic_source)
run('validate', label='semantic-subsections')
semantic_elements = elements('specifications/FencedSemantic.md')
assert set(semantic_elements) == {'Fence Vocabulary', 'Fence Query'}, semantic_elements
ontology = semantic_elements['Fence Vocabulary']['ontology']['ontology']
assert ontology['language'] == 'turtle' and 'fence:Item a owl:Class' in ontology['content'], ontology
record = json.loads(run('semantic', 'query', 'export', '--name', 'Fence Query', '--json'))
assert record['content'] == query_text, record
assert record['line_number'] == next(
    i + 1 for i, line in enumerate(semantic_source.splitlines()) if line == '```sparql'
), record
print('PASS semantic-subsections-and-source-location', flush=True)

# Existing HTML details behavior must not leak fence state into later elements.
fixture.write_text(document('<details>\n  * ```\n### Hidden HTML Heading\n</details>\n'))
run('validate', label='html-details')
assert set(elements()) == {'Fence Parent', 'Fence Example', 'Fence Following'}
print('PASS html-details', flush=True)
