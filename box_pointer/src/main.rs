#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};
fn main() {
    // Boxes allow you to store data on the heap rather than the stack.
    // What remains on the stack is the pointer to the heap data.
    let b = Box::new(5);
    println!("b = {}", b);

    // Enabling Recursive Types with Boxes
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}
