// enum IpAddrKind {
//     V4,
//     V6,
// }
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
/*
This enum has four variants with different types:

Message enum whose variants each store different amounts and types of values

- Quit has no data associated with it at all.
- Move has named fields like a struct does.
- Write includes a single String.
- ChangeColor includes three i32 values.

*/

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_fn() {
        println!("Let's Learn Rust")
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    let home = IpAddrKind::V4(127, 0, 0, 1);

    let loopback = IpAddrKind::V6(String::from("::1"));

    // Option Enum
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    let some_number = Some(41);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x = 5;
    let y = Some(5);

    let sum = x + y.unwrap_or(0);

    println!("The sum of x + y = {}", sum);

    // The match Control Flow Construct
    value_in_cents(Coin::Quarter(UsState::Alaska));

    // Matching with Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Catch-all Patterns and the _ Placeholder
    // We no longer need to use the catch-all value, so we can change our code to use _ instead of the variable named other
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}

    // Concise Control Flow with if let
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // Or we could use an if let and else expression like this:
    /*
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    */
    println!("The final count is: {}", count);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}
