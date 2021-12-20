use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::fmt::Display;

fn main() {
    let tweet = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, you probably already know"),
        reply: true,
        retweet: false,
    };
    println!("{}", tweet.summarize());

    let article = NewsArticle{
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("{}", article.summarize());
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub location: String,
    pub content: String,
}

// Some traits contain default implementations
pub trait Summary2 {
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

// You do no need to implement methods of the trait with default implementations
impl Summary2 for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

// Traits as function parameters
fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

// The above is syntactic sugar for the following code
fn notify2<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// The generic type syntax makes the following more concise
fn notify3<T: Summary>(item1: &T, item2: &T) {
    println!("{}\n{}", item1.summarize(), item2.summarize());
}

// This function only takes types that implement the Summary and Display traits
fn notify4(item: &(impl Summary + Display)) {}

// Alternate syntax
fn notify5<T: Summary + Display>(item: &T) {}

// For two type parameters you can use
fn some_function<T: Display + Clone, U: Clone + Debug>(item1: &T, item2: &U) -> i32 { 0 }

// Can also use the `where` syntax
fn some_function2<T, U>(item1: &T, item2: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{ 0 }

// Can also use `impl` in return types
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// Compiler error, `impl` in return type doesn't allow you to return multiple types
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

// Implementing `largest` function from `generics/`
fn largest<T: Copy + PartialOrd>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Pair<T> {
    x: T,
    y: T,
}

// Implements `new` for Pair which contains any type
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Pair<T> {
        Pair{x: x, y: y}
    }
}

// Implements `cmp_display` for Pair only if T implements PartialOrd and Display
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("largest value is {}", self.x);
        } else {
            println!("largest value is {}", self.y);
        }
    }
}

// Can also conditionally implement traits for types that only implement certain traits
// impl<T: Display> ToString for T {}
