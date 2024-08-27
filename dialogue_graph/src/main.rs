use std::{
    collections::HashMap,
    fs::{self, DirEntry},
    io::Error,
};

use petgraph::{dot::Dot, prelude::*};

const PATH_TO_DIR: &str = "assets/dialogue";

fn try_read_yarn_contents(entry: Result<DirEntry, Error>) -> Option<(String, String)> {
    let entry = entry.expect("Can't get entry in current dir");
    let npc_file_name = entry
        .file_name()
        .into_string()
        .expect("Can't convert OsString to String")
        .split('.')
        .collect::<Vec<&str>>()[0]
        .to_string();
    let path = entry.path();

    if !path.is_file() {
        return None;
    }

    if let Some(ext) = path.extension() {
        if ext == "yarn" {
            return Some((
                (fs::read_to_string(path).expect("Should have been able to read the file")),
                npc_file_name,
            ));
        }
    }
    None
}

/// Counts the whitespaces at the start of the string.
/// As soon as a non-whitespace character is encountered, break.
fn count_whitespaces(line: &str) -> usize {
    let mut count = 0;
    for c in line.chars() {
        if !c.is_whitespace() {
            break;
        }
        count += 1;
    }
    count
}

fn get_latest_player_option(player_options: [Option<NodeIndex>; 10]) -> Option<NodeIndex> {
    for i in 0..player_options.len() {
        let option = player_options[player_options.len() - 1 - i];
        if option.is_some() {
            return option;
        }
    }
    return None;
}

fn construct_graph(contents: String, _npc_file_name: String) -> Graph<String, usize, Directed> {
    let dialogue_lines = contents.lines();
    let mut graph: Graph<String, usize, Directed> = Graph::default();

    let mut title_indices = HashMap::new();
    for title in dialogue_lines
        .clone()
        .filter(|l| l.trim().starts_with("title: "))
    {
        let index = graph.add_node(title.trim().to_string());
        assert!(
            title_indices.insert(title.trim(), index).is_none(),
            "There are two titles with the same name. The tests should cover this, are the tests passing?"
        );
    }

    let mut current_title = NodeIndex::new(0);
    let mut current_player_options = [None; 10];

    for line in dialogue_lines {
        if line.trim().starts_with("title: ") {
            current_title = *title_indices.get(&line).unwrap();
            // Clear the player options
            current_player_options = [None; 10];
        } else if line.trim().starts_with("-> ") {
            let whitespaces = count_whitespaces(line);
            assert!(
                whitespaces % 4 == 0,
                "Whitespaces should always be 4 chars long"
            );
            let whitespaces = whitespaces / 4;

            let n = graph.add_node(line.trim().to_string());
            if whitespaces == 0 {
                graph.update_edge(current_title, n, whitespaces);
            } else {
                let parent =
                    current_player_options[whitespaces - 1].expect("Player option should be set");
                graph.update_edge(parent, n, whitespaces);
            }
            current_player_options[whitespaces] = Some(n);
        } else if line.trim().starts_with("<<jump") {
            let title: &str = &format!(
                "title: {}",
                line.trim().split(" ").collect::<Vec<&str>>()[1]
                    .strip_suffix(">>")
                    .expect("Jump commands should always end on '>>'"),
            );
            let title_node = *title_indices.get(title).expect("Titles should all be set");

            let weight = count_whitespaces(line);
            assert!(
                weight % 4 == 0,
                "Whitespaces should always be 4 chars long."
            );
            let weight = weight / 4;

            if weight == 0 {
                graph.update_edge(title_node, title_node, weight);
            } else {
                let option_node = get_latest_player_option(current_player_options)
                    .expect("Player option should be set");
                graph.update_edge(option_node, title_node, weight);
            }
        }
    }
    graph
}

fn main() {
    for entry in fs::read_dir(PATH_TO_DIR).expect("Can't read entries in current dir") {
        let (contents, npc_file_name) = match try_read_yarn_contents(entry) {
            Some(r) => r,
            None => continue,
        };

        let graph = construct_graph(contents, npc_file_name);
        println!("{:?}", Dot::new(&graph));
        break;
    }
}
