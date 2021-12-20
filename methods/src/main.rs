fn main() {
    // Compute the area of a rectangle
    let width = 50;
    let height = 30;
    println!("The area of the rectangle is {}", area1(width, height));

    // Could also use tuples
    let dims = (30, 50);
    println!("The area of the rectangle is {}", area2(dims));

    let rect = Rectangle{width: 50, height: 30};
    println!("The area of the rectangle is {}", area3(&rect));

    println!("rect {:?}", rect);
    // Multiline syntax
    println!("rect {:#?}", rect);

    println!("The area of the rectangle is {}", rect.area());

    let rect2 = Rectangle{width: 40, height: 20};
    let rect3 = Rectangle{width: 30, height: 60};
    println!("{} {}", rect.can_hold(&rect2), rect.can_hold(&rect3));

    let sq = Rectangle::square(3);
}

fn area1(width: i32, height: i32) -> i32 { width * height }

fn area2(dims: (i32, i32)) -> i32 { dims.0 * dims.1 }

// first line allows us to print the struct as a debug string
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

fn area3(rect: &Rectangle) -> i32 { rect.width * rect.height }

// Implementing area() method
// Structs can have multiple `impl` blocks
impl Rectangle {
    // Borrows an immutable reference to the instance
    // Methods can also take ownership or borrow mutable references
    fn area(&self) -> i32 { self.width * self.height }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function, like static methods in other languages
    fn square(size: i32) -> Rectangle {
        Rectangle{width: size, height: size}
    }
}
