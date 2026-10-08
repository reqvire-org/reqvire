import hashlib,json,pathlib,subprocess,sys
bin,workspace=sys.argv[1:];root=pathlib.Path(workspace)
def run(*args,ok=True,input=None):
 p=subprocess.run([bin,*args],cwd=root,input=input,text=True,capture_output=True)
 assert (p.returncode==0)==ok,(args,p.returncode,p.stdout,p.stderr)
 return p.stdout

def query(text,produces='',use=True):
 return f'''### Portable Query

Portable query purpose.

#### Query
```sparql
{text}
```
{produces}
#### Metadata
  * type: semantic-query

#### Relations
'''+('  * use: [Item Ontology](#item-ontology)\n' if use else '')

fixture=root/'specifications/Queries.md';initial=fixture.read_bytes()
records=json.loads(run('semantic','query','list','--json'))
assert len(records)==1 and records[0]['query_form']=='SELECT'
iri=records[0]['iri'];assert iri=='urn:reqvire:semantic-query:item-lookup'
assert json.loads(run('semantic','query','list','--namespace-base','https://example.org/items#','--json'))==records
assert json.loads(run('semantic','query','list','--namespace-base','https://unrelated.example','--json'))==[]
collected=json.loads(run('collect','Item Lookup','--json'))
assert 'Item Ontology' in json.dumps(collected) and 'SELECT ?item' in json.dumps(collected)
assert 'Item Lookup' in run('collect','Item Ontology','--direction','DOWNSTREAM','--json')
assert records[0]['line_number']==next(i+1 for i,l in enumerate(initial.decode().splitlines()) if l=='```sparql')
print('PASS discovery-and-context-filter')
a=json.loads(run('semantic','query','export','--name','Item Lookup','--json'))
raw=run('semantic','query','export','--iri',iri)
assert a['content']==raw and a['sha256']==hashlib.sha256(raw.encode()).hexdigest()
run('semantic','query','export','--name','Item Lookup','--output','output/query.sparql')
artifact=root/'output/query.sparql';assert artifact.read_bytes()==raw.encode()
run('semantic','query','check','--name','Item Lookup','--artifact',str(artifact))
artifact.write_bytes(b'stale\n');before=artifact.read_bytes()
stale=json.loads(run('semantic','query','check','--name','Item Lookup','--artifact',str(artifact),'--json',ok=False))
assert stale['status']=='stale' and stale['actual_sha256']==hashlib.sha256(before).hexdigest() and artifact.read_bytes()==before
missing=json.loads(run('semantic','query','check','--name','Item Lookup','--artifact','output/missing','--json',ok=False))
assert missing['status']=='missing' and missing.get('actual_sha256') is None
# A directory is an I/O error, not a missing or stale artifact.
unreadable=run('semantic','query','check','--name','Item Lookup','--artifact','output','--json',ok=False)
assert '"status": "missing"' not in unreadable and '"status": "stale"' not in unreadable
run('semantic','query','export','--name','Item Lookup','--output','output',ok=False)
assert artifact.read_bytes()==before
for args in [('--name','Missing'),('--name','Item Lookup','--iri',iri),('--name','Item Lookup','--json','--output',str(artifact)),()]:
 run('semantic','query','export',*args,ok=False)
assert artifact.read_bytes()==before
print('PASS artifacts-selection-and-drift')
valid=[('ASK','ASK { <https://people.example/alice> a item:Item }',''),('DESCRIBE','DESCRIBE <https://people.example/alice>',''),('CONSTRUCT','CONSTRUCT { ?s item:name ?o } WHERE { ?s item:name ?o }','\n#### Produces\n  * property: item:name\n  * property: <https://example.org/items#name>\n'),('SELECT','SELECT ?s FROM <https://data.example/absent> WHERE { SERVICE SILENT <https://service.invalid/sparql> { ?s a item:Item } FILTER(<https://extension.example/function>(?s)) }','')]
for form,text,produces in valid:
 fixture.write_bytes(initial+b'\n'+query('PREFIX item: <https://example.org/items#>\n'+text,produces).encode())
 run('validate');record=json.loads(run('semantic','query','export','--name','Portable Query','--json'));assert record['query_form']==form
 if produces:assert record['materializes_properties']==['https://example.org/items#name']
print('PASS four-forms-and-portable-runtime-features')
invalid=[('ASK { ?s a item:Missing }',''),('ASK { ?s item:Item ?o }',''),('SELECT ?s WHERE { ?s (item:name/item:missing) ?o }',''),('CONSTRUCT { ?s item:missing ?o } WHERE { ?s item:name ?o }',''),('SELECT ?s WHERE { FILTER EXISTS { ?s a item:Missing } }',''),('SELECT ?s WHERE { VALUES ?s { "x"^^item:Missing } }',''),('ASK { ?s a item:Item }','\n#### Produces\n  * property: item:name\n'),('CONSTRUCT { ?s item:name ?o } WHERE { ?s item:name ?o }','\n#### Produces\n  * property: item:Item\n'),('CONSTRUCT {} WHERE {}','\n#### Produces\n  * family: item:Item\n'),('CONSTRUCT {} WHERE {}','\n#### Produces\n  * other: item:Item\n'),('CONSTRUCT {} WHERE {}','\n#### Produces\n'),('INSERT DATA { <urn:x> a item:Item }','')]
for text,produces in invalid:
 fixture.write_bytes(initial+b'\n'+query('PREFIX item: <https://example.org/items#>\n'+text,produces).encode());run('validate',ok=False)
fixture.write_bytes(initial)
for text in ['ASK { ?s a item:Item }','ASK { <relative> ?p ?o }']:
 fixture.write_bytes(initial+b'\n'+query(text).encode());run('validate',ok=False)
fixture.write_bytes(initial+b'\n'+query('ASK {}',use=False).encode());run('validate',ok=False)
fixture.write_bytes(initial)
print('PASS syntax-schema-role-and-produces-rejections')
# Invalid mutation must leave the entire authored file byte-for-byte unchanged.
expected=root/'output/expected-before.md';expected.write_bytes(initial)
run('add','specifications/Queries.md',input=query('ASK { ?s a <https://example.org/items#Missing> }'),ok=False)
subprocess.run(['diff','-u',str(expected),str(fixture)],check=True)
run('add','specifications/Queries.md',input=query('ASK {}'))
valid_before=fixture.read_bytes();expected.write_bytes(valid_before)
run('add','specifications/Queries.md','--override',input=query('ASK { ?s a <https://example.org/items#Missing> }'),ok=False)
subprocess.run(['diff','-u',str(expected),str(fixture)],check=True)
fixture.write_bytes(initial)
print('PASS atomic-add-and-override')
# Raw multi-line literal bytes and comments survive parse, content hashes and export.
document=b'PREFIX item: <https://example.org/items#>\nSELECT ?s WHERE { ?s item:name """line one\r\n#### Concept References\r\nliteral text\r\nline two""" }\n'
fixture.write_bytes(initial+b'\n'+query(document.decode()).encode())
a=json.loads(run('semantic','query','export','--name','Portable Query','--json'))
assert a['content'].encode()==document
run('format','--fix')
formatted=json.loads(run('semantic','query','export','--name','Portable Query','--json'))
assert formatted['content'].encode()==document and formatted['sha256']==a['sha256']
fixture.write_bytes(fixture.read_bytes().replace(b'Portable query purpose.',b'Updated purpose.'))
b=json.loads(run('semantic','query','export','--name','Portable Query','--json'));assert a['sha256']==b['sha256'] and a['purpose']!=b['purpose']
fixture.write_bytes(initial)
print('PASS-exact-bytes-and-metadata-independent-hash')
ttl=run('semantic','export','--layer','queries','--namespace-base','https://example.org/items');assert iri in ttl and 'queryText' in ttl
assert iri not in run('semantic','export','--layer','queries','--namespace-base','https://unrelated.example')
print('PASS-query-rdf-layer')
# Validate-all retains invalid candidates and the global model failure.
fixture.write_bytes(initial+b'\n'+query('ASK { ?s a <https://example.org/items#Missing> }').encode())
report=json.loads(run('semantic','query','validate','--json',ok=False))
assert report['valid'] is False and len(report['queries'])==2
assert any(q['diagnostics'] for q in report['queries']) and report['model_errors']
fixture.write_bytes(initial)
print('PASS-candidate-validation-diagnostics')
# An ontology rebase rewrites dependent query vocabulary and Produces, not string values.
construct='PREFIX item: <https://example.org/items#>\n# <https://example.org/items#name>\nCONSTRUCT { ?s item:name """<https://example.org/items#name>\n#### Produces\n  * property: item:name\n""" } WHERE { ?s item:name ?n }'
fixture.write_bytes(initial+b'\n'+query(construct,'\n#### Produces\n  * property: item:name\n').encode())
new_ontology=initial.decode().split('---')[0].split('# Elements\n\n')[1].replace('https://example.org/items','https://example.org/new-items').replace('ontology_prefix: item','ontology_prefix: newitem').replace('@prefix item:','@prefix newitem:').replace('item:Item','newitem:Item').replace('item:name','newitem:name')
run('add','specifications/Queries.md','--override',input=new_ontology)
q=json.loads(run('semantic','query','export','--name','Portable Query','--json'))
assert 'PREFIX item: <https://example.org/new-items#>' in q['content']
assert '# <https://example.org/items#name>' in q['content'] and '"""<https://example.org/items#name>\n#### Produces\n  * property: item:name\n"""' in q['content'],repr(q['content'])
assert q['materializes_properties']==['https://example.org/new-items#name']
expected_content=fixture.read_text()
assert '* property: newitem:name' in expected_content
# Re-parse the persisted file to validate all rewritten references.
run('validate')
fixture.write_bytes(initial)
print('PASS-ontology-rebase-query-and-produces')
# Different branch vocabulary stays unavailable; using child also makes ancestors available.
child='''\n### Child Vocabulary

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Item Ontology](#item-ontology)

#### Ontology
```turtle
@prefix item: <https://example.org/items#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
item:Child a owl:Class .
```
---
'''
fixture.write_bytes(initial+child.encode()+query('PREFIX item: <https://example.org/items#>\nASK { ?s a item:Child }').encode())
run('validate',ok=False)
fixture.write_bytes(initial+child.encode()+query('PREFIX item: <https://example.org/items#>\nASK { ?s a item:Item, item:Child }').replace('use: [Item Ontology](#item-ontology)','use: [Child Vocabulary](#child-vocabulary)').encode())
run('validate');fixture.write_bytes(initial)
print('PASS-ontology-context-ancestor-boundary')
# External RDF descriptions are available as vocabulary, not promoted to managed artifacts.
external=root/'specifications/external.ttl'
external.write_text('''@prefix ext: <https://imports.example/vocab#> .
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
<https://imports.example/vocab> a owl:Ontology .
ext:Record a owl:Class .
ext:Unused a owl:Class .
ext:value a owl:DatatypeProperty .
ext:output a owl:ObjectProperty .
<urn:imported-query> a reqvire:SemanticQuery ; reqvire:queryName "Item Lookup" ; reqvire:queryText "ASK {}" .
''')
ext_section='''\n#### External Ontology
  * prefix: ext
  * namespace: https://imports.example/vocab#
  * resource: https://imports.example/vocab
  * source: external.ttl
  * format: turtle
'''
with_external=initial.replace(b'---',ext_section.encode()+b'---',1)
fixture.write_bytes(with_external)
assert len(json.loads(run('semantic','query','list','--json')))==1
fixture.write_bytes(with_external+b'\n'+query('PREFIX ext: <https://imports.example/vocab#>\nASK { ?s a ext:Record ; ext:value ?v }').encode())
fixture.write_bytes(fixture.read_bytes()+b'\n---\n'+query('CONSTRUCT {} WHERE {}','\n#### Produces\n  * property: ext:output\n').replace('### Portable Query','### External Output').encode())
run('validate')
used=run('semantic','export','--layer','external-used')
assert 'Record' in used and 'value' in used and 'output' in used and 'Unused' not in used
fixture.write_bytes(initial+b'\n'+query('PREFIX ext: <https://imports.example/vocab#>\nASK { ?s a ext:Record }').encode())
run('validate',ok=False);fixture.write_bytes(initial);external.unlink()
print('PASS-imported-vocabulary-and-native-provenance')
# Incoming usedBy provides the same context as an outgoing use.
inverse=initial.replace(b'  * use: [Item Ontology](#item-ontology)\n',b'').replace(b'---',b'#### Relations\n  * usedBy: [Item Lookup](#item-lookup)\n---',1)
fixture.write_bytes(inverse);run('validate');assert json.loads(run('semantic','query','list','--json'))[0]['namespaces']==['https://example.org/items#']
fixture.write_bytes(initial)
print('PASS-inverse-usedBy-context')
# Runtime syntax is portable, and headings inside SPARQL literal strings are not Markdown.
for text in ['BASE <https://people.example/>\nASK { <alice> a <https://example.org/items#Item> }','SELECT ?s FROM NAMED <urn:dataset> WHERE { GRAPH <urn:graph> { SERVICE <https://unavailable.invalid/query> { ?s ?p ?o } } }','SELECT ("""first\n#### Ontology\n### Not an element\n---\nlast""" AS ?text) WHERE {}']:
 fixture.write_bytes(initial+b'\n'+query(text).encode());run('validate')
fixture.write_bytes(initial)
print('PASS-runtime-datasets-base-and-literal-headings')
# Complete grammar failures, with no artifacts written on rejection.
for content in [query('ASK {}').replace('```sparql','```turtle'),query(''),query('ASK {}').replace('#### Query','#### Other'),query('ASK {}')+'\n#### Query\n```sparql\nASK {}\n```\n',query('ASK {}').replace('  * type: semantic-query','  * type: semantic-query\n  * query_form: ASK'),query('ASK {}')+'\n#### Purpose\nHidden purpose\n']:
 fixture.write_bytes(initial+b'\n'+content.encode());run('validate',ok=False)
 run('semantic','query','export','--name','Portable Query','--output','output/forbidden.sparql',ok=False)
 assert not (root/'output/forbidden.sparql').exists()
fixture.write_bytes(initial+b'\n### Unrelated Requirement\n\nThe system SHALL perform a task.\n\n#### Metadata\n  * type: requirement\n\n#### Produces\n  * property: item:name\n')
run('validate',ok=False)
fixture.write_bytes(initial)
print('PASS-query-grammar-and-export-failure-atomicity')
# Produces accepts all supported property kinds and typed relation families.
families=initial.replace(b'item:name a owl:DatatypeProperty ; rdfs:domain item:Item .',b'item:name a owl:DatatypeProperty ; rdfs:domain item:Item .\nitem:annotation a owl:AnnotationProperty .\n<https://www.reqvire.org/ontology#RelationFamily> a owl:Class .\nitem:family a <https://www.reqvire.org/ontology#RelationFamily> .')
fixture.write_bytes(families+b'\n'+query('CONSTRUCT {} WHERE {}','\n#### Produces\n  * property: item:name\n  * property: item:annotation\n  * property: rdfs:label\n  * family: item:family\n').encode())
record=json.loads(run('semantic','query','export','--name','Portable Query','--json'))
assert len(record['materializes_properties'])==3 and record['materializes_families']==['https://example.org/items#family']
run('export','--output','output/site')
store=json.JSONDecoder().raw_decode((root/'output/site/assets/project-store.js').read_text().split('=',1)[1].lstrip())[0]
graph=store['ontology']['graph_data']
native=[n for n in graph['nodes'] if n['semantic_type']=='semantic-query']
assert len(native)==2
node=next(n for n in native if n['label']=='Portable Query')
assert node['source_kind']=='query' and node['query']['form']=='CONSTRUCT'
assert node['query']['text']==record['content']
assert node['sources'][0]['link']=='#/content/specifications/Queries.md#portable-query'
assert node['query']['ontologies'][0]['source']=='specifications/Queries.md#item-ontology'
assert {t['iri'] for t in node['query']['produces_properties']}==set(record['materializes_properties'])
assert {t['iri'] for t in node['query']['produces_families']}==set(record['materializes_families'])
for target in ['https://example.org/items#name','https://example.org/items#annotation','https://example.org/items#family']:
 assert any(n['id']==target for n in graph['nodes']),target
 assert any(e['source']==node['id'] and e['target']==target and e['label']=='declares output' for e in graph['edges']),target
print('PASS-explorer-query-graph-projection')
fixture.write_bytes(fixture.read_bytes().replace(b'  * property: item:annotation\n',b''))
assert json.loads(run('semantic','query','export','--name','Portable Query','--json'))['sha256']==record['sha256']
fixture.write_bytes(initial)
print('PASS-produces-property-kinds-and-families')
# Prefix resolution covers each used namespace and deduplicates inherited context.
child_base=child.replace('  * type: ontology','  * type: ontology\n  * ontology_base: https://example.org/sub\n  * ontology_prefix: sub').replace('@prefix item: <https://example.org/items#> .','@prefix sub: <https://example.org/sub#> .').replace('item:Child a owl:Class .','<https://example.org/sub> a owl:Ontology ; owl:imports <https://example.org/items> .\nsub:Child a owl:Class .')
fixture.write_bytes(initial+child_base.encode()+query('PREFIX item: <https://example.org/items#>\nPREFIX sub: <https://example.org/sub#>\nASK { ?s a item:Item, sub:Child }').replace('  * use: [Item Ontology](#item-ontology)','  * use: [Item Ontology](#item-ontology)\n  * use: [Child Vocabulary](#child-vocabulary)').encode())
run('validate')
for ns in ['https://example.org/items','https://example.org/sub#']:
 selected=json.loads(run('semantic','query','list','--name','Portable Query','--namespace-base',ns,'--json'));assert len(selected)==1
fixture.write_bytes(initial)
print('PASS-multiple-used-ontology-namespaces')
# Vocabulary changes propagate to query users, while query edits do not impact the ontology.
subprocess.run(['git','add','specifications/Queries.md'],cwd=root,check=True,capture_output=True)
subprocess.run(['git','-c','user.name=Query Test','-c','user.email=query@example.invalid','commit','--allow-empty','-m','Query baseline'],cwd=root,check=True,capture_output=True)
fixture.write_bytes(initial.replace(b'item:Item a owl:Class .',b'item:Item a owl:Class ; rdfs:comment "Changed vocabulary documentation" .'))
impact=json.loads(run('change-impact','--json'))
ontology_change=next(e for e in impact['changed'] if e['name']=='Item Ontology')
assert any(edge.get('usedBy',{}).get('name')=='Item Lookup' for edge in ontology_change['change_impact_tree'])
fixture.write_bytes(initial.replace(b'Lists items for downstream consumers.',b'Updated query purpose.'))
impact=json.loads(run('change-impact','--json'))
assert any(e['name']=='Item Lookup' for e in impact['changed'])
query_change=next(e for e in impact['changed'] if e['name']=='Item Lookup')
assert query_change['change_impact_tree']==[]
fixture.write_bytes(initial)
print('PASS-ontology-query-change-impact-direction')
# Supported moves and renames retain query bytes and update generated identity.
original=json.loads(run('semantic','query','export','--name','Item Lookup','--json'))
run('mv','Item Lookup','specifications/Moved.md')
moved=json.loads(run('semantic','query','export','--name','Item Lookup','--json'))
assert moved['iri']==original['iri'] and moved['sha256']==original['sha256']
assert moved['source_files']==['specifications/Moved.md']
run('rename','Item Lookup','Renamed Lookup')
renamed=json.loads(run('semantic','query','export','--name','Renamed Lookup','--json'))
assert renamed['iri']=='urn:reqvire:semantic-query:renamed-lookup' and renamed['sha256']==original['sha256']
run('semantic','query','export','--iri',original['iri'],ok=False)
# Removing used vocabulary or its schema terms must reject without persisting changes.
expected=root/'output/expected-ontology.md';expected.write_bytes(fixture.read_bytes())
run('rm','Item Ontology',ok=False)
subprocess.run(['diff','-u',str(expected),str(fixture)],check=True)
print('PASS-move-rename-and-used-ontology-removal')
