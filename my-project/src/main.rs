use crate::garden::vegetables::Asparagus;

pub mod garden; // src/garden.rs 포함

fn main() {
    let plant = Asparagus {};
    println!("i'm growing {:?}!", plant);
}
