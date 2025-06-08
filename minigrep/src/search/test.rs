use crate::search;

#[test]
fn search_string_empty_query() {
    let content = "This is a test.\nAnother line.";
    let results = search::search_string("", content);

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].1, "This is a test.");
    assert_eq!(results[0].0, 1);
    assert_eq!(results[1].1, "Another line.");
    assert_eq!(results[1].0, 2);
}

#[test]
fn search_string_no_match() {
    let content = "This is a test.\nAnother line.";
    let results = search::search_string("not found", content);

    assert!(results.is_empty());
}

#[test]
fn search_string_single_match() {
    let content = "This is a test.\nAnother line.";
    let results = search::search_string("test", content);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].0, 1);
    assert_eq!(results[0].1, "This is a test.");
}