use std::{
    fs::{self, DirEntry},
    io::Error,
    path::PathBuf,
};

use strsim::levenshtein;

const PATH_TO_DIR: &str = "assets/dialogue";

const MAX_SIMILARITY_DISTANCE: usize = 4;

fn try_yarn_file(entry: Result<DirEntry, Error>) -> Option<PathBuf> {
    let entry = entry.expect("Can't get entry in current dir");
    let path = entry.path();

    if !path.is_file() {
        return None;
    }

    if let Some(ext) = path.extension() {
        if ext == "yarn" {
            return Some(path);
        }
    }
    return None;
}

fn try_read_yarn_contents(entry: Result<DirEntry, Error>) -> Option<String> {
    let path = match try_yarn_file(entry) {
        Some(r) => r,
        None => return None,
    };

    Some(fs::read_to_string(path).expect("Should have been able to read the file"))
}

/// This essentially tests for typos in the commands of the yarn files.
#[test]
fn validate_custom_commands() {
    let custom_commands = ["stop_chat", "target_npc_mentioned", "trigger_ending"];
    for entry in fs::read_dir(PATH_TO_DIR).expect("Can't read entries in current dir") {
        let contents = match try_read_yarn_contents(entry) {
            Some(r) => r,
            None => continue,
        };

        for line in contents.split('\n') {
            let line = line.trim();
            if !line.starts_with("<<") {
                continue;
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
        }
    }
}

#[test]
fn validate_target_npc_mentioned_command() {
    for entry in fs::read_dir(PATH_TO_DIR).expect("Can't read entries in current dir") {
        let contents = match try_read_yarn_contents(entry) {
            Some(r) => r,
            None => continue,
        };

        for line in contents.split('\n') {
            let line = line.trim();
            if line.starts_with("<<target_npc_mentioned ") {
                assert!(line == "<<target_npc_mentioned {$name} {$target_npc}>>");
            }
        }
    }
}

#[test]
fn validate_stop_chat_command() {
    for entry in fs::read_dir(PATH_TO_DIR).expect("Can't read entries in current dir") {
        let contents = match try_read_yarn_contents(entry) {
            Some(r) => r,
            None => continue,
        };

        for line in contents.split('\n') {
            let line = line.trim();
            if line.starts_with("<<stop_chat") {
                assert!(line == "<<stop_chat>>");
            }
        }
    }
}

#[test]
fn validate_trigger_ending() {
    for entry in fs::read_dir(PATH_TO_DIR).expect("Can't read entries in current dir") {
        let contents = match try_read_yarn_contents(entry) {
            Some(r) => r,
            None => continue,
        };

        for line in contents.split('\n') {
            let line = line.trim();
            if line.starts_with("<<trigger_ending") {
                assert!(line == "<<trigger_ending {$name}>>");
            }
        }
    }
}
