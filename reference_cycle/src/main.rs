use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

// List class with a tail method
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // Creating a reference cycle with the List enum

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a = {:?}", a);
    println!("a rc count = {}", Rc::strong_count(&a));
    println!("a.tail = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("b = {:?}", b);
    println!("a rc count = {}", Rc::strong_count(&a));
    println!("b.tail = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count = {}", Rc::strong_count(&b));
    println!("a rc count = {}", Rc::strong_count(&a));

    // Next line will cause stack overflow
    // println!("a.tail = {:?}", a.tail());

    // Demonstrate weak references with a tree data structure

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

     *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // Now let's pay attention to reference counts
    // when branch goes in and out of scope

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {} leaf weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {} weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch));

        println!(
            "leaf strong = {} leaf weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf));
    }

    println!("leaf.parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {} leaf weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf));
}
