pub struct Result {
    pub line_number: u32,
    pub content: String,
}

pub fn search_string(query: &str, content: &str, case_sensitive: bool) -> Vec<Result> {
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            if case_sensitive {
                line.contains(query)
            } else {
                line.to_lowercase().contains(&query.to_lowercase())
            }
        })
        .map(|(index, line)| Result {
            line_number: (index + 1) as u32,
            content: line.to_string(),
        })
        .collect()
}

#[cfg(test)]
mod test;