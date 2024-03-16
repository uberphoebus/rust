fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let second = tup.1;
    println!("y = {second}");

    let a = [1, 2, 3, 4, 5];
    let b = [3; 5];
    
    let first = a[0];
    let last = a[4];
    println!("first = {first}");
    println!("last = {last}");
}