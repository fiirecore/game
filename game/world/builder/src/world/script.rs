use std::path::PathBuf;

use firecore_world_lib::script::world::WorldScript;

pub fn load_script_entries(script_path: PathBuf) -> Vec<WorldScript> {
    let mut scripts = Vec::new();
    if let Ok(dir) = std::fs::read_dir(script_path) {
        for entry in dir {
            if let Ok(entry) = entry {
                let file = entry.path();
                match std::fs::read_to_string(&file) {
                    Ok(content) => {
                        let script: Result<WorldScript, ron::Error> = ron::from_str(&content);
                        match script {
                            Ok(script) => {
                                scripts.push(script);
                            },
                            Err(err) => {
                                panic!("Could not parse script at {:?} with error {} at position {}", file, err, err.position);
                            }
                        }
                    },
                    Err(err) => {
                        eprintln!("Could not get script entry at {:?} as string with error {}", file, err);
                    }
                }
            }
        }
    }
    scripts
}