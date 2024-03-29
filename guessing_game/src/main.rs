use std::io; // standard lib
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let x = 5;
    let y = 10;
    println!("x = {x} and y + 2 = {}", y + 2);

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {

        println!("Please input your guess.");
        
        let _apples = 5; // immutable
        let mut guess = String::new(); // variable
        
        io::stdin()
            .read_line(&mut guess) // reference
            .expect("Failed to read line"); // result instacne -> expect method
    
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}"); // placeholder
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
