pub struct Args {
    pub query: String,
    pub filepath: String,
}

impl Args {
    pub fn new(args: Vec<String>) -> Result<Args, String> {
        if args.len() < 3 {
            return Err(String::from("Not enough arguments provided"));
        }

        let query = args.get(1).ok_or(String::from("Missing query string"))?;
        let filepath = args.get(2).ok_or(String::from("Missing filepath"))?;

        Ok(Args {
            query: query.to_string(),
            filepath: filepath.to_string(),
        })
    }
}

#[cfg(test)]
mod test;
