use crate::args_parser::Args;

#[test]
fn new_insufficient_args() {
    let args = vec![String::from("foo")];
    let result = Args::new(args);

    assert!(result.is_err());
}

#[test]
fn new_valid_args() {
    let args = vec![
        String::from("foo"),
        String::from("query"),
        String::from("filepath"),
    ];
    let result = Args::new(args);

    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(args.query, "query");
    assert_eq!(args.filepath, "filepath");
}