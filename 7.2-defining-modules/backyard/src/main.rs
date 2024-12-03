use crate::garden::vegetables::Asparagus;

// statement to include code src/garden.rs
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
    // Printed: I'm growing Asparagus!
}
