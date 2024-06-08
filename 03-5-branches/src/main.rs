fn main() {

    //////////////////////////////////////////////////
    // if
    //////////////////////////////////////////////////
    //////////////////////////////////////////////////
    let number = 3;
    if number < 5 {
        println!("Condition was true.");
    } else {
        println!("Condition was false.");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("Number is divisible by 4.");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3.");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2.");
    } else {
        println!("Number is not divisible by 4, 3, or 2.");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);


    //////////////////////////////////////////////////
    // loop
    //////////////////////////////////////////////////
    //////////////////////////////////////////////////
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End: count = {}", count);


    //////////////////////////////////////////////////
    // while
    //////////////////////////////////////////////////
    //////////////////////////////////////////////////
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    //////////////////////////////////////////////////
    // for
    //////////////////////////////////////////////////
    //////////////////////////////////////////////////
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("The result is: {}", result);
}
