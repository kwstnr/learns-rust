use std::env;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let case_sensitive = env::var("CASE_SENSITIVE").unwrap_or_else(|_| "0".to_string());
    run(&args, &case_sensitive);
}
