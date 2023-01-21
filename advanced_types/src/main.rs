fn main() {
    // Creating Type Synonyms with Type Aliases
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // Using long type in many places - Introducing a type alias Thunk to reduce repetition
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        // --snip--
    }

    // The Never Type that Never Returns
    fn bar() -> ! {
        // --snip--
    }

    // Dynamically Sized Types and the Sized Trait
    let s1: &str = "Hello there!";
    let s2: &str = "How's it going?";

    fn generic<T>(t: T) {
        // --snip--
    }
    // is actually treated as though we had written this:

    fn generics<T: Sized>(t: T) {
        // --snip--
    }

    // By default, generic functions will work only on types that have a known size at compile time. However, you can use the following special syntax to relax this restriction:

    fn genericss<T: ?Sized>(t: &T) {
        // --snip--
    }
}
