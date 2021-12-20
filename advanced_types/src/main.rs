use std::fmt;
use std::io::Error;

fn main() {
    // Can type alias primitive types, but this won't give you type checking
    type Kilometers = i32;

    let x: Kilometers = 3;
    let y: i32 = 8;
    println!("x + y is {}", x + y);

    let _f: Thunk = Box::new(|| println!("hi!"));

    // Compiler error, `str` does not have a known size at compile time
    // let s1: str = "Hello there!";
}


// Use a type alias for repetitive types
type Thunk = Box<dyn Fn() + Send + 'static>;
fn takes_long_type(f: Thunk) {}
fn returns_long_type() -> Thunk {
    Box::new(|| println!("hi!"))
}

// Use type alias to shorten long generic types
type Result<T> = std::result::Result<T, std::io::Error>;
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<usize>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn flush_all(&mut self) -> Result<usize>;
}

// `!` indicates a function will never return
fn never_returns() -> ! {
    panic!("never returns!");
}

// `Sized` trait restricts generics to only fixed-size types
fn generic<T: Sized>(t: T) { /* ... */ }
// By default, all generics must be fixed-size, but we can relax that
// requirement using this syntax
fn generic_relaxed<T: ?Sized>(t: &T) { /* ... */ }
