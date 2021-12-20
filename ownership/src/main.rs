fn main() {
    // Immutable
    let _s1: String = String::from("Hello");
    let _s1: &str = "Hello";

    // Can be mutated
    let mut s2 = String::from("Hello");
    s2.push_str(", world!");
    println!("{}", s2);

    {
        let _s3 = String::from("hello");
    } // scope is no longer valid, _s3.drop() implicitly called

    // Move the string from s3 to s4
    let _s3 = String::from("hello");
    let _s4 = _s3;
    // Compiler error if uncommented, since s3 no longer owns the string.
    // println!("{}, world!", _s3);

    let s5 = String::from("hello");
    let s6 = s5.clone();
    println!("s5 is {}, s6 is {}", s5, s6);

    // The following code is valid with integers since they are stored on the stack
    let x = 5;
    let y = x;
    println!("x is {}, y is {}", x, y);

    let s7 = String::from("hello");
    takes_ownership(s7);
    // Compiler error if uncommented
    // println!("{}", s7);

    let x = 5;
    makes_copy(x);
    println!("x is {}", x);

    let s8 = gives_ownership();
    println!("{}", s8);

    let s9 = String::from("hello");
    let s10 = takes_and_gives_back_ownership(s9);
    // Uncomment for compiler error
    // println!("{}", s9);
    println!("{}", s10);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(i: i32) {
    println!("{}", i);
}

fn gives_ownership() -> String {
    let result = String::from("hello");
    result
}

fn takes_and_gives_back_ownership(s: String) -> String { s }
