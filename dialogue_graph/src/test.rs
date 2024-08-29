use std::fs::read_to_string;

use petgraph::dot::Dot;
use pretty_assertions::assert_eq;

use crate::construct_graph;

const PATH_TO_YARN: &str = "./DUMMY.yarn";
const PATH_TO_DOT: &str = "./DUMMY.dot";

#[test]
fn graph_construction() {
    let contents =
        read_to_string(PATH_TO_YARN).expect(&format!("Can't read file: '{}'", PATH_TO_YARN));
    let dot_contents =
        read_to_string(PATH_TO_DOT).expect(&format!("Can't read file: '{}'", PATH_TO_DOT));

    let graph = construct_graph(contents);
    let dot = Dot::new(&graph);
    assert_eq!(dot_contents, dot.to_string());
}
