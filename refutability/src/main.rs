fn main() {
    // Compiler error, `let` must use an irrefutable pattern
    // let Some(x) = Some(3);

    // Refutable patterns can be used with `if let`
    if let Some(x) = Some(3) {
        println!("x is {}", x);
    }

    // `if let` can use irrefutable patterns but it throws a compiler warning
    if let x = 5 {
        println!("x is {}", x);
    }
}
