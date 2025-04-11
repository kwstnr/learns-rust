use std::collections::HashMap;

fn main() {
    let mut vec = Vec::new();

    vec.push(5);
    vec.push(6);

    let vec: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    dbg!(&v);
    println!("v: {:?}", v);
    println!("v: {:#?}", v);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6); 
    // upper does not compile, because on line 26 a immutable borrow was done.
    // On line 28 we want to do a mutable borrow though, rust does not allow mutable and immutable
    // borrows at the same time!

    println!("The first element is: {first}");

    // Iterating
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for mut i in &mut v {
        *i += 5;
    }

    for i in &v {
        println!("{i}");
    }

    // Strings, which are collections of chars
    println!("**** Strings, collections of chars ****");
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // note: s1 has ben moved here and can no longer be used
    
    // println!("s1: {s1}"); // -> does not compile because s1 was moved
    println!("s2: {s2}");
    println!("s3: {s3}");

    // also interesting:
    let string_addition = String::from("Hello, ");
    let string_to_add = String::from("World!");
    // let sum = string_addition + string_to_add; // -> does not compile because beneath the
                                                  // + operator, the add function is used, which's
                                                  // signature looks like this:
                                                  // `fn add(self, s: &str) -> String`

    // let sum = &string_addition + &string_to_add; // -> does also not compile!
    
    let sum = string_addition + &string_to_add; // -> the only way this compiles!
    println!("sum {sum}");
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let formatted_string = format!("{s1}-{s2}-{s3}"); // -> takes references of all parameters
    
    println!("{formatted_string} was created from {s1}, {s2} and {s3}"); // no ownership was taken
                                                                         // from format! macro

    // hasmaps
    println!("**** Hashmaps ****");


    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert("Yellow".to_string(), 50);

    let team_name = String::from("Red");

    // scores.insert(team_name, 60);
    // println!("team_name: {team_name}"); // -> does not compile, because team_name was moved into
                                           // scores.insert

    // scores.insert(&team_name, 60); // does also not compile, interesting
    
    // the only way to keep track of keys of a hashmap is by inserting a copy or clone of the key:
    scores.insert(team_name.clone(), 60);
    println!("team_name: {team_name}");

    //iterating over hashmap:
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // counting words in a string
    let text = "hello world hello";
    let map = counting_words(text);
    println!("map: {map:?}");
}

fn counting_words(text: &str) -> HashMap<&str, u32> {
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    map
}
