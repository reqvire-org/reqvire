//! Native, non-executing SPARQL artifacts. All delivery surfaces share this index.
use super::*;
use oxigraph::model::{Literal, NamedNode, Quad};
use spargebra::{algebra::*, term::*, Query, SparqlParser};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct QuerySource {
    pub query: Option<FencedBlock>,
    pub purpose: Option<String>,
    pub produces: Vec<(String, String, usize)>,
    pub diagnostics: Vec<String>,
}

impl QuerySource {
    pub fn parse(content: &str) -> Self {
        let mut source = Self {
            query: None,
            purpose: None,
            produces: Vec::new(),
            diagnostics: Vec::new(),
        };
        let mut section = "";
        let mut sections = BTreeSet::new();
        let mut fence = 0;
        let mut language = String::new();
        let mut text = String::new();
        let mut line_number = 0;
        let mut purpose = String::new();
        for (i, line) in content.split_inclusive('\n').enumerate() {
            let trimmed = line.trim();
            if fence > 0 {
                if trimmed.len() >= fence && trimmed.chars().all(|c| c == '`') {
                    if section == "Query" {
                        if source.query.is_some() {
                            source
                                .diagnostics
                                .push("Query requires exactly one fenced sparql block".into());
                        }
                        source.query = Some(FencedBlock {
                            language: language.clone(),
                            content: text.clone(),
                            line_number,
                        });
                    }
                    fence = 0;
                } else {
                    text.push_str(line);
                }
                continue;
            }
            if let Some(name) = trimmed.strip_prefix("#### ") {
                section = name;
                if name == "Purpose" {
                    source
                        .diagnostics
                        .push("Query purpose belongs in introductory prose".into());
                }
                if !sections.insert(name) {
                    source.diagnostics.push(format!("Duplicate {name} section"));
                }
                continue;
            }
            if trimmed.starts_with("```") {
                fence = trimmed.chars().take_while(|c| *c == '`').count();
                language = trimmed[fence..].trim().to_owned();
                text.clear();
                line_number = i + 1;
                continue;
            }
            match section {
                "" => purpose.push_str(line),
                "Produces" if !trimmed.is_empty() => {
                    if let Some((key, value)) =
                        trimmed.strip_prefix("* ").and_then(|v| v.split_once(':'))
                    {
                        if matches!(key.trim(), "property" | "family") {
                            source
                                .produces
                                .push((key.trim().into(), value.trim().into(), i + 1));
                        } else {
                            source
                                .diagnostics
                                .push(format!("Unknown Produces key '{key}' at line {}", i + 1));
                        }
                    } else {
                        source
                            .diagnostics
                            .push(format!("Invalid Produces entry at line {}", i + 1));
                    }
                }
                "Query" if !trimmed.is_empty() => source
                    .diagnostics
                    .push("Query only accepts one fenced sparql document".into()),
                _ => {}
            }
        }
        if fence > 0 {
            source.diagnostics.push("Unclosed Query fence".into());
        }
        if sections.contains("Produces") && source.produces.is_empty() {
            source
                .diagnostics
                .push("Produces requires at least one property or family".into());
        }
        if !purpose.trim().is_empty() {
            source.purpose = Some(purpose.trim().into());
        }
        match &source.query {
            Some(block) if block.language == "sparql" && !block.content.trim().is_empty() => {}
            _ => source
                .diagnostics
                .push("Query requires exactly one nonempty fenced sparql block".into()),
        }
        source
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct QueryRecord {
    pub iri: String,
    pub name: String,
    pub purpose: Option<String>,
    pub query_form: Option<String>,
    pub materializes_properties: BTreeSet<String>,
    pub materializes_families: BTreeSet<String>,
    pub ontology_context: BTreeSet<String>,
    pub namespaces: BTreeSet<String>,
    pub referenced_terms: BTreeSet<String>,
    pub source_elements: Vec<String>,
    pub source_files: Vec<String>,
    pub line_number: usize,
    pub diagnostics: Vec<SemanticDiagnostic>,
    #[serde(skip)]
    pub content: String,
}

/// Trim outer blank lines only. Internal bytes, including CRLF literals, are data.
pub fn render_document(text: &str) -> String {
    let lines: Vec<_> = text.split_inclusive('\n').collect();
    let start = lines
        .iter()
        .position(|line| !line.trim().is_empty())
        .unwrap_or(lines.len());
    let end = lines
        .iter()
        .rposition(|line| !line.trim().is_empty())
        .map_or(start, |i| i + 1);
    let mut result = lines[start..end].concat();
    if result.ends_with('\n') {
        result.pop();
        if result.ends_with('\r') {
            result.pop();
        }
    }
    result.push('\n');
    result
}

impl QueryRecord {
    pub fn artifact(&self) -> Result<Value, ReqvireError> {
        if !self.diagnostics.is_empty() {
            return Err(ReqvireError::ProcessError(format!(
                "Invalid query '{}': {:?}",
                self.name, self.diagnostics
            )));
        }
        let mut result = serde_json::to_value(self)
            .map_err(|e| ReqvireError::SerializationError(e.to_string()))?;
        result["content"] = json!(self.content);
        result["sha256"] = json!(crate::hashing::sha256_hex(self.content.as_bytes()));
        Ok(result)
    }
}

impl SemanticIndex {
    pub fn select_queries(
        &self,
        name: Option<&str>,
        iri: Option<&str>,
        namespace: Option<&str>,
    ) -> Result<Vec<&QueryRecord>, ReqvireError> {
        if name.is_some() && iri.is_some() {
            return Err(ReqvireError::ProcessError(
                "Choose either name or iri".into(),
            ));
        }
        let namespace =
            namespace.map(|n| export::export_filter_term_namespace(n, &self.ontology_documents));
        let matches: Vec<_> = self
            .queries
            .iter()
            .filter(|q| {
                name.is_none_or(|n| q.name == n)
                    && iri.is_none_or(|i| q.iri == i)
                    && namespace.as_ref().is_none_or(|n| q.namespaces.contains(n))
            })
            .collect();
        if (name.is_some() || iri.is_some()) && matches.len() != 1 {
            return Err(ReqvireError::ProcessError(format!(
                "Query selector matched {} native queries; expected exactly one",
                matches.len()
            )));
        }
        Ok(matches)
    }
    pub fn query_validation_report(
        &self,
        name: Option<&str>,
        iri: Option<&str>,
        model_errors: Vec<String>,
    ) -> Result<Value, ReqvireError> {
        let queries = self.select_queries(name, iri, None)?;
        let valid = model_errors.is_empty() && queries.iter().all(|q| q.diagnostics.is_empty());
        Ok(json!({"valid":valid,"queries":queries,"model_errors":model_errors}))
    }
    pub fn queries_turtle(&self) -> String {
        let mut out = String::new();
        for q in &self.queries {
            if !q.diagnostics.is_empty() {
                continue;
            }
            let subject = format!("<{}>", q.iri);
            out.push_str(&format!("{subject} a <{REQVIRE_NS}SemanticQuery> ;\n  <{REQVIRE_NS}queryName> {} ;\n  <{REQVIRE_NS}queryText> {} ;\n  <{REQVIRE_NS}queryForm> {} .\n", Literal::new_simple_literal(&q.name),Literal::new_simple_literal(&q.content),Literal::new_simple_literal(q.query_form.as_deref().unwrap_or_default())));
            for source in &q.source_elements {
                out.push_str(&format!(
                    "{subject} <{REQVIRE_NS}queryElement> {} .\n",
                    export::element_iri_from_identifier(source)
                ));
            }
            for ontology in &q.ontology_context {
                out.push_str(&format!(
                    "{subject} <{REQVIRE_NS}use> {} .\n",
                    export::element_iri_from_identifier(ontology)
                ));
            }
            for file in &q.source_files {
                out.push_str(&format!(
                    "{subject} <{REQVIRE_NS}filePath> {} .\n",
                    Literal::new_simple_literal(file)
                ));
            }
            out.push_str(&format!(
                "{subject} <{REQVIRE_NS}lineNumber> {} .\n",
                q.line_number
            ));
            if let Some(purpose) = &q.purpose {
                out.push_str(&format!(
                    "{subject} <{REQVIRE_NS}queryPurpose> {} .\n",
                    Literal::new_simple_literal(purpose)
                ));
            }
            for (key, values) in [
                ("queryMaterializesProperty", &q.materializes_properties),
                ("queryMaterializesFamily", &q.materializes_families),
            ] {
                for value in values {
                    out.push_str(&format!("{subject} <{REQVIRE_NS}{key}> <{value}> .\n"));
                }
            }
        }
        out
    }
}

pub(super) fn index_queries(registry: &GraphRegistry, index: &mut SemanticIndex) {
    for element in registry.get_all_elements() {
        if !element.element_type.is_semantic_query() {
            if crate::parser::has_subsection(&element.content, "Produces") {
                index.diagnostics.push(SemanticDiagnostic {
                    source: element.identifier.clone(),
                    file_path: element.file_path.clone(),
                    line_number: element.line_number,
                    message: "Produces belongs on semantic-query elements".into(),
                });
            }
            continue;
        }
        let missing_source = QuerySource {
            query: None,
            purpose: None,
            produces: Vec::new(),
            diagnostics: vec!["Missing parsed Query source".into()],
        };
        let source = element.semantic_query.as_ref().unwrap_or(&missing_source);
        let context: BTreeSet<_> = registry
            .semantic_contract_used_ontology_context(&element.identifier)
            .into_iter()
            .collect();
        let mut q = QueryRecord {
            iri: format!("urn:reqvire:semantic-query:{}", element.id),
            name: element.name.clone(),
            purpose: source.purpose.clone(),
            query_form: None,
            materializes_properties: BTreeSet::new(),
            materializes_families: BTreeSet::new(),
            ontology_context: context.clone(),
            namespaces: index
                .ontology_documents
                .iter()
                .filter(|d| d.element_identifiers.iter().any(|e| context.contains(e)))
                .map(|d| d.term_namespace.clone())
                .collect(),
            referenced_terms: BTreeSet::new(),
            source_elements: vec![element.identifier.clone()],
            source_files: vec![element.file_path.clone()],
            line_number: element.query_line_number.unwrap_or_else(|| {
                element.line_number + source.query.as_ref().map_or(0, |b| b.line_number)
            }),
            diagnostics: vec![],
            content: source
                .query
                .as_ref()
                .map_or(String::new(), |b| render_document(&b.content)),
        };
        let mut messages = source.diagnostics.clone();
        for key in element.metadata.keys() {
            if key != "type" {
                messages.push(format!("Semantic query metadata '{key}' is unsupported; identity and query form are generated"));
            }
        }
        if context.is_empty() {
            messages
                .push("Semantic query must use at least one ontology through use/usedBy".into());
        }
        let quads = index.reachable_ontology_context_quads(&context);
        let mut roles = Roles::new(&quads);
        match SparqlParser::new().parse_query(&q.content) {
            Ok(query) => {
                let (form, pattern) = match &query {
                    Query::Select { pattern, .. } => ("SELECT", pattern),
                    Query::Ask { pattern, .. } => ("ASK", pattern),
                    Query::Describe { pattern, .. } => ("DESCRIBE", pattern),
                    Query::Construct {
                        pattern, template, ..
                    } => {
                        for t in template {
                            roles.triple(t);
                        }
                        ("CONSTRUCT", pattern)
                    }
                };
                q.query_form = Some(form.into());
                roles.pattern(pattern);
                if form != "CONSTRUCT" && !source.produces.is_empty() {
                    messages.push("Produces is valid only for CONSTRUCT queries".into());
                }
            }
            Err(error) => messages.push(format!("Invalid SPARQL query: {error}")),
        }
        let mut prefixes: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for (prefix, namespace) in [
            ("rdf", o_kernel::vocab::reserved::RDF_NS),
            ("rdfs", o_kernel::vocab::reserved::RDFS_NS),
            ("owl", o_kernel::vocab::reserved::OWL_NS),
            ("xsd", o_kernel::vocab::reserved::XSD_NS),
            ("sh", o_kernel::vocab::reserved::SHACL_NS),
        ] {
            prefixes
                .entry(prefix.into())
                .or_default()
                .insert(namespace.into());
        }
        for block in &index.blocks {
            if context.contains(&block.source) {
                for (p, n) in parse_turtle_prefix_declarations(&block.content) {
                    prefixes.entry(p).or_default().insert(n);
                }
            }
        }
        for source in &index.external_sources {
            if source.builtin || context.contains(&source.owner_identifier) {
                prefixes
                    .entry(source.prefix.clone())
                    .or_default()
                    .insert(source.namespace.clone());
            }
        }
        for (key, value, line) in &source.produces {
            let iri = if value.starts_with('<') && value.ends_with('>') {
                Some(value[1..value.len() - 1].to_owned())
            } else {
                value.split_once(':').and_then(|(prefix, local)| {
                    prefixes
                        .get(prefix)
                        .filter(|v| v.len() == 1)
                        .map(|v| format!("{}{local}", v.first().unwrap()))
                })
            };
            match iri.filter(|i| NamedNode::new(i.as_str()).is_ok()) {
                Some(iri) => {
                    if key == "property" {
                        roles.check(&iri, "property");
                        q.materializes_properties.insert(iri);
                    } else if quads.iter().any(|t| {
                        subject_iri(&t.subject) == Some(&iri)
                            && t.predicate.as_str() == RDF_TYPE
                            && term_iri(&t.object) == Some(&format!("{REQVIRE_NS}RelationFamily"))
                    }) {
                        q.materializes_families.insert(iri);
                    } else {
                        messages.push(format!("Produces family '{value}' at line {line} is not a RelationFamily in the used context"));
                    }
                }
                None => messages.push(format!(
                    "Invalid or ambiguous Produces IRI '{value}' at line {line}"
                )),
            }
        }
        q.referenced_terms = roles.references;
        q.referenced_terms
            .extend(q.materializes_families.iter().cloned());
        messages.extend(roles.errors);
        q.diagnostics = messages
            .into_iter()
            .map(|message| SemanticDiagnostic {
                source: element.identifier.clone(),
                file_path: element.file_path.clone(),
                line_number: q.line_number,
                message,
            })
            .collect();
        index.diagnostics.extend(q.diagnostics.iter().cloned());
        index.queries.push(q);
    }
    index
        .queries
        .sort_by(|a, b| (&a.iri, &a.name).cmp(&(&b.iri, &b.name)));
}

struct Roles {
    types: BTreeMap<String, BTreeSet<String>>,
    errors: BTreeSet<String>,
    references: BTreeSet<String>,
}
impl Roles {
    fn new(quads: &[Quad]) -> Self {
        let mut result = Self {
            types: BTreeMap::new(),
            errors: BTreeSet::new(),
            references: BTreeSet::new(),
        };
        for q in quads {
            result.add(q);
        }
        // Use the pinned language vocabularies, with the same datatype policy as ontology validation.
        for ttl in [
            o_kernel::vocab::reserved::RDF_TURTLE,
            o_kernel::vocab::reserved::RDFS_TURTLE,
            o_kernel::vocab::reserved::OWL_TURTLE,
            o_kernel::vocab::reserved::SHACL_TURTLE,
        ] {
            for q in RdfParser::from_format(RdfFormat::Turtle)
                .for_reader(ttl.as_bytes())
                .flatten()
            {
                result.add(&q);
            }
        }
        result
    }
    fn add(&mut self, q: &Quad) {
        if q.predicate.as_str() == RDF_TYPE {
            if let (Some(s), Some(o)) = (subject_iri(&q.subject), term_iri(&q.object)) {
                self.types.entry(s.into()).or_default().insert(o.into());
            }
        }
    }
    fn check(&mut self, iri: &str, role: &str) {
        self.references.insert(iri.to_owned());
        let types = self.types.get(iri);
        let valid = types.is_some_and(|t| match role {
            "class" => t.contains(OWL_CLASS) || t.contains(RDFS_CLASS),
            "datatype" => t.contains(RDFS_DATATYPE),
            _ => [
                RDF_PROPERTY,
                OWL_OBJECT_PROPERTY,
                OWL_DATATYPE_PROPERTY,
                OWL_ANNOTATION_PROPERTY,
            ]
            .iter()
            .any(|k| t.contains(*k)),
        }) || (role == "datatype"
            && o_kernel::vocab::reserved::is_builtin_datatype_iri(iri));
        if !valid {
            self.errors.insert(format!("Query {role} <{iri}> is undeclared, has the wrong role, or is outside the used ontology context"));
        }
    }
    fn literal(&mut self, l: &Literal) {
        self.check(l.datatype().as_str(), "datatype");
    }
    fn term(&mut self, t: &TermPattern) {
        if let TermPattern::Literal(l) = t {
            self.literal(l);
        }
    }
    fn triple(&mut self, t: &TriplePattern) {
        self.term(&t.subject);
        self.term(&t.object);
        if let NamedNodePattern::NamedNode(p) = &t.predicate {
            self.check(p.as_str(), "property");
            if p.as_str() == RDF_TYPE {
                if let TermPattern::NamedNode(c) = &t.object {
                    self.check(c.as_str(), "class");
                }
            }
        }
    }
    fn path(&mut self, p: &PropertyPathExpression) {
        match p {
            PropertyPathExpression::NamedNode(n) => self.check(n.as_str(), "property"),
            PropertyPathExpression::Reverse(p)
            | PropertyPathExpression::ZeroOrMore(p)
            | PropertyPathExpression::OneOrMore(p)
            | PropertyPathExpression::ZeroOrOne(p) => self.path(p),
            PropertyPathExpression::Sequence(a, b) | PropertyPathExpression::Alternative(a, b) => {
                self.path(a);
                self.path(b);
            }
            PropertyPathExpression::NegatedPropertySet(ns) => {
                for n in ns {
                    self.check(n.as_str(), "property");
                }
            }
        }
    }
    fn expr(&mut self, e: &Expression) {
        use Expression::*;
        match e {
            Literal(l) => self.literal(l),
            Exists(p) => self.pattern(p),
            Or(a, b)
            | And(a, b)
            | Equal(a, b)
            | SameTerm(a, b)
            | Greater(a, b)
            | GreaterOrEqual(a, b)
            | Less(a, b)
            | LessOrEqual(a, b)
            | Add(a, b)
            | Subtract(a, b)
            | Multiply(a, b)
            | Divide(a, b) => {
                self.expr(a);
                self.expr(b);
            }
            In(a, bs) => {
                self.expr(a);
                for b in bs {
                    self.expr(b);
                }
            }
            UnaryPlus(a) | UnaryMinus(a) | Not(a) => self.expr(a),
            If(a, b, c) => {
                self.expr(a);
                self.expr(b);
                self.expr(c);
            }
            Coalesce(xs) | FunctionCall(_, xs) => {
                for x in xs {
                    self.expr(x);
                }
            }
            NamedNode(_) | Variable(_) | Bound(_) => {}
        }
    }
    fn pattern(&mut self, p: &GraphPattern) {
        use GraphPattern::*;
        match p {
            Bgp { patterns } => {
                for t in patterns {
                    self.triple(t);
                }
            }
            Path {
                subject,
                path,
                object,
            } => {
                self.term(subject);
                self.path(path);
                self.term(object);
                if matches!(path,PropertyPathExpression::NamedNode(n) if n.as_str()==RDF_TYPE) {
                    if let TermPattern::NamedNode(c) = object {
                        self.check(c.as_str(), "class");
                    }
                }
            }
            Join { left, right }
            | Lateral { left, right }
            | Union { left, right }
            | Minus { left, right } => {
                self.pattern(left);
                self.pattern(right);
            }
            LeftJoin {
                left,
                right,
                expression,
            } => {
                self.pattern(left);
                self.pattern(right);
                if let Some(e) = expression {
                    self.expr(e);
                }
            }
            Filter { expr, inner } => {
                self.pattern(inner);
                self.expr(expr);
            }
            Extend {
                expression, inner, ..
            } => {
                self.pattern(inner);
                self.expr(expression);
            }
            Graph { inner, .. }
            | Service { inner, .. }
            | Project { inner, .. }
            | Distinct { inner }
            | Reduced { inner }
            | Slice { inner, .. } => self.pattern(inner),
            OrderBy { inner, expression } => {
                self.pattern(inner);
                for e in expression {
                    match e {
                        OrderExpression::Asc(e) | OrderExpression::Desc(e) => self.expr(e),
                    }
                }
            }
            Group {
                inner, aggregates, ..
            } => {
                self.pattern(inner);
                for (_, a) in aggregates {
                    if let AggregateExpression::FunctionCall { expr, .. } = a {
                        self.expr(expr);
                    }
                }
            }
            Values { bindings, .. } => {
                for row in bindings {
                    for t in row.iter().flatten() {
                        if let GroundTerm::Literal(l) = t {
                            self.literal(l);
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn rewrite_iri_tokens(text: &str, mappings: &BTreeMap<String, String>) -> String {
    let mut output = String::new();
    let mut i = 0;
    while i < text.len() {
        let rest = &text[i..];
        let ch = rest.chars().next().unwrap();
        if ch == '#' {
            let length = rest.find('\n').unwrap_or(rest.len());
            output.push_str(&rest[..length]);
            i += length;
        } else if ch == '\'' || ch == '"' {
            let delimiter = if rest.starts_with(&ch.to_string().repeat(3)) {
                ch.to_string().repeat(3)
            } else {
                ch.to_string()
            };
            let start = i;
            i += delimiter.len();
            while i < text.len() {
                if text[i..].starts_with('\\') {
                    i += 1;
                    if i < text.len() {
                        i += text[i..].chars().next().unwrap().len_utf8();
                    }
                } else if text[i..].starts_with(&delimiter) {
                    i += delimiter.len();
                    break;
                } else {
                    i += text[i..].chars().next().unwrap().len_utf8();
                }
            }
            output.push_str(&text[start..i]);
        } else if ch == '<' {
            if let Some(end) = rest.find('>') {
                let iri = &rest[1..end];
                if !iri.chars().any(char::is_whitespace) {
                    output.push('<');
                    output.push_str(mappings.get(iri).map_or(iri, String::as_str));
                    output.push('>');
                    i += end + 1;
                    continue;
                }
            }
            output.push(ch);
            i += 1;
        } else {
            output.push(ch);
            i += ch.len_utf8();
        }
    }
    output
}
