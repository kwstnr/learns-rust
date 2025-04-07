fn main() {
    println!("Hello, world!");

    // this is on the heap
    let mut s = String::from("hello");

    s.push_str(" world");

    println!("{}", s);

    // this is on the stack
    let mut x = "test";
    println!("{}", x);

    x = "test2";

    println!("{}", x);

    // String objects (call by reference basically)
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}", s1); // this will cause an error because s1 is `moved` to s2

    println!("{}", s2); // this will work because s2 is a copy of s1

    // deep copies
    let s3 = String::from("hello, I am being deep copied");
    let mut s4 = s3.clone();
    s4.push_str(" and I was deep copied");

    println!("{}", s3); // this will work because s3 is a deep copy of s4
    println!("{}", s4);

    // stack only data: copy
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");

    // ownership tests
    let s5 = String::from("hello");

    let s6 = takes_ownership(s5); // s5's value moves into the function, it implements the Drop trait...

    // println!("{s5}"); // ... and so is no longer valid here
    println!("{s6}"); // s6 is valid here

    let x = 5; // x is a stack variable

    makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay

    println!("{x}"); // x is still valid here

    // giving and returning ownership
    let s1 = gives_ownership();         // gives_ownership moves its return
    // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3

    println!("{s1}"); // s1 is valid here
    // println!("{s2}"); // s2 is not valid here, it was moved into the function "takes_and_gives_back"
    println!("{s3}"); // s3 is valid here

    // giving back ownership using tuples
    let s4 = String::from("hello");
    let (s5, len) = calculate_length(s4); // s4 is moved into the function
    // calculate_length, which also moves its return value into s5
    // and len
    println!("The length of '{}' is {}.", s5, len); // s5 is valid herea

    // references and borrowing
    let s6 = String::from("hello");
    let len = calculate_length_ref(&s6); // s6 is borrowed, so it is not moved

    println!("The length of '{}' is {}.", s6, len); // s6 is valid here

    // mutable references
    println!("****** mutable references *****");

    let mut s7 = String::from("hello");
    println!("I am s7 before change: {}", s7); // s7 is valid here
    let r1 = &mut s7; // r1 is a mutable reference to s7
    println!("I am r1 before change, i was borrowed from s7: {}", r1); // r1 is valid here

    change_string(r1); // r1 is passed to the function, which changes s7
    // println!("{}", s7); // s7 is not valid here it was borrowed by r1 but never returned
    println!("I am r1 after change i am still borrowed, but will return now: {}", r1);
    println!("I am s7 after change, i have been returned: {}", s7);

    // slices
    println!("****** slices *****");
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello: {}, world: {}", hello, world);

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word_with_slices(&my_string[0..6]);
    let word = first_word_with_slices(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word_with_slices(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word_with_slices(&my_string_literal[0..6]);
    let word = first_word_with_slices(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_with_slices(my_string_literal);

    // slices of arrays
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn takes_ownership(some_string: String) -> String { // some_string comes into scope
    println!("{some_string}");
    some_string.replace("e", "E")
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str(" world");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_with_slices(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
