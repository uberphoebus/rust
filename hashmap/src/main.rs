use std::collections::HashMap;

fn main() {

    let mut scores1 = HashMap::new();
    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score1 = scores1.get(&team_name).copied().unwrap_or(0);

    println!("score1 = {score1}");
    for (key, value) in &scores1 {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // update
    let mut scores2 = HashMap::new();

    scores2.insert(String::from("Blue"), 10);
    scores2.insert(String::from("Blue"), 25);
    scores2.entry(String::from("Yellow")).or_insert(50);
    scores2.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores2);


let text = "hello world wonderful world";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
println!("{:?}", map);

}
