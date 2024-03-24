fn main() {
    let mut s1 = String::new();
    let data = "initial contents";
    let s1 = data.to_string();
    let s2 = "initial contents".to_string();
    let s3 = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 = {s2}");

let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;
    println!("s3 = {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = format!("{s1}-{s2}-{s3}");
    println!("s4 = {s4}");

    // let s1 = String::from("hello");
    // let h = s1[0];

let hello = "Здравствуйте";
let s = &hello[0..4];

for c in "Зд".chars() {
    println!("{c}");
}

for b in "Зд".bytes() {
    println!("{b}");
}

}
