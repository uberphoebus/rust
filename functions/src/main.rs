fn main() {
    println!("Hello, world!");

    another_function(5, 'p');

    let y = 6;
    // let x = (let y = 6);
    let y = {
        let x = 3;
        x + 1
    };
    println!("{y}");

    let x = five();
    println!("{x}");
}

fn another_function(x: i32, c: char) {
    println!("Another function.");
    println!("x = {x}");
    println!("c = {c}");
}

fn five() -> i32 {
    5
}