use crate::search;

#[test]
fn search_string_empty_query() {
    let content = "This is a test.\nAnother line.";
    let results = search::search_string("", content, false);

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].content, "This is a test.");
    assert_eq!(results[0].line_number, 1);
    assert_eq!(results[1].content, "Another line.");
    assert_eq!(results[1].line_number, 2);
}

#[test]
fn search_string_no_match() {
    let content = "This is a test.\nAnother line.";
    let results = search::search_string("not found", content, false);

    assert!(results.is_empty());
}

#[test]
fn search_string_single_match() {
    let content = "This is a test.\nAnother line.";
    let results = search::search_string("test", content, false);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].line_number, 1);
    assert_eq!(results[0].content, "This is a test.");
}

#[test]
fn search_string_case_sensitive_no_match() {
    let content = "This is a test.\nAnother line.";
    let results = search::search_string("Test", content, true);

    assert!(results.is_empty());
}

#[test]
fn search_string_case_sensitive_match() {
    let content = "This is a test.\nAnother line.";
    let results = search::search_string("test", content, true);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].line_number, 1);
    assert_eq!(results[0].content, "This is a test.");
}