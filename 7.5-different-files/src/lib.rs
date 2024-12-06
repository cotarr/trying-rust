
// Start with listing 7-11 from book, it compiles
// Move module1 in next 5 lines to separate file
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// Add module declaration for external file
mod front_of_house;

pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
