use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

/* Attribute-like macros
#[route(GET, "/")]
fn index() {}

#[proc_macro_attribute]
pub fn route(
    attr: TokenStream, // GET, "/"
    item: TokenStream, // fn index() { }
) -> TokenStream {
}
*/

/* Function-like macros
let sql = sql!(SELECT * FROM posts WHERE id=1);

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {}
*/

fn main() {
    // Declarative Macros with macro_rules! for General Metaprogramming
    let v: Vec<u32> = vec![1, 2, 3];
    let v: Vec<&str> = vec!["a", "b", "c", "d"];

    Pancakes::hello_macro();
}
