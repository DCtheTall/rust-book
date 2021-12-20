// `use` to import individual enums and structs is idiomatic
use std::collections::HashMap;
// The exception is libraries which define structs with the same name
// use std::fmt;
// use std::io;
// Could also use an alias
use std::fmt::Result;
use std::io::Result as IoResult;

use rand::Rng;

// Instead of
// use std::cmp::Ordering;
// use std::io;
// You can do
use std::{cmp::Ordering, fmt};

// Instead of
// use std::io;
// use std::io::Write;
// You can do, `self` imports `std::io`
use std::io::{self, Write};

// Can bring in all public members of a module
use std::collections::*;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("secret number is {}", secret_number);
}

// Compiler error if uncommented
// fn function1() -> fmt::Result {}
// fn function2() -> io::Result<()> {}
