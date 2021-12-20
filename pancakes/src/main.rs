use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
  fn hello_macro() {
    println!("Hello, Macro! My name is Pancakes");
  }
}

#[derive(HelloMacro)]
struct Foobar;

fn main() {
    Pancakes::hello_macro();
    Foobar::hello_macro();
}
