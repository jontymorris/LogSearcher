use std::fs;

use clap::Parser;
use dirs;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t=false)]
    pub clean: bool,

    #[arg(short, long, default_value_t=false)]
    pub search: bool,

    #[arg(short, long)]
    pub keyword: Option<String>,

    #[arg(short, long)]
    pub input: Option<String>,

    #[arg(short, long)]
    pub output: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub input: String,
    pub output: String,
    pub whitelist: Vec<String>,
    pub blacklist: Vec<String>,
}

fn get_config() -> Config {
    let default_config = Config {
        input: String::from("./input"),
        output: String::from("./output"),
        whitelist: vec![],
        blacklist: vec![],
    };
    
    let config_path = dirs::home_dir();
    if config_path.is_none() {
        println!("[INFO] Home directory not found.");
        return default_config;
    }

    let mut config_path = config_path.unwrap();
    config_path.push("log_searcher.config");
    if !config_path.exists() {
        println!("[INFO] No config file found.");
        return default_config;
    }

    let content = fs::read_to_string(&config_path);
    if content.is_err() {
        println!("[INFO] Unable to read config file.");
        return default_config;
    }

    let content = content.unwrap();
    let config = serde_json::from_str(&content);
    if config.is_err() {
        println!("[INFO] Config file is malformed.");
        return default_config;
    }

    config.unwrap()
}

pub fn get_args_and_config() -> (Args, Config) {
    let args = Args::parse();
    let mut config = get_config();

    if args.input.is_some() {
        config.input = args.input.clone().unwrap();
    }

    if args.output.is_some() {
        config.output = args.output.clone().unwrap();
    }

    if args.keyword.is_some() {
        config.whitelist.push(args.keyword.clone().unwrap());
    }

    (args, config)
}
