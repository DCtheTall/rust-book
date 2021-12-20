fn main() {
    let _guess: u32 = "42".parse().expect("Not a number!");
    // Compiler error if uncommented
    // let _guess = "42".parse().expect("Not a number!");

    let x = 2.0; // f64
    println!("x is {}", x);

    let y: f32 = 3.0;
    println!("y is {}", y);

    println!("10 + 5 is {}", 10 + 5);
    println!("95.5 - 4.3 is {}", 95.5 - 4.3);
    println!("56.7 / 32.2 is {}", 56.7 / 32.2);
    println!("43 % 5 is {}", 43 % 5);

    // Booleans
    let _t = true;
    let _f: bool = false;

    // Chars
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Destructuring tuples
    let (_x, y, _z) = tup;
    println!("y is {}", y);
    // Accessing tuple elements
    let y = tup.1;
    println!("y is {}", y);

    // Arrays
    let _a = [1, 2, 3, 4, 5];
    // Fixed size array
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    // Array of 5 3's.
    let _a = [3; 5];
    // Access elements
    let _a = [1, 2, 3, 4, 5];
    let first = _a[0];
    println!("first is {}", first);
    let second = _a[1];
    println!("second is {}", second);
}
