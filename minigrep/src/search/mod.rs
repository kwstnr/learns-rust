pub fn search_string(query: &str, content: &str) -> Vec<(u32, String)> {
    let mut results = Vec::new();
    let mut index = 1;
    for line in content.lines() {
        if line.contains(query) {
            results.push((index, line.to_string()));
        }
        index = index + 1;
    }
    results
}

#[cfg(test)]
mod test;