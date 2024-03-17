fn main() {
    let s = String::from("hello");
    let hello = &s[0..5];
    let world = &s[6..10];
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}