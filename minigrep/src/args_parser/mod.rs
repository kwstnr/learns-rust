pub struct Args {
    pub query: String,
    pub filepath: String,
    pub case_sensitive: bool,
}

impl Args {
    pub fn new(args: &Vec<String>, case_sensitive: &str) -> Result<Args, String> {
        if args.len() < 3 {
            return Err(String::from("Not enough arguments provided"));
        }

        let query = args.get(1).ok_or(String::from("Missing query string"))?;
        let filepath = args.get(2).ok_or(String::from("Missing filepath"))?;

        Ok(Args {
            query: query.to_string(),
            filepath: filepath.to_string(),
            case_sensitive: parse_case_sensitive(case_sensitive),
        })
    }
}

fn parse_case_sensitive(var: &str) -> bool {
    let trimmed = var.trim();
    match trimmed.parse::<i32>() {
        Ok(num) => return if num > 0 { true } else { false },
        _ => None::<i32>,
    };

    match trimmed.to_lowercase().as_str() {
        "true" => true,
        _ => false,
    }
}

#[cfg(test)]
mod test;
