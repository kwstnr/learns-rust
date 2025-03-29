fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');

    // statements and expressions
    println!("**** Statements and Expressions ****");

    // expected expression, found statement
    // let x = (let y = 6);

    let y = {
        let x = 3;
        x + 1 // expressions don't have ending semicolons, semicolons turn it to a statement
    };

    println!("The value of y is: {y}");

    // function returning a value
    let x = five();
    println!("The value of x (five) is: {x}");

    // function returning a value
    let five = 5;
    let x = plus_one(five);
    println!("The value of x (plus_one) is: {x}");

    // changing variables
    let mut changeable = 5;
    println!("The value of changeable (before) is: {changeable}");
    change(&mut changeable, 5);
    println!("The value of changeable (after) is: {changeable}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn change(x: &mut i32, addition: i32) {
    *x += addition;
}