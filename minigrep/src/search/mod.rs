pub struct Result {
    pub line_number: u32,
    pub content: String,
}

pub fn search_string(query: &str, content: &str) -> Vec<Result> {
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .map(|(index, line)| Result {
            line_number: (index + 1) as u32,
            content: line.to_string(),
        })
        .collect()
}

#[cfg(test)]
mod test;