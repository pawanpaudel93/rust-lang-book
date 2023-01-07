use rand::Rng;
// Specifying a nested path to bring multiple items with the same prefix into scope
// use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;

// Combining the paths into one use statement. This line brings std::io and std::io::Write into scope.
// use std::io::{self, Write};

// This use statement brings all public items defined in std::collections into the current scope.
// use std::collections::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
