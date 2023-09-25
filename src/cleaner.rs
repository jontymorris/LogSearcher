use std::{path::PathBuf, fs};

use serde_json::Value;
use walkdir::WalkDir;

use crate::config::Config;

fn clean_file(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(&path)?;
    let parsed_json: Value = serde_json::from_str(&content)?;
    let formatted_content = serde_json::to_string_pretty(&parsed_json)?;
    fs::write(path, formatted_content)?;

    Ok(())
}

pub fn clean_folder(config: &Config) {
    for entry in WalkDir::new(&config.input) {
        let entry = entry.unwrap();
        let is_file = entry.file_type().is_file();

        if !is_file {
            continue;
        }

        let file_path = entry.into_path();
        if clean_file(&file_path).is_ok() {
            println!("Cleaned {}", file_path.display());
        }
    }
}