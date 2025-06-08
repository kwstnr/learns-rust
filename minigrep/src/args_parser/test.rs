use crate::args_parser::Args;

#[test]
fn new_insufficient_args() {
    let args = vec![String::from("foo")];
    let result = Args::new(&args, "");
    assert!(result.is_err());
}

#[test]
fn new_valid_args() {
    let args = vec![
        String::from("foo"),
        String::from("query"),
        String::from("filepath"),
    ];
    let result = Args::new(&args, "");

    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(args.query, "query");
    assert_eq!(args.filepath, "filepath");
}

#[test]
fn new_valid_args_case_sensitive() {
    let args = vec![
        String::from("foo"),
        String::from("query"),
        String::from("filepath"),
    ];
    let result = Args::new(&args, "true");

    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(args.case_sensitive, true);
}

#[test]
fn new_valid_args_case_insensitive() {
    let args = vec![
        String::from("foo"),
        String::from("query"),
        String::from("filepath"),
    ];
    let result = Args::new(&args, "false");

    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(args.case_sensitive, false);
}

#[test]
fn new_valid_args_other_str_case_sensitive() {
    let args = vec![
        String::from("foo"),
        String::from("query"),
        String::from("filepath"),
    ];
    let result = Args::new(&args, "invalid");

    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(args.case_sensitive, false);
}

#[test]
fn new_valid_args_case_sensitive_zero() {
    let args = vec![
        String::from("foo"),
        String::from("query"),
        String::from("filepath"),
    ];
    let result = Args::new(&args, "0");

    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(args.case_sensitive, false);
}

#[test]
fn new_valid_args_case_sensitive_one() {
    let args = vec![
        String::from("foo"),
        String::from("query"),
        String::from("filepath"),
    ];
    let result = Args::new(&args, "1");

    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(args.case_sensitive, true);
}

#[test]
fn parse_case_sensitive_true() {
    let result = super::parse_case_sensitive("true");
    assert!(result);
}

#[test]
fn parse_case_sensitive_false() {
    let result = super::parse_case_sensitive("false");
    assert!(!result);
}

#[test]
fn parse_case_sensitive_invalid() {
    let result = super::parse_case_sensitive("invalid");
    assert!(!result);
}

#[test]
fn parse_case_sensitive_zero() {
    let result = super::parse_case_sensitive("0");
    assert!(!result);
}

#[test]
fn parse_case_sensitive_one() {
    let result = super::parse_case_sensitive("1");
    assert!(result);
}

#[test]
fn parse_case_sensitive_other_number() {
    let result = super::parse_case_sensitive("42");
    assert!(result);
}

#[test]
fn parse_case_sensitive_empty() {
    let result = super::parse_case_sensitive("");
    assert!(!result);
}