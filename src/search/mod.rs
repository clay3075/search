use std::fs;
use std::io;
use std::io::BufRead;
pub mod search_result;

pub fn search_file(path: &String, query: &String) -> Result<Vec<search_result::SearchResult>, io::Error> {
    let file = fs::File::open(&path).unwrap();
    let buf_reader = io::BufReader::new(file);
    let mut search_results: Vec<search_result::SearchResult> = vec!();
    let mut line_number = 1;

    for line_result in buf_reader.lines() {
        match line_result {
            Ok(line) => {
                if line.contains(query) {
                    let cursor_pos = line.find(query).unwrap() as u64;
                    search_results.push(search_result::SearchResult::new(path.to_string(), query.to_string(), line_number, cursor_pos));
                }   
            }, 
            Err(_err) => {

            }
        }
        line_number =  line_number + 1;
    }

    return Ok(search_results);
}

pub fn search_directory(path: &String, query: &String) -> Result<Vec<search_result::SearchResult>, io::Error> {
    let dir_contents = fs::read_dir(&path)?;
    let mut search_results: Vec<search_result::SearchResult> = vec!();

    for content in dir_contents {
        let content = content?;
        let content_path = content.path().to_str().unwrap().to_string();
        if content.metadata()?.is_dir() {
            search_results.extend(search_directory(&content_path, &query)?);
        } else {
            search_results.extend(search_file(&content_path, &query)?);
        }
    }

    return Ok(search_results);
}

pub fn print_results(search_results: &Vec<search_result::SearchResult>) {
    if search_results.is_empty() {
        println!("No entries were found.");
    } else {
        println!("{} occurrences", search_results.len());
        for result in search_results {
            println!("{}: line {}:{}", result.path, result.line_number, result.cursor_pos);
        }
    }
}