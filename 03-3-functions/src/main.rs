fn main() {
    println!("\nHello, world!");
    another_function(5, 'h');
}

fn another_function(x: i32, unit_label: char) {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of unit_label is: {}", unit_label);
}
