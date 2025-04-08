fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddrEnum::V4(127, 0, 0, 1);
    let loopback = IpAddrEnum::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // Options:

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // -> does not compile because x and y are different types

    if y.is_some() {
        println!("y is some");
    } else {
        println!("y is none");
    }

    if absent_number.is_none() {
        println!("absent_number is none");
    } else {
        println!("absent_number is some");
    }

    if y.is_some_and(|n| n > 0) {
        println!("y is some and greater than 0");
    } else {
        println!("y is none or not greater than 0");
    }

    if y.is_none_or(|n| n > 0) {
        println!("y is none or greater than 0");
    } else {
        println!("y is some and not greater than 0");
    }

    match y {
        Some(n) => println!("y: {}", n),
        None => println!("No y"),
    }

    match absent_number {
        Some(n) => println!("absent_number: {}", n),
        None => println!("No absent_number"),
    }

    // matching in general
    println!("\n**** Matching in general ****");

    let coin = Coin::Quarter;

    match coin {
        Coin::Penny => println!("This is a penny"),
        Coin::Nickel => println!("This is a nickel"),
        Coin::Dime => println!("This is a dime"),
        Coin::Quarter => println!("This is a quarter"),
    }

    println!("coin has value of {}", value_in_cents(coin));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five: {:?}", five);
    println!("six: {:?}", six);
    println!("none: {:?}", none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // let else
    fn let_else_some() {
        let config_max = Some(3u8);
        let Some(max) = config_max else {
            println!("No max configured");
            return;
        };
        println!("The maximum is configured to be {max}");
        println!("and imagine this, max + 1 is {}", max + 1);
    }

    fn let_else_none() {
        let coin: Option<Coin> = None;
        let Some(coin) = coin else {
            println!("coin was none");
            return;
        };
        println!("coin was {:?}", coin);
    }

    let_else_some();
    let_else_none();
}

enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {
    println!("Routing the IP address...");
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// these enums are part of the standard library,
// but we can still create and use our own definition because standard library's
// enums are not brought into scope

enum IpAddrEnum {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
} // look at this, different values associated with enums!

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
} // look at this, even functions on Messages!

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
