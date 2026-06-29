use oxigraph::io::{RdfFormat, RdfParser};
use oxigraph::model::Quad;

pub(crate) fn parse_test_quads(turtle: &str) -> Vec<Quad> {
    RdfParser::from_format(RdfFormat::Turtle)
        .for_reader(turtle.as_bytes())
        .map(|quad| quad.expect("test Turtle should parse"))
        .collect()
}
