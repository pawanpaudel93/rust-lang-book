fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn do_thrice<T>(f: T, arg: i32) -> i32
where
    T: Fn(i32) -> i32,
{
    f(arg) + f(arg) + f(arg)
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

// Returning Closures
fn returns_closure(a: i32) -> Box<dyn Fn(i32) -> i32> {
    // |x| x + 1
    if a > 0 {
        Box::new(move |b: i32| a + b)
    } else {
        Box::new(move |b: i32| a - b)
    }
}

fn main() {
    // Function Pointers
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    let answer = do_thrice(add_one, 5);
    println!("The answer is: {}", answer);

    let answer = do_thrice(|x| x + 1, 5);
    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("{:?}", list_of_strings);

    // Or we could name a function as the argument to map instead of the closure, like this:
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("{:?}", list_of_strings);

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("{:?}", list_of_statuses);
}
