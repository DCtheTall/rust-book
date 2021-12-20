fn main() {
    println!("penny is worth {} cent", value_in_cents(Coin::Penny));
    let quarter = Coin::Quarter(UsState(String::from("Arkansas")));
    println!("query is worth {} cents", value_in_cents(quarter));

    // Match also uses _ as a placeholder for all other variants
    let x = 3u8;
    match x {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // Instead of having to do...
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // ...`if let` is more concise
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let coin = Coin::Penny;

    // Similarly instead of this...
    // (Cannot uncomment since UsState does not implement the `Copy` trait)
    // let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    // ...`if let`
    let mut count = 0;
    let alaska = UsState(String::from("Alaska"));
    if let Coin::Quarter(alaska) = coin {
        println!("State quarter from {:?}!", UsState(String::from("Alaska")));
    } else {
        count += 1;
    }
}

#[derive(Debug)]
struct UsState(String);

impl Copy {
    fn copy(&self) -> UsState { *self }   
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(c: Coin) -> u8 {
    match c {
        Coin::Penny => {
            println!("lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state is {:?}", state);
            25
        },
    }
}

// Operating on an Option type
fn plus_one(x: Option<i8>) -> Option<i8> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// Compiler error, match must handle all variants
// fn plus_one(x: Option<i8>) -> Option<i8> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }
