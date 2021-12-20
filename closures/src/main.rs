use std::thread;
use std::time::Duration;

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout4(simulated_intensity, simulated_random_number);

    // All equivalent
    fn add_one_v1(x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // Can't compile the last two until the closures are invoked
    // let add_one_v3 = |x| { x + 1 };
    // let add_one_v4 = |x| x + 1;

    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // Compiler error, `x` has an inferred type of String from the line above
    // let n = example_closure(5);

    // This code will panic since Cacher can only cache one result
    // To fix this Cacher would need to store a hashmap of args -> results
    // let mut c = Cacher::new(|a| a);
    // let v1 = c.value(1);
    // let v2 = c.value(2);
    // assert_eq!(v2, 2);

    // Closures can use variables from their parent scope
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    // Compiler error, functions cannot use variables in the parent scope
    // fn equal_to_x(z: u32) -> bool { z == x }
    // assert!(equal_to_x(y));

    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // Compiler error, `equal_to_x` now owns `x`
    // println!("{:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// Calls expensive function twice, too slow
fn generated_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity));
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity));
        }
    }
}

// Refactor above function to cache result of expensive calculation
// But sometimes this function does not use the result
fn generated_workout2(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}

// Refactor `generate_workout` to use a closure instead
fn generated_workout3(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

// Structs with closure fields need to use generics/traits
struct Cacher<T> where T: Fn(u32) -> u32 {
    calculate: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculate: T) -> Cacher<T> {
        Cacher{calculate, value: None}
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculate)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// Use Cacher for generate_workout
fn generate_workout4(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
