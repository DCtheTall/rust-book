use std::mem::drop;

fn main() {
    let c = CustomSmartPointer{data: String::from("stuff")};
    let d = CustomSmartPointer{data: String::from("other stuff")};
    println!("Created smart pointers");
    drop(c);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
