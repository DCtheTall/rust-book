use std::fmt::Display;

fn main() {
    // Compiler error, r tries to use a value it borrowed after it was dropped
    // let r: &i32;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("{}", r);

    // This code works
    {
        let r: &i32;
        let x = 5;
        r = &x;
        println!("{}", r);
    }

    // This works since string1, string2, and result are in the same scope
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // This works since string1 is in the outer scope and string2/result are both in the inner scope
    let string1 = String::from("abcd");
    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    // Compiler error, result lives longer than string2 and result might borrow string2's value
    // let string1 = String::from("abcd");
    // let result: &str;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str()); 
    // }
    // println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // `static` lifetime lasts entire program, string literals have this lifetime by default
    let s: &'static str = "Hello, world!";
}

// Compiler error, needs a lifetime parameter
// Rust can't tell if the function will return a reference to `a` or `b`
// fn longest(a: &str, b: &str) -> &str {
//     if a.len() > b.len() {
//         a
//     } else {
//         b
//     }
// }

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

// Compiler error, result.as_str() borrows from a value in the function's scope
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// Structs that borrow values must include lifetime parameters
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Functions with only 1 parameter do not need lifetime annotations
fn first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Lifetime annotations for methods
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 { 3 }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// function using generics, lifetime parameters, and trait bounding
fn longest_with_announcement<'a, T>(a: &'a str, b: &'a str, ann: T) -> &'a str where T: Display {
    println!("Attention please: {}", ann);
    if (a.len() > b.len()) {
        return a;
    }
    b
}
