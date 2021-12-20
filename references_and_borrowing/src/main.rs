fn main() {
    let s1 = String::from("hello");
    let len = string_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    let mut s = String::from("hello");
    change_string(&mut s);
    println!("{}", s);

    // Compiler error below, can only have one mutable reference at a time
    // Rust does this to prevent data races
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{} {}", r1, r2);

    // Multiple immutable borrows are ok
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);

    // You can make a mutable borrow after making immutable ones
    let r3 = &mut s;
    println!("{}", r3);

    // Compiler error, you cannot make an immutable borrow to an object
    // after making a mutable borrow from it
    // println!("{} {} {}", r1, r2, r3);
}

fn string_length(s: &String) -> usize { s.len() }

// This function causes compiler error since it modifies a reference
// Rust references are constant by default
// fn change_string(s: &String) {
//     s.push_str(", world!");
// }

fn change_string(s: &mut String) {
    s.push_str(", world!");
}

// This function causes a compiler error since it returns a reference to an object
// whose value went out of scope before the reference.
// Rust does this to prevent dangling pointers
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
