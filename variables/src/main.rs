use std::io;

fn main() {
    // Does not compile: "cannot assign twice to immutable variable `x`"
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // This compiles: "The value of x is: 5" and "The value of x is: 6"
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // always immutable constants!
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // shadowing:
    println!("***** shadowing *****");

    let x = 5;

    println!("The value of x is: {x}");

    let x = x + 1;

    println!("The value of x before the scope is: {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x in the end is: {x}");

    // two different types!
    let spaces = "   ";
    let spaces = spaces.len();

    // two different types without shadowing, does not work
    // let mut spaces = "   ";
    // spaces = spaces.len();

    //tuples:
    println!("***** tuples *****");
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");

    // arrays
    println!("***** arrays *****");
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {first}");
    println!("The value of second is: {second}");

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // [3, 3, 3, 3, 3]

    let a = [1, 2, 3, 4, 5];

    //println!("test whole array output: {}", &a); // "cannot be formatted with the default formatter"
    println!("test whole array output: {:?}", &a); // "[1, 2, 3, 4, 5]"

    println!("**** array indexing game ****");

    println!("Please enter an array index.");

    let a = [1, 2, 3, 4, 5];

    // this gives compile time error:
    // let test_index: usize = 5;
    // let x = a[test_index]; // "index out of bounds: the length is 5 but the index is 5"

    // this can give runtime error or "panic"
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}
