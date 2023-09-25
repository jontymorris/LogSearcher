mod config;
mod searcher;
mod cleaner;

use crate::{config::get_args_and_config, searcher::search_folder, cleaner::clean_folder};

fn main() {
    let (args, config) = get_args_and_config();

    if args.search {
        search_folder(&config)
    }

    else if args.clean {
        clean_folder(&config);
    }
}
