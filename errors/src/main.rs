use std::{
    fs,
    fs::File,
    io::{Error, ErrorKind, Read},
    net::IpAddr,
};

fn main() {
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // Alternative
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Shortcuts for Panic on Error: unwrap and expect
    let file = File::open("hello.txt").unwrap(); // unwrap file else panic if file absent
    let file = File::open("hello.txt").expect("hello.txt shouldn't be included in this project");

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
}

// Propagating Errors
fn read_username_from_file() -> Result<String, Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// A Shortcut for Propagating Errors: the ? Operator
fn read_username_from_file_alternative() -> Result<String, Error> {
    let mut username_file_result = File::open("hello.txt")?;
    let mut username = String::new();
    username_file_result.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_chaining() -> Result<String, Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// Using fs::read_to_string instead of opening and then reading the file
fn read_username_from_file_simpler() -> Result<String, Error> {
    fs::read_to_string("hello.txt")
}

// Using the ? operator on an Option<T> value
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// Changing main to return Result<(), E> allows the use of the ? operator on Result values
/*
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
*/
// For now, you can read Box<dyn Error> to mean “any kind of error.” Using ? on a Result value in a main function with the error type Box<dyn Error> is allowed, because it allows any Err value to be returned early. Even though the body of this main function will only ever return errors of type std::io::Error, by specifying Box<dyn Error>, this signature will continue to be correct even if more code that returns other errors is added to the body of main.

// A Guess type that will only continue with values between 1 and 100
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
