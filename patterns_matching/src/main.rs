fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message1 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Message2 {
    Hello { id: i32 },
}

fn main() {
    // match Arms
    let x = Some(1);
    match x {
        None => None,
        Some(i) => Some(i + 1),
    };

    //Conditional if let Expressions
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let Conditional Loops
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for Loops
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let Statements
    let x = 5;

    // let PATTERN = EXPRESSION;
    let (x, y, z) = (1, 2, 3);

    // Function Parameters
    let point = (3, 4);
    print_coordinates(&point);

    // >> Refutability: Whether a Pattern Might Fail to Match
    // Patterns come in two forms: refutable and irrefutable. Patterns that will match for any possible value passed are irrefutable.

    // Irrefutable
    let x = 5;

    // refutable
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("{}", x);
    }

    // >> Pattern Syntax
    // Matching Literals

    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Named Variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    // Multiple Patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Ranges of Values with ..=
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // >> Destructuring to Break Apart Values
    // Destructuring Structs
    let p = Point { x: 0, y: 7, z: 0 };

    let Point { x: a, y: b, z: c } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p = Point { x: 0, y: 7, z: 1 };

    match p {
        Point { x, y: 0, z: 1 } => println!("On the x axis at {x}"),
        Point { x: 0, y, z } => println!("On the y axis at {y}"),
        Point { x, y, z } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // Destructuring Enums
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }

    // Destructuring Nested Structs and Enums
    let msg = Message1::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message1::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message1::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }

    // Destructuring Structs and Tuples
    let ((feet, inches), Point { x, y, z }) = ((3, 10), Point { x: 3, y: -10, z: 1 });

    // Ignoring Values in a Pattern
    foo(3, 4);

    // Ignoring Parts of a Value with a Nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    // Ignoring multiple parts of a tuple
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    // Ignoring an Unused Variable by Starting Its Name with _
    let s = Some(String::from("Hello!"));

    // if let Some(_s) = s {
    if let Some(_) = s {
        // Note that there is a subtle difference between using only _ and using a name that starts with an underscore. The syntax _x still binds the value to the variable, whereas _ doesn’t bind at all.
        println!("found a string");
    }

    println!("{:?}", s);

    // Ignoring Remaining Parts of a Value with ..
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // Extra Conditionals with Match Guards
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ Bindings
    let msg = Message2::Hello { id: 5 };
    match msg {
        Message2::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message2::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message2::Hello { id } => println!("Found some other id: {}", id),
    }
}
