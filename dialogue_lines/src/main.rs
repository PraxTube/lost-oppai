use std::{
    collections::HashMap,
    fs::{self, DirEntry},
    io::Error,
};

const PATH_TO_DIR: &str = "assets/dialogue";
const MAX_NPC_DISPLAY_NAME: usize = 12;

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

/// Loop over all yarn files in `PATH_TO_DIR` and apply the predicate on each line.
fn apply_to_lines<F>(mut predicate: F)
where
    F: FnMut(&str, &str),
{
    for entry in fs::read_dir(PATH_TO_DIR).expect("Can't read entries in current dir") {
        let (contents, npc_file_name) = match try_read_yarn_contents(entry) {
            Some(r) => r,
            None => continue,
        };

        for line in contents.lines().map(str::trim) {
            predicate(line.trim(), &npc_file_name)
        }
    }
}

fn number_npc_lines() -> usize {
    let mut number_of_lines = 0;
    apply_to_lines(|line, _| {
        if line.starts_with("{$") && line.split('}').collect::<Vec<&str>>()[1].starts_with(':') {
            number_of_lines += 1;
        }
    });
    number_of_lines
}

fn number_player_options() -> usize {
    let mut number_of_options = 0;
    apply_to_lines(|line, _| {
        if line.starts_with("-> ") {
            number_of_options += 1;
        }
    });
    number_of_options
}

fn number_player_lines() -> usize {
    let mut number_of_lines = 0;
    apply_to_lines(|line, _| {
        if line.starts_with("You: ") {
            number_of_lines += 1;
        }
    });
    number_of_lines
}

fn print_individual_npc_lines() {
    let mut npc_lines: HashMap<String, usize> = HashMap::new();

    apply_to_lines(|line, file_name| {
        let line_parts = line.split('}').collect::<Vec<&str>>();
        let (name_with_prefix, rest) = if line_parts.len() == 2 {
            (line_parts[0], line_parts[1])
        } else {
            return;
        };

        if !rest.starts_with(':') {
            return;
        }

        if let Some(name) = name_with_prefix.strip_prefix("{$") {
            let name = if name == "name" { file_name } else { name };
            match npc_lines.get_mut(name) {
                Some(r) => *r += 1,
                None => assert!(npc_lines.insert(name.to_string(), 1).is_none()),
            };
        }
    });

    let mut npc_lines_vec = npc_lines.into_iter().collect::<Vec<(String, usize)>>();
    npc_lines_vec.sort_by(|x, y| y.1.cmp(&x.1));

    for (key, value) in npc_lines_vec {
        let chars = key.chars().count();
        assert!(chars <= MAX_NPC_DISPLAY_NAME);
        let npc_display_name = key + &" ".repeat(MAX_NPC_DISPLAY_NAME - chars);
        println!("  {}: {}", npc_display_name, value);
    }
}

fn main() {
    println!("Total player lines: {}", number_player_lines());
    println!("Total player options: {}", number_player_options());
    println!("Total NPC lines: {}", number_npc_lines());
    print_individual_npc_lines();
}
