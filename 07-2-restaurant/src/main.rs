use rand::Rng;
use std::collections::HashMap;
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
// use std::io::{self, Write};

// use std::collections::*;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("{:?}", map);

    let secret_number = rand::thread_rng().gen_range(1..=101);
    println!("The secret number is: {}", secret_number);
}
