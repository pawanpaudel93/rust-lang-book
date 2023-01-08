struct Point<T, U> {
    x: T,
    y: U,
}

struct Pointt<T> {
    x: T,
    y: T,
}

impl<T> Pointt<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// An impl block that only applies to a struct with a particular concrete type for the generic type parameter T
impl Pointt<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Pointtt<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Pointtt<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Pointtt<X2, Y2>) -> Pointtt<X1, Y2> {
        Pointtt {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = get_largest(&number_list);

    println!("The largest number is {}", largest);

    // A Point<T> struct that holds x and y values of type T
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    // In Enum Definitions

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // In Method Definitions
    let p = Pointt { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    // A method that uses generic types different from its struct’s definition
    let p1 = Pointtt { x: 5, y: 10.4 };
    let p2 = Pointtt { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn get_largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
