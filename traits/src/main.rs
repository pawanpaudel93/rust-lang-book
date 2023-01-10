use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// type T implements the PartialOrd trait that enables comparison and the Display trait that enables printing.
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

/* Blanket Implementation
    We can also conditionally implement a trait for any type that implements another trait.
    Implementations of a trait on any type that satisfies the trait bounds are called blanket
    implementations and are extensively used in the Rust standard library.
    For example, the standard library implements the ToString trait on any type that implements the Display trait.
    The impl block in the standard library looks similar to this code:

    impl<T: Display> ToString for T {
    // --snip--
    }

    Because the standard library has this blanket implementation,
    we can call the to_string method defined by the ToString trait on any type that implements the Display trait.
    For example, we can turn integers into their corresponding String values like this because integers implement Display:

    let s = 3.to_string();
ß*/

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// Trait Bound Syntax
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
// pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// Specifying Multiple Trait Bounds with the + Syntax

// pub fn notify(item: &(impl Summary + Display)) {}
// pub fn notify<T: Summary + Display>(item: &T) {}

// Clearer Trait Bounds with where Clauses

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {}

// Returning Types that Implement Traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    notify(&article);

    returns_summarizable().summarize();
}
