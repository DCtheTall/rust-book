fn main() {
    // You can use `match` for literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => println!("anything"),
    }

    let x = Some(10);
    let y = 5;
    match x {
        Some(50) => println!("x is 50"),
        // Introduces a new variable, `y`, so this pattern matches any Option
        // with a value
        Some(y) => println!("x = y, y is {:?}", y),
        _ => println!("x is {:?}", x),
    }

    // Can match multiple patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Match a range of values
    let x = 3;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("anything"),
    }
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII value"),
        'k'..='z' => println!("later ASCII value"),
        _ => println!("anything else"),
    }

    // Destructuring
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point{x: 3, y: 5};
    let Point{x: a, y: b} = p;
    assert_eq!(3, a);
    assert_eq!(5, b);
    let Point{x, y} = p;
    assert_eq!(3, x);
    assert_eq!(5, y);

    // `match` can also use destructuring
    let p = Point{x: 0, y: 7};
    match p {
        Point{x, y: 0} => println!("on the x-axis at {}", x),
        Point{x: 0, y} => println!("on the y-axis at {}", y),
        Point{x, y} => println!("not on any axis, p = ({}, {})", x, y),
    }

    // Destructuring enums
    enum Message {
        Quit,
        Move{x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => println!("Quit variant has not data to destructure"),
        Message::Move{x, y} => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("ChangeColor to ({}, {}, {})", r, g, b);
        },
    }

    // Destructuring nested structs
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message2 {
        Quit,
        Move{x: i32, y: i32},
        Write(String),
        ChangeColor(Color),
    }
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("ChangeColor r = {} g = {} b = {}", r, g, b);
        },
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("ChangeColor h = {} s = {} v = {}", h, s, v);
        },
        _ => (),
    }

    // Destructuring struct and tuple simultaneously
    let ((feet, inches), Point{x, y}) = ((3, 10), Point{x: 3, y: -10});

    foo(3, 4);

    // You can ignore values in `match` expressions
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Cannot overwrite existing setting"),
        _ => {
            setting_value = new_setting_value;
        },
    }

    // You can also ignore some values of tuples using `_`
    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (first, _, third, _, fifth) => {
            println!(
                "first is {} third is {} fifth is {}", first, third, fifth);
        },
    }

    // Ignore unused variables by prefixing their name with `_`
    let _x = 5;
    let y = 3;

    // Compiler error, `_s` takes ownership of the String
    // let s = Some(String::from("hello"));
    // if let Some(_s) = s {
    //     println!("found a string");
    // }
    // println!("{:?}", s);

    // This is ok
    let s = Some(String::from("hello"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    // Use `..` to ignore other fields
    struct Point2 {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point2{x: 0, y: 0, z: 0};
    match origin {
        Point2{x, ..} => println!("x is {}", x),
    }

    // Can also use `..` for tuples
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => println!("first is {} last is {}", first, last),
    }

    // Compiler error, `..` can only be used once per tuple pattern
    // match numbers {
    //     (.., second, ..) => println!("second is {}", second),
    // }

    // Can add extra conditionals to `match`
    let x = Some(4);
    match x {
        Some(x) if x < 5 => println!("{} is less than 5", x),
        Some(x) => println!("x is {}", x),
        None => (),
    }

    // Match conditionals reuse variables from outer scope
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("x is 50"),
        Some(n) if n == y => println!("x = y, y is {}", y),
        _ => println!("Default case, x is {:?}", x),
    }

    // Match conditionals use the pattern then the conditional
    // Below the logic is `(4 | 5 | 6) if y` not `4 | 5 | (6 if y)`
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // `@` lets us bind struct fields to variables while testing them
    enum Message3 {
        Hello{id: i32},
    }
    let msg = Message3::Hello{id: 3};
    match msg {
        Message3::Hello{
            id: id_variable @ 3..=7,
        } => println!("Found id in range: {}", id_variable),
        Message3::Hello{id: 10..=12} => {
            println!("Found an id in another range");
        },
        Message3::Hello{id} => println!("Found other id: {}", id),
    }
}

// You can ignore function parameters my naming them `_`
fn foo(_: i32, y: i32) {
    println!("Code only uses y = {}", y);
}
