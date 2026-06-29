use crate::rdf::named_or_blank_node_key;
use crate::stable::stable_hash;
use crate::vocab::*;
use oxigraph::model::{NamedOrBlankNode, Quad, Term};
use std::collections::{BTreeMap, BTreeSet};

use super::{owl_expression, owl_property, rdf_rdfs, restriction, shacl_overlay};

#[derive(Debug, Clone)]
pub struct SourcedQuad {
    pub source: String,
    pub quad: Quad,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OntologyConstructTermKind {
    Iri,
    BlankNode,
    Literal,
}

impl OntologyConstructTermKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Iri => "iri",
            Self::BlankNode => "blank-node",
            Self::Literal => "literal",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OntologyConstructTerm {
    pub kind: OntologyConstructTermKind,
    pub value: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OntologyConstructEvidence {
    pub source: String,
    pub subject: OntologyConstructTerm,
    pub predicate: OntologyConstructTerm,
    pub object: OntologyConstructTerm,
}

#[derive(Debug, Clone)]
pub struct OntologyConstructMember {
    pub sequence_index: usize,
    pub term: OntologyConstructTerm,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct OntologyConstructSource {
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OntologySymbol {
    pub concept_name: String,
    pub raw_unicode_code_point: String,
    pub rendered_unicode_character: String,
    pub tooltip: String,
    pub accessible_label: String,
}

#[derive(Debug, Clone)]
pub struct OntologyConstruct {
    pub id: String,
    pub family: OntologyConstructFamily,
    pub kind: OntologyConstructKind,
    pub subject: OntologyConstructTerm,
    pub predicate: Option<OntologyConstructTerm>,
    pub object: Option<OntologyConstructTerm>,
    pub property: Option<OntologyConstructTerm>,
    pub members: Vec<OntologyConstructMember>,
    pub property_characteristic: Option<OntologyPropertyCharacteristic>,
    pub restriction_kind: Option<OntologyRestrictionKind>,
    pub class_expression_kind: Option<OntologyClassExpressionKind>,
    pub shape_overlay_kind: Option<OntologyShapeOverlayKind>,
    pub symbol: Option<OntologySymbol>,
    pub source: String,
    pub evidence: Vec<OntologyConstructEvidence>,
}

#[derive(Debug, Clone)]
pub struct OntologyConstructProjection {
    pub id: String,
    pub family: OntologyConstructFamily,
    pub derivation_mode: &'static str,
    pub construct_ids: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct OntologyProjection {
    pub projections: Vec<OntologyConstructProjection>,
    pub constructs: Vec<OntologyConstruct>,
    pub symbols: Vec<OntologySymbol>,
}

#[derive(Debug, Clone)]
pub struct OntologyConstructClassifierOptions {
    pub id_namespace: String,
}

impl Default for OntologyConstructClassifierOptions {
    fn default() -> Self {
        Self {
            id_namespace: "urn:o-kernel".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OntologyConstructFamily {
    PropertyDomainRange,
    SubclassMembership,
    DisjointEquivalenceInverse,
    PropertyChain,
    PropertyCharacteristic,
    Restriction,
    ClassExpression,
    ShapeOverlay,
}

impl OntologyConstructFamily {
    fn as_str(self) -> &'static str {
        match self {
            Self::PropertyDomainRange => "property-domain-range",
            Self::SubclassMembership => "subclass-membership",
            Self::DisjointEquivalenceInverse => "disjoint-equivalence-inverse",
            Self::PropertyChain => "property-chain",
            Self::PropertyCharacteristic => "property-characteristic",
            Self::Restriction => "restriction",
            Self::ClassExpression => "class-expression",
            Self::ShapeOverlay => "shape-overlay",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OntologyConstructKind {
    PropertyDomain,
    PropertyRange,
    SubclassInclusion,
    Membership,
    Disjointness,
    EquivalenceGroup,
    InverseProperty,
    PropertyChain,
    PropertyCharacteristic,
    Restriction,
    ClassExpression,
    ShapeOverlay,
}

impl OntologyConstructKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::PropertyDomain => "property-domain",
            Self::PropertyRange => "property-range",
            Self::SubclassInclusion => "subclass-inclusion",
            Self::Membership => "membership",
            Self::Disjointness => "disjointness",
            Self::EquivalenceGroup => "equivalence-group",
            Self::InverseProperty => "inverse-property",
            Self::PropertyChain => "property-chain",
            Self::PropertyCharacteristic => "property-characteristic",
            Self::Restriction => "restriction",
            Self::ClassExpression => "class-expression",
            Self::ShapeOverlay => "shape-overlay",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OntologyPropertyCharacteristic {
    Functional,
    InverseFunctional,
    Symmetric,
    Asymmetric,
    Reflexive,
    Irreflexive,
    Transitive,
}

impl OntologyPropertyCharacteristic {
    fn as_str(self) -> &'static str {
        match self {
            Self::Functional => "functional",
            Self::InverseFunctional => "inverse-functional",
            Self::Symmetric => "symmetric",
            Self::Asymmetric => "asymmetric",
            Self::Reflexive => "reflexive",
            Self::Irreflexive => "irreflexive",
            Self::Transitive => "transitive",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OntologyRestrictionKind {
    Universal,
    Existential,
    HasValue,
    Cardinality,
    MinCardinality,
    MaxCardinality,
    QualifiedCardinality,
    MinQualifiedCardinality,
    MaxQualifiedCardinality,
    OnClass,
    OnDataRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OntologyClassExpressionKind {
    Intersection,
    Union,
    Complement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OntologyShapeOverlayKind {
    NodeShape,
    PropertyShape,
}

#[derive(Debug, Clone)]
struct EquivalenceEdge {
    subject: OntologyConstructTerm,
    object: OntologyConstructTerm,
    evidence: OntologyConstructEvidence,
}

pub(crate) struct ConstructMut<'a> {
    construct: Option<&'a mut OntologyConstruct>,
}

impl ConstructMut<'_> {
    pub(crate) fn with_property(self, property: Option<OntologyConstructTerm>) {
        if let Some(construct) = self.construct {
            construct.property = property;
        }
    }

    pub(crate) fn with_shape_overlay_kind(
        self,
        shape_overlay_kind: Option<OntologyShapeOverlayKind>,
    ) {
        if let Some(construct) = self.construct {
            construct.shape_overlay_kind = shape_overlay_kind;
        }
    }
}

pub fn classify_ontology_constructs(quads: &[Quad]) -> OntologyProjection {
    classify_ontology_constructs_with_options(quads, &OntologyConstructClassifierOptions::default())
}

pub fn classify_ontology_constructs_with_options(
    quads: &[Quad],
    options: &OntologyConstructClassifierOptions,
) -> OntologyProjection {
    let sourced: Vec<SourcedQuad> = quads
        .iter()
        .map(|quad| SourcedQuad {
            source: "source://unnamed".to_string(),
            quad: quad.clone(),
        })
        .collect();
    classify_ontology_constructs_with_sources_and_options(&sourced, options)
}

pub fn classify_ontology_constructs_with_sources(quads: &[SourcedQuad]) -> OntologyProjection {
    classify_ontology_constructs_with_sources_and_options(
        quads,
        &OntologyConstructClassifierOptions::default(),
    )
}

pub fn classify_ontology_constructs_with_sources_and_options(
    quads: &[SourcedQuad],
    options: &OntologyConstructClassifierOptions,
) -> OntologyProjection {
    let object_index = collect_projection_object_index(quads);
    let rdf_lists = collect_projection_rdf_lists(quads);
    let mut builder =
        OntologyConstructBuilder::new_with_options(rdf_lists, object_index, options.clone());

    for sourced_quad in quads {
        let handled = rdf_rdfs::classify(&mut builder, sourced_quad);
        if handled {
            continue;
        }
        let handled = owl_property::classify(&mut builder, sourced_quad);
        if handled {
            continue;
        }
        let handled = owl_expression::classify(&mut builder, sourced_quad);
        if handled {
            continue;
        }
        let handled = restriction::classify(&mut builder, sourced_quad);
        if handled {
            continue;
        }
        let handled = shacl_overlay::classify(&mut builder, sourced_quad);
        if handled {
            continue;
        }
    }

    builder.finish()
}

pub(crate) struct OntologyConstructBuilder {
    constructs: BTreeMap<String, OntologyConstruct>,
    equivalence_edges: Vec<EquivalenceEdge>,
    rdf_lists: BTreeMap<String, Vec<OntologyConstructMember>>,
    object_index: BTreeMap<(String, String), Vec<OntologyConstructTerm>>,
    options: OntologyConstructClassifierOptions,
}

impl OntologyConstructBuilder {
    #[cfg(test)]
    pub(crate) fn new(
        rdf_lists: BTreeMap<String, Vec<OntologyConstructMember>>,
        object_index: BTreeMap<(String, String), Vec<OntologyConstructTerm>>,
    ) -> Self {
        Self::new_with_options(
            rdf_lists,
            object_index,
            OntologyConstructClassifierOptions::default(),
        )
    }

    pub(crate) fn new_with_options(
        rdf_lists: BTreeMap<String, Vec<OntologyConstructMember>>,
        object_index: BTreeMap<(String, String), Vec<OntologyConstructTerm>>,
        options: OntologyConstructClassifierOptions,
    ) -> Self {
        Self {
            constructs: BTreeMap::new(),
            equivalence_edges: Vec::new(),
            rdf_lists,
            object_index,
            options,
        }
    }

    pub(crate) fn record_equivalence(&mut self, sourced_quad: &SourcedQuad) {
        let source = &sourced_quad.source;
        let quad = &sourced_quad.quad;
        let evidence = OntologyConstructEvidence {
            source: source.clone(),
            subject: construct_term_from_subject(&quad.subject),
            predicate: construct_term_from_predicate(quad.predicate.as_str()),
            object: construct_term_from_term(&quad.object),
        };

        self.equivalence_edges.push(EquivalenceEdge {
            subject: construct_term_from_subject(&quad.subject),
            object: construct_term_from_term(&quad.object),
            evidence,
        });
    }

    pub(crate) fn add_type_construct(&mut self, sourced_quad: &SourcedQuad) {
        let quad = &sourced_quad.quad;
        let source = &sourced_quad.source;
        let Some(object_iri) = term_iri(&quad.object) else {
            return;
        };

        if let Some(characteristic) = property_characteristic_for_type(object_iri) {
            self.add_direct_construct(
                source,
                quad,
                OntologyConstructFamily::PropertyCharacteristic,
                OntologyConstructKind::PropertyCharacteristic,
                Some(symbol(characteristic.as_str())),
                Some(characteristic),
                None,
                None,
                Vec::new(),
            );
            return;
        }

        if is_declaration_type(object_iri) || object_iri == OWL_RESTRICTION {
            if object_iri == SH_NODE_SHAPE || object_iri == SH_PROPERTY_SHAPE {
                self.add_shape_overlay_construct(source, quad);
            }
            return;
        }

        self.add_direct_construct(
            source,
            quad,
            OntologyConstructFamily::SubclassMembership,
            OntologyConstructKind::Membership,
            Some(symbol("member-of")),
            None,
            None,
            None,
            Vec::new(),
        );
    }

    pub(crate) fn add_restriction_construct(
        &mut self,
        source: &str,
        quad: &Quad,
        restriction_kind: OntologyRestrictionKind,
        symbol: Option<OntologySymbol>,
    ) {
        let property = self
            .objects_for_subject(&quad.subject, OWL_ON_PROPERTY)
            .first()
            .cloned();
        self.add_direct_construct(
            source,
            quad,
            OntologyConstructFamily::Restriction,
            OntologyConstructKind::Restriction,
            symbol,
            None,
            Some(restriction_kind),
            None,
            Vec::new(),
        )
        .with_property(property);
    }

    pub(crate) fn add_class_expression_construct(
        &mut self,
        source: &str,
        quad: &Quad,
        expression_kind: OntologyClassExpressionKind,
        symbol: Option<OntologySymbol>,
    ) {
        let members = self.members_for_list_term(&quad.object);
        self.add_direct_construct(
            source,
            quad,
            OntologyConstructFamily::ClassExpression,
            OntologyConstructKind::ClassExpression,
            symbol,
            None,
            None,
            Some(expression_kind),
            members,
        );
    }

    pub(crate) fn add_shape_overlay_construct(&mut self, source: &str, quad: &Quad) {
        let shape_overlay_kind = match quad.predicate.as_str() {
            RDF_TYPE if term_iri(&quad.object) == Some(SH_NODE_SHAPE) => {
                Some(OntologyShapeOverlayKind::NodeShape)
            }
            RDF_TYPE if term_iri(&quad.object) == Some(SH_PROPERTY_SHAPE) => {
                Some(OntologyShapeOverlayKind::PropertyShape)
            }
            _ => self.shape_overlay_kind_for_subject(&quad.subject),
        };

        self.add_direct_construct(
            source,
            quad,
            OntologyConstructFamily::ShapeOverlay,
            OntologyConstructKind::ShapeOverlay,
            None,
            None,
            None,
            None,
            Vec::new(),
        )
        .with_shape_overlay_kind(shape_overlay_kind);
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn add_direct_construct(
        &mut self,
        source: &str,
        quad: &Quad,
        family: OntologyConstructFamily,
        kind: OntologyConstructKind,
        symbol: Option<OntologySymbol>,
        property_characteristic: Option<OntologyPropertyCharacteristic>,
        restriction_kind: Option<OntologyRestrictionKind>,
        class_expression_kind: Option<OntologyClassExpressionKind>,
        members: Vec<OntologyConstructMember>,
    ) -> ConstructMut<'_> {
        let evidence = OntologyConstructEvidence {
            source: source.to_string(),
            subject: construct_term_from_subject(&quad.subject),
            predicate: construct_term_from_predicate(quad.predicate.as_str()),
            object: construct_term_from_term(&quad.object),
        };

        let subject = construct_term_from_subject(&quad.subject);
        let predicate = construct_term_from_predicate(quad.predicate.as_str());
        let object = construct_term_from_term(&quad.object);
        let object_id = construct_term_node_id(&object);

        let id = construct_id(
            &self.options.id_namespace,
            kind,
            &subject,
            Some(&predicate),
            object_id.as_ref(),
            &members,
        );

        let construct = OntologyConstruct {
            id: id.clone(),
            family,
            kind,
            subject,
            predicate: Some(predicate),
            object: Some(object),
            property: None,
            members,
            property_characteristic,
            restriction_kind,
            class_expression_kind,
            shape_overlay_kind: None,
            symbol,
            source: source.to_string(),
            evidence: vec![evidence],
        };

        self.constructs.entry(id.clone()).or_insert(construct);
        ConstructMut {
            construct: self.constructs.get_mut(&id),
        }
    }

    pub(crate) fn objects_for_subject(
        &self,
        subject: &NamedOrBlankNode,
        predicate: &str,
    ) -> Vec<OntologyConstructTerm> {
        self.object_index
            .get(&(named_or_blank_node_key(subject), predicate.to_string()))
            .cloned()
            .unwrap_or_default()
    }

    pub(crate) fn members_for_list_term(&self, term: &Term) -> Vec<OntologyConstructMember> {
        match term {
            Term::BlankNode(node) => self
                .rdf_lists
                .get(&node.to_string())
                .cloned()
                .unwrap_or_default(),
            Term::NamedNode(node) if node.as_str() == RDF_NIL => Vec::new(),
            _ => Vec::new(),
        }
    }

    pub(crate) fn shape_overlay_kind_for_subject(
        &self,
        subject: &NamedOrBlankNode,
    ) -> Option<OntologyShapeOverlayKind> {
        let subject_types = self.objects_for_subject(subject, RDF_TYPE);
        if subject_types.iter().any(|term| term.value == SH_NODE_SHAPE) {
            return Some(OntologyShapeOverlayKind::NodeShape);
        }
        if subject_types
            .iter()
            .any(|term| term.value == SH_PROPERTY_SHAPE)
        {
            return Some(OntologyShapeOverlayKind::PropertyShape);
        }

        let subject_term = construct_term_from_subject(subject);
        self.constructs
            .values()
            .filter(|construct| construct.subject == subject_term)
            .find_map(|construct| construct.shape_overlay_kind)
    }

    pub(crate) fn finish(mut self) -> OntologyProjection {
        self.materialize_equivalence_groups();

        let mut constructs: Vec<_> = self.constructs.into_values().collect();
        constructs.sort_by(|a, b| {
            a.family
                .cmp(&b.family)
                .then_with(|| a.kind.cmp(&b.kind))
                .then_with(|| a.id.cmp(&b.id))
        });

        let mut projections_by_family: BTreeMap<OntologyConstructFamily, Vec<String>> =
            BTreeMap::new();
        let mut symbols = BTreeSet::new();
        for construct in &constructs {
            projections_by_family
                .entry(construct.family)
                .or_default()
                .push(construct.id.clone());
            if let Some(symbol) = &construct.symbol {
                symbols.insert(symbol.clone());
            }
        }

        let projections = projections_by_family
            .into_iter()
            .map(|(family, mut construct_ids)| {
                construct_ids.sort();
                construct_ids.dedup();
                OntologyConstructProjection {
                    id: format!(
                        "{}:ontology-projection:{}:{}",
                        self.options.id_namespace,
                        family.as_str(),
                        stable_hash(&construct_ids.join("|"))
                    ),
                    family,
                    derivation_mode: "direct-authored",
                    construct_ids,
                }
            })
            .collect();

        OntologyProjection {
            projections,
            constructs,
            symbols: symbols.into_iter().collect(),
        }
    }

    fn materialize_equivalence_groups(&mut self) {
        if self.equivalence_edges.is_empty() {
            return;
        }

        let mut parent: BTreeMap<String, String> = BTreeMap::new();
        let mut terms: BTreeMap<String, OntologyConstructTerm> = BTreeMap::new();
        for edge in &self.equivalence_edges {
            let subject_key = construct_term_key(&edge.subject);
            let object_key = construct_term_key(&edge.object);
            terms.insert(subject_key.clone(), edge.subject.clone());
            terms.insert(object_key.clone(), edge.object.clone());
            uf_union_strings(&mut parent, &subject_key, &object_key);
        }

        let members: Vec<String> = parent.keys().cloned().collect();
        let mut groups: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for member in members {
            let root = uf_find_string(&mut parent, &member);
            groups.entry(root).or_default().insert(member);
        }

        for group_members in groups.values().filter(|members| members.len() > 1) {
            let Some(first_member_key) = group_members.iter().next() else {
                continue;
            };
            let Some(subject) = terms.get(first_member_key).cloned() else {
                continue;
            };

            let mut evidence: Vec<_> = self
                .equivalence_edges
                .iter()
                .filter(|edge| {
                    group_members.contains(&construct_term_key(&edge.subject))
                        && group_members.contains(&construct_term_key(&edge.object))
                })
                .map(|edge| edge.evidence.clone())
                .collect();
            evidence.sort_by(|a, b| a.source.cmp(&b.source));
            evidence.dedup_by_key(|item| item.source.clone());

            let Some(source) = evidence.first().map(|evidence| evidence.source.clone()) else {
                continue;
            };

            let mut members: Vec<_> = group_members
                .iter()
                .filter_map(|member_key| terms.get(member_key))
                .map(|term| OntologyConstructMember {
                    sequence_index: 0,
                    term: term.clone(),
                    source: source.clone(),
                })
                .collect();
            for (index, member) in members.iter_mut().enumerate() {
                member.sequence_index = index;
            }

            let id = construct_id(
                &self.options.id_namespace,
                OntologyConstructKind::EquivalenceGroup,
                &subject,
                None,
                None,
                &members,
            );
            let construct = OntologyConstruct {
                id: id.clone(),
                family: OntologyConstructFamily::DisjointEquivalenceInverse,
                kind: OntologyConstructKind::EquivalenceGroup,
                subject,
                predicate: None,
                object: None,
                property: None,
                members,
                property_characteristic: None,
                restriction_kind: None,
                class_expression_kind: None,
                shape_overlay_kind: None,
                symbol: Some(symbol("logical-equivalence")),
                source,
                evidence,
            };
            self.constructs.entry(id).or_insert(construct);
        }
    }
}

fn collect_projection_rdf_lists(
    inputs: &[SourcedQuad],
) -> BTreeMap<String, Vec<OntologyConstructMember>> {
    let mut first_values: BTreeMap<String, (OntologyConstructTerm, String)> = BTreeMap::new();
    let mut rest_targets: BTreeMap<String, String> = BTreeMap::new();
    let mut list_nodes = BTreeSet::new();

    for input in inputs {
        let quad = &input.quad;
        let subject = named_or_blank_node_key(&quad.subject);
        match quad.predicate.as_str() {
            RDF_FIRST => {
                list_nodes.insert(subject.clone());
                first_values.insert(
                    subject,
                    (construct_term_from_term(&quad.object), input.source.clone()),
                );
            }
            RDF_REST => {
                list_nodes.insert(subject.clone());
                match &quad.object {
                    Term::BlankNode(node) => {
                        let target = node.to_string();
                        list_nodes.insert(target.clone());
                        rest_targets.insert(subject, target);
                    }
                    Term::NamedNode(node) if node.as_str() == RDF_NIL => {
                        rest_targets.insert(subject, RDF_NIL.to_string());
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    let mut lists = BTreeMap::new();
    for head in list_nodes {
        let mut members = Vec::new();
        let mut current = head.as_str();
        let mut seen = BTreeSet::new();

        while seen.insert(current.to_string()) {
            if let Some((term, source)) = first_values.get(current) {
                members.push(OntologyConstructMember {
                    sequence_index: members.len(),
                    term: term.clone(),
                    source: source.clone(),
                });
            }

            let Some(next) = rest_targets.get(current) else {
                break;
            };
            if next == RDF_NIL {
                break;
            }
            current = next;
        }

        if !members.is_empty() {
            lists.insert(head, members);
        }
    }

    lists
}

fn collect_projection_object_index(
    inputs: &[SourcedQuad],
) -> BTreeMap<(String, String), Vec<OntologyConstructTerm>> {
    let mut object_index: BTreeMap<(String, String), Vec<OntologyConstructTerm>> = BTreeMap::new();

    for input in inputs {
        let quad = &input.quad;
        let key = (
            named_or_blank_node_key(&quad.subject),
            quad.predicate.as_str().to_string(),
        );
        let value = construct_term_from_term(&quad.object);
        object_index.entry(key).or_default().push(value);
    }

    for values in object_index.values_mut() {
        values.sort();
        values.dedup();
    }

    object_index
}

pub(crate) fn construct_term_from_subject(subject: &NamedOrBlankNode) -> OntologyConstructTerm {
    match subject {
        NamedOrBlankNode::NamedNode(node) => construct_term_from_predicate(node.as_str()),
        NamedOrBlankNode::BlankNode(node) => OntologyConstructTerm {
            kind: OntologyConstructTermKind::BlankNode,
            value: node.to_string(),
            label: "anonymous node".to_string(),
        },
    }
}

fn construct_term_from_predicate(predicate: &str) -> OntologyConstructTerm {
    OntologyConstructTerm {
        kind: OntologyConstructTermKind::Iri,
        value: predicate.to_string(),
        label: compact_iri_label(predicate),
    }
}

fn construct_term_from_term(term: &Term) -> OntologyConstructTerm {
    match term {
        Term::NamedNode(node) => construct_term_from_predicate(node.as_str()),
        Term::BlankNode(node) => OntologyConstructTerm {
            kind: OntologyConstructTermKind::BlankNode,
            value: node.to_string(),
            label: "anonymous node".to_string(),
        },
        Term::Literal(literal) => OntologyConstructTerm {
            kind: OntologyConstructTermKind::Literal,
            value: literal.value().to_string(),
            label: literal.value().to_string(),
        },
        #[allow(unreachable_patterns)]
        _ => OntologyConstructTerm {
            kind: OntologyConstructTermKind::Literal,
            value: String::new(),
            label: String::new(),
        },
    }
}

fn construct_term_node_id(term: &OntologyConstructTerm) -> Option<OntologyConstructTerm> {
    match term.kind {
        OntologyConstructTermKind::Iri | OntologyConstructTermKind::BlankNode => Some(term.clone()),
        OntologyConstructTermKind::Literal => None,
    }
}

fn construct_term_key(term: &OntologyConstructTerm) -> String {
    format!("{}:{}", term.kind.as_str(), term.value)
}

fn construct_id(
    id_namespace: &str,
    kind: OntologyConstructKind,
    subject: &OntologyConstructTerm,
    predicate: Option<&OntologyConstructTerm>,
    object: Option<&OntologyConstructTerm>,
    members: &[OntologyConstructMember],
) -> String {
    let mut canonical = format!(
        "{}|{}:{}",
        kind.as_str(),
        subject.kind.as_str(),
        subject.value
    );
    if let Some(predicate) = predicate {
        canonical.push_str(&format!(
            "|p:{}:{}",
            predicate.kind.as_str(),
            predicate.value
        ));
    }
    if let Some(object) = object {
        canonical.push_str(&format!("|o:{}:{}", object.kind.as_str(), object.value));
    }
    for member in members {
        canonical.push_str(&format!(
            "|m:{}:{}:{}",
            member.sequence_index,
            member.term.kind.as_str(),
            member.term.value
        ));
    }

    format!(
        "{}:ontology-construct:{}:{}",
        id_namespace,
        kind.as_str(),
        stable_hash(&canonical)
    )
}

fn compact_iri_label(value: &str) -> String {
    value
        .trim_matches(|c| c == '<' || c == '>')
        .rsplit(['/', '#'])
        .find(|part| !part.is_empty())
        .unwrap_or(value)
        .to_string()
}

pub(crate) fn symbol(concept_name: &str) -> OntologySymbol {
    let (raw_unicode_code_point, rendered_unicode_character, tooltip) = match concept_name {
        "disjointness" => ("U+27C2", "⟂", "Disjointness"),
        "intersection" => ("U+2229", "∩", "Intersection"),
        "union" => ("U+222A", "∪", "Union"),
        "logical-implication" => ("U+21D2", "⇒", "Implies"),
        "logical-equivalence" => ("U+21D4", "⇔", "Equivalent"),
        "universal-restriction" => ("U+2200", "∀", "Universal restriction"),
        "existential-restriction" => ("U+2203", "∃", "Existential restriction"),
        "member-of" => ("U+2208", "∈", "Member of"),
        "subset-or-equal" => ("U+2286", "⊆", "Subset or equal"),
        "symmetric" => ("U+2194", "↔", "Symmetric property"),
        "inverse-property" => ("U+27F2", "⟲", "Inverse property"),
        "reflexive" => ("U+25CB", "○", "Reflexive property"),
        "irreflexive" => ("U+2209", "∉", "Irreflexive property"),
        "transitive" => ("U+25B3", "△", "Transitive property"),
        "asymmetric" => ("U+21AE", "↮", "Asymmetric property"),
        "inverse-functional" => ("U+2190", "←", "Inverse functional property"),
        "functional" => ("U+2192", "→", "Functional property"),
        _ => ("U+0000", "", ""),
    };

    OntologySymbol {
        concept_name: concept_name.to_string(),
        raw_unicode_code_point: raw_unicode_code_point.to_string(),
        rendered_unicode_character: rendered_unicode_character.to_string(),
        tooltip: tooltip.to_string(),
        accessible_label: tooltip.to_string(),
    }
}

pub(crate) fn property_characteristic_for_type(
    type_iri: &str,
) -> Option<OntologyPropertyCharacteristic> {
    match type_iri {
        OWL_FUNCTIONAL_PROPERTY => Some(OntologyPropertyCharacteristic::Functional),
        OWL_INVERSE_FUNCTIONAL_PROPERTY => Some(OntologyPropertyCharacteristic::InverseFunctional),
        OWL_SYMMETRIC_PROPERTY => Some(OntologyPropertyCharacteristic::Symmetric),
        OWL_ASYMMETRIC_PROPERTY => Some(OntologyPropertyCharacteristic::Asymmetric),
        OWL_REFLEXIVE_PROPERTY => Some(OntologyPropertyCharacteristic::Reflexive),
        OWL_IRREFLEXIVE_PROPERTY => Some(OntologyPropertyCharacteristic::Irreflexive),
        OWL_TRANSITIVE_PROPERTY => Some(OntologyPropertyCharacteristic::Transitive),
        _ => None,
    }
}

fn is_declaration_type(type_iri: &str) -> bool {
    matches!(
        type_iri,
        OWL_CLASS
            | RDFS_CLASS
            | RDF_PROPERTY
            | OWL_ANNOTATION_PROPERTY
            | OWL_OBJECT_PROPERTY
            | OWL_DATATYPE_PROPERTY
            | OWL_NAMED_INDIVIDUAL
            | RDFS_DATATYPE
            | SH_NODE_SHAPE
            | SH_PROPERTY_SHAPE
    )
}

fn uf_find_string(parent: &mut BTreeMap<String, String>, id: &str) -> String {
    let mut root = id.to_string();
    while let Some(next) = parent.get(&root) {
        if next == &root {
            break;
        }
        root = next.clone();
    }

    let mut current = id.to_string();
    while current != root {
        let next = parent
            .get(&current)
            .cloned()
            .unwrap_or_else(|| root.clone());
        parent.insert(current.clone(), root.clone());
        current = next;
    }

    root
}

fn uf_union_strings(parent: &mut BTreeMap<String, String>, a: &str, b: &str) {
    parent.entry(a.to_string()).or_insert_with(|| a.to_string());
    parent.entry(b.to_string()).or_insert_with(|| b.to_string());
    let root_a = uf_find_string(parent, a);
    let root_b = uf_find_string(parent, b);
    if root_a != root_b {
        parent.insert(root_a, root_b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxigraph::model::{BlankNode, NamedNode, Quad, Term};

    fn bn(name: &str) -> BlankNode {
        BlankNode::new(name).expect("blank node id should parse")
    }

    fn iri(value: &str) -> NamedNode {
        NamedNode::new(value).expect("iri should parse")
    }

    #[test]
    fn property_domain_classifier_preserves_term_values() {
        let source = "test";
        let subject = iri("https://example.org/p");
        let object = iri("https://example.org/o");
        let quad = SourcedQuad {
            source: source.to_string(),
            quad: Quad {
                subject: NamedOrBlankNode::NamedNode(subject.clone()),
                predicate: NamedNode::new(RDFS_DOMAIN).expect(""),
                object: Term::NamedNode(object.clone()),
                graph_name: oxigraph::model::GraphName::DefaultGraph,
            },
        };

        let projection = classify_ontology_constructs_with_sources(&[quad]);
        assert_eq!(projection.constructs.len(), 1);
        let construct = &projection.constructs[0];
        assert_eq!(construct.kind, OntologyConstructKind::PropertyDomain);
        assert_eq!(construct.source, source);
        assert_eq!(construct.subject.value, subject.as_str());
        assert_eq!(
            construct.object.as_ref().map(|term| term.value.as_str()),
            Some(object.as_str())
        );
        assert!(construct.id.starts_with("urn:o-kernel:ontology-construct:"));
        assert!(projection.projections[0]
            .id
            .starts_with("urn:o-kernel:ontology-projection:"));
    }

    #[test]
    fn classifier_options_control_id_namespace() {
        let quad = SourcedQuad {
            source: "test".to_string(),
            quad: Quad {
                subject: NamedOrBlankNode::NamedNode(iri("https://example.org/p")),
                predicate: NamedNode::new(RDFS_DOMAIN).expect("predicate IRI"),
                object: Term::NamedNode(iri("https://example.org/o")),
                graph_name: oxigraph::model::GraphName::DefaultGraph,
            },
        };

        let projection = classify_ontology_constructs_with_sources_and_options(
            &[quad],
            &OntologyConstructClassifierOptions {
                id_namespace: "urn:example:kernel".to_string(),
            },
        );

        assert!(projection.constructs[0]
            .id
            .starts_with("urn:example:kernel:ontology-construct:"));
        assert!(projection.projections[0]
            .id
            .starts_with("urn:example:kernel:ontology-projection:"));
    }

    #[test]
    fn property_chain_preserves_rdf_list_order() {
        let chain = bn("chain");
        let first = bn("first");
        let second = bn("second");

        let a = SourcedQuad {
            source: "test".to_string(),
            quad: Quad {
                subject: chain.clone().into(),
                predicate: NamedNode::new(RDF_FIRST).expect(""),
                object: Term::NamedNode(iri("https://example.org/a")),
                graph_name: oxigraph::model::GraphName::DefaultGraph,
            },
        };
        let b = SourcedQuad {
            source: "test".to_string(),
            quad: Quad {
                subject: chain.clone().into(),
                predicate: NamedNode::new(RDF_REST).expect(""),
                object: Term::BlankNode(first.clone()),
                graph_name: oxigraph::model::GraphName::DefaultGraph,
            },
        };
        let c = SourcedQuad {
            source: "test".to_string(),
            quad: Quad {
                subject: first.clone().into(),
                predicate: NamedNode::new(RDF_FIRST).expect(""),
                object: Term::NamedNode(iri("https://example.org/b")),
                graph_name: oxigraph::model::GraphName::DefaultGraph,
            },
        };
        let d = SourcedQuad {
            source: "test".to_string(),
            quad: Quad {
                subject: first.clone().into(),
                predicate: NamedNode::new(RDF_REST).expect(""),
                object: Term::BlankNode(second.clone()),
                graph_name: oxigraph::model::GraphName::DefaultGraph,
            },
        };
        let e = SourcedQuad {
            source: "test".to_string(),
            quad: Quad {
                subject: second.clone().into(),
                predicate: NamedNode::new(RDF_FIRST).expect(""),
                object: Term::NamedNode(iri("https://example.org/c")),
                graph_name: oxigraph::model::GraphName::DefaultGraph,
            },
        };
        let f = SourcedQuad {
            source: "test".to_string(),
            quad: Quad {
                subject: second.clone().into(),
                predicate: NamedNode::new(RDF_REST).expect(""),
                object: Term::NamedNode(iri(RDF_NIL)),
                graph_name: oxigraph::model::GraphName::DefaultGraph,
            },
        };
        let g = SourcedQuad {
            source: "test".to_string(),
            quad: Quad {
                subject: chain.clone().into(),
                predicate: NamedNode::new(OWL_PROPERTY_CHAIN_AXIOM).expect(""),
                object: Term::BlankNode(chain.clone()),
                graph_name: oxigraph::model::GraphName::DefaultGraph,
            },
        };

        let projection = classify_ontology_constructs_with_sources(&[a, b, c, d, e, f, g]);
        assert_eq!(projection.constructs.len(), 1);
        let members = &projection.constructs[0].members;
        assert_eq!(members.len(), 3);
        assert_eq!(members[0].term.value, "https://example.org/a");
        assert_eq!(members[1].term.value, "https://example.org/b");
        assert_eq!(members[2].term.value, "https://example.org/c");
    }
}
