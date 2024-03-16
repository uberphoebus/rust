fn main() {

    let mut x = 5;
    println!("value x = {x}");
    x = 6;
    println!("value x = {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("const {THREE_HOURS_IN_SECONDS}");

    let a = 5;
    let a = a + 1;
    println!("a 1 = {a}");
    {
        let a = a * 2;
        println!("a 2 = {a}");
    }
    println!("a 3 = {a}");

    let spaces = "     ";
    let spaces = spaces.len();
    println!("spaces = {spaces}");
}
