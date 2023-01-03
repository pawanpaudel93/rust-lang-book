fn main() {
    /* Ownership Rules

        Each value in Rust has an owner.
        There can only be one owner at a time.
        When the owner goes out of scope, the value will be dropped.
    */
    // variable scope
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // Memory and Allocation
    // Variables and Data Interacting with Move
    let x = 5;
    let y = x; // Copy

    // Now let’s look at the String version:
    let s1 = String::from("hello");
    let s2 = s1.clone(); // copies both stack and heap data of string
    let s3 = s1; // s1 moved to s3 and s1 becomes invalid as both cannot free same memory when scope ends
    println!("{} {}", s2, s3);

    // Ownership and Functions
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function and so is no longer valid here
    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still use x afterward

    // Return Values and Scope
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    println!("s1 = {}, s3 = {}", s1, s3);

    /* The Rules of References

        At any given time, you can have either one mutable reference or any number of immutable references.
        References must always be valid.
    */
    // References and Borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    // Mutable references
    let mut s = String::from("hello");
    change(&mut s);
    println!("The changed string is: {}", s);

    // Dangling references
    // let reference_to_nothing = dangle();

    // The Slice Type
    let s = String::from("Hello world"); // word will get the value 5
    let s2 = "hello world";
    let word = first_word(&s);
    let word1 = first_word(&s2);
    println!("{}, {}", word, word1);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// fn dangle() -> &String {
//     let s = String::from("hell0");
//     &s
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
