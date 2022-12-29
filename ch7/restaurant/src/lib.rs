mod front_of_house;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

mod customer {
    // the "use" statement below doesn't apply to the customer module
    // so we need to define it again in the module
    use crate::front_of_house::hosting;

    pub fn _eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

// non-idiomatic to specify function name in use path
// use crate::front_of_house::hosting::add_to_waitlist;

// idiomatic to specify structs, enums, etc. in use path
// use crate::back_of_house::Appetizer;
// use crate::back_of_house::Breakfast;

use back_of_house::Appetizer;
pub use front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // public structs and enums example
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please", meal.toast);

    println!("with some {:?} and {:?}", Appetizer::Soup, Appetizer::Salad);

    // with use
    hosting::add_to_waitlist();
}

// use "as" to alias names that may conflict
// use std::fmt::Result;
// use std::io::Result as IoResult;

// bring in items in nested paths using one line
// use std::{fmt::Result, io::Result as IoResult};

// using different levels
// use std::io;
// use std::io::Write;
// -> use std::io::{self, Write};

// glob to bring all public items
// use std::io::*;
