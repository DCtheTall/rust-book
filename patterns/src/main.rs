fn main() {
    // `match` is a pattern
    let opt = Some(5);
    match opt {
        Some(v) => println!("value is {}", v),
        None => panic!("should not happen"),
    }

    // So is `if let`
    if let Some(v) = opt {
        println!("value is {}", v);
    }
    let res: Result<u8, _> = "34".parse();
    if let Ok(v) = res {
        println!("value is {}", v);
    }

    // There is also `while let`
    let mut stack = vec![];
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(v) = stack.pop() {
        println!("top is {}", v);
    }

    // `for` loops use patterns, the syntax is
    // `for PATTERN in EXPRESSION`
    let v = vec!['a', 'b', 'c'];
    for (i, el) in v.iter().enumerate() {
        println!("index {} is {}", i, el);
    }

    // `let` statements also use patterns
    // `let PATTERN = EXPRESSION;`
    let x = 5;
    let (x, y, z) = (1, 2, 3);

    let point = (3, 4);
    print_coordinates(&point);
}

// Function parameters can be patterns too
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("point is ({}, {})", x, y);
}
