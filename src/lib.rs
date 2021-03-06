use std::io::{self, Write};

fn serve() {}
mod front_of_house;

// use keyword
// either absolute or relative
// use crate::front_of_house::hosting;
// using pub, external code can reference the hosting module
pub use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

mod back_of_house {
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
}

pub fn eat() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat")
}
