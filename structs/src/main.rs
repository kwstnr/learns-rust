fn main() {
    println!("Hello, structs!");

    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user@example.com"),
        sign_in_count: 1,
    };

   user1.sign_in_count = 2;

    let user2 = build_user(String::from("user2"), String::from("user2"));

    println!("User 1: {} - {}", user1.username, user1.email);
    println!("User 2: {} - {}", user2.username, user2.email);

    // user2.sign_in_count = 3; // -> not mutable

    let user3 = User {
        username: String::from("user3"),
        email: String::from("user@example.com"),
        active: user2.active,
        sign_in_count: user2.sign_in_count,
    };

    let user4 = User {
        username: String::from("user4"),
        ..user3
    };

    // println!("User 3: {} - {}", user3.username, user3.email); // -> does not compile, user3.email was moved (its a String object on the heap)!

    // tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("origin: ({}, {}, {})", origin.0, origin.1, origin.2);

    let Point(x, y, z) = origin;

    // unit like structs

    struct AlwaysEqual;

    let subject = AlwaysEqual;

    // Struct Data Ownership
    struct ReferenceUser {
        active: bool,
        // username: &str, // -> not allowed, missing lifetime specifier
        // email: &str, // -> not allowed, missing lifetime specifier
        sign_in_count: u64,
    }

    // exmaple program using structs
    println!("**** Example program using structs ****");

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_dimension(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_rectangle(&rect2)
    );

    // derived traits
    // println!("rect2 is {}", rect2); // -> does not compile, Rectangle does not implement the Display trait

    // we can use {:#?} for pretty-printing the struct
    println!("rect2 is {:#?}", rect2); // -> oh no! does not work before adding the debug trait, rect2 did not implement the Debug trait

    // dbg! macro
    dbg!(&rect2);

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(scale * 30),
        height: 50,
    };

    dbg!(rect3);

    // println!("rect3 is {:#?}", rect3); // -> does not compile, rect3 was moved into the dbg! macro

    let rect4 = Rectangle {
        width: dbg!(scale * 30),
        height: 50,
    };

    dbg!(&rect4); // -> works, rect4 is borrowed into the dbg! macro
    // or
    let rect4 = dbg!(rect4); // -> works, because debug returns ownership
    println!("rect4 is {:#?}", rect4); // -> works, rect4 is borrowed into the dbg! macro

    // methods
    println!("\n**** Methods ****");
    let rect5 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", rect5.area());

    if rect5.width() {
        println!("The rectangle has a nonzero width; it is {}", rect5.width);
    }

    let rect6 = Rectangle {
        width: 10,
        height: 20,
    };

    if rect5.can_hold(&rect6) {
        println!("rect5 can hold rect6");
    } else {
        println!("rect5 cannot hold rect6");
    }

    let square = Rectangle::square(10);
    println!("The area of the square is {} square pixels.", square.area());
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_dimension(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)] // with this we can use pretty-printing
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function, not method!
    // also namespaced!
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn area_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
