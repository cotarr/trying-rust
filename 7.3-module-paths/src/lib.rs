//
// Module scope, how to make public
//
#[allow(dead_code)]
mod front_of_house {
    // Add "pub" keyword to make module hosting public
    pub mod hosting {
        // Add "pub" keyword to function to make name visible outside module
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
   
}
// Public function visible outside library crate
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

//
// Example using "super" to call function in parent function
//
#[allow(dead_code)]
fn deliver_order() {}
#[allow(dead_code)]
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}

//
// Example struct scope (fields private by default)
//
#[allow(dead_code)]
mod back_of_house2 {
    // Define as struct with one public and one private field
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    // The method is public because struct enclosing it is public
    impl Breakfast {
        // Function inside the method would be private, needs pub
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                // Both toast and seasonal_fruit are visible because enclosed in struct
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
pub fn eat_at_restaurant2() {
    // Breakfast (struct method) and summer (internal function) are declared public
    let mut meal = back_of_house2::Breakfast::summer("Rye");
    // struct variable was declared public and mutable, can be assigned new value
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Will be error because struct variable private by default
    // Error: field `seasonal_fruit` of struct `Breakfast` is private
    // meal.seasonal_fruit = String::from("blueberries");
}

//
// Example enum different, all enum field public if enum is public
//
mod back_of_house3 {
    pub enum Appetizer {
        // These are public
        Soup,
        Salad,
    }
}
// Enum fields are accessable
#[allow(unused_variables)]
pub fn eat_at_restaurant3() {
    let order1 = back_of_house3::Appetizer::Soup;
    let order2 = back_of_house3::Appetizer::Salad;
}
