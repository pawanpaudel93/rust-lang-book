#[derive(Debug)]

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // The &self is actually short for self: &Self.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    structs_basics();

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square = Rectangle::square(10);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect3));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn structs_basics() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // let name = user1.username;
    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(String::from("pawa@email.com"), String::from("pawan"));

    let user3 = User {
        active: user2.active,
        username: user2.username,
        email: String::from("another@example.com"),
        sign_in_count: user2.sign_in_count,
    };

    // Using Tuple Structs without Named Fields to Create Different Types
    // Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples
    // tuple struct instances are similar to tuples in that you can destructure them into their individual pieces, and you can use a . followed by the index to access an individual value.
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Color(value_1, value_2, value_3) = black;
    let value_1 = black.0;

    println!("user1: {:?}, user3: {:?}", user1, user3);
}

// A build_user function that takes an email and username and returns a User instance
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
