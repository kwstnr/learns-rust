use adder::greeting;

mod common;

#[test]
fn it_greeting_contains_name() {
    common::setup();
    let name = "Alice";
    let result = greeting(name);
    assert!(result.contains("Hello"), "Greeting did not contain the name: {}", name);
}