use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RelationProjectionDirection {
    Forward,
    Inverse,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NormalizedRelationProjection {
    pub(super) forward_property: String,
    pub(super) inverse_property: String,
    pub(super) direction: RelationProjectionDirection,
}

pub(super) fn ontology_term_role(type_iri: &str) -> Option<OntologyTermRole> {
    match type_iri {
        OWL_CLASS | RDFS_CLASS => Some(OntologyTermRole::Class),
        RDFS_DATATYPE => Some(OntologyTermRole::Datatype),
        RDF_PROPERTY => Some(OntologyTermRole::Property),
        OWL_ANNOTATION_PROPERTY => Some(OntologyTermRole::AnnotationProperty),
        OWL_OBJECT_PROPERTY => Some(OntologyTermRole::ObjectProperty),
        OWL_DATATYPE_PROPERTY => Some(OntologyTermRole::DatatypeProperty),
        OWL_NAMED_INDIVIDUAL => Some(OntologyTermRole::NamedIndividual),
        _ => None,
    }
}

pub(super) fn build_ontology_projection(
    _registry: &GraphRegistry,
    blocks: &[SemanticBlock],
) -> OntologyProjectionGraph {
    let mut sourced_quads = Vec::new();
    let mut source_lookup = BTreeMap::new();

    for block in blocks {
        let source_key = ontology_projection_source_key(block);
        source_lookup.insert(source_key.clone(), projection_source(block));
        for quad in &block.quads {
            sourced_quads.push(constructs::SourcedQuad {
                source: source_key.clone(),
                quad: quad.clone(),
            });
        }
    }

    let projection = constructs::classify_ontology_constructs_with_sources_and_options(
        &sourced_quads,
        &constructs::OntologyConstructClassifierOptions {
            id_namespace: "urn:reqvire".to_string(),
        },
    );
    kernel_projection_to_reqvire_graph(&projection, &source_lookup)
}

pub(super) fn ontology_projection_source_key(block: &SemanticBlock) -> String {
    format!(
        "{}#{}:{}",
        block.source,
        block.kind.as_str(),
        block.line_number
    )
}

pub(super) fn kernel_projection_to_reqvire_graph(
    projection: &constructs::OntologyProjection,
    source_lookup: &BTreeMap<String, OntologyProjectionSource>,
) -> OntologyProjectionGraph {
    let mut constructs = Vec::new();
    for construct in &projection.constructs {
        constructs.push(kernel_projection_construct_to_reqvire(
            construct,
            source_lookup,
        ));
    }
    constructs.sort_by(|a, b| {
        a.family
            .cmp(&b.family)
            .then_with(|| a.kind.cmp(&b.kind))
            .then_with(|| a.id.cmp(&b.id))
    });

    let mut projections = Vec::new();
    for projection in &projection.projections {
        projections.push(OntologyConstructProjection {
            id: projection.id.clone(),
            family: to_reqvire_construct_family(projection.family),
            derivation_mode: to_reqvire_projection_derivation_mode(projection.derivation_mode),
            construct_ids: projection.construct_ids.clone(),
        });
    }
    projections.sort_by_key(|projection| projection.id.clone());

    OntologyProjectionGraph {
        id: "urn:reqvire:ontology-projection:graph:direct-authored".to_string(),
        derivation_mode: OntologyProjectionDerivationMode::DirectAuthored,
        projections,
        constructs,
        symbols: projection.symbols.iter().map(to_reqvire_symbol).collect(),
    }
}

pub(super) fn kernel_projection_construct_to_reqvire(
    construct: &constructs::OntologyConstruct,
    source_lookup: &BTreeMap<String, OntologyProjectionSource>,
) -> OntologyConstruct {
    OntologyConstruct {
        id: construct.id.clone(),
        family: to_reqvire_construct_family(construct.family),
        kind: to_reqvire_construct_kind(construct.kind),
        subject: to_reqvire_projection_term(&construct.subject),
        predicate: construct.predicate.as_ref().map(to_reqvire_projection_term),
        object: construct.object.as_ref().map(to_reqvire_projection_term),
        property: construct.property.as_ref().map(to_reqvire_projection_term),
        members: construct
            .members
            .iter()
            .map(|member| OntologyConstructMember {
                sequence_index: member.sequence_index,
                term: to_reqvire_projection_term(&member.term),
                source: projection_source_for_key(&member.source, source_lookup),
            })
            .collect(),
        property_characteristic: construct
            .property_characteristic
            .map(to_reqvire_property_characteristic),
        restriction_kind: construct.restriction_kind.map(to_reqvire_restriction_kind),
        class_expression_kind: construct
            .class_expression_kind
            .map(to_reqvire_class_expression_kind),
        shape_overlay_kind: construct
            .shape_overlay_kind
            .map(to_reqvire_shape_overlay_kind),
        symbol: construct.symbol.as_ref().map(to_reqvire_symbol),
        provenance: OntologyProjectionProvenance {
            derivation_mode: OntologyProjectionDerivationMode::DirectAuthored,
            source: projection_source_for_key(&construct.source, source_lookup),
            evidence: construct
                .evidence
                .iter()
                .map(|evidence| OntologyProjectionEvidence {
                    source: projection_source_for_key(&evidence.source, source_lookup),
                    subject: to_reqvire_projection_term(&evidence.subject),
                    predicate: to_reqvire_projection_term(&evidence.predicate),
                    object: to_reqvire_projection_term(&evidence.object),
                })
                .collect(),
        },
    }
}

pub(super) fn projection_source_for_key(
    key: &str,
    source_lookup: &BTreeMap<String, OntologyProjectionSource>,
) -> OntologyProjectionSource {
    if let Some(source) = source_lookup.get(key) {
        return source.clone();
    }

    let (source_element_identifier, block_kind, line_number) = parse_projection_source_key(key);
    OntologyProjectionSource {
        source_block: key.to_string(),
        source_element_identifier,
        source_name: "Unknown source".to_string(),
        file_path: String::new(),
        line_number,
        block_kind,
    }
}

pub(super) fn parse_projection_source_key(key: &str) -> (String, String, usize) {
    let (source_element_identifier, suffix) = key.rsplit_once('#').unwrap_or((key, ""));
    let (block_kind, line_number_text) = suffix.rsplit_once(':').unwrap_or((suffix, "0"));
    let line_number = line_number_text.parse::<usize>().unwrap_or(0);
    (
        source_element_identifier.to_string(),
        block_kind.to_string(),
        line_number,
    )
}

pub(super) fn to_reqvire_projection_derivation_mode(
    mode: &str,
) -> OntologyProjectionDerivationMode {
    match mode {
        "direct-authored" => OntologyProjectionDerivationMode::DirectAuthored,
        _ => OntologyProjectionDerivationMode::DirectAuthored,
    }
}

pub(super) fn to_reqvire_projection_term(
    term: &constructs::OntologyConstructTerm,
) -> OntologyProjectionTerm {
    OntologyProjectionTerm {
        kind: match term.kind {
            constructs::OntologyConstructTermKind::Iri => OntologyProjectionTermKind::Iri,
            constructs::OntologyConstructTermKind::BlankNode => {
                OntologyProjectionTermKind::BlankNode
            }
            constructs::OntologyConstructTermKind::Literal => OntologyProjectionTermKind::Literal,
        },
        value: term.value.clone(),
        label: term.label.clone(),
    }
}

pub(super) fn to_reqvire_construct_family(
    family: constructs::OntologyConstructFamily,
) -> OntologyConstructFamily {
    match family {
        constructs::OntologyConstructFamily::PropertyDomainRange => {
            OntologyConstructFamily::PropertyDomainRange
        }
        constructs::OntologyConstructFamily::SubclassMembership => {
            OntologyConstructFamily::SubclassMembership
        }
        constructs::OntologyConstructFamily::DisjointEquivalenceInverse => {
            OntologyConstructFamily::DisjointEquivalenceInverse
        }
        constructs::OntologyConstructFamily::PropertyChain => {
            OntologyConstructFamily::PropertyChain
        }
        constructs::OntologyConstructFamily::PropertyCharacteristic => {
            OntologyConstructFamily::PropertyCharacteristic
        }
        constructs::OntologyConstructFamily::Restriction => OntologyConstructFamily::Restriction,
        constructs::OntologyConstructFamily::ClassExpression => {
            OntologyConstructFamily::ClassExpression
        }
        constructs::OntologyConstructFamily::ShapeOverlay => OntologyConstructFamily::ShapeOverlay,
    }
}

pub(super) fn to_reqvire_construct_kind(
    kind: constructs::OntologyConstructKind,
) -> OntologyConstructKind {
    match kind {
        constructs::OntologyConstructKind::PropertyDomain => OntologyConstructKind::PropertyDomain,
        constructs::OntologyConstructKind::PropertyRange => OntologyConstructKind::PropertyRange,
        constructs::OntologyConstructKind::SubclassInclusion => {
            OntologyConstructKind::SubclassInclusion
        }
        constructs::OntologyConstructKind::Membership => OntologyConstructKind::Membership,
        constructs::OntologyConstructKind::Disjointness => OntologyConstructKind::Disjointness,
        constructs::OntologyConstructKind::EquivalenceGroup => {
            OntologyConstructKind::EquivalenceGroup
        }
        constructs::OntologyConstructKind::InverseProperty => {
            OntologyConstructKind::InverseProperty
        }
        constructs::OntologyConstructKind::PropertyChain => OntologyConstructKind::PropertyChain,
        constructs::OntologyConstructKind::PropertyCharacteristic => {
            OntologyConstructKind::PropertyCharacteristic
        }
        constructs::OntologyConstructKind::Restriction => OntologyConstructKind::Restriction,
        constructs::OntologyConstructKind::ClassExpression => {
            OntologyConstructKind::ClassExpression
        }
        constructs::OntologyConstructKind::ShapeOverlay => OntologyConstructKind::ShapeOverlay,
    }
}

pub(super) fn to_reqvire_property_characteristic(
    characteristic: constructs::OntologyPropertyCharacteristic,
) -> OntologyPropertyCharacteristic {
    match characteristic {
        constructs::OntologyPropertyCharacteristic::Functional => {
            OntologyPropertyCharacteristic::Functional
        }
        constructs::OntologyPropertyCharacteristic::InverseFunctional => {
            OntologyPropertyCharacteristic::InverseFunctional
        }
        constructs::OntologyPropertyCharacteristic::Symmetric => {
            OntologyPropertyCharacteristic::Symmetric
        }
        constructs::OntologyPropertyCharacteristic::Asymmetric => {
            OntologyPropertyCharacteristic::Asymmetric
        }
        constructs::OntologyPropertyCharacteristic::Reflexive => {
            OntologyPropertyCharacteristic::Reflexive
        }
        constructs::OntologyPropertyCharacteristic::Irreflexive => {
            OntologyPropertyCharacteristic::Irreflexive
        }
        constructs::OntologyPropertyCharacteristic::Transitive => {
            OntologyPropertyCharacteristic::Transitive
        }
    }
}

pub(super) fn to_reqvire_restriction_kind(
    restriction: constructs::OntologyRestrictionKind,
) -> OntologyRestrictionKind {
    match restriction {
        constructs::OntologyRestrictionKind::Universal => OntologyRestrictionKind::Universal,
        constructs::OntologyRestrictionKind::Existential => OntologyRestrictionKind::Existential,
        constructs::OntologyRestrictionKind::HasValue => OntologyRestrictionKind::HasValue,
        constructs::OntologyRestrictionKind::Cardinality => OntologyRestrictionKind::Cardinality,
        constructs::OntologyRestrictionKind::MinCardinality => {
            OntologyRestrictionKind::MinCardinality
        }
        constructs::OntologyRestrictionKind::MaxCardinality => {
            OntologyRestrictionKind::MaxCardinality
        }
        constructs::OntologyRestrictionKind::QualifiedCardinality => {
            OntologyRestrictionKind::QualifiedCardinality
        }
        constructs::OntologyRestrictionKind::MinQualifiedCardinality => {
            OntologyRestrictionKind::MinQualifiedCardinality
        }
        constructs::OntologyRestrictionKind::MaxQualifiedCardinality => {
            OntologyRestrictionKind::MaxQualifiedCardinality
        }
        constructs::OntologyRestrictionKind::OnClass => OntologyRestrictionKind::OnClass,
        constructs::OntologyRestrictionKind::OnDataRange => OntologyRestrictionKind::OnDataRange,
    }
}

pub(super) fn to_reqvire_class_expression_kind(
    class_expression: constructs::OntologyClassExpressionKind,
) -> OntologyClassExpressionKind {
    match class_expression {
        constructs::OntologyClassExpressionKind::Intersection => {
            OntologyClassExpressionKind::Intersection
        }
        constructs::OntologyClassExpressionKind::Union => OntologyClassExpressionKind::Union,
        constructs::OntologyClassExpressionKind::Complement => {
            OntologyClassExpressionKind::Complement
        }
    }
}

pub(super) fn to_reqvire_shape_overlay_kind(
    overlay: constructs::OntologyShapeOverlayKind,
) -> OntologyShapeOverlayKind {
    match overlay {
        constructs::OntologyShapeOverlayKind::NodeShape => OntologyShapeOverlayKind::NodeShape,
        constructs::OntologyShapeOverlayKind::PropertyShape => {
            OntologyShapeOverlayKind::PropertyShape
        }
    }
}

pub(super) fn to_reqvire_symbol(symbol: &constructs::OntologySymbol) -> OntologySymbol {
    OntologySymbol {
        concept_name: symbol.concept_name.clone(),
        raw_unicode_code_point: symbol.raw_unicode_code_point.clone(),
        rendered_unicode_character: symbol.rendered_unicode_character.clone(),
        tooltip: symbol.tooltip.clone(),
        accessible_label: symbol.accessible_label.clone(),
    }
}

pub(super) fn projection_source(block: &SemanticBlock) -> OntologyProjectionSource {
    OntologyProjectionSource {
        source_block: format!(
            "{}#{}:{}",
            block.source,
            block.kind.as_str(),
            block.line_number
        ),
        source_element_identifier: block.source.clone(),
        source_name: block.source_name.clone(),
        file_path: block.file_path.clone(),
        line_number: block.line_number,
        block_kind: block.kind.as_str().to_string(),
    }
}

pub(super) fn projection_term_key(term: &OntologyProjectionTerm) -> String {
    format!("{}:{}", term.kind.as_str(), term.value)
}

pub(super) use o_kernel::stable::stable_hash;
