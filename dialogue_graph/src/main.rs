#[cfg(test)]
mod test;

use core::panic;
use std::{
    collections::HashMap,
    fs::{self, create_dir, remove_dir_all, DirEntry, File},
    io::{Error, Write},
    path::Path,
};

use petgraph::{dot::Dot, prelude::*};

const PATH_TO_DIR: &str = "assets/dialogue";
const OUPUT_PATH: &str = "graphs";

struct Container {
    title: NodeIndex,
    player_options: [Option<NodeIndex>; 10],
    graph: Graph<String, usize, Directed>,
    title_indices: HashMap<String, NodeIndex>,
}

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

fn index_from_whitespaces(line: &str) -> usize {
    let whitespaces = count_whitespaces(line);
    assert!(
        whitespaces % 4 == 0,
        "Whitespaces should always be 4 chars long"
    );
    whitespaces / 4
}

fn handle_player_option(container: &mut Container, line: &str) {
    let index = index_from_whitespaces(line);

    let end = container.graph.add_node(line.trim().to_string());
    let start = if index == 0 {
        container.title
    } else {
        container.player_options[index - 1].unwrap_or(container.title)
    };
    container.graph.update_edge(start, end, index);
    container.player_options[index] = Some(end);
}

fn handle_jump_command(container: &mut Container, line: &str) {
    let title: &str = &format!(
        "title: {}",
        line.trim().split(" ").collect::<Vec<&str>>()[1]
            .strip_suffix(">>")
            .expect("Jump commands should always end on '>>'"),
    );
    let title_node = *container
        .title_indices
        .get(title)
        .expect("Titles should all be set");

    fn get_option(player_options: [Option<NodeIndex>; 10], index: usize) -> Option<NodeIndex> {
        if index == 0 {
            return None;
        }

        match player_options[index - 1] {
            Some(r) => Some(r),
            None => get_option(player_options, index - 1),
        }
    }

    let index = index_from_whitespaces(line);
    let option_node = get_option(container.player_options, index).unwrap_or(container.title);
    container.graph.update_edge(option_node, title_node, index);
}

fn handle_ending_command(container: &mut Container, line: &str) {
    let weight = index_from_whitespaces(line);

    let start = if weight == 0 {
        container.title
    } else {
        get_option(container.player_options, weight)
    };

    fn get_option(player_options: [Option<NodeIndex>; 10], weight: usize) -> NodeIndex {
        if weight == 0 {
            panic!("current_player_options is empty but it's supposed to have at least one non-empty value");
        }

        match player_options[weight - 1] {
            Some(r) => r,
            None => get_option(player_options, weight - 1),
        }
    }

    let end = container.graph.add_node("Fin".to_string());
    container.graph.update_edge(start, end, weight);
}

fn clear_player_options(container: &mut Container, line: &str) {
    let index = index_from_whitespaces(line);
    for i in (index..container.player_options.len()).rev() {
        container.player_options[i] = None;
    }
}

fn construct_graph(contents: String) -> Graph<String, usize, Directed> {
    let dialogue_lines = contents.lines();

    let mut container = Container {
        title: NodeIndex::new(0),
        player_options: [None; 10],
        graph: Graph::default(),
        title_indices: HashMap::new(),
    };

    for title in dialogue_lines
        .clone()
        .filter(|l| l.trim().starts_with("title: "))
    {
        let index = container.graph.add_node(title.trim().to_string());
        assert!(
            container.title_indices.insert(title.trim().to_string(), index).is_none(),
            "There are two titles with the same name. The tests should cover this, are the tests passing?"
        );
    }

    for line in dialogue_lines {
        clear_player_options(&mut container, line);
        if line.trim().starts_with("title: ") {
            container.title = *container.title_indices.get(line).unwrap();
        } else if line.trim().starts_with("-> ") {
            handle_player_option(&mut container, line);
        } else if line.trim().starts_with("<<jump ") {
            handle_jump_command(&mut container, line);
        } else if line.trim().starts_with("<<trigger_ending ") {
            handle_ending_command(&mut container, line);
        }
    }
    container.graph
}

fn main() {
    if Path::new(OUPUT_PATH).exists() {
        remove_dir_all(OUPUT_PATH).expect("Couldn't hard remove graph output dir");
        create_dir(OUPUT_PATH).expect("Couldn't create graph output dir");
    }

    for entry in fs::read_dir(PATH_TO_DIR).expect("Can't read entries in current dir") {
        let (contents, npc_file_name) = match try_read_yarn_contents(entry) {
            Some(r) => r,
            None => continue,
        };

        let path = &format!("{}/{}.dot", OUPUT_PATH, npc_file_name);
        let mut file = match File::create(path) {
            Ok(r) => r,
            Err(err) => panic!("Can't create/open file: '{}', {}", path, err),
        };

        let graph = construct_graph(contents);
        file.write(Dot::new(&graph).to_string().as_bytes())
            .expect(&format!("Couldn't write to file: '{}'", path));
    }
}
