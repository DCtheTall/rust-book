fn main() {
    let w = 5;
    println!("w is {}", w);
    // Compiler error if below is uncommented.
    // w = 6;

    // Mutable variable
    let mut x = 4;
    println!("x is {}", x);
    x = 5;
    println!("x is {}", x);

    // Shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("y is {}", y);

    // This code works
    let spaces = "     ";
    let spaces = spaces.len();
    // This will cause compiler error.
    // let spaces = "    ";
    // spaces = spaces.len();
    println!("spaces {}", spaces);
}
