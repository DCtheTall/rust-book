fn main() {
    println!("answer is {}", do_twice(add_one, 5));

    // Use a function pointer as a parameter to `map`
    let v = vec![1, 2, 3];
    let _v_str: Vec<String> = v.iter().map(ToString::to_string).collect();

    // You can exploit how enums are implemented to instantiate a number of
    // variants
    enum Status {
        Value(u32),
        Stop,
    }
    let _v: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

fn add_one(x: i32) -> i32 { x + 1 }

// This function takes a function pointer as a parameter
fn do_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    return f(x) + f(x);
}

// Compiler error, closures have unknown size at compile time
// fn returns_closure() -> dyn Fn(i32) -> i32 {
//     |x| x + 1
// }

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
