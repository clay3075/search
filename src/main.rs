use std::env;
use std::path::Path;
use std::fs;
mod search;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query: &String = &args[1];
    let current_path: String = env::current_dir().unwrap().to_str().unwrap().to_string();
    let path: &String = if args.len() > 2 && Path::new(&args[2]).exists() { &args[2] } else { &current_path };
    let meta = fs::metadata(path).unwrap();
    let search_results: Result<Vec<search::search_result::SearchResult>, std::io::Error>;

    if meta.is_file() {
        search_results = search::search_file(&path, &query);
    } else {
        search_results = search::search_directory(&path, &query);
    }

    match search_results {
        Ok(results) => {
            search::print_results(&results);
        },
        Err(err) => {
            println!("Error occurred during search: {}", err);
        }
    }
    
}
