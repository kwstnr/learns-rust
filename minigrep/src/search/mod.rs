pub fn search_string(query: &str, content: &str) -> Vec<String> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line.to_string());
        }
    }
    results
}

#[cfg(test)]
mod test;