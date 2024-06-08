fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => {
            println!("max: {}", max);
        }
        _ => (),
    }

    // if let
    if let Some(max) = config_max {
        println!("max: {}", max);
    }

    // if let with else
    let mut count = 0;
    if let tmp = count {
        println!("count: {}", tmp);
    } else {
        println!("count is not set");
    }
}
