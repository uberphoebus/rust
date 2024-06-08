use std::collections::HashMap;

fn main() {
    // declare a new hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    println!("team_name: {}", team_name);
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score: {:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // overwrite a value
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // only insert if the key has no value
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Green")).or_insert(50);
    println!("{:?}", scores);

    // update a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
