use std::ops::Deref;

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Also works with smart pointer
    let y = Box::new(x);
    assert_eq!(5, *y);

    let y = MyBox::new(x);
    // Implicity calls y.dref()
    assert_eq!(5, *y);

    // Deref coercion derefs MyBox -> String -> &str automatically
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // Without deref coercion we'd need to do something like this
    hello(&(*m));
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> { MyBox(x) }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { &self.0 }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}
