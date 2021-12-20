fn main() {
    let numbers = vec![10, 20, 30, 50, 40];
    println!("largest number {}", largest_i32(&numbers));

    let chars = vec!['a', 'y', 'x', 'b'];
    println!("largest char {}", largest_char(&chars));

    let p = Point{x: 10, y: 20};
    let p = Point{x: 10.0, y: 20.0};
    println!("p.x is {}", p.x());
    println!("p distance from origin {}", p.distance_from_origin());
    // Compiler error, cannot use two different types
    // let p = Point{x: 10, y: 20.0};

    let p1 = Point2{x: 10, y: 20.0};

    let p2 = Point2{x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {} p3.y = {}", p3.x, p3.y);
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Function with type parameter, T
// T is missing std::cmp::PartialOrd trait, so > does not work
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

struct Point<T> {x: T, y: T}

struct Point2<T, U> {x: T, y: U}

// Implement generic x method for Point
impl<T> Point<T> {
    fn x(&self) -> &T { &self.x }
}

// Can implement concreate types using
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2{x: self.x, y: other.y}
    }
}
