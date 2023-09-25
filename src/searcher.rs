use std::{path::PathBuf, str::FromStr};
use walkdir::WalkDir;

use crate::config::Config;

fn is_whitelist(value: &str, config: &Config) -> bool {
    return config.whitelist.iter().any(|x| value.contains(x));
}

fn is_blacklist(value: &str, config: &Config) -> bool {
    return config.blacklist.iter().any(|x| value.contains(x));
}

fn check_file_content(path: &PathBuf, config: &Config) -> bool {
    let content = std::fs::read_to_string(path);
    if content.is_err() {
        return false;
    }

    let content = content.unwrap();
    is_whitelist(content.as_str(), config)
}

fn copy_to_output(path: &PathBuf, config: &Config) {
    println!("Found {}", path.display());

    let mut input_folder = path.clone();
    if input_folder.is_file() {
        input_folder.pop();
    }

    let name = input_folder.file_name().unwrap();
    let mut output_folder = PathBuf::from_str(config.output.as_str()).unwrap();
    output_folder.push(name);

    let copy = copy_dir::copy_dir(input_folder, output_folder);
    if copy.is_err() {
        println!("{}: {}", path.display(), copy.unwrap_err().to_string());
    }
}

pub fn search_folder(config: &Config) {
    let mut it = WalkDir::new(&config.input).into_iter();
    loop {
        let entry = match it.next() {
            None => break,
            Some(Err(err)) => panic!("ERROR: {}", err),
            Some(Ok(entry)) => entry,
        };

        let is_file = entry.file_type().is_file();
        let file_name = entry.file_name().to_string_lossy().into_owned();

        if is_blacklist(&file_name, &config) {
            it.skip_current_dir();
            continue;
        }

        let file_path = entry.into_path();

        if is_whitelist(file_name.as_str(), config) {
            copy_to_output(&file_path, config);
            it.skip_current_dir();
        }

        else if is_file && check_file_content(&file_path, config) {
            copy_to_output(&file_path, config);
            it.skip_current_dir();
        }
    }
}
