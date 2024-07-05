use std::{
    fs::{self, DirEntry},
    io::Error,
    str::FromStr,
};

use strsim::levenshtein;

use crate::npc::NpcDialogue;

const PATH_TO_DIR: &str = "assets/dialogue";

const MAX_SIMILARITY_DISTANCE: usize = 4;

const REQUIRED_VARIABLES: [&str; 3] = ["name", "target_npc", "talked_with_target_npc"];

fn try_read_yarn_contents(entry: Result<DirEntry, Error>) -> Option<(String, String)> {
    let entry = entry.expect("Can't get entry in current dir");
    let npc_file_name = entry
        .file_name()
        .into_string()
        .expect("Can't convert OsString to String")
        .split(".")
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

fn validate_lines<F>(predicate: F)
where
    F: Fn(&str, &str),
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

/// This essentially tests for typos in the commands of the yarn files.
#[test]
fn validate_custom_commands() {
    let custom_commands = ["stop_chat", "target_npc_mentioned", "trigger_ending"];

    validate_lines(|line, _| {
        if !line.starts_with("<<") {
            return;
        }

        let mut closest_command: &str = custom_commands[0];
        let mut closest_distance = usize::max_value();

        let command = line.split(' ').collect::<Vec<&str>>()[0]
            .trim_start_matches("<<")
            .trim_end_matches(">>");
        for custom_command in custom_commands {
            let distance = levenshtein(command, custom_command);
            if distance < closest_distance {
                closest_command = custom_command;
                closest_distance = distance;
            }
        }

        if closest_distance < MAX_SIMILARITY_DISTANCE {
            assert!(
                command == closest_command,
                "{command} and {closest_command} are {closest_distance} levenshtein close but don't match!"
            );
        }
    });
}

#[test]
fn validate_target_npc_mentioned_command() {
    validate_lines(|line, _| {
        if line.starts_with("<<target_npc_mentioned ") {
            assert!(line == "<<target_npc_mentioned {$name} {$target_npc}>>");
        }
    })
}

#[test]
fn validate_stop_chat_command() {
    validate_lines(|line, _| {
        if line.starts_with("<<stop_chat") {
            assert!(line == "<<stop_chat>>");
        }
    })
}

#[test]
fn validate_trigger_ending() {
    validate_lines(|line, _| {
        if line.starts_with("<<trigger_ending") {
            assert!(line == "<<trigger_ending {$name}>>");
        }
    })
}

#[test]
fn validate_npc_names_existence() {
    validate_lines(|line, _| {
        if line.starts_with("<<set $name") || line.starts_with("<<set $target_npc") {
            let parts: Vec<&str> = line.split("\"").collect();
            assert!(
                parts.len() == 3,
                "Length of parts is not 3, instead it's {}, {:?}",
                parts.len(),
                parts
            );

            let npc_name = parts[1].trim_start_matches("_");
            match NpcDialogue::from_str(npc_name) {
                Ok(_) => {}
                Err(err) => panic!(
                    "The npc name doesn't match any NpcDialogue names, {}\n{}",
                    npc_name, err
                ),
            };
        }
    })
}

#[test]
fn match_names_with_files() {
    validate_lines(|line, npc_file_name| {
        if line.starts_with("<<set $name") {
            let parts: Vec<&str> = line.split("\"").collect();
            assert!(
                parts.len() == 3,
                "Length of parts is not 3, instead it's {}, {:?}",
                parts.len(),
                parts
            );

            let npc_name = parts[1].trim_start_matches("_");
            if npc_name.to_lowercase() != npc_file_name {
                panic!(
                    "Name of npc is {} in yarn file, but yarn file is named {}",
                    npc_name, npc_file_name
                );
            }
        }
    });
}

#[test]
fn check_all_required_variables() {
    for entry in fs::read_dir(PATH_TO_DIR).expect("Can't read entries in current dir") {
        let file_name = entry
            .as_ref()
            .expect("Can't get entry in current dir")
            .file_name();
        let (contents, _) = match try_read_yarn_contents(entry) {
            Some(r) => r,
            None => continue,
        };

        let mut contains_variables = [false; REQUIRED_VARIABLES.len()];

        for line in contents.lines().map(str::trim) {
            if line.starts_with("<<set ") {
                let command = line.split(' ').collect::<Vec<&str>>()[1].trim_start_matches("$");
                if let Some(index) = REQUIRED_VARIABLES.iter().position(|cmd| *cmd == command) {
                    contains_variables[index] = true;
                }
            }
        }

        assert!(
            !contains_variables.contains(&false),
            "Not all required variables present in {:?}. Missing variables:\n {:?}",
            file_name,
            REQUIRED_VARIABLES
                .iter()
                .enumerate()
                .filter(|(i, _)| !contains_variables[*i])
                .collect::<Vec<(usize, &&str)>>()
        );
    }
}
