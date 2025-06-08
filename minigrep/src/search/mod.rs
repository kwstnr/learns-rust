pub struct Result {
    pub line_number: u32,
    pub content: String,
}

pub fn search_string(query: &str, content: &str) -> Vec<Result> {
    let mut results = Vec::new();
    let mut index = 1;
    for line in content.lines() {
        if line.contains(query) {
            results.push(Result {
                line_number: index.clone(),
                content: line.to_string(),
            });
        }
        index = index + 1;
    }
    results
}

#[cfg(test)]
mod test;