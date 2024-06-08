use std::any::type_name;

fn main() {
    // declare a new string
    let mut s = String::new();
    let data = "initial contents"; // str literal
    println!("{}", data);
    let s = data.to_string();
    // let s = "initial contents".to_string();
    println!("{}", s);
    let s = String::from("initial contents");

    // update string
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("{}", s2);

    // concatenate strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    println!("{}", s3);

    // format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // indexing into strings
    let s1 = String::from("hello");
    // let h = s1[0]; // error: cannot index into a string

    // slicing strings
    let hello = "hello";
    let s = &hello[0..1];
    println!("{}", s);

    // iterating over strings
    for c in "hello".chars() {
        println!("{}", c);
    }

    for b in "hello".bytes() {
        println!("{}", b);
    }
}
