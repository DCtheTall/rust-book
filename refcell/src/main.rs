use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Compiler error, cannot make a mutable borrow of an immutable variable
    // let x = 5;
    // let y = &mut x;

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::clone(&a)));
    let c = Rc::new(Cons(Rc::new(RefCell::new(4)), Rc::clone(&a)));
    *value.borrow_mut() += 10;
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
}

// Use Rc and RefCell to have mutable data that can have multiple owners
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}