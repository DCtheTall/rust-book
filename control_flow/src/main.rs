fn main() {
    let n = 7;

    if n < 5 {
        println!("n is less than 5");
    } else {
        println!("n is not less than 5");
    }

    // if n {
    //     println!("This will throw a compiler error");
    // }

    if n % 4 == 0 {
        println!("n is a multiple of 4");
    } else if n % 4 == 1 {
        println!("n / 4 has a remainder of 1");
    } else if n % 4 == 2 {
        println!("n / 4 has a remainder of 2");
    } else {
        println!("n / 4 has a remainder of 3");
    }

    // Ternary
    let cond = true;
    let m = if cond { 5 } else { 6 };
    // Compiler error if uncommented
    // let m = if cond { 5 } else { "six" };
    println!("m is {}", m);

    // `loop` keyword
    let mut i = 0;
    loop {
        if i == 10 {
            break;
        }
        i += 1;
    }

    // Can assign using `loop`
    i = 0;
    let result = loop {
        i += 1;
        if i == 10 {
            break i * 2;
        }
    };
    println!("result is {}", result);

    // `while` loop
    i = 3;
    while i != 0 {
        println!("{}!", i);
        i -= 1;
    }
    println!("LIFTOFF!");

    // You can use `while` to iterate through an array
    let arr = [10, 20, 30, 40, 50];
    i = 0;
    while i < 5 {
        println!("arr[{}] is {}", i, arr[i]);
        i += 1;
    }
    // but it is better to use `for`
    for element in arr.iter() {
        println!("value is {}", element);
    }

    // For example the countdown above can be done with `for`
    for n in (1..4).rev() {
        println!("{}!", n);
    }
    println!("LIFTOFF!");
}
