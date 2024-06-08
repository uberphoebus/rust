use crate::garden::vegetables::Asparagus;

pub mod garden; // This line is added to import the garden module.

fn main() {
    let plant = Asparagus {};
    println!("Growing {:?}", plant);
}
