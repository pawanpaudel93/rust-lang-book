fn variables_constants() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const SUBSCRIBER_COUNT: u32 = 100_000;
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // x is 12
    }

    println!("The value of x is: {x}"); // x is 6 when the scope is over and inner shadowing ends and x returns to being 6
}

fn data_types() {
    // Data Types - Scalar Types
    // Integers
    let a = 98_222; // Decimal
    let b = 0xfff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte

    // Floating-point numbers
    let f = 2.0;
    let g: f32 = 3.0;

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // Booleans
    let t = true;
    let f = false;

    // Character
    let c = 'z';
    let z = 'Z';

    // Data Types - Compound types
    // tuple
    let tup = ("Let's Get Rusty", 10_000);
    let (channel, sub_count) = tup; //destructure
    let sub_count = tup.1;

    // array
    let error_codes = [200, 404, 500]; // array are fixed size in rust use vector for dynamic size
    let not_found = error_codes[1];

    let byte = [0; 8]; // array of length 8 containing all elements as 0
}

fn control_flows() {
    // Control Flow
    // if-else
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // using if else to return value in single line
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // loop with return value
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };
    println!("The result is {}", result);

    // while
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in 0..4 {
        println!("The value is: {}", number);
    }
}

fn main() {
    variables_constants();
    shadowing();
    data_types();
    control_flows();
}
