use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
struct Pancakes;


#[derive(HelloMacro)]
struct DrivedPancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
    DrivedPancakes::hello_macro();
}