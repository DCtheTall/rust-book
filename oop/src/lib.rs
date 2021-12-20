// Example of using `struct` and `impl` in Rust to define a type of object

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(v) => {
                self.update_average();
                Some(v)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 { self.average }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// Example of using traits to define common behavior between objects

pub trait Draw {
    fn draw(&self);
}

// Box<dyn Draw> is a Box containing any object that implements the `Draw`
// trait, `dyn Draw` is a trait object
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
    // Rust knows each element in the vector has a `draw` method
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Implementing a trait for an object
pub struct Button {
    width: u32,
    height: u32,
    label: String,
}
impl Draw for Button {
    fn draw(&self) {
        // Code to draw a button on the GUI...
    }
}

// Compiler error, some traits cannot be used as trait objects
// pub trait Clone {
//     fn clone(&self) -> Self;
// }
// pub struct Screen2 {
//     components: Vec<Box<dyn Clone>>,
// }
