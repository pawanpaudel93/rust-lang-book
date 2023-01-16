use core::slice;

extern "C" {
    fn abs(input: i32) -> i32;
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // using safe rust
    // let len = values.len();
    // assert!(mid <= len);
    // (&mut values[..mid], &mut values[mid..])

    // using unsafe rust
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

static HELLO_WORLD: &str = "Hello, World";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    // Creating raw pointers from references
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Calling an Unsafe Function or Method
    unsafe fn dangerous() {}

    unsafe { dangerous() }

    // Creating a Safe Abstraction over Unsafe Code
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // Using extern Functions to Call External Code
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Accessing or Modifying a Mutable Static Variable
    println!("name is : {}", HELLO_WORLD);
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // Implementing Unsafe Trait
    // We can use unsafe to implement an unsafe trait. A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify.
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }

    /* Accessing Fields of a Union
        A union is similar to a struct, but only one declared field is used in a particular instance at one time.
        Unions are primarily used to interface with unions in C code. Accessing union fields is unsafe because Rust can’t guarantee the type of the data currently being stored in the union instance.
    */
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a RUST function from C")
}
