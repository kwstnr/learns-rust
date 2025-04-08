use crate::garden::vegetables::Asparagus;

pub mod garden; // looks for andn includes code found in garden.rs

fn main() {
    let plant = Asparagus {}; // Create an instance of the Asparagus struct
    // The Asparagus struct is a custom data type defined in the vegetables module
    // so it's found under the garden module, which includes the vegetables module and has the public struct Asparagus
    println!("I'm growing {plant:?}!");
}