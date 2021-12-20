fn main() {
    // Using a Box to store an i32
    let b = Box::new(5);
    println!("b = {}", b);

    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// Compiler error, compiler cannot determine size of this struct
// enum List {
//     Cons(i32, List),
//     Nil,
// }

// Box stores its memory on the heap, so it can store a dynamic size of memory
// while only taking a fixed amount on the stack
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};
