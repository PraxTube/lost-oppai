use std::fs::{read_dir, read_to_string};

use petgraph::{dot::Dot, Direction};
use pretty_assertions::assert_eq;

use crate::{construct_graph, try_read_yarn_contents, PATH_TO_DIALOGUES};

const PATH_TO_YARN: &str = "./DUMMY.yarn";
const PATH_TO_DOT: &str = "./DUMMY.dot";

#[test]
fn graph_construction() {
    let contents = read_to_string(PATH_TO_YARN)
        .unwrap_or_else(|_| panic!("Can't read file: '{}'", PATH_TO_YARN));
    let dot_contents = read_to_string(PATH_TO_DOT)
        .unwrap_or_else(|_| panic!("Can't read file: '{}'", PATH_TO_DOT));

    let graph = construct_graph(contents);
    let dot = Dot::new(&graph);
    assert_eq!(dot_contents, dot.to_string());
}

#[test]
fn validate_no_hanging_nodes() {
    for entry in
        read_dir(format!("../{}", PATH_TO_DIALOGUES)).expect("Can't read entries in current dir")
    {
        let (contents, npc_file_name) = match try_read_yarn_contents(entry) {
            Some(r) => r,
            None => continue,
        };

        let graph = construct_graph(contents);
        for index in graph.node_indices().skip(1) {
            assert!(
                graph.edges_directed(index, Direction::Incoming).count() != 0,
                "There is a node that doesn't have any incoming edges. This should only be the case for the very first node.\nfile: '{}', node: '{}'",
                npc_file_name,
                graph.node_weight(index).expect("Node should exist")
            );
        }
    }
}
