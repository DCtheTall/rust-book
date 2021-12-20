// Main script for a package library

mod front_of_house {
    // Public submodule
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    // Private submodule
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::seat_at_table();
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        // Relative path of function in the same module
        cook_order();

        // Relative path of function in parent module
        super::serve_order();
    }
    
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // If an enum is public, all its variants are public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant2() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Compiler error, seasonal_fruit is private
    // meal.seasonal_fruit = String::from("blueberries");
}

// `use` let's us shorten the path to functions.
use crate::front_of_house::hosting;
// Can also use `self` to do relative imports with `use`
// use self::front_of_house::hosting;
// You could do this, but it's idiomatic to import modules, not individual functions
// use self::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant3() {
    hosting::add_to_waitlist();
}

// Can use `pub` keyword to make functions in this module available to users of this library
// pub use crate::front_of_house::hosting;
