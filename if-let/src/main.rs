#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("the maximum is config to be {}", max),
        _ => (),
    }
    if let Some(max) = config_max {
        println!("the maximum is config to be {}", max);
    }

    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("state quarter from {:?}!", state),
        _ => count += 1,
    }

    let state = UsState::Alabama;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("state quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
