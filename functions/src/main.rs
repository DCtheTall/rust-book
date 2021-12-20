fn main() {
    println!("Hello, world!");

    let _y = 6;
    // Uncomment for compiler error.
    // let x = (let y = 6);
    let y = {
        let x = 3;
        // No semicolon in last expression
        x + 1
    };

    let x = five();
    let x = plus_one(x);

    another_function(x, y);
}

// Parameters
fn another_function(x: i32, y: i32) {
    println!("x is {}", x);
    println!("y is {}", y);
}

// Return values
// Can omit "return" and ";" in last expression.
fn five() -> i32 { 5 }

// Putting the semicolon after "x + 1" will result in compiler error
// since statements do not evaluate to a value
fn plus_one(x: i32) -> i32 { x + 1 }
