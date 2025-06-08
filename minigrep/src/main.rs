use std::env;
use std::fs;
use minigrep::args_parser::Args;
use minigrep::search;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = match Args::new(args) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let file_content = match fs::read_to_string(&args.filepath) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", args.filepath, e);
            std::process::exit(1);
        }
    };

    let results = search::search_string(&args.query, &file_content);

    if results.is_empty() {
        println!("No matches found for '{}'", args.query);
    } else {
        println!("Matches found for '{}':", args.query);
        for result in results {
            println!("{}: {}", result.line_number, result.content);
        }
    }
}
