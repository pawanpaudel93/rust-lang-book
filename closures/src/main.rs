use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Adding optional type annotations of the parameter and return value types in the closure
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let s = expensive_closure(5);
    println!("{}", s);

    // Capturing References or Moving Ownership
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // Defining and calling a closure that captures a mutable reference

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);

    // If you want to force the closure to take ownership of the values it uses in the environment even though the body of the closure doesn’t strictly need ownership, you can use the move keyword before the parameter list.
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);

    // FnOnce, FnMut, Fn
}
