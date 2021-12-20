use std::fmt;
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/*

Definition of Add trait, uses generic type with default value

trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

*/

#[derive(Debug, Copy, Clone, PartialEq)]
struct Millimeters(u32);

struct Meters(u32);

// Can define Add for Millimeters where the rhs operand's type is Meters
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (1000 * other.0))
    }
}

trait Wizard {
    fn fly(&self);
}

trait Pilot {
    fn fly(&self);
}

struct Human;

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking...");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waves arms furiously");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Implementing a supertrait, any trait that implements OutlinePrint must also
// implement fmt::Display
trait OutlinePrint: fmt::Display {
    fn print_outline(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let p1 = Point{x: 1, y: 0};
    let p2 = Point{x: 2, y: 3};
    assert_eq!(Point{x: 3, y: 3}, p1 + p2);

    assert_eq!(Millimeters(1001), Millimeters(1) + Meters(1));

    // Fully qualified method names to disambiguate methods with the same name
    let person = Human;
    Wizard::fly(&person);
    Pilot::fly(&person);
    person.fly();
    // For static methods...
    println!("{}", Dog::baby_name());
    println!("{}", <Dog as Animal>::baby_name());

    Point{x: 3, y: 5}.print_outline();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
