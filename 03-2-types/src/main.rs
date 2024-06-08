fn main() {
    let x = 2.0;
    let y: f32 = 3.0;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup); // {:?} is a debug print
    let (a, b, c) = tup;
    println!("The value of a is: {}", a);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let arr = [1, 2, 3, 4, 5];
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr = [3; 5]; // [3, 3, 3, 3, 3]

    let first = arr[0];
    let second = arr[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);
    
}
