use std::fs;

const PATH_TO_DIR: &str = "assets/dialogue";

#[test]
fn validate_target_npc_mentioned_command() {
    for entry in fs::read_dir(PATH_TO_DIR).expect("Can't read entries in current dir") {
        let entry = entry.expect("Can't get entry in current dir");
        let path = entry.path();

        if !path.is_file() {
            continue;
        }
        if let Some(ext) = path.extension() {
            if ext != "yarn" {
                continue;
            }
        }

        let contents = fs::read_to_string(path).expect("Should have been able to read the file");

        for line in contents.split('\n') {
            let line = line.trim();
            if line.starts_with("<<target_npc_mentioned ") {
                assert!(line == "<<target_npc_mentioned {$name} {$target_npc}>>");
            }
        }
    }
}
